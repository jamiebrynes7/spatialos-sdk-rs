use crate::worker::schema::{owned::OwnableImpl, Owned, SchemaObject};
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct SchemaCommandRequest(PhantomData<*mut Schema_CommandRequest>);

impl SchemaCommandRequest {
    pub fn new() -> Owned<SchemaCommandRequest> {
        unsafe { Owned::new(Schema_CreateCommandRequest()) }
    }

    pub fn object(&self) -> &SchemaObject {
        unsafe { SchemaObject::from_raw(Schema_GetCommandRequestObject(self.as_ptr())) }
    }

    pub fn object_mut(&mut self) -> &mut SchemaObject {
        unsafe { SchemaObject::from_raw_mut(Schema_GetCommandRequestObject(self.as_ptr())) }
    }

    // Methods for raw pointer conversion.
    // -----------------------------------

    pub(crate) unsafe fn from_raw<'a>(raw: *mut Schema_CommandRequest) -> &'a Self {
        &*(raw as *mut _)
    }

    pub(crate) fn as_ptr(&self) -> *mut Schema_CommandRequest {
        self as *const _ as *mut _
    }
}

impl OwnableImpl for SchemaCommandRequest {
    type Raw = Schema_CommandRequest;

    unsafe fn destroy(me: *mut Self::Raw) {
        Schema_DestroyCommandRequest(me);
    }
}

// SAFETY: It should be safe to send a `SchemaCommandRequest` between threads, so long as
// it's only ever accessed from one thread at a time. It has unsynchronized internal
// mutability (when getting an object field, it will automatically add a new object
// if one doesn't already exist), so it cannot be `Sync`.
unsafe impl Send for SchemaCommandRequest {}

#[cfg(test)]
mod tests {
    use super::SchemaCommandRequest;
    use static_assertions::*;

    assert_impl_all!(SchemaCommandRequest: Send);
    assert_not_impl_any!(SchemaCommandRequest: Sync);
}
