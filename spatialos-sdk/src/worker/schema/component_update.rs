use crate::worker::schema::{owned::OwnableImpl, Owned, SchemaObject};
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct SchemaComponentUpdate(PhantomData<*mut Schema_ComponentUpdate>);

impl SchemaComponentUpdate {
    pub fn new() -> Owned<SchemaComponentUpdate> {
        unsafe { Owned::new(Schema_CreateComponentUpdate()) }
    }

    pub fn fields(&self) -> &SchemaObject {
        unsafe { SchemaObject::from_raw(Schema_GetComponentUpdateFields(self.as_ptr())) }
    }

    pub fn fields_mut(&mut self) -> &mut SchemaObject {
        unsafe { SchemaObject::from_raw_mut(Schema_GetComponentUpdateFields(self.as_ptr())) }
    }

    pub fn events(&self) -> &SchemaObject {
        unsafe { SchemaObject::from_raw(Schema_GetComponentUpdateEvents(self.as_ptr())) }
    }

    pub fn events_mut(&mut self) -> &mut SchemaObject {
        unsafe { SchemaObject::from_raw_mut(Schema_GetComponentUpdateEvents(self.as_ptr())) }
    }

    // TODO: Cleared fields.

    // Methods for raw pointer conversion.
    // -----------------------------------

    pub(crate) unsafe fn from_raw<'a>(raw: *mut Schema_ComponentUpdate) -> &'a Self {
        &*(raw as *mut _)
    }

    pub(crate) fn as_ptr(&self) -> *mut Schema_ComponentUpdate {
        self as *const _ as *mut _
    }
}

impl OwnableImpl for SchemaComponentUpdate {
    type Raw = Schema_ComponentUpdate;

    unsafe fn destroy(me: *mut Self::Raw) {
        Schema_DestroyComponentUpdate(me);
    }
}
