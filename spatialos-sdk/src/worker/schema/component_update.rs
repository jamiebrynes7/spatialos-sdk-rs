use crate::worker::schema::{Owned, OwnedPointer, PointerType, SchemaObject};
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct SchemaComponentUpdate(PhantomData<*mut Schema_ComponentUpdate>);

impl SchemaComponentUpdate {
    pub fn new() -> Owned<SchemaComponentUpdate> {
        Owned::new()
    }

    pub fn fields(&self) -> &SchemaObject {
        unsafe { SchemaObject::from_raw(Schema_GetComponentUpdateFields(self.as_ptr() as *mut _)) }
    }

    pub fn fields_mut(&mut self) -> &mut SchemaObject {
        unsafe { SchemaObject::from_raw_mut(Schema_GetComponentUpdateFields(self.as_ptr_mut())) }
    }

    pub fn events(&self) -> &SchemaObject {
        unsafe { SchemaObject::from_raw(Schema_GetComponentUpdateEvents(self.as_ptr() as *mut _)) }
    }

    pub fn events_mut(&mut self) -> &mut SchemaObject {
        unsafe { SchemaObject::from_raw_mut(Schema_GetComponentUpdateEvents(self.as_ptr_mut())) }
    }

    // TODO: Cleared fields.
}

unsafe impl PointerType for SchemaComponentUpdate {
    type Raw = Schema_ComponentUpdate;
}

unsafe impl OwnedPointer for SchemaComponentUpdate {
    const CREATE_FN: unsafe extern "C" fn() -> *mut Self::Raw = Schema_CreateComponentUpdate;
    const DESTROY_FN: unsafe extern "C" fn(*mut Self::Raw) = Schema_DestroyComponentUpdate;
}

// SAFETY: It should be safe to send a `SchemaComponentUpdate` between threads, so long as
// it's only ever accessed from one thread at a time. It has unsynchronized internal
// mutability (when getting an object field, it will automatically add a new object
// if one doesn't already exist), so it cannot be `Sync`.
unsafe impl Send for SchemaComponentUpdate {}

#[cfg(test)]
mod tests {
    pointer_type_tests!(super::SchemaComponentUpdate);
}
