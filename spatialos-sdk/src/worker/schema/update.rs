use crate::worker::schema::{FieldId, FieldUpdate, Object};
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

pub struct Update(PhantomData<*mut Schema_Object>);

impl Update {
    /// Converts a raw `Schema_Object` pointer to a `&Update`.
    ///
    /// # Safety
    ///
    /// * `raw` must be a valid pointer to a schema object created by the C SDK.
    /// * `raw` must have come from a `Schema_ComponentUpdate`. Any other `Schema_Object`
    ///   pointer is not safe to treat as an `Update`.
    /// * The lifetime `'a` must be setup correctly to tie the data to its owner.
    pub(crate) unsafe fn from_raw<'a>(raw: *const Schema_Object) -> &'a Self {
        &*(raw as *const _)
    }

    pub(crate) unsafe fn from_raw_mut<'a>(raw: *mut Schema_Object) -> &'a mut Self {
        &mut *(raw as *mut _)
    }

    pub(crate) fn as_object(&self) -> &Object {
        unsafe { &*(self as *const _ as *const _) }
    }

    pub(crate) fn as_object_mut(&mut self) -> &mut Object {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }

    pub fn field<T: FieldUpdate>(&self, field: FieldId) -> Option<T::RustType> {
        T::get_update(self, field)
    }
}

unsafe impl Send for Update {}
