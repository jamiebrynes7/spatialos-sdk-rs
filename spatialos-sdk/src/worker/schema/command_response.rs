use crate::worker::schema::{owned::OwnableImpl, Owned, PointerType, SchemaObject};
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct SchemaCommandResponse(PhantomData<*mut Schema_CommandResponse>);

impl SchemaCommandResponse {
    pub fn new() -> Owned<SchemaCommandResponse> {
        Owned::new()
    }

    pub fn object(&self) -> &SchemaObject {
        unsafe { SchemaObject::from_raw(Schema_GetCommandResponseObject(self.as_ptr())) }
    }

    pub fn object_mut(&mut self) -> &mut SchemaObject {
        unsafe { SchemaObject::from_raw_mut(Schema_GetCommandResponseObject(self.as_ptr())) }
    }

    // Methods for raw pointer conversion.
    // -----------------------------------

    pub(crate) unsafe fn from_raw<'a>(raw: *mut Schema_CommandResponse) -> &'a Self {
        &*(raw as *mut _)
    }

    pub(crate) fn as_ptr(&self) -> *mut Schema_CommandResponse {
        self as *const _ as *mut _
    }
}

unsafe impl PointerType for SchemaCommandResponse {
    type Raw = Schema_CommandResponse;
}

unsafe impl OwnableImpl for SchemaCommandResponse {
    const CREATE_FN: unsafe extern "C" fn() -> *mut Self::Raw = Schema_CreateCommandResponse;
    const DESTROY_FN: unsafe extern "C" fn(*mut Self::Raw) = Schema_DestroyCommandResponse;
}

// SAFETY: It should be safe to send a `SchemaCommandResonse` between threads, so long as
// it's only ever accessed from one thread at a time. It has unsynchronized internal
// mutability (when getting an object field, it will automatically add a new object
// if one doesn't already exist), so it cannot be `Sync`.
unsafe impl Send for SchemaCommandResponse {}

#[cfg(test)]
mod tests {
    use super::SchemaCommandResponse;
    use static_assertions::*;

    assert_impl_all!(SchemaCommandResponse: Send);
    assert_not_impl_any!(SchemaCommandResponse: Sync);
}
