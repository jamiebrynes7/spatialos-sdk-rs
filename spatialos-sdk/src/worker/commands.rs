use crate::worker::query::EntityQuery;
use crate::worker::EntityId;

use spatialos_sdk_sys::worker::{
    Schema_CommandRequest, Schema_CommandResponse, Worker_CommandRequest, Worker_CommandResponse,
};

// TODO: Wrap Schema_CommandRequest
pub struct CommandRequest {
    pub component_id: u32,
    pub schema_type: *mut Schema_CommandRequest,
}

impl From<&Worker_CommandRequest> for CommandRequest {
    fn from(command_request: &Worker_CommandRequest) -> Self {
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

impl From<&Worker_CommandResponse> for CommandResponse {
    fn from(command_response: &Worker_CommandResponse) -> Self {
        CommandResponse {
            component_id: command_response.component_id,
            schema_type: command_response.schema_type,
        }
    }
}

pub struct IncomingCommandRequest {}

pub struct OutgoingCommandRequest {}

// =============================== World Commands =============================== //
pub struct ReserveEntityIdsRequest(pub u32);

pub struct CreateEntityRequest {}

pub struct DeleteEntityRequest(pub EntityId);

pub struct EntityQueryRequest(pub EntityQuery);
