//! A type-erased [`Arc`], used to pass user-defined data types to the C API.
//!
//! A common pattern with the C SDK in order to move serialization off the main
//! thread is to pass a pointer to some user-defined data to the C API, along with
//! a vtable of serialization functions, and allow the C SDK to perform
//! serialization automatically. In order to make use of this approach, we need a
//! type-erased, thread-safe, reference counted pointer type; We need an [`Arc`],
//! but type-erased. [`UserHandle`] provides this functionality, providing a safe
//! way to create type-erased handles that are automatically freed when dropped.
//!
//! This module also provides a [`RawHandle`] type definition and convenience
//! methods for working with raw handles. These are less safe to work with than
//! [`UserHandle`], but are necessary when communicating with the C API. Where
//! possible, prefer to use [`UserHandle`], and only use [`RawHandle`] for FFI.
//!
//! [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
//! [`UserHandle`]: struct.UserHandle.html

use crate::worker::schema;
use std::{any::Any, mem, os::raw, sync::Arc};

/// Raw pointer to data contained in a [`UserHandle`].
///
/// [`UserHandle`]: struct.UserHandle.html
pub type RawHandle = *mut raw::c_void;

/// Type-erased handle to user-provided data.
pub type UserHandle = Arc<dyn Any>;

pub fn new<T: 'static>(data: schema::Result<T>) -> UserHandle {
    Arc::new(data) as _
}

/// Returns the raw handle for the given user handle.
pub fn get_raw(handle: &UserHandle) -> RawHandle {
    &**handle as *const _ as *mut _
}

/// Directly allocates a raw handle for `data`.
// TODO: Write compile tests for the `'static'` bound.
pub fn allocate_raw<T: 'static>(data: schema::Result<T>) -> RawHandle {
    Arc::into_raw(Arc::new(data)) as *mut _
}

/// Directly drops a raw handle.
///
/// This is functionally equivalent to using [`UserHandle::from_raw`] to reconstruct
/// the handle, then immediately dropping it.
///
/// # Safety
///
/// This function must be called with the correct type `T` for the specified handle.
/// Failing to do so will cause the data to be treated as the wrong type, which will
/// result in undefined behavior.
///
/// [`UserHandle::from_raw`]: struct.UserHandle.html#method.from_raw
pub unsafe fn drop_raw<T: 'static>(handle: RawHandle) {
    let _ = Arc::<schema::Result<T>>::from_raw(handle as *const _);
}

/// Directly clones a raw handle.
///
/// This function reconstructs the raw handle, clones it, and returns the new raw
/// handle. As such, it increases the ref count for the handle by one.
///
/// # Safety
///
/// This function must be called with the correct type `T` for the specified handle.
/// Failing to do so will cause the data to be treated as the wrong type, which will
/// result in undefined behavior.
pub unsafe fn clone_raw<T: 'static>(handle: RawHandle) -> RawHandle {
    let original = Arc::<schema::Result<T>>::from_raw(handle as *const _);
    let copy = original.clone();
    mem::forget(original);
    Arc::into_raw(copy) as *mut _
}

/// Dereferences a `RawHandle` into the appropriate Rust type.
///
/// This is a helper function for working with raw handles within the vtable
/// functions. In particular, the serialization functions are given a raw handle
/// that they must dereference as the correct Rust type. Using this method instead
/// of casting/dereferencing the raw pointer directly helps ensure that we keep our
/// type definitions lined up.
///
/// # Safety
///
/// * This function must be called with the correct type `T` for the specified handle.
/// * The pointer passed in for `handle` must have been created with either `new` or
///   `allocate_raw`, i.e. it must be a valid user handle.
pub unsafe fn deref_raw<'a, T: 'static>(handle: RawHandle) -> &'a schema::Result<T> {
    &*handle.cast()
}
