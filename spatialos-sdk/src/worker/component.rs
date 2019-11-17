use crate::worker::schema::{self, *};
use spatialos_sdk_sys::worker::*;
use std::{
    borrow::Cow, collections::hash_map::HashMap, mem, os::raw, ptr, ptr::NonNull, sync::Arc,
};

// Re-export inventory so generated code doesn't require the user to add inventory to their
// Cargo.toml
pub use inventory;

pub type ComponentId = Worker_ComponentId;
pub type CommandIndex = Worker_CommandIndex;

pub trait ComponentUpdate<C: Component> {
    fn merge(&mut self, update: Self);
}

pub trait ComponentData<C: Component> {
    fn merge(&mut self, update: C::Update);
}

// A trait that's implemented by a component to convert to/from schema handle types.
pub trait Component
where
    Self: std::marker::Sized,
{
    type Update;
    type CommandRequest;
    type CommandResponse;

    const ID: ComponentId;

    fn from_data(data: &schema::SchemaComponentData) -> Result<Self, String>;
    fn from_update(update: &schema::SchemaComponentUpdate) -> Result<Self::Update, String>;
    fn from_request(
        command_index: CommandIndex,
        request: &schema::SchemaCommandRequest,
    ) -> Result<Self::CommandRequest, String>;
    fn from_response(
        command_index: CommandIndex,
        response: &schema::SchemaCommandResponse,
    ) -> Result<Self::CommandResponse, String>;

    fn to_data(data: &Self) -> Result<Owned<SchemaComponentData>, String>;
    fn to_update(update: &Self::Update) -> Result<Owned<SchemaComponentUpdate>, String>;
    fn to_request(request: &Self::CommandRequest) -> Result<Owned<SchemaCommandRequest>, String>;
    fn to_response(
        response: &Self::CommandResponse,
    ) -> Result<Owned<SchemaCommandResponse>, String>;

    fn get_request_command_index(request: &Self::CommandRequest) -> u32;
    fn get_response_command_index(response: &Self::CommandResponse) -> u32;
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

    // NOTE: We manually declare that the component impl `ObjectField` and `Clone`
    // here, but in practice this will always be true for all component types. Future
    // iterations should clean this up such that the `Component` trait can imply these
    // other bounds automatically (i.e. by making them super traits of `Component`).
    pub fn get<C: Component + ObjectField + Clone>(&self) -> Option<Cow<'_, C>> {
        if C::ID != self.component_id {
            return None;
        }

        let cow = if let Some(user_handle) = &self.user_handle {
            let component = unsafe { &*user_handle.cast().as_ptr() };
            Cow::Borrowed(component)
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

    // NOTE: We manually declare that the update impl `ObjectField` and `Clone`
    // here, but in practice this will always be true for all component types. Future
    // iterations should clean this up such that the `Component` trait can imply these
    // other bounds automatically (i.e. by making them super traits of `Component`).
    pub(crate) fn get<C>(&self) -> Option<Cow<'_, C::Update>>
    where
        C: Component,
        C::Update: ObjectField + Clone,
    {
        if C::ID != self.component_id {
            return None;
        }

        let cow = if let Some(user_handle) = &self.user_handle {
            let component = unsafe { &*user_handle.cast().as_ptr() };
            Cow::Borrowed(component)
        } else {
            Cow::Owned(ObjectField::from_object(self.schema_type.fields()))
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

    // NOTE: We manually declare that the update impl `ObjectField` and `Clone`
    // here, but in practice this will always be true for all component types. Future
    // iterations should clean this up such that the `Component` trait can imply these
    // other bounds automatically (i.e. by making them super traits of `Component`).
    pub(crate) fn get<C>(&self) -> Option<Cow<'_, C::CommandRequest>>
    where
        C: Component,
        C::CommandRequest: ObjectField + Clone,
    {
        if C::ID != self.component_id {
            return None;
        }

        let cow = if let Some(user_handle) = &self.user_handle {
            let component = unsafe { &*user_handle.cast().as_ptr() };
            Cow::Borrowed(component)
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

    // NOTE: We manually declare that the update impl `ObjectField` and `Clone`
    // here, but in practice this will always be true for all component types. Future
    // iterations should clean this up such that the `Component` trait can imply these
    // other bounds automatically (i.e. by making them super traits of `Component`).
    pub fn get<C>(&self) -> Option<Cow<'_, C::CommandResponse>>
    where
        C: Component,
        C::CommandResponse: ObjectField + Clone,
    {
        if C::ID != self.component_id {
            return None;
        }

        let cow = if let Some(user_handle) = &self.user_handle {
            let component = unsafe { &*user_handle.cast().as_ptr() };
            Cow::Borrowed(component)
        } else {
            Cow::Owned(ObjectField::from_object(self.schema_type.object()))
        };

        Some(cow)
    }
}

inventory::collect!(VTable);

lazy_static::lazy_static! {
    pub(crate) static ref DATABASE: ComponentDatabase = {

        let mut vtables = Vec::new();
        let mut index_map = HashMap::new();

        for (i, table) in inventory::iter::<VTable>.into_iter().enumerate() {
            vtables.push(table.vtable);
            index_map.insert(table.vtable.component_id, i);
        }

        ComponentDatabase {
            component_vtables: vtables,
            index_map,
        }
    };
}

#[derive(Clone, Debug)]
pub(crate) struct ComponentDatabase {
    component_vtables: Vec<Worker_ComponentVtable>,
    index_map: HashMap<ComponentId, usize>,
}

impl ComponentDatabase {
    pub(crate) fn has_vtable(&self, id: ComponentId) -> bool {
        self.index_map.contains_key(&id)
    }

    pub(crate) fn get_vtable(&self, id: ComponentId) -> Option<&Worker_ComponentVtable> {
        self.index_map
            .get(&id)
            .map(|index| &self.component_vtables[*index])
    }

    pub(crate) fn to_worker_sdk(&self) -> *const Worker_ComponentVtable {
        self.component_vtables.as_ptr()
    }

    pub(crate) fn len(&self) -> usize {
        self.component_vtables.len()
    }
}

unsafe impl Sync for ComponentDatabase {}
unsafe impl Send for ComponentDatabase {}

pub(crate) fn handle_allocate<T>(data: T) -> *mut raw::c_void {
    Arc::into_raw(Arc::new(data)) as *mut _
}

pub(crate) unsafe fn handle_free<T>(handle: *mut raw::c_void) {
    let _ = Arc::<T>::from_raw(handle as *const _);
}

pub(crate) unsafe fn handle_copy<T>(handle: *mut raw::c_void) -> *mut raw::c_void {
    let original = Arc::<T>::from_raw(handle as *const _);
    let copy = original.clone();
    mem::forget(original);
    Arc::into_raw(copy) as *mut _
}

pub struct VTable {
    vtable: Worker_ComponentVtable,
}

impl VTable {
    pub fn new<C: Component>() -> Self {
        VTable {
            vtable: Worker_ComponentVtable {
                component_id: C::ID,
                user_data: ptr::null_mut(),
                command_request_free: Some(vtable_command_request_free::<C>),
                command_request_copy: Some(vtable_command_request_copy::<C>),
                command_request_deserialize: Some(vtable_command_request_deserialize::<C>),
                command_request_serialize: Some(vtable_command_request_serialize::<C>),
                command_response_free: Some(vtable_command_response_free::<C>),
                command_response_copy: Some(vtable_command_response_copy::<C>),
                command_response_deserialize: Some(vtable_command_response_deserialize::<C>),
                command_response_serialize: Some(vtable_command_response_serialize::<C>),
                component_data_free: Some(vtable_component_data_free::<C>),
                component_data_copy: Some(vtable_component_data_copy::<C>),
                component_data_deserialize: Some(vtable_component_data_deserialize::<C>),
                component_data_serialize: Some(vtable_component_data_serialize::<C>),
                component_update_free: Some(vtable_component_update_free::<C>),
                component_update_copy: Some(vtable_component_update_copy::<C>),
                component_update_deserialize: Some(vtable_component_update_deserialize::<C>),
                component_update_serialize: Some(vtable_component_update_serialize::<C>),
            },
        }
    }
}

unsafe extern "C" fn vtable_component_data_free<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) {
    handle_free::<C>(handle)
}

unsafe extern "C" fn vtable_component_data_copy<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) -> *mut raw::c_void {
    handle_copy::<C>(handle)
}

unsafe extern "C" fn vtable_component_data_deserialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    data: *mut Schema_ComponentData,
    handle_out: *mut *mut Worker_ComponentDataHandle,
) -> u8 {
    let schema_data = schema::SchemaComponentData::from_raw(data);
    let deserialized_result = C::from_data(schema_data);
    if let Ok(deserialized_data) = deserialized_result {
        *handle_out = handle_allocate(deserialized_data);
        1
    } else {
        0
    }
}

unsafe extern "C" fn vtable_component_data_serialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    data: *mut *mut Schema_ComponentData,
) {
    let client_data = &*(handle as *const C);
    if let Ok(schema_data) = C::to_data(client_data) {
        *data = schema_data.into_raw();
    } else {
        *data = ptr::null_mut();
    }
}

unsafe extern "C" fn vtable_component_update_free<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) {
    handle_free::<C>(handle)
}

unsafe extern "C" fn vtable_component_update_copy<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) -> *mut raw::c_void {
    handle_copy::<C>(handle)
}

unsafe extern "C" fn vtable_component_update_deserialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    update: *mut Schema_ComponentUpdate,
    handle_out: *mut *mut Worker_ComponentUpdateHandle,
) -> u8 {
    let schema_update = SchemaComponentUpdate::from_raw(update);
    let deserialized_result = C::from_update(schema_update);
    if let Ok(deserialized_update) = deserialized_result {
        *handle_out = handle_allocate(deserialized_update);
        1
    } else {
        0
    }
}

unsafe extern "C" fn vtable_component_update_serialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    update: *mut *mut Schema_ComponentUpdate,
) {
    let data = &*(handle as *const _);
    let schema_result = C::to_update(data);
    if let Ok(schema_update) = schema_result {
        *update = schema_update.into_raw();
    } else {
        *update = ptr::null_mut();
    }
}

unsafe extern "C" fn vtable_command_request_free<C: Component>(
    _: u32,
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) {
    handle_free::<C>(handle)
}

unsafe extern "C" fn vtable_command_request_copy<C: Component>(
    _: u32,
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) -> *mut raw::c_void {
    handle_copy::<C>(handle)
}

unsafe extern "C" fn vtable_command_request_deserialize<C: Component>(
    _: u32,
    command_index: u32,
    _: *mut raw::c_void,
    request: *mut Schema_CommandRequest,
    handle_out: *mut *mut Worker_CommandRequestHandle,
) -> u8 {
    let schema_request = SchemaCommandRequest::from_raw(request);
    let deserialized_result = C::from_request(command_index, &schema_request);
    if let Ok(deserialized_request) = deserialized_result {
        *handle_out = handle_allocate(deserialized_request);
        1
    } else {
        0
    }
}

unsafe extern "C" fn vtable_command_request_serialize<C: Component>(
    _: u32,
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    request: *mut *mut Schema_CommandRequest,
) {
    let data = &*(handle as *const _);
    let schema_result = C::to_request(data);
    if let Ok(schema_request) = schema_result {
        *request = schema_request.into_raw();
    } else {
        *request = ptr::null_mut();
    }
}

unsafe extern "C" fn vtable_command_response_free<C: Component>(
    _: u32,
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) {
    handle_free::<C>(handle)
}

unsafe extern "C" fn vtable_command_response_copy<C: Component>(
    _: u32,
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) -> *mut raw::c_void {
    handle_copy::<C>(handle)
}

unsafe extern "C" fn vtable_command_response_deserialize<C: Component>(
    _: u32,
    command_index: u32,
    _: *mut raw::c_void,
    response: *mut Schema_CommandResponse,
    handle_out: *mut *mut Worker_CommandRequestHandle,
) -> u8 {
    let schema_response = SchemaCommandResponse::from_raw(response);
    let deserialized_result = C::from_response(command_index, &schema_response);
    if let Ok(deserialized_response) = deserialized_result {
        *handle_out = handle_allocate(deserialized_response);
        1
    } else {
        0
    }
}

unsafe extern "C" fn vtable_command_response_serialize<C: Component>(
    _: u32,
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    response: *mut *mut Schema_CommandResponse,
) {
    let data = &*(handle as *const _);
    let schema_result = C::to_response(data);
    if let Ok(schema_response) = schema_result {
        *response = schema_response.into_raw();
    } else {
        *response = ptr::null_mut();
    }
}
