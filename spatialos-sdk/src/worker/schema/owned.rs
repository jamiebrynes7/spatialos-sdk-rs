//! A pointer type for owned schema data types.
//!
//! When you own an instance of some schema data type, such as
//! [`ComponentData`], the [`Owned`] smart pointer handles automatically
//! destroying the object when it goes out of scope. In this way, it behaves like
//! [`Box`] for SpatialOS-specific types.
//!
//! You cannot directly create an `Owned<T>`, instead each type that can be owned
//! directly will provide its own type-appropriate constructors. See
//! [`ComponentData::new`] for an example.
//!
//! [`ComponentData`]: ../struct.ComponentData.html
//! [`ComponentData::new`]: ../struct.ComponentData.html#method.new
//! [`Owned`]: struct.Owned.html
//! [`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html

use std::{
    mem,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

pub(crate) use self::private::OwnableImpl;

mod private {
    use crate::worker::schema::PointerType;

    /// Private imlementation of the `Ownable` trait. Used to hide implementation
    /// details and seal the trait from downstream implementations.
    pub unsafe trait OwnableImpl: PointerType {
        const CREATE_FN: unsafe extern "C" fn() -> *mut Self::Raw;
        const DESTROY_FN: unsafe extern "C" fn(*mut Self::Raw);
    }
}

pub trait Ownable: OwnableImpl {}

impl<T> Ownable for T where T: OwnableImpl {}

/// Like [`Box`], but for SpatialOS schema types.
///
/// See the [module-level documentation](index.html) for more.
///
/// [`Box`]: https://doc.rust-lang.org/std/boxed/struct.Box.html
#[derive(Debug)]
pub struct Owned<T: Ownable>(NonNull<T::Raw>);

impl<T: Ownable> Owned<T> {
    pub fn new() -> Self {
        let raw = unsafe { T::CREATE_FN() };
        Self(NonNull::new(raw).expect("Cannot create `Owned` from null pointer"))
    }

    /// Converts an owned piece of schema data back into the raw type without dropping it.
    ///
    /// This transfers ownership of the data to the caller, so the caller needs to
    /// ensure that the appropriate steps are taken to free the data. If the raw data is
    /// passed to the C API, the C SDK will take ownership of the data and will free it
    /// when it's done.
    pub(crate) fn into_raw(self) -> *mut T::Raw {
        let raw = self.0.as_ptr();
        mem::forget(self);
        raw
    }
}

impl<T: Ownable> Drop for Owned<T> {
    fn drop(&mut self) {
        unsafe {
            T::DESTROY_FN(self.0.as_ptr());
        }
    }
}

impl<T: Ownable> Deref for Owned<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.0.cast().as_ptr() }
    }
}

impl<T: Ownable> DerefMut for Owned<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0.cast().as_ptr() }
    }
}

unsafe impl<T: Ownable + Send> Send for Owned<T> {}
