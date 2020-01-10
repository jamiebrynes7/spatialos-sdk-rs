use crate::worker::{
    handle,
    schema::{self, *},
};
use spatialos_sdk_sys::worker::*;
use std::{borrow::Cow, ptr::NonNull};

// Re-export inventory so generated code doesn't require the user to add inventory to their
// Cargo.toml
pub use inventory;

pub type ComponentId = Worker_ComponentId;
pub type CommandIndex = Worker_CommandIndex;

// A trait that's implemented by a component to convert to/from schema handle types.
pub trait Component: ObjectField {
    type Update: Update<Component = Self>;
    type CommandRequest: 'static;
    type CommandResponse: 'static;

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

pub trait Update: 'static + Sized + Clone {
    type Component: Component<Update = Self>;

    fn from_schema(update: &SchemaComponentUpdate) -> schema::Result<Self>;
    fn into_schema(&self, update: &mut SchemaComponentUpdate);
    fn merge(&mut self, other: Self);
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
pub(crate) struct ComponentDataRef<'a> {
    component_id: ComponentId,
    schema_type: &'a SchemaComponentData,
    user_handle: Option<NonNull<Worker_ComponentDataHandle>>,
}

impl<'a> ComponentDataRef<'a> {
    pub(crate) unsafe fn from_raw(data: &'a Worker_ComponentData) -> Self {
        Self {
            component_id: data.component_id,
            schema_type: SchemaComponentData::from_raw(data.schema_type),
            user_handle: NonNull::new(data.user_handle),
        }
    }

    pub fn get<C: Component>(&self) -> Option<Cow<'_, schema::Result<C>>> {
        if C::ID != self.component_id {
            return None;
        }

        let cow = if let Some(user_handle) = &self.user_handle {
            Cow::Borrowed(unsafe { handle::deref_raw(user_handle.as_ptr()) })
        } else {
            Cow::Owned(ObjectField::from_object(self.schema_type.fields()))
        };

        Some(cow)
    }
}

#[derive(Debug)]
pub(crate) struct ComponentUpdateRef<'a> {
    component_id: ComponentId,
    schema_type: &'a SchemaComponentUpdate,
    user_handle: Option<NonNull<Worker_ComponentUpdateHandle>>,
}

impl<'a> ComponentUpdateRef<'a> {
    pub(crate) unsafe fn from_raw(update: &Worker_ComponentUpdate) -> Self {
        Self {
            component_id: update.component_id,
            schema_type: SchemaComponentUpdate::from_raw(update.schema_type),
            user_handle: NonNull::new(update.user_handle),
        }
    }

    pub(crate) fn get<C: Component>(&self) -> Option<Cow<'_, schema::Result<C::Update>>> {
        if C::ID != self.component_id {
            return None;
        }

        let cow = if let Some(user_handle) = &self.user_handle {
            Cow::Borrowed(unsafe { handle::deref_raw(user_handle.as_ptr()) })
        } else {
            Cow::Owned(Update::from_schema(&self.schema_type))
        };

        Some(cow)
    }
}

#[derive(Debug)]
pub(crate) struct CommandRequestRef<'a> {
    component_id: ComponentId,
    command_index: FieldId,
    schema_type: &'a SchemaCommandRequest,
    user_handle: Option<NonNull<Worker_CommandRequestHandle>>,
}

impl<'a> CommandRequestRef<'a> {
    pub(crate) unsafe fn from_raw(request: &Worker_CommandRequest) -> Self {
        Self {
            component_id: request.component_id,
            command_index: request.command_index,
            schema_type: SchemaCommandRequest::from_raw(request.schema_type),
            user_handle: NonNull::new(request.user_handle),
        }
    }

    // NOTE: We manually declare that the request impl `ObjectField`
    // here, but in practice this will always be true for all component types. Future
    // iterations should clean this up such that the `Component` trait can imply these
    // other bounds automatically (i.e. by making them super traits of `Component`).
    pub(crate) fn get<C>(&self) -> Option<Cow<'_, schema::Result<C::CommandRequest>>>
    where
        C: Component,
        C::CommandRequest: ObjectField,
    {
        if C::ID != self.component_id {
            return None;
        }

        let cow = if let Some(user_handle) = &self.user_handle {
            Cow::Borrowed(unsafe { handle::deref_raw(user_handle.as_ptr()) })
        } else {
            Cow::Owned(ObjectField::from_object(self.schema_type.object()))
        };

        Some(cow)
    }
}

#[derive(Debug)]
pub struct CommandResponseRef<'a> {
    component_id: ComponentId,
    command_index: FieldId,
    schema_type: &'a SchemaCommandResponse,
    user_handle: Option<NonNull<Worker_CommandResponseHandle>>,
}

impl<'a> CommandResponseRef<'a> {
    pub(crate) unsafe fn from_raw(response: &Worker_CommandResponse) -> Self {
        Self {
            component_id: response.component_id,
            command_index: response.command_index,
            schema_type: SchemaCommandResponse::from_raw(response.schema_type),
            user_handle: NonNull::new(response.user_handle),
        }
    }

    // NOTE: We manually declare that the response impl `ObjectField`
    // here, but in practice this will always be true for all component types. Future
    // iterations should clean this up such that the `Component` trait can imply these
    // other bounds automatically (i.e. by making them super traits of `Component`).
    pub fn get<C>(&self) -> Option<Cow<'_, schema::Result<C::CommandResponse>>>
    where
        C: Component,
        C::CommandResponse: ObjectField,
    {
        if C::ID != self.component_id {
            return None;
        }

        let cow = if let Some(user_handle) = &self.user_handle {
            Cow::Borrowed(unsafe { handle::deref_raw(user_handle.as_ptr()) })
        } else {
            Cow::Owned(ObjectField::from_object(self.schema_type.object()))
        };

        Some(cow)
    }
}
