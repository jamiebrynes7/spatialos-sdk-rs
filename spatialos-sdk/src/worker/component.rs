use worker::internal::schema;
use spatialos_sdk_sys::worker;
use std::collections::HashMap;

pub type ComponentId = u32;

pub trait ComponentMetaclass {
    type Data;
    type Update;

    fn component_id() -> ComponentId;
}

pub trait ComponentUpdate<M: ComponentMetaclass> {
    fn to_data(self) -> M::Data;
}

pub trait ComponentData<M: ComponentMetaclass> {
    fn to_update(self) -> M::Update;
    fn merge(&mut self, update: M::Update);
}

pub trait ComponentVtable<M: ComponentMetaclass> {
    fn serialize_update(update: &M::Update) -> schema::SchemaComponentUpdate;
    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Option<M::Update>;
    fn serialize_data(update: &M::Data) -> schema::SchemaComponentData;
    fn deserialize_data(update: &schema::SchemaComponentData) -> Option<M::Data>;
    // TODO: Command requests and command responses.
    fn create_internal_vtable() -> worker::Worker_ComponentVtable;
}

// TODO: CommandRequestSerializer and CommandResponseSerializer

// A data structure which represents all known component types.
pub struct ComponentDatabase {
    component_vtables: HashMap<ComponentId, worker::Worker_ComponentVtable>
}

impl ComponentDatabase {
    fn new() -> Self {
        ComponentDatabase{component_vtables: HashMap::new()}
    }

    fn add_component<M: ComponentMetaclass, V: ComponentVtable<M>>(mut self) -> Self {
        self.component_vtables.insert(M::component_id(), V::create_internal_vtable());
        self
    }
}

pub mod internal {
use spatialos_sdk_sys::worker::{
    Schema_ComponentData, Schema_ComponentUpdate, Worker_ComponentData, Worker_ComponentUpdate,
    Schema_CommandRequest, Schema_CommandResponse, Worker_CommandRequest, Worker_CommandResponse,
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

// TODO: Wrap Schema_CommandRequest
pub struct CommandRequest {
    pub component_id: u32,
    pub schema_type: *mut Schema_CommandRequest,
}

unsafe impl WorkerSdkConversion<Worker_CommandRequest> for CommandRequest {
    unsafe fn from_worker_sdk(command_request: &Worker_CommandRequest) -> Self {
        CommandRequest {
            component_id: command_request.component_id,
            schema_type: command_request.schema_type,
        }
    }
}

// TODO: Wrap Schema_CommandResponse
pub struct CommandResponse {
    pub component_id: u32,
    pub schema_type: *mut Schema_CommandResponse,
}

unsafe impl WorkerSdkConversion<Worker_CommandResponse> for CommandResponse {
    unsafe fn from_worker_sdk(command_response: &Worker_CommandResponse) -> Self {
        CommandResponse {
            component_id: command_response.component_id,
            schema_type: command_response.schema_type,
        }
    }
}
}
