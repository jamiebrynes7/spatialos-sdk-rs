use std::{
    mem,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

pub trait TypeWrapper {
    type Raw;

    unsafe fn destroy(me: *mut Self::Raw);
}

/// Like `Box`, but for SpatialOS schema types.
#[derive(Debug)]
pub struct Owned<T: TypeWrapper>(NonNull<T::Raw>);

impl<T: TypeWrapper> Owned<T> {
    pub unsafe fn new(raw: *mut T::Raw) -> Self {
        Self(NonNull::new(raw).expect("Cannot create `Owned` from null pointer"))
    }

    /// Converts an owned piece of schema data back into the raw type without dropping it.
    ///
    /// This transfers ownership of the data to the caller, so the caller needs to
    /// ensure that the appropriate steps are taken to free the data. If the raw data is
    /// passed to the C API, the C SDK will take ownership of the data and will free it
    /// when it's done.
    pub fn into_raw(self) -> *mut T::Raw {
        let raw = self.0.as_ptr();
        mem::forget(self);
        raw
    }
}

impl<T: TypeWrapper> Drop for Owned<T> {
    fn drop(&mut self) {
        unsafe {
            T::destroy(self.0.as_ptr());
        }
    }
}

impl<T: TypeWrapper> Deref for Owned<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.0.cast().as_ptr() }
    }
}

impl<T: TypeWrapper> DerefMut for Owned<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0.cast().as_ptr() }
    }
}
