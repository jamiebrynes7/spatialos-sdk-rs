use crate::worker::schema::{DataPointer, Owned, OwnedPointer, SchemaObject};
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

pub struct SchemaGenericData(PhantomData<*mut Schema_GenericData>);

impl SchemaGenericData {
    pub fn new() -> Owned<Self> {
        Owned::new()
    }

    pub fn object(&self) -> &SchemaObject {
        unsafe { SchemaObject::from_raw(Schema_GetGenericDataObject(self.as_ptr() as *mut _)) }
    }

    pub fn object_mut(&mut self) -> &mut SchemaObject {
        unsafe { SchemaObject::from_raw_mut(Schema_GetGenericDataObject(self.as_ptr_mut())) }
    }
}

unsafe impl DataPointer for SchemaGenericData {
    type Raw = Schema_GenericData;
}

unsafe impl OwnedPointer for SchemaGenericData {
    const CREATE_FN: unsafe extern "C" fn() -> *mut Self::Raw = Schema_CreateGenericData;
    const DESTROY_FN: unsafe extern "C" fn(*mut Self::Raw) = Schema_DestroyGenericData;
    const COPY_FN: unsafe extern "C" fn(*const Self::Raw) -> *mut Self::Raw =
        Schema_CopyGenericData;
}

// SAFETY: It should be safe to send a `SchemaGenericData` between threads, so long as
// it's only ever accessed from one thread at a time. It has unsynchronized internal
// mutability (when getting an object field, it will automatically add a new object
// if one doesn't already exist), so it cannot be `Sync`.
unsafe impl Send for SchemaGenericData {}

impl ToOwned for SchemaGenericData {
    type Owned = Owned<Self>;

    fn to_owned(&self) -> Self::Owned {
        Owned::from(self)
    }
}

#[cfg(test)]
mod tests {
    pointer_type_tests!(super::SchemaGenericData);
}
