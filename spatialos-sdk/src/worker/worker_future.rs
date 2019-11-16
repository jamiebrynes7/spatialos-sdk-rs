use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;
use std::thread::JoinHandle;

pub enum WorkerFuture<T: WorkerSdkFuture> {
    NotStarted(T),
    InProgress(WorkerFutureHandle<T>),
}

pub struct WorkerFutureHandle<T: WorkerSdkFuture> {
    pub(crate) ptr: *mut T::RawPointer,
    pub(crate) shared_result: Arc<Mutex<Option<T::Output>>>,
    pub(crate) thread: Option<JoinHandle<()>>
}

impl<T: WorkerSdkFuture> Clone for WorkerFutureHandle<T> {
    fn clone(&self) -> Self {
        WorkerFutureHandle {
            ptr: self.ptr,
            shared_result: self.shared_result.clone(),
            thread: None
        }
    }
}

// SAFE: It should be safe to send WorkerFutureHandle<T> between threads given the following
// conditions around *mut T::RawPointer.
//
// First, some context. For the Worker SDK futures, there are only two operations you can
// perform with this pointer:
//
//  * Poll (blocking or with timeout)
//  * Destroy
//
// Given the following rules, we can guarantee that its thread-safe:
//
//  1. Poll cannot be called after Destroy.
//
// We ensure this cannot be true as we store a reference to the handle of the thread where Poll
// could be running. If this reference is present in the handle during drop(), we join that thread.
// This guarantees us that poll has already begun (and then finished) by the time Destroy is called.
// Since we only ever call Poll once, we cannot possibly call Poll after Destroy.
//
// This has the side effect the the Drop impl can block, but this was the already the case anyway with the
// Worker SDK future Destroy methods (which I believe wait for the future to complete).
unsafe impl<T: WorkerSdkFuture> Send for WorkerFutureHandle<T> {}

impl<T: WorkerSdkFuture> WorkerFutureHandle<T> {
    pub(crate) fn new(ptr: *mut T::RawPointer) -> Self {
        WorkerFutureHandle {
            ptr,
            shared_result: Arc::new(Mutex::new(None)),
            thread: None,
        }
    }
}

pub trait WorkerSdkFuture : Unpin + Send {
    type RawPointer;
    type Output;

    fn start(&self) -> *mut Self::RawPointer;
    unsafe fn get(ptr: *mut Self::RawPointer) -> Self::Output;
    unsafe fn destroy(ptr: *mut Self::RawPointer);
}

impl<T: WorkerSdkFuture + 'static> Future for WorkerFuture<T> {
    type Output = T::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut next_state;

        match self.as_mut().get_mut() {
            WorkerFuture::NotStarted(future) => {
                let mut handle = WorkerFutureHandle::new(future.start());

                let thread_handle = handle.clone();
                let thread_waker = cx.waker().clone();

                handle.thread = Some(thread::spawn(move || unsafe {
                    let value = T::get(thread_handle.ptr);
                    thread_handle.shared_result.lock().unwrap().replace(value);
                    thread_waker.wake();
                }));

                next_state = Some(WorkerFuture::InProgress(handle));
            }
            WorkerFuture::InProgress(context) => {
                return Poll::Ready(context.shared_result.lock().unwrap().take().unwrap());
            }
        }

        if let Some(ref mut next) = next_state {
            std::mem::swap(self.as_mut().get_mut(), next);
        }

        Poll::Pending
    }
}

impl<T: WorkerSdkFuture> Drop for WorkerFuture<T> {
    fn drop(&mut self) {
        if let WorkerFuture::InProgress(handle) = self {
            if let Some(join_handle) = handle.thread.take() {
                join_handle.join().expect("Failed to join thread");
            }

            unsafe {
                T::destroy(handle.ptr);
            }
        }
    }
}
