use crate::worker::schema::{FieldId, SchemaPrimitiveField};
use spatialos_sdk_sys::worker::*;

#[derive(Debug)]
pub struct SchemaObject {
    pub(crate) internal: *mut Schema_Object,
}

impl SchemaObject {
    fn field<T: SchemaPrimitiveField>(&self, field: FieldId) -> Option<T> {
        if self.count::<T>(field) == 0 {
            None
        } else {
            T::get(self, field)
        }
    }

    fn index_field<T: SchemaPrimitiveField>(&self, field: FieldId, index: usize) -> T {
        T::index(self, field, index)
    }

    fn field_count<T: SchemaPrimitiveField>(&self, field: FieldId) -> usize {
        T::count(self, field)
    }

    fn add_field<T: SchemaPrimitiveField>(&mut self, field: FieldId, value: &T) {
        T::add(self, field, value);
    }

    fn add_field_list<T: SchemaPrimitiveField>(&mut self, field: FieldId, value: &[T]) {
        T::add_list(self, field, value);
    }
}
