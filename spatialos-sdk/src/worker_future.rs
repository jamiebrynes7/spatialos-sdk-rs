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

pub trait WorkerSdkFuture: Send + Unpin + 'static {
    type RawPointer;
    type Output: Send;

    fn start(&mut self) -> *mut Self::RawPointer;

    /// This method corresponds to the Worker_{Type}_Get C API call which _blocks_
    /// until the future returns.
    ///
    /// # Safety
    ///
    /// This method should only be called once. Calling it more than once is an error.
    unsafe fn get(&mut self, ptr: *mut Self::RawPointer) -> Self::Output;

    /// This method corresponds to the Worker_{Type}_Destroy C API call which cancels
    /// and disposes the native future.
    ///
    /// # Safety
    /// This method should only be called once. Calling it more than once is an error.
    unsafe fn destroy(&mut self, ptr: *mut Self::RawPointer);
}

impl<T: WorkerSdkFuture> Future for WorkerFuture<T> {
    type Output = T::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = &mut self.as_mut().get_mut().inner;
        // This panics if the future is polled after completion. This is acceptable according to
        // the Future trait contract - https://doc.rust-lang.org/std/future/trait.Future.html#panics
        let mut future_state = inner.take().unwrap();

        match future_state {
            WorkerFutureState::NotStarted(mut ftr) => {
                let (tx, rx) = futures::channel::oneshot::channel();
                let task = cx.waker().clone();

                let thread = thread::spawn(move || unsafe {
                    let ptr = ftr.start();
                    let value = ftr.get(ptr);
                    tx.send(value).unwrap_or(());
                    task.wake();
                    ftr.destroy(ptr);
                });

                let handle = InProgressHandle::<T> {
                    result_rx: rx,
                    thread: Some(thread),
                };

                *inner = Some(WorkerFutureState::InProgress(handle));
            }
            WorkerFutureState::InProgress(ref mut handle) => {
                if let Some(val) = handle.result_rx.try_recv().unwrap_or(None) {
                    return Poll::Ready(val);
                }

                *inner = Some(future_state);
            }
        }

        Poll::Pending
    }
}
