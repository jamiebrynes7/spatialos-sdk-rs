use crate::worker::schema::{ArrayField, FieldId, SchemaField};
use spatialos_sdk_sys::worker::*;

#[derive(Debug)]
pub struct ObjectMut<'owner> {
    raw: &'owner mut Schema_Object,
}

impl<'owner> ObjectMut<'owner> {
    pub(crate) unsafe fn from_raw(raw: &'owner mut Schema_Object) -> Self {
        Self { raw }
    }

    pub fn add_field<T: SchemaField>(&mut self, field: FieldId, value: &T::RustType) {
        T::add_field(self, field, value);
    }

    pub fn add_field_array<T: ArrayField>(&mut self, field: FieldId, value: &[T::RustType]) {
        T::add_field_list(self, field, value);
    }

    pub fn as_ref(&'owner self) -> ObjectRef<'owner> {
        ObjectRef { raw: self.raw }
    }

    pub fn as_ptr(&mut self) -> *mut Schema_Object {
        self.raw
    }
}

impl<'a> AsRef<Schema_Object> for ObjectMut<'a> {
    fn as_ref(&self) -> &Schema_Object {
        self.raw
    }
}

impl<'a> AsMut<Schema_Object> for ObjectMut<'a> {
    fn as_mut(&mut self) -> &mut Schema_Object {
        self.raw
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ObjectRef<'a> {
    raw: &'a Schema_Object,
}

impl<'a> ObjectRef<'a> {
    pub unsafe fn from_raw(raw: &'a Schema_Object) -> Self {
        Self { raw }
    }

    pub fn field<T: SchemaField>(self, field: FieldId) -> T::RustType {
        T::get_field(self, field)
    }

    pub fn field_array<T: ArrayField>(self, field: FieldId) -> Vec<T::RustType> {
        let mut result = Vec::new();
        T::get_field_list(self, field, &mut result);
        result
    }

    pub fn as_ptr(self) -> *mut Schema_Object {
        self.raw as *const _ as *mut _
    }
}

impl<'a> AsRef<Schema_Object> for ObjectRef<'a> {
    fn as_ref(&self) -> &Schema_Object {
        self.raw
    }
}
