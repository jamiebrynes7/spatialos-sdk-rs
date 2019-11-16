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

// SAFETY: It should be safe to send a `SchemaComponentUpdate` between threads, so long as
// it's only ever accessed from one thread at a time. It has unsynchronized internal
// mutability (when getting an object field, it will automatically add a new object
// if one doesn't already exist), so it cannot be `Sync`.
unsafe impl Send for SchemaComponentUpdate {}

#[cfg(test)]
mod tests {
    use super::SchemaComponentUpdate;
    use static_assertions::*;

    assert_impl_all!(SchemaComponentUpdate: Send);
    assert_not_impl_any!(SchemaComponentUpdate: Sync);
}
