use crate::worker::{
    component::{Component, ComponentId},
    schema::{
        object::{ObjectMut, ObjectRef},
        SchemaObjectType,
    },
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
pub struct ComponentData {
    raw: NonNull<Schema_ComponentData>,
}

impl ComponentData {
    pub fn new<C: Component>(component: &C) -> Self {
        // Create the underlying `Schema_ComponentData` and retrieve the fields object.
        let raw = NonNull::new(unsafe { Schema_CreateComponentData(C::ID) }).unwrap();
        let mut result = Self { raw };

        // Populate the schema data from the component.
        component.into_object(&mut result.fields_mut());

        result
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetComponentDataComponentId(self.raw.as_ref()) }
    }

    pub fn fields(&self) -> ObjectRef<'_> {
        self.as_ref().fields()
    }

    pub fn fields_mut(&mut self) -> ObjectMut<'_> {
        unsafe { ObjectMut::from_raw(&mut *Schema_GetComponentDataFields(self.raw.as_ptr())) }
    }

    pub fn as_ref(&self) -> ComponentDataRef<'_> {
        ComponentDataRef {
            raw: unsafe { self.raw.as_ref() },
        }
    }

    pub fn deserialize<T: SchemaObjectType>(&self) -> T {
        T::from_object(self.fields())
    }

    /// Converts the `OwnedComponentData` into a `*mut Schema_ComponentData` that can be
    /// passed to the C API.
    ///
    /// This transfers ownership of the data to the caller, so the caller needs to
    /// ensure that the appropriate steps are taken to free the data. If the raw data is
    /// passed to the C API, the C SDK will take ownership of the data and will free it
    /// when it's done.
    pub fn into_raw(self) -> *mut Schema_ComponentData {
        let raw = self.raw;
        mem::forget(self);
        raw.as_ptr()
    }
}

impl Drop for ComponentData {
    fn drop(&mut self) {
        unsafe {
            Schema_DestroyComponentData(self.raw.as_ptr());
        }
    }
}

/// Serialized schema data for a compnent owned by the C SDK.
///
/// The lifetime parameter tracks the parent data that owns the schema data
/// (generally an `OpList`), such the `SchemaComponentData` instance cannot live
/// its parent.
#[derive(Debug)]
pub struct ComponentDataRef<'owner> {
    raw: &'owner Schema_ComponentData,
}

impl<'owner> ComponentDataRef<'owner> {
    pub unsafe fn from_raw(raw: &'owner Schema_ComponentData) -> Self {
        Self { raw }
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetComponentDataComponentId(self.raw) }
    }

    pub fn fields(&self) -> ObjectRef<'owner> {
        unsafe {
            ObjectRef::from_raw(&*Schema_GetComponentDataFields(
                self.raw as *const _ as *mut _,
            ))
        }
    }

    pub fn deserialize<T: SchemaObjectType>(&self) -> T {
        T::from_object(self.fields())
    }
}
