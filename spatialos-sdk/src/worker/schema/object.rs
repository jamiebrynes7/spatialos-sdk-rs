use crate::worker::schema::{FieldId, SchemaPrimitiveField};
use spatialos_sdk_sys::worker::*;

#[derive(Debug)]
pub struct SchemaObject {
    pub(crate) internal: *mut Schema_Object,
}

impl SchemaObject {
    pub fn get<T: SchemaPrimitiveField>(&self, field: FieldId) -> Option<T::RustType> {
        if self.count::<T>(field) == 0 {
            None
        } else {
            T::get(self, field)
        }
    }

    pub fn get_index<T: SchemaPrimitiveField>(&self, field: FieldId, index: usize) -> T::RustType {
        T::index(self, field, index)
    }

    pub fn count<T: SchemaPrimitiveField>(&self, field: FieldId) -> usize {
        T::count(self, field)
    }

    pub fn add<T: SchemaPrimitiveField>(&mut self, field: FieldId, value: &T::RustType) {
        T::add(self, field, value);
    }

    pub fn get_list<T: SchemaPrimitiveField>(&self, field: FieldId) -> Vec<T::RustType> {
        T::get_list(self, field)
    }

    pub fn add_list<T: SchemaPrimitiveField>(&mut self, field: FieldId, value: &[T::RustType]) {
        T::add_list(self, field, value);
    }

    // TODO: Hook up the lifetimes of the schema objects. This is unsound as it exists now.
    pub fn get_object(&self, field: FieldId) -> SchemaObject {
        let internal = unsafe { Schema_GetObject(self.internal, field) };
        SchemaObject { internal }
    }

    pub fn object_count(&self, field: FieldId) -> usize {
        let count = unsafe { Schema_GetObjectCount(self.internal, field) };
        count as usize
    }

    pub fn index_object(&self, field: FieldId, index: usize) -> SchemaObject {
        let internal = unsafe { Schema_IndexObject(self.internal, field, index as u32) };
        SchemaObject { internal }
    }

    // TODO: Hook up the lifetimes of the schema objects. This is unsound as it exists now.
    pub fn add_object(&mut self, field: FieldId) -> SchemaObject {
        let internal = unsafe { Schema_AddObject(self.internal, field) };
        SchemaObject { internal }
    }
}
