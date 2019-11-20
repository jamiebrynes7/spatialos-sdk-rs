use crate::worker::{
    component::{Component, Update},
    schema::{DataPointer, Field, FieldId, Owned, OwnedPointer, SchemaObject},
};
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct SchemaComponentUpdate(PhantomData<*mut Schema_ComponentUpdate>);

impl SchemaComponentUpdate {
    pub fn new() -> Owned<Self> {
        Owned::new()
    }

    pub fn from_update<U: Update>(update: &U) -> Owned<Self> {
        let mut result = Owned::new();
        update.into_update(&mut result);
        result
    }

    pub fn deserialize<C: Component>(&self) -> C::Update {
        C::Update::from_update(&self)
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

    pub fn field_cleared(&self, field: FieldId) -> bool {
        0 != unsafe { Schema_IsComponentUpdateFieldCleared(self.as_ptr() as *mut _, field) }
    }

    pub fn add_cleared(&mut self, field: FieldId) {
        unsafe {
            Schema_AddComponentUpdateClearedField(self.as_ptr_mut(), field);
        }
    }

    pub fn get_field<T>(&self, field: FieldId) -> Option<T::RustType>
    where
        T: Field,
    {
        T::get_update(self, field)
    }

    pub fn add_field<T>(&mut self, field: FieldId, value: &Option<T::RustType>)
    where
        T: Field,
    {
        T::add_update(self, field, value);
    }
}

unsafe impl DataPointer for SchemaComponentUpdate {
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
