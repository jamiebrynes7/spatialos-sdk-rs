//! A type-erased [`Arc`], used to pass user-defined data types to the C API.
//!
//! A common pattern with the C SDK in order to move serialization off the main
//! thread is to pass a pointer to some user-defined data to the C API, along with
//! a vtable of serialization functions, and allow the C SDK to perform
//! serialization automatically. In order to make use of this approach, we need a
//! type-erased, thread-safe, reference counted pointer type; We need an [`Arc`],
//! but type-erased.
//!
//! [`UserHandle`] provides this functionality, providing a safe way to create
//! type-erased handles that are automatically freed when dropped.
//!
//! [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
//! [`UserHandle`]: struct.UserHandle.html

use std::{mem, os::raw, sync::Arc};

pub type RawHandle = *mut raw::c_void;

/// Type-erased [`Arc`], used to pass user-defined data types to the C API.
///
/// See the [module-level documentation](index.html) for more.
///
/// [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct UserHandle {
    raw: RawHandle,
    clone_fn: unsafe fn(RawHandle) -> RawHandle,
    drop_fn: unsafe fn(RawHandle),
}

impl UserHandle {
    /// Creates a new type-erased handle from the specified data.
    pub fn new<T>(data: T) -> Self {
        Self {
            raw: allocate_raw(data),
            clone_fn: clone_raw::<T>,
            drop_fn: drop_raw::<T>,
        }
    }

    /// Reconstructs a handle from a raw pointer.
    ///
    /// # Safety
    ///
    /// This function must be called with the correct type `T` for the specified handle.
    /// Failing to do so will cause the data to be treated as the wrong type, which will
    /// result in undefined behavior.
    pub unsafe fn from_raw<T>(raw: RawHandle) -> Self {
        Self {
            raw,
            clone_fn: clone_raw::<T>,
            drop_fn: drop_raw::<T>,
        }
    }

    /// Returns the raw version of the handle.
    ///
    /// This method doesn't consume the handle, and it will still be dropped when it
    /// goes out of scope. If this is the last instance of this handle, that means the
    /// handle's data will be freed. As such, the returned pointer is only safe to be
    /// used while the full handle (or another handle to the same data) is still in
    /// scope.
    ///
    /// When passing user handles to the C API functions like
    /// `Worker_Connection_SendCreateEntityRequest`, the C API will make a clone of the
    /// handle. This means it should be safe to drop the handle once the C API has had
    /// a chance to make its own copy.
    pub fn raw(&self) -> RawHandle {
        self.raw
    }
}

impl Clone for UserHandle {
    fn clone(&self) -> Self {
        let raw = unsafe { (self.clone_fn)(self.raw) };
        Self {
            raw,
            clone_fn: self.clone_fn,
            drop_fn: self.drop_fn,
        }
    }
}

impl Drop for UserHandle {
    fn drop(&mut self) {
        unsafe {
            (self.drop_fn)(self.raw);
        }
    }
}

pub fn allocate_raw<T>(data: T) -> RawHandle {
    Arc::into_raw(Arc::new(data)) as *mut _
}

pub unsafe fn drop_raw<T>(handle: RawHandle) {
    let _ = Arc::<T>::from_raw(handle as *const _);
}

pub unsafe fn clone_raw<T>(handle: RawHandle) -> RawHandle {
    let original = Arc::<T>::from_raw(handle as *const _);
    let copy = original.clone();
    mem::forget(original);
    Arc::into_raw(copy) as *mut _
}
