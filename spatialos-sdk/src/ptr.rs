//! Utilities for handling raw pointers.

use derivative::Derivative;
use std::cmp::Ordering;

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
///
/// # Safety
///
/// This wrapper doesn't enforce any safety guarantees. Notably:
///
/// * It doesn't require that the wrapped pointer be non-null.
/// * [`get`] directly returns a copy of the raw pointer, which can then be copied
///   and used in unsafe ways.
/// * The raw pointer can be copied before creating the `MutPtr` wrapper, and the
///   copy can still be used in an unsafe way.
///
/// `MutPtr` only acts as a helper to avoid accidentally using the pointer through
/// a shared reference, but any code using it must still take care to correctly
/// enforce safety invariants.
///
/// [`get`]: #method.get
#[derive(Derivative)]
#[derivative(
    Debug(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Hash(bound = "")
)]
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

impl<T> Ord for MutPtr<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ptr.cmp(&other.ptr)
    }
}

impl<T> PartialOrd for MutPtr<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
