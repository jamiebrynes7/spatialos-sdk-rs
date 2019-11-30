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

struct SendPtr<T> {
    pub ptr: *mut T,
}

unsafe impl<T> Send for SendPtr<T> {}

pub trait WorkerSdkFuture: Unpin + Send {
    type RawPointer;
    type Output: Send;

    fn start(&self) -> *mut Self::RawPointer;
    unsafe fn get(ptr: *mut Self::RawPointer) -> Self::Output;
    unsafe fn destroy(ptr: *mut Self::RawPointer);
}

impl<T: WorkerSdkFuture + 'static> Future for WorkerFuture<T> {
    type Output = T::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut next_state = None;
        let mut result = Poll::Pending;

        match &mut self.as_mut().get_mut().inner {
            WorkerFutureState::NotStarted(future) => {
                let (tx, rx) = futures::channel::oneshot::channel();
                let thread_waker = cx.waker().clone();
                let send_ptr = SendPtr {
                    ptr: future.start(),
                };

                let thread = thread::spawn(move || unsafe {
                    let value = T::get(send_ptr.ptr);
                    tx.send(value).unwrap_or(());
                    thread_waker.wake();
                    T::destroy(send_ptr.ptr)
                });

                next_state = Some(WorkerFutureState::InProgress(InProgressHandle {
                    result_rx: rx,
                    thread: Some(thread),
                }));
            }
            WorkerFutureState::InProgress(context) => {
                if let Some(val) = context.result_rx.try_recv().unwrap_or(None) {
                    result = Poll::Ready(val)
                }
            }
        }

        if let Some(ref mut next) = next_state {
            std::mem::swap(&mut self.as_mut().get_mut().inner, next);
        }

        result
    }
}
