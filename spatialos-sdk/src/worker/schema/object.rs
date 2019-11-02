use crate::worker::schema::{FieldId, SchemaPrimitiveField};
use spatialos_sdk_sys::worker::*;

#[derive(Debug)]
pub struct SchemaObject {
    pub(crate) internal: *mut Schema_Object,
}

impl SchemaObject {
    pub fn field<T: SchemaPrimitiveField>(&self, field: FieldId) -> Option<T::RustType> {
        if self.field_count::<T>(field) == 0 {
            None
        } else {
            T::get(self, field)
        }
    }

    pub fn index_field<T: SchemaPrimitiveField>(
        &self,
        field: FieldId,
        index: usize,
    ) -> T::RustType {
        T::index(self, field, index)
    }

    pub fn field_count<T: SchemaPrimitiveField>(&self, field: FieldId) -> usize {
        T::count(self, field)
    }

    pub fn add_field<T: SchemaPrimitiveField>(&mut self, field: FieldId, value: &T::RustType) {
        T::add(self, field, value);
    }

    pub fn add_field_list<T: SchemaPrimitiveField>(
        &mut self,
        field: FieldId,
        value: &[T::RustType],
    ) {
        T::add_list(self, field, value);
    }
}
