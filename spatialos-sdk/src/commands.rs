use crate::{
    component::{Component, ComponentId},
    entity::Entity,
    query::EntityQuery,
    schema::{self, DataPointer, FieldId, Owned, SchemaCommandRequest, SchemaCommandResponse},
    EntityId,
};
use spatialos_sdk_sys::worker::*;
use std::ops::DerefMut;

pub type CommandIndex = Worker_CommandIndex;

pub trait Commands: Sized + Clone {
    type Component: Component;
    type Request: Request<Commands = Self>;
    type Response: Response<Commands = Self>;
}

pub trait Request: Sized + Clone {
    type Commands: Commands<Request = Self>;

    fn from_schema(index: CommandIndex, request: &SchemaCommandRequest) -> schema::Result<Self>;
    fn into_schema(&self, request: &mut SchemaCommandRequest) -> CommandIndex;
}

pub struct CommandRequest {
    pub schema_data: Owned<SchemaCommandRequest>,
    pub component_id: ComponentId,
    pub command_index: CommandIndex,
}

impl<U: Request> From<&U> for CommandRequest {
    fn from(request: &U) -> Self {
        let mut schema_request = SchemaCommandRequest::new();
        let index = request.into_schema(schema_request.deref_mut());

        CommandRequest {
            schema_data: schema_request,
            component_id: <<U as Request>::Commands as Commands>::Component::ID,
            command_index: index,
        }
    }
}

pub trait Response: Sized + Clone {
    type Commands: Commands<Response = Self>;

    fn from_schema(index: CommandIndex, request: &SchemaCommandResponse) -> schema::Result<Self>;
    fn into_schema(&self, request: &mut SchemaCommandResponse) -> CommandIndex;
}

pub struct CommandResponse {
    pub schema_data: Owned<SchemaCommandResponse>,
    pub component_id: ComponentId,
    pub command_index: CommandIndex,
}

impl<U: Response> From<&U> for CommandResponse {
    fn from(response: &U) -> Self {
        let mut schema_response = SchemaCommandResponse::new();
        let index = response.into_schema(schema_response.deref_mut());

        CommandResponse {
            schema_data: schema_response,
            component_id: <<U as Response>::Commands as Commands>::Component::ID,
            command_index: index,
        }
    }
}

#[derive(Debug)]
pub struct CommandRequestRef<'a> {
    pub component_id: ComponentId,
    pub command_index: FieldId,
    pub schema_type: &'a SchemaCommandRequest,
}

impl<'a> CommandRequestRef<'a> {
    pub(crate) unsafe fn from_raw(request: &Worker_CommandRequest) -> Self {
        Self {
            component_id: request.component_id,
            command_index: request.command_index,
            schema_type: SchemaCommandRequest::from_raw(request.schema_type),
        }
    }

    pub(crate) fn get<C: Commands>(&self) -> Option<schema::Result<C::Request>> {
        if C::Component::ID != self.component_id {
            return None;
        }

        Some(C::Request::from_schema(
            self.command_index,
            self.schema_type,
        ))
    }
}

#[derive(Debug)]
pub struct CommandResponseRef<'a> {
    pub component_id: ComponentId,
    pub command_index: FieldId,
    pub schema_type: &'a SchemaCommandResponse,
}

impl<'a> CommandResponseRef<'a> {
    pub(crate) unsafe fn from_raw(response: &Worker_CommandResponse) -> Self {
        Self {
            component_id: response.component_id,
            command_index: response.command_index,
            schema_type: SchemaCommandResponse::from_raw(response.schema_type),
        }
    }

    pub fn get<C: Commands>(&self) -> Option<schema::Result<C::Response>> {
        if C::Component::ID != self.component_id {
            return None;
        }

        Some(C::Response::from_schema(
            self.command_index,
            self.schema_type,
        ))
    }
}

/// Additional parameters for sending command requests.
///
/// Additional parameters passed to [`Connection::send_command_request`]. Note that all
/// parameters are kept private and the struct can only be initialized with default
/// values in order to make it possible to add new parameters without a breaking change.
///
/// If you would like to use a method-chaining style when initializing the parameters,
/// the [tap] crate is recommended. The examples below demonstrate this.
///
/// # Parameters
///
/// * `allow_short_circuit: false` allow the command to skip being routed through SpatialOS if
///   the worker is sending a command to itself. This avoids a round trip, but
///   SpatialOS cannot guarantee that the command will be fully delivered and executed.
///   [See the docs for more information.][short-circuit]
///
/// # Examples
///
/// ```
/// use spatialos_sdk::commands::CommandParameters;
/// use tap::*;
///
/// let params = CommandParameters::new()
///     .tap(CommandParameters::allow_short_circuit);
/// ```
///
/// [short-circuit]: https://docs.improbable.io/reference/14.1/shared/design/commands#component-commands
/// [tap]: https://crates.io/crates/tap
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommandParameters {
    allow_short_circuit: bool,
}

impl CommandParameters {
    /// Creates a new `CommandParameters` with default values for all parameters.
    pub fn new() -> CommandParameters {
        Default::default()
    }

    /// Sets the `allow_short_circuit` parameter to `true`.
    pub fn allow_short_circuit(&mut self) {
        self.allow_short_circuit = true;
    }

    pub(crate) fn to_worker_sdk(self) -> Worker_CommandParameters {
        Worker_CommandParameters {
            allow_short_circuit: self.allow_short_circuit as u8,
        }
    }
}

// =============================== World Commands =============================== //
#[derive(Debug)]
pub struct ReserveEntityIdsRequest(pub u32);

#[derive(Debug)]
pub struct CreateEntityRequest(pub Entity, pub Option<EntityId>);

#[derive(Debug)]
pub struct DeleteEntityRequest(pub EntityId);

#[derive(Debug)]
pub struct EntityQueryRequest(pub EntityQuery);
