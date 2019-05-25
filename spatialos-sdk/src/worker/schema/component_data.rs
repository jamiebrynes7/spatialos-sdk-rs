use crate::worker::{
    component::{Component, ComponentId},
    schema::{object::Object, owned::*, SchemaObjectType},
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
pub struct ComponentData(PhantomData<*mut Schema_ComponentData>);

impl ComponentData {
    pub fn new<C: Component>(component: &C) -> Owned<Self> {
        // Create the underlying `Schema_ComponentData` and wrap it in a smart pointer.
        let mut result: Owned<Self> = unsafe { Owned::new(Schema_CreateComponentData(C::ID)) };

        // Populate the schema data from the component.
        let component_data = &mut *result;
        let fields: &mut Object = component_data.fields_mut();
        component.into_object(fields);

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

    pub(crate) unsafe fn from_raw<'a>(raw: *mut Schema_ComponentData) -> &'a Self {
        &*(raw as *mut _)
    }

    pub(crate) fn as_ptr(&self) -> *mut Schema_ComponentData {
        self as *const _ as *mut _
    }
}

impl OwnableImpl for ComponentData {
    type Raw = Schema_ComponentData;

    unsafe fn destroy(inst: *mut Self::Raw) {
        Schema_DestroyComponentData(inst);
    }
}

unsafe impl Send for ComponentData {}
