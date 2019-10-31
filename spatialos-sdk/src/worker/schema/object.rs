use crate::worker::{component::ComponentId, schema::SchemaFieldContainer};
use spatialos_sdk_sys::worker::*;

#[derive(Debug)]
pub struct SchemaObject {
    pub(super) internal: *mut Schema_Object,
}

impl SchemaObject {
    pub fn field<T>(&self, field_id: ComponentId) -> SchemaFieldContainer<T> {
        SchemaFieldContainer {
            field_id,
            container: self,
            _phantom: Default::default(),
        }
    }
}
