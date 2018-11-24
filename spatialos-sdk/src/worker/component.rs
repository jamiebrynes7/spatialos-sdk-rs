use spatialos_sdk_sys::worker::{
    Schema_ComponentData, Schema_ComponentUpdate, Worker_ComponentData, Worker_ComponentUpdate,
};

use worker::internal::worker_sdk_conversion::WorkerSdkConversion;

// TODO: Wrap Schema_ComponentData
pub struct ComponentData {
    pub component_id: u32,
    pub schema_type: *mut Schema_ComponentData,
}

unsafe impl WorkerSdkConversion<Worker_ComponentData> for ComponentData {
    unsafe fn from_worker_sdk(component_data: &Worker_ComponentData) -> ComponentData {
        ComponentData {
            component_id: component_data.component_id,
            schema_type: component_data.schema_type,
        }
    }
}

// TODO: Wrap Schema_ComponentUpdate
pub struct ComponentUpdate {
    pub component_id: u32,
    pub schema_type: *mut Schema_ComponentUpdate,
}

unsafe impl WorkerSdkConversion<Worker_ComponentUpdate> for ComponentUpdate {
    unsafe fn from_worker_sdk(component_update: &Worker_ComponentUpdate) -> ComponentUpdate {
        ComponentUpdate {
            component_id: component_update.component_id,
            schema_type: component_update.schema_type,
        }
    }
}
