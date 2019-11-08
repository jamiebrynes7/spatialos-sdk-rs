use crate::worker::schema::{owned::*, SchemaObject};
use spatialos_sdk_sys::worker::*;
use static_assertions::*;
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
    pub fn new() -> Owned<SchemaComponentData> {
        unsafe { Owned::new(Schema_CreateComponentData()) }
    }

    pub fn fields(&self) -> &SchemaObject {
        unsafe { SchemaObject::from_raw(Schema_GetComponentDataFields(self.as_ptr())) }
    }

    pub fn fields_mut(&mut self) -> &mut SchemaObject {
        unsafe { SchemaObject::from_raw_mut(Schema_GetComponentDataFields(self.as_ptr())) }
    }

    // Methods for raw pointer conversion.
    // -----------------------------------

    pub(crate) unsafe fn from_raw<'a>(raw: *mut Schema_ComponentData) -> &'a Self {
        &*(raw as *mut _)
    }

    pub(crate) unsafe fn from_raw_mut<'a>(raw: *mut Schema_ComponentData) -> &'a mut Self {
        &mut *(raw as *mut _)
    }

    pub(crate) fn as_ptr(&self) -> *mut Schema_ComponentData {
        self as *const _ as *mut _
    }
}

impl Default for Owned<SchemaComponentData> {
    fn default() -> Self {
        SchemaComponentData::new()
    }
}

impl OwnableImpl for SchemaComponentData {
    type Raw = Schema_ComponentData;

    unsafe fn destroy(inst: *mut Self::Raw) {
        Schema_DestroyComponentData(inst);
    }
}

// SAFETY: It should be safe to send a `SchemaComponentData` between threads, so long as
// it's only ever accessed from one thread at a time. It has unsynchronized internal
// mutability (when getting an object field, it will automatically add a new object
// if one doesn't already exist), so it cannot be `Sync`.
unsafe impl Send for SchemaComponentData {}

assert_impl_all!(SchemaComponentData: Send);
assert_not_impl_any!(SchemaComponentData: Sync);
