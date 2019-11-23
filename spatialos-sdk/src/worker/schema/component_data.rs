use crate::worker::{
    component::Component,
    schema::{DataPointer, Owned, OwnedPointer, Result, SchemaObject},
};
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

/// Serialized schema data for a component, owned by the Rust SDK.
///
/// For maximum efficiency, the serialized data may borrow data from the component
/// used to create an `OwnedComponentData` instance. The lifetime parameter
/// tracks this borrow, such that an `OwnedComponentData` cannot outlive the
/// data it borrows.
#[derive(Debug)]
pub struct SchemaComponentData(PhantomData<*mut Schema_ComponentData>);

impl SchemaComponentData {
    pub fn new() -> Owned<Self> {
        Owned::new()
    }

    pub fn from_component<C: Component>(component: &C) -> Owned<Self> {
        let mut result = Self::new();
        component.into_object(result.fields_mut());
        result
    }

    pub fn deserialize<C: Component>(&self) -> Result<C> {
        C::from_object(self.fields())
    }

    pub fn fields(&self) -> &SchemaObject {
        unsafe { SchemaObject::from_raw(Schema_GetComponentDataFields(self.as_ptr() as *mut _)) }
    }

    pub fn fields_mut(&mut self) -> &mut SchemaObject {
        unsafe { SchemaObject::from_raw_mut(Schema_GetComponentDataFields(self.as_ptr_mut())) }
    }
}

unsafe impl DataPointer for SchemaComponentData {
    type Raw = Schema_ComponentData;
}

unsafe impl OwnedPointer for SchemaComponentData {
    const CREATE_FN: unsafe extern "C" fn() -> *mut Self::Raw = Schema_CreateComponentData;
    const DESTROY_FN: unsafe extern "C" fn(*mut Self::Raw) = Schema_DestroyComponentData;
}

// SAFETY: It should be safe to send a `SchemaComponentData` between threads, so long as
// it's only ever accessed from one thread at a time. It has unsynchronized internal
// mutability (when getting an object field, it will automatically add a new object
// if one doesn't already exist), so it cannot be `Sync`.
unsafe impl Send for SchemaComponentData {}

#[cfg(test)]
mod test {
    pointer_type_tests!(super::SchemaComponentData);
}
