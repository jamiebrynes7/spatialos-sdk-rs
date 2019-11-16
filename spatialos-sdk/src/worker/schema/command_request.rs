use crate::worker::schema::{Owned, OwnedPointer, PointerType, SchemaObject};
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct SchemaCommandRequest(PhantomData<*mut Schema_CommandRequest>);

impl SchemaCommandRequest {
    pub fn new() -> Owned<SchemaCommandRequest> {
        Owned::new()
    }

    pub fn object(&self) -> &SchemaObject {
        unsafe { SchemaObject::from_raw(Schema_GetCommandRequestObject(self.as_ptr() as *mut _)) }
    }

    pub fn object_mut(&mut self) -> &mut SchemaObject {
        unsafe { SchemaObject::from_raw_mut(Schema_GetCommandRequestObject(self.as_ptr_mut())) }
    }
}

unsafe impl PointerType for SchemaCommandRequest {
    type Raw = Schema_CommandRequest;
}

unsafe impl OwnedPointer for SchemaCommandRequest {
    const CREATE_FN: unsafe extern "C" fn() -> *mut Self::Raw = Schema_CreateCommandRequest;
    const DESTROY_FN: unsafe extern "C" fn(*mut Self::Raw) = Schema_DestroyCommandRequest;
}

// SAFETY: It should be safe to send a `SchemaCommandRequest` between threads, so long as
// it's only ever accessed from one thread at a time. It has unsynchronized internal
// mutability (when getting an object field, it will automatically add a new object
// if one doesn't already exist), so it cannot be `Sync`.
unsafe impl Send for SchemaCommandRequest {}

#[cfg(test)]
mod tests {
    pointer_type_tests!(super::SchemaCommandRequest);
}
