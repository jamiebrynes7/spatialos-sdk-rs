use crate::worker::schema::{ArrayField, FieldId, SchemaField};
use spatialos_sdk_sys::worker::*;
use std::{slice, marker::PhantomData};

pub struct Object(PhantomData<*mut Schema_Object>);

impl Object {
    pub(crate) unsafe fn from_raw<'a>(raw: *const Schema_Object) -> &'a Self {
        &*(raw as *const _)
    }

    pub(crate) unsafe fn from_raw_mut<'a>(raw: *mut Schema_Object) -> &'a mut Self {
        &mut *(raw as *mut _)
    }

    pub fn field<T: SchemaField>(&self, field: FieldId) -> T::RustType {
        T::get_field(self, field)
    }

    pub fn add_field<T: SchemaField>(&mut self, field: FieldId, value: &T::RustType) {
        T::add_field(self, field, value);
    }

    pub fn field_array<T: ArrayField>(&self, field: FieldId) -> Vec<T::RustType> {
        let mut result = Vec::new();
        T::get_field_list(self, field, &mut result);
        result
    }

    pub fn add_field_array<T: ArrayField>(&mut self, field: FieldId, value: &[T::RustType]) {
        T::add_field_list(self, field, value);
    }

    pub fn object_field(&self, field: FieldId) -> &Object {
        unsafe { Object::from_raw(Schema_GetObject(self.as_ptr(), field)) }
    }

    pub fn add_object_field(&mut self, field: FieldId) -> &mut Object {
        unsafe { Object::from_raw_mut(Schema_AddObject(self.as_ptr(), field)) }
    }

    pub fn object_field_count(&self, field: FieldId) -> u32 {
        unsafe { Schema_GetObjectCount(self.as_ptr(), field) }
    }

    pub fn index_object_field(&self, field: FieldId, index: u32) -> &Object {
        unsafe { Object::from_raw(Schema_IndexObject(self.as_ptr(), field, index)) }
    }

    pub(crate) fn add_bytes(&mut self, field: FieldId, bytes: &[u8]) {
        // Create a buffer owned by `object` and populate that buffer with `bytes`.
        let buffer = unsafe {
            let data = Schema_AllocateBuffer(self.as_ptr(), bytes.len() as _);
            slice::from_raw_parts_mut(data, bytes.len())
        };
        buffer.copy_from_slice(bytes);

        // Add `buffer` to `object` as the field.
        unsafe {
            Schema_AddBytes(self.as_ptr(), field, buffer.as_ptr(), buffer.len() as _);
        }
    }

    pub(crate) fn bytes_count(&self, field: FieldId) -> u32 {
        unsafe { Schema_GetBytesCount(self.as_ptr(), field) }
    }

    pub(crate) fn get_bytes(&self, field: FieldId) -> &[u8] {
        unsafe {
            let data = Schema_GetBytes(self.as_ptr(), field);
            let len = Schema_GetBytesLength(self.as_ptr(), field);
            std::slice::from_raw_parts(data, len as usize)
        }
    }


    pub(crate) fn index_bytes(&self, field: FieldId, index: u32) -> &[u8] {
        unsafe {
            let data = Schema_IndexBytes(self.as_ptr(), field, index);
            let len = Schema_IndexBytesLength(self.as_ptr(), field, index);
            std::slice::from_raw_parts(data, len as usize)
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut Schema_Object {
        self as *const _ as *mut _
    }
}

unsafe impl Send for Object {}
