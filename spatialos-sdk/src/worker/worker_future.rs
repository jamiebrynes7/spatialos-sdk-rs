use futures::channel::oneshot::*;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    thread,
    thread::JoinHandle,
};

pub struct WorkerFuture<T: WorkerSdkFuture> {
    inner: Option<WorkerFutureState<T>>,
}

impl<T: WorkerSdkFuture> WorkerFuture<T> {
    pub fn new(future: T) -> Self {
        WorkerFuture {
            inner: Some(WorkerFutureState::NotStarted(future)),
        }
    }
}

enum WorkerFutureState<T: WorkerSdkFuture> {
    NotStarted(T),
    InProgress(InProgressHandle<T>),
}

struct InProgressHandle<T: WorkerSdkFuture> {
    pub result_rx: Receiver<T::Output>,
    pub thread: Option<JoinHandle<()>>,
}

impl<T: WorkerSdkFuture> Drop for WorkerFuture<T> {
    fn drop(&mut self) {
        if let Some(future_state) = &mut self.inner {
            if let WorkerFutureState::InProgress(handle) = future_state {
                handle.thread.take().unwrap().join().unwrap();
            }
        }
    }
}

pub trait WorkerSdkFuture: Send + Unpin {
    type RawPointer;
    type Output: Send;

    fn start(&self) -> *mut Self::RawPointer;
    unsafe fn get(ptr: *mut Self::RawPointer) -> Self::Output;
    unsafe fn destroy(ptr: *mut Self::RawPointer);
}

impl<T: WorkerSdkFuture + 'static> Future for WorkerFuture<T> {
    type Output = T::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = &mut self.as_mut().get_mut().inner;
        // This panics if the future is polled after completion. This is acceptable according to
        // the Future trait contract - https://doc.rust-lang.org/std/future/trait.Future.html#panics
        let mut future_state = inner.take().unwrap();

        match future_state {
            WorkerFutureState::NotStarted(ftr) => {
                let (tx, rx) = futures::channel::oneshot::channel();
                let task = cx.waker().clone();

                let thread = thread::spawn(move || unsafe {
                    let ptr = ftr.start();
                    let value = T::get(ptr);
                    tx.send(value).unwrap_or(());
                    task.wake();
                    T::destroy(ptr)
                });

                let handle = InProgressHandle::<T> {
                    result_rx: rx,
                    thread: Some(thread),
                };

                *inner = Some(WorkerFutureState::InProgress(handle));
                Poll::Pending
            }
            WorkerFutureState::InProgress(ref mut handle) => {
                if let Some(val) = handle.result_rx.try_recv().unwrap_or(None) {
                    return Poll::Ready(val);
                }

                *inner = Some(future_state);
                Poll::Pending
            }
        }
    }
}
