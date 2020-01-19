use crate::worker::schema::{self, *};
use spatialos_sdk_sys::worker::*;
use std::ops::DerefMut;

pub type ComponentId = Worker_ComponentId;
pub type CommandIndex = Worker_CommandIndex;

// A trait that's implemented by a component to convert to/from schema handle types.
pub trait Component: ObjectField {
    type Update: Update<Component = Self>;
    type CommandRequest;
    type CommandResponse;

    const ID: ComponentId;

    fn merge_update(&mut self, update: Self::Update);

    fn from_request(
        command_index: CommandIndex,
        request: &schema::SchemaCommandRequest,
    ) -> schema::Result<Self::CommandRequest>;
    fn from_response(
        command_index: CommandIndex,
        response: &schema::SchemaCommandResponse,
    ) -> schema::Result<Self::CommandResponse>;

    fn to_request(request: &Self::CommandRequest) -> Owned<SchemaCommandRequest>;
    fn to_response(response: &Self::CommandResponse) -> Owned<SchemaCommandResponse>;

    fn get_request_command_index(request: &Self::CommandRequest) -> u32;
    fn get_response_command_index(response: &Self::CommandResponse) -> u32;
}

pub trait Update: Sized + Clone {
    type Component: Component<Update = Self>;

    fn from_schema(update: &SchemaComponentUpdate) -> schema::Result<Self>;
    fn into_schema(&self, update: &mut SchemaComponentUpdate);
    fn merge(&mut self, other: Self);
}

pub struct ComponentUpdate {
    pub schema_data: Owned<SchemaComponentUpdate>,
    pub component_id: ComponentId,
}

impl<U: Update> From<&U> for ComponentUpdate {
    fn from(update: &U) -> Self {
        let mut schema_update = SchemaComponentUpdate::new();
        update.into_schema(schema_update.deref_mut());

        ComponentUpdate {
            schema_data: schema_update,
            component_id: U::Component::ID,
        }
    }
}

/// Additional parameters for sending component updates.
///
/// Additional parameters passed to [`Connection::send_component_update`]. Note that
/// all parameters are kept private and the struct can only be initialized with
/// default values in order to make it possible to add new parameters without a
/// breaking change.
///
/// If you would like to use a method-chaining style when initializing the parameters,
/// the [tap] crate is recommended. The examples below demonstrate this.
///
/// # Parameters
///
///
/// * `loopback` (disabled by default) - Allow the update to be sent back to the worker
///   without waiting to be routed through SpatialOS. This allows the worker to receive
///   the update op faster, but risks that the worker will receive the update when the
///   the update actually failed (due to the worker losing authority).
///
/// > TODO: Include a link to the relevant SpatialOS docs once they've been updated to
/// > include more information.
///
/// # Examples
///
/// ```
/// use spatialos_sdk::worker::component::UpdateParameters;
/// use tap::*;
///
/// let params = UpdateParameters::new()
///     .tap(UpdateParameters::allow_loopback);
/// ```
///
/// [tap]: https://crates.io/crates/tap
#[derive(Debug, Clone, Copy, Default)]
pub struct UpdateParameters {
    loopback: bool,
}

impl UpdateParameters {
    pub fn new() -> Self {
        Default::default()
    }

    /// Enables loopback short-circuiting for the component update message.
    ///
    /// [See the docs for more information.][short-circuit]
    ///
    /// [short-circuit]: https://docs.improbable.io/reference/13.5/shared/design/commands#properties
    pub fn allow_loopback(&mut self) {
        self.loopback = true;
    }

    pub(crate) fn to_worker_sdk(self) -> Worker_UpdateParameters {
        Worker_UpdateParameters {
            loopback: if self.loopback {
                Worker_ComponentUpdateLoopback_WORKER_COMPONENT_UPDATE_LOOPBACK_SHORT_CIRCUITED as _
            } else {
                Worker_ComponentUpdateLoopback_WORKER_COMPONENT_UPDATE_LOOPBACK_NONE as _
            },
        }
    }
}

#[derive(Debug)]
pub struct ComponentDataRef<'a> {
    pub component_id: ComponentId,
    pub schema_type: &'a SchemaComponentData,
}

impl<'a> ComponentDataRef<'a> {
    pub(crate) unsafe fn from_raw(data: &'a Worker_ComponentData) -> Self {
        Self {
            component_id: data.component_id,
            schema_type: SchemaComponentData::from_raw(data.schema_type),
        }
    }

    pub fn get<C: Component>(&self) -> Option<schema::Result<C>> {
        if C::ID != self.component_id {
            return None;
        }

        Some(self.schema_type.deserialize())
    }
}

#[derive(Debug)]
pub struct ComponentUpdateRef<'a> {
    pub component_id: ComponentId,
    pub schema_type: &'a SchemaComponentUpdate,
}

impl<'a> ComponentUpdateRef<'a> {
    pub(crate) unsafe fn from_raw(update: &Worker_ComponentUpdate) -> Self {
        Self {
            component_id: update.component_id,
            schema_type: SchemaComponentUpdate::from_raw(update.schema_type),
        }
    }

    pub(crate) fn get<C: Component>(&self) -> Option<schema::Result<C::Update>> {
        if C::ID != self.component_id {
            return None;
        }

        Some(self.schema_type.deserialize())
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

    // NOTE: We manually declare that the request impl `ObjectField`
    // here, but in practice this will always be true for all component types. Future
    // iterations should clean this up such that the `Component` trait can imply these
    // other bounds automatically (i.e. by making them super traits of `Component`).
    pub(crate) fn get<C>(&self) -> Option<schema::Result<C::CommandRequest>>
    where
        C: Component,
        C::CommandRequest: ObjectField,
    {
        if C::ID != self.component_id {
            return None;
        }

        Some(ObjectField::from_object(self.schema_type.object()))
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

    // NOTE: We manually declare that the response impl `ObjectField`
    // here, but in practice this will always be true for all component types. Future
    // iterations should clean this up such that the `Component` trait can imply these
    // other bounds automatically (i.e. by making them super traits of `Component`).
    pub fn get<C>(&self) -> Option<schema::Result<C::CommandResponse>>
    where
        C: Component,
        C::CommandResponse: ObjectField,
    {
        if C::ID != self.component_id {
            return None;
        }

        Some(ObjectField::from_object(self.schema_type.object()))
    }
}
