use crate::worker::schema::SchemaObject;
use spatialos_sdk_sys::worker::*;

#[derive(Debug)]
pub struct SchemaComponentUpdate {
    pub(crate) internal: *mut Schema_ComponentUpdate,
}

impl SchemaComponentUpdate {
    pub fn new() -> SchemaComponentUpdate {
        SchemaComponentUpdate {
            internal: unsafe { Schema_CreateComponentUpdate() },
        }
    }

    pub fn fields(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateFields(self.internal) },
        }
    }

    pub fn fields_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateFields(self.internal) },
        }
    }

    pub fn events(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateEvents(self.internal) },
        }
    }

    pub fn events_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateEvents(self.internal) },
        }
    }

    // TODO: Cleared fields.
}

impl Default for SchemaComponentUpdate {
    fn default() -> Self {
        Self::new()
    }
}
