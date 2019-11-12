pub(crate) mod utils;

use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;

pub enum WorkerFuture<T : WorkerSdkFuture + Unpin> {
    NotStarted(T),
    InProgress(*mut T::RawPointer),
    Done
}

pub trait WorkerSdkFuture {
    type RawPointer;
    type Output;

    fn start(&self) -> *mut Self::RawPointer;
    fn poll(ptr: *mut Self::RawPointer) -> Option<Self::Output>;
    fn destroy(ptr: *mut Self::RawPointer);
}

impl<T: WorkerSdkFuture + Unpin> Future for WorkerFuture<T> {
    type Output = T::Output;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut result = Poll::Pending;
        let mut next = None;

        match self.as_mut().get_mut() {
            WorkerFuture::NotStarted(future) => {
                let ptr= future.start();
                next = Some(WorkerFuture::InProgress(ptr));
            },
            WorkerFuture::InProgress(ptr) => {
                if let Some(value) = T::poll(*ptr) {
                    result = Poll::Ready(value);
                    next = Some(WorkerFuture::Done);
                }
            },
            WorkerFuture::Done => panic!("Future already completed".to_string())
        }

        if let Some(ref mut next) = next {
            std::mem::swap(self.as_mut().get_mut(), next);
        }

        result
    }
}

impl<T: WorkerSdkFuture + Unpin> Drop for WorkerFuture<T> {
    fn drop(&mut self) {
        if let WorkerFuture::InProgress(ptr) = self {
            T::destroy(*ptr);
        }
    }
}