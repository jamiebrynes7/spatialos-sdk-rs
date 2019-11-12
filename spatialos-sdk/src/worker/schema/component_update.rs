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

    pub fn fields(&self) -> &SchemaObject {
        unsafe { SchemaObject::from_raw(Schema_GetComponentUpdateFields(self.internal)) }
    }

    pub fn fields_mut(&mut self) -> &mut SchemaObject {
        unsafe { SchemaObject::from_raw_mut(Schema_GetComponentUpdateFields(self.internal)) }
    }

    pub fn events(&self) -> &SchemaObject {
        unsafe { SchemaObject::from_raw(Schema_GetComponentUpdateEvents(self.internal)) }
    }

    pub fn events_mut(&mut self) -> &mut SchemaObject {
        unsafe { SchemaObject::from_raw_mut(Schema_GetComponentUpdateEvents(self.internal)) }
    }

    // TODO: Cleared fields.
}

impl Default for SchemaComponentUpdate {
    fn default() -> Self {
        Self::new()
    }
}
