use spatialos_sdk_sys::worker::{
    Schema_ComponentData, Schema_ComponentUpdate, Worker_ComponentData, Worker_ComponentUpdate,
};

use crate::worker::ComponentId;

// TODO: Wrap Schema_ComponentData
pub struct ComponentData<'a> {
    pub component_id: ComponentId,
    pub schema_type: &'a Schema_ComponentData,
}

impl<'a> From<&Worker_ComponentData> for ComponentData<'a> {
    fn from(data: &Worker_ComponentData) -> Self {
        ComponentData {
            component_id: data.component_id,
            schema_type: unsafe { &*data.schema_type },
        }
    }
}

// TODO: Wrap Schema_ComponentUpdate
pub struct ComponentUpdate<'a> {
    pub component_id: ComponentId,
    pub schema_type: &'a Schema_ComponentUpdate,
}

impl<'a> From<&Worker_ComponentUpdate> for ComponentUpdate<'a> {
    fn from(update: &Worker_ComponentUpdate) -> Self {
        ComponentUpdate {
            component_id: update.component_id,
            schema_type: unsafe { &*update.schema_type },
        }
    }
}
