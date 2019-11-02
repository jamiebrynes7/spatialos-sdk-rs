use crate::worker::schema::SchemaObject;
use spatialos_sdk_sys::worker::*;

#[derive(Debug)]
pub struct SchemaComponentData {
    pub(crate) internal: *mut Schema_ComponentData,
}

impl SchemaComponentData {
    pub fn new() -> SchemaComponentData {
        SchemaComponentData {
            internal: unsafe { Schema_CreateComponentData() },
        }
    }

    pub fn fields(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentDataFields(self.internal) },
        }
    }

    pub fn fields_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentDataFields(self.internal) },
        }
    }
}

impl Default for SchemaComponentData {
    fn default() -> Self {
        Self::new()
    }
}