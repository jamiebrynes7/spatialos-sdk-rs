use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;

pub enum WorkerFuture<T: WorkerSdkFuture + Unpin + Send> {
    NotStarted(T),
    InProgress(WorkerFutureHandle<T>),
}

pub struct WorkerFutureHandle<T: WorkerSdkFuture + Unpin + Send> {
    pub(crate) ptr: *mut T::RawPointer,
    pub(crate) shared_result: Arc<Mutex<Option<T::Output>>>,
}

impl<T: WorkerSdkFuture + Unpin + Send> Clone for WorkerFutureHandle<T> {
    fn clone(&self) -> Self {
        WorkerFutureHandle {
            ptr: self.ptr,
            shared_result: self.shared_result.clone(),
        }
    }
}

unsafe impl<T: WorkerSdkFuture + Unpin + Send> Send for WorkerFutureHandle<T> {}

impl<T: WorkerSdkFuture + Unpin + Send> WorkerFutureHandle<T> {
    pub(crate) fn new(ptr: *mut T::RawPointer) -> Self {
        WorkerFutureHandle {
            ptr,
            shared_result: Arc::new(Mutex::new(None)),
        }
    }
}

pub trait WorkerSdkFuture {
    type RawPointer;
    type Output;

    fn start(&self) -> *mut Self::RawPointer;
    unsafe fn get(ptr: *mut Self::RawPointer) -> Self::Output;
    unsafe fn destroy(ptr: *mut Self::RawPointer);
}

impl<T: WorkerSdkFuture + Unpin + Send + 'static> Future for WorkerFuture<T> {
    type Output = T::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut next_state;

        match self.as_mut().get_mut() {
            WorkerFuture::NotStarted(future) => {
                let handle = WorkerFutureHandle::new(future.start());

                let thread_handle = handle.clone();
                let thread_waker = cx.waker().clone();

                thread::spawn(move || unsafe {
                    let value = T::get(thread_handle.ptr);
                    thread_handle.shared_result.lock().unwrap().replace(value);
                    thread_waker.wake();
                });

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

impl<T: WorkerSdkFuture + Unpin + Send> Drop for WorkerFuture<T> {
    fn drop(&mut self) {
        if let WorkerFuture::InProgress(handle) = self {
            unsafe {
                T::destroy(handle.ptr);
            }
        }
    }
}
