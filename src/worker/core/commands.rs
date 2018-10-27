use worker::core::internal::*;

// TODO: Wrap Schema_CommandRequest
pub struct CommandRequest {
    pub component_id: u32,
    pub schema_type: *mut Schema_CommandRequest
}

impl CommandRequest {
    pub(crate) fn from_worker_sdk(command_request: &Worker_CommandRequest) -> Self {
        CommandRequest {
            component_id: command_request.component_id,
            schema_type: command_request.schema_type
        }
    }
}

// TODO: Wrap Schema_CommandResponse
pub struct CommandResponse {
    pub component_id: u32,
    pub schema_type: *mut Schema_CommandResponse
}

impl CommandResponse {
    pub(crate) fn from_worker_sdk(command_response: &Worker_CommandResponse) -> Self {
        CommandResponse {
            component_id: command_response.component_id,
            schema_type: command_response.schema_type
        }
    }
}

pub struct IncomingCommandRequest {
    
}

pub struct OutgoingCommandRequest {
    
}

// =============================== World Commands =============================== //
pub struct ReserveEntityIdsRequest {
    
}

pub struct CreateEntityRequest {
    
}

pub struct DeleteEntityRequest {
    
}

pub struct EntityQueryRequest {
    
}
