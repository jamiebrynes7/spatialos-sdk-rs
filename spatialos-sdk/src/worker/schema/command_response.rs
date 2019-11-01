use crate::worker::schema::SchemaObject;
use spatialos_sdk_sys::worker::*;

#[derive(Debug)]
pub struct SchemaCommandResponse {
    pub(crate) internal: *mut Schema_CommandResponse,
}

impl SchemaCommandResponse {
    pub fn new() -> SchemaCommandResponse {
        SchemaCommandResponse {
            internal: unsafe { Schema_CreateCommandResponse() },
        }
    }

    pub fn object(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetCommandResponseObject(self.internal) },
        }
    }

    pub fn object_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetCommandResponseObject(self.internal) },
        }
    }
}

impl Default for SchemaCommandResponse {
    fn default() -> Self {
        Self::new()
    }
}
