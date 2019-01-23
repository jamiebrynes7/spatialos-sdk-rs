use crate::worker::query::EntityQuery;
use crate::worker::{ComponentId, EntityId};

use spatialos_sdk_sys::worker::{
    Schema_CommandRequest, Schema_CommandResponse, Worker_CommandRequest, Worker_CommandResponse,
};

// TODO: Wrap Schema_CommandRequest
pub struct CommandRequest<'a> {
    pub component_id: ComponentId,
    pub schema_type: &'a Schema_CommandRequest,
}

impl<'a> From<&'a Worker_CommandRequest> for CommandRequest<'a> {
    fn from(command_request: &'a Worker_CommandRequest) -> Self {
        CommandRequest {
            component_id: command_request.component_id,
            schema_type: unsafe { &*command_request.schema_type },
        }
    }
}

// TODO: Wrap Schema_CommandResponse
pub struct CommandResponse<'a> {
    pub component_id: ComponentId,
    pub schema_type: &'a Schema_CommandResponse,
}

impl<'a> From<&'a Worker_CommandResponse> for CommandResponse<'a> {
    fn from(command_response: &Worker_CommandResponse) -> Self {
        CommandResponse {
            component_id: command_response.component_id,
            schema_type: unsafe { &*command_response.schema_type },
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
