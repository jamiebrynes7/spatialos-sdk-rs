use crate::worker::schema::{DataPointer, Field, FieldId};
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct SchemaObject(PhantomData<*mut Schema_Object>);

impl SchemaObject {
    pub fn get<T: Field>(&self, field: FieldId) -> T::RustType {
        T::get_or_default(self, field)
    }

    pub fn get_index<T: Field>(&self, field: FieldId, index: usize) -> T::RustType {
        T::index(self, field, index)
    }

    pub fn count<T: Field>(&self, field: FieldId) -> usize {
        T::count(self, field)
    }

    pub fn add<T: Field>(&mut self, field: FieldId, value: &T::RustType) {
        T::add(self, field, value);
    }

    pub fn get_list<T: Field>(&self, field: FieldId) -> Vec<T::RustType> {
        T::get_list(self, field)
    }

    pub fn add_list<T: Field>(&mut self, field: FieldId, value: &[T::RustType]) {
        T::add_list(self, field, value);
    }

    pub fn get_object(&self, field: FieldId) -> &SchemaObject {
        unsafe { Self::from_raw(Schema_GetObject(self.as_ptr() as *mut _, field)) }
    }

    pub fn object_count(&self, field: FieldId) -> usize {
        let count = unsafe { Schema_GetObjectCount(self.as_ptr(), field) };
        count as usize
    }

    pub fn index_object(&self, field: FieldId, index: usize) -> &SchemaObject {
        unsafe {
            Self::from_raw(Schema_IndexObject(
                self.as_ptr() as *mut _,
                field,
                index as u32,
            ))
        }
    }

    pub fn add_object(&mut self, field: FieldId) -> &mut SchemaObject {
        unsafe { Self::from_raw_mut(Schema_AddObject(self.as_ptr_mut(), field)) }
    }
}

unsafe impl DataPointer for SchemaObject {
    type Raw = Schema_Object;
}

// SAFETY: It should be safe to send a `SchemaObject` between threads, so long as
// it's only ever accessed from one thread at a time. It has unsynchronized internal
// mutability (when getting an object field, it will automatically add a new object
// if one doesn't already exist), so it cannot be `Sync`.
unsafe impl Send for SchemaObject {}

#[cfg(test)]
mod tests {
    pointer_type_tests!(super::SchemaObject);
}
