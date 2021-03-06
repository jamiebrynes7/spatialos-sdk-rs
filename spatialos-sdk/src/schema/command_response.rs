use crate::schema::{DataPointer, Owned, OwnedPointer, SchemaObject};
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct SchemaCommandResponse(PhantomData<*mut Schema_CommandResponse>);

impl SchemaCommandResponse {
    pub fn new() -> Owned<SchemaCommandResponse> {
        Owned::new()
    }

    pub fn object(&self) -> &SchemaObject {
        unsafe { SchemaObject::from_raw(Schema_GetCommandResponseObject(self.as_ptr() as *mut _)) }
    }

    pub fn object_mut(&mut self) -> &mut SchemaObject {
        unsafe { SchemaObject::from_raw_mut(Schema_GetCommandResponseObject(self.as_ptr_mut())) }
    }
}

unsafe impl DataPointer for SchemaCommandResponse {
    type Raw = Schema_CommandResponse;
}

unsafe impl OwnedPointer for SchemaCommandResponse {
    const CREATE_FN: unsafe extern "C" fn() -> *mut Self::Raw = Schema_CreateCommandResponse;
    const DESTROY_FN: unsafe extern "C" fn(*mut Self::Raw) = Schema_DestroyCommandResponse;
    const COPY_FN: unsafe extern "C" fn(*const Self::Raw) -> *mut Self::Raw =
        Schema_CopyCommandResponse;
}

// SAFETY: It should be safe to send a `SchemaCommandResonse` between threads, so long as
// it's only ever accessed from one thread at a time. It has unsynchronized internal
// mutability (when getting an object field, it will automatically add a new object
// if one doesn't already exist), so it cannot be `Sync`.
unsafe impl Send for SchemaCommandResponse {}

impl ToOwned for SchemaCommandResponse {
    type Owned = Owned<Self>;

    fn to_owned(&self) -> Self::Owned {
        Owned::from(self)
    }
}

#[cfg(test)]
mod tests {
    pointer_type_tests!(super::SchemaCommandResponse);
}
