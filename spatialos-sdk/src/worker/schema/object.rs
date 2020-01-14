use crate::worker::schema::{DataPointer, Field, FieldId, IndexedField, Result};
use crate::worker::utils::cstr_to_string;
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct SchemaObject(PhantomData<*mut Schema_Object>);

impl SchemaObject {
    pub fn get<T: Field>(&self, field: FieldId) -> Result<T::RustType> {
        T::get(self, field)
    }

    pub fn get_index<T: IndexedField>(&self, field: FieldId, index: usize) -> Result<T::RustType> {
        T::index(self, field, index)
    }

    pub fn count<T: IndexedField>(&self, field: FieldId) -> usize {
        T::count(self, field)
    }

    pub fn add<T: Field>(&mut self, field: FieldId, value: &T::RustType) {
        T::add(self, field, value);
    }

    pub fn get_list<T: IndexedField>(&self, field: FieldId) -> Result<Vec<T::RustType>> {
        T::get_list(self, field)
    }

    pub fn add_list<T: IndexedField>(&mut self, field: FieldId, value: &[T::RustType]) {
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

    pub fn unique_field_ids(&self) -> Vec<FieldId> {
        unsafe {
            let count = Schema_GetUniqueFieldIdCount(self.as_ptr());
            let mut buffer = Vec::with_capacity(count as usize);
            Schema_GetUniqueFieldIds(self.as_ptr(), buffer.as_mut_ptr());
            buffer
        }
    }

    pub fn copy_from(&mut self, other: &SchemaObject) -> std::result::Result<(), String> {
        unsafe {
            let length = Schema_GetWriteBufferLength(other.as_ptr());
            let buffer = Schema_AllocateBuffer(self.as_ptr_mut(), length);
            let result = Schema_SerializeToBuffer(other.as_ptr(), buffer, length);

            if result == 0 {
                return Err(cstr_to_string(Schema_GetError(other.as_ptr())));
            }

            let result = Schema_MergeFromBuffer(self.as_ptr_mut(), buffer, length);

            if result == 0 {
                return Err(cstr_to_string(Schema_GetError(self.as_ptr())));
            }
        }

        Ok(())
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
