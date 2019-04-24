use crate::worker::{
    component::{Component, ComponentId},
    schema::{object::Object, owned::*, SchemaObjectType},
};
use spatialos_sdk_sys::worker::*;
use std::{mem, ptr::NonNull};

/// Serialized schema data for a component, owned by the Rust SDK.
///
/// For maximum efficiency, the serialized data may borrow data from the component
/// used to create an `OwnedComponentData` instance. The lifetime parameter
/// tracks this borrow, such that an `OwnedComponentData` cannot outlive the
/// data it borrows.
#[derive(Debug)]
pub struct ComponentData(Schema_ComponentData);

impl Destroy for ComponentData {
    unsafe fn destroy(inst: *mut Self) {
        Schema_DestroyComponentData(inst as *mut _);
    }
}

impl ComponentData {
    pub fn new<C: Component>(component: &C) -> Owned<Self> {
        // Create the underlying `Schema_ComponentData` and retrieve the fields object.
        let mut result = unsafe { Owned::new(Schema_CreateComponentData(C::ID) as *mut _) };

        // Populate the schema data from the component.
        component.into_object(result.fields_mut());

        result
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetComponentDataComponentId(self.as_ptr()) }
    }

    pub fn fields(&self) -> &Object {
        unsafe { Object::from_raw(Schema_GetComponentDataFields(self.as_ptr())) }
    }

    pub fn fields_mut(&mut self) -> &mut Object {
        unsafe { Object::from_raw_mut(Schema_GetComponentDataFields(self.as_ptr())) }
    }

    pub fn deserialize<T: SchemaObjectType>(&self) -> T {
        T::from_object(self.fields())
    }

    pub(crate) fn as_ptr(&self) -> *mut Schema_ComponentData {
        self as *const _ as *mut _
    }
}
