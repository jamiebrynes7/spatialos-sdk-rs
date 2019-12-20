use futures::channel::oneshot::*;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    thread,
    thread::JoinHandle,
};

pub struct WorkerFuture<T: WorkerSdkFuture> {
    inner: WorkerFutureState<T>,
}

impl<T: WorkerSdkFuture> WorkerFuture<T> {
    pub fn new(future: T) -> Self {
        WorkerFuture {
            inner: WorkerFutureState::NotStarted(future),
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
        if let WorkerFutureState::InProgress(handle) = &mut self.inner {
            handle.thread.take().unwrap().join().unwrap();
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
        let future_state = &mut self.as_mut().get_mut().inner;

        match future_state {
            // We don't want to borrow the WorkerSdkFuture object. We want to take it so we can
            // send it to the thread which will block on the Worker SDK future.
            // To do this, we construct the next state and then mem swap them.
            // We do have to add the thread handle _after_ we start the thread though,
            // so it gets a little awkward.
            // Rust's enum destructing lets us down a little bit here.
            WorkerFutureState::NotStarted(_) => {
                let (tx, rx) = futures::channel::oneshot::channel();
                let task = cx.waker().clone();

                let mut state = WorkerFutureState::InProgress(InProgressHandle::<T> {
                    result_rx: rx,
                    thread: None,
                });

                std::mem::swap(future_state, &mut state);

                // This is always true.
                if let WorkerFutureState::NotStarted(ftr) = state {
                    let thread = thread::spawn(move || unsafe {
                        let ptr = ftr.start();
                        let value = T::get(ptr);
                        tx.send(value).unwrap_or(());
                        task.wake();
                        T::destroy(ptr)
                    });

                    // As is this one.
                    if let WorkerFutureState::InProgress(handle) = future_state {
                        handle.thread = Some(thread);
                    }
                }

                Poll::Pending
            }
            WorkerFutureState::InProgress(handle) => {
                if let Some(val) = handle.result_rx.try_recv().unwrap_or(None) {
                    return Poll::Ready(val);
                }

                Poll::Pending
            }
        }
    }
}
