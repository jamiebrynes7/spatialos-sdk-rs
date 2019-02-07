//! Utilities for handling raw pointers.

/// Wrapper around a raw pointer that ensures the pointer can only be accessed
/// through `&mut self`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutPtr<T> {
    ptr: *mut T,
}

impl<T> MutPtr<T> {
    pub fn new(ptr: *mut T) -> Self {
        MutPtr { ptr }
    }

    /// Returns the raw pointer.
    pub fn get(&mut self) -> *mut T {
        self.ptr
    }

    /// Returns `true` if the pointer is null.
    ///
    /// Note that this only requires `&self` since it never dereferences the pointer. See the
    /// primitive [`is_null`] method for more information about the general behavior of pointer
    /// null checks.
    ///
    /// [`is_null`]: https://doc.rust-lang.org/std/primitive.pointer.html
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}