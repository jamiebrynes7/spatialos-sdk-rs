use spatialos_sdk_sys::worker::{
    Schema_ComponentData, Schema_ComponentUpdate, Worker_ComponentData, Worker_ComponentUpdate,
};

use crate::worker::ComponentId;

// TODO: Wrap Schema_ComponentData
pub struct ComponentData<'a> {
    pub component_id: ComponentId,
    pub schema_type: &'a mut Schema_ComponentData,
}

impl From<&Worker_ComponentData> for ComponentData {
    fn from(data: &Worker_ComponentData) -> Self {
        ComponentData {
            component_id: data.component_id,
            schema_type: data.schema_type,
        }
    }
}

// TODO: Wrap Schema_ComponentUpdate
pub struct ComponentUpdate<'a> {
    pub component_id: ComponentId,
    pub schema_type: &'a mut Schema_ComponentUpdate,
}

impl From<&Worker_ComponentUpdate> for ComponentUpdate {
    fn from(update: &Worker_ComponentUpdate) -> Self {
        ComponentUpdate {
            component_id: update.component_id,
            schema_type: update.schema_type,
        }
    }
}
