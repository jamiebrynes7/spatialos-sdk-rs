use crate::worker::schema::{ArrayField, FieldId, SchemaField};
use spatialos_sdk_sys::worker::*;

pub struct Object(Schema_Object);

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

    pub fn as_ptr(&self) -> *mut Schema_Object {
        self as *const _ as *mut _
    }
}
