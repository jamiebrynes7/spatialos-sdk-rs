//! Utilities for handling raw pointers.

/// Wrapper around a raw pointer that ensures the pointer can only be accessed
/// through a mutable reference.
///
/// Some types in the C API can be safely shared between threads as long as they are
/// only accessed by one thread at a time. We can enforce this invariant in Rust
/// without a `Mutex` by requiring that the data be accessed only through a mutable
/// reference. However, a `*mut T` doesn't follow Rust's usual ownership rules, and
/// can still be used through a shared reference.
///
/// `MutPtr` wraps wraps a raw pointer and only allows the raw value to be accessed
/// through a mutable reference, statically ensuring that the thread-safety
/// requirements aren't violated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MutPtr<T> {
    ptr: *mut T,
}

impl<T> MutPtr<T> {
    /// Creates a new `MutPtr` wrapping `ptr`.
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
