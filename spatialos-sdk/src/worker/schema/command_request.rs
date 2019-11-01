use crate::worker::schema::SchemaObject;
use spatialos_sdk_sys::worker::*;

#[derive(Debug)]
pub struct SchemaCommandRequest {
    pub(crate) internal: *mut Schema_CommandRequest,
}

impl SchemaCommandRequest {
    pub fn new() -> SchemaCommandRequest {
        SchemaCommandRequest {
            internal: unsafe { Schema_CreateCommandRequest() },
        }
    }

    pub fn object(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetCommandRequestObject(self.internal) },
        }
    }

    pub fn object_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetCommandRequestObject(self.internal) },
        }
    }
}

impl Default for SchemaCommandRequest {
    fn default() -> Self {
        Self::new()
    }
}
