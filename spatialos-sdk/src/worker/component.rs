use crate::worker::internal::schema;
use spatialos_sdk_sys::worker::*;
use std::os::raw;
use std::sync::Arc;
use std::{mem, ptr};

pub type ComponentId = u32;

pub trait ComponentUpdate<C: Component> {
    fn merge(&mut self, update: Self);
}

pub trait ComponentData<C: Component> {
    fn merge(&mut self, update: C::Update);
}

// A trait that's implemented by a type to convert to/from schema objects.
pub trait TypeConversion
where
    Self: std::marker::Sized,
{
    fn from_type(input: &schema::SchemaObject) -> Result<Self, String>;
    fn to_type(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String>;
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
    fn from_request(request: &schema::SchemaCommandRequest)
        -> Result<Self::CommandRequest, String>;
    fn from_response(
        response: &schema::SchemaCommandResponse,
    ) -> Result<Self::CommandResponse, String>;

    fn to_data(data: &Self) -> Result<schema::SchemaComponentData, String>;
    fn to_update(update: &Self::Update) -> Result<schema::SchemaComponentUpdate, String>;
    fn to_request(request: &Self::CommandRequest) -> Result<schema::SchemaCommandRequest, String>;
    fn to_response(
        response: &Self::CommandResponse,
    ) -> Result<schema::SchemaCommandResponse, String>;

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
/// * `loopback: false` allow the update to be sent back to the worker without waiting
///   to be routed through SpatialOS. This allows the worker to receive the update op
///   faster, but risks that the worker won't know if the update actually failed. [See
///   the docs for more information.][short-circuit]
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
/// [short-circuit]: https://docs.improbable.io/reference/13.5/shared/design/commands#properties
/// [tap]: https://crates.io/crates/tap
#[derive(Debug, Clone, Default)]
pub struct UpdateParameters {
    loopback: bool,
}

impl UpdateParameters {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn allow_loopback(&mut self) {
        self.loopback = true;
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_Alpha_UpdateParameters {
        Worker_Alpha_UpdateParameters {
            loopback: if self.loopback {
                Worker_Alpha_ComponentUpdateLoopback_WORKER_COMPONENT_UPDATE_LOOPBACK_SHORT_CIRCUITED as _
            } else {
                Worker_Alpha_ComponentUpdateLoopback_WORKER_COMPONENT_UPDATE_LOOPBACK_NONE as _
            },
        }
    }
}

// Internal untyped component data objects.
pub(crate) mod internal {
    use crate::worker::internal::schema::*;
    use spatialos_sdk_sys::worker::*;
    use std::marker::PhantomData;

    use crate::worker::component::ComponentId;

    #[derive(Debug)]
    pub struct ComponentData<'a> {
        pub component_id: ComponentId,
        pub schema_type: SchemaComponentData,
        pub user_handle: *const Worker_ComponentDataHandle,

        // NOTE: `user_handle` is borrowing data owned by the parent object, but it's a
        // type-erased pointer that may be null, so we just mark that we're borrowing
        // *something*.
        pub _marker: PhantomData<&'a ()>,
    }

    impl<'a> From<&'a Worker_ComponentData> for ComponentData<'a> {
        fn from(data: &Worker_ComponentData) -> Self {
            ComponentData {
                component_id: data.component_id,
                schema_type: SchemaComponentData {
                    component_id: data.component_id,
                    internal: data.schema_type,
                },
                user_handle: data.user_handle,
                _marker: PhantomData,
            }
        }
    }

    #[derive(Debug)]
    pub struct ComponentUpdate<'a> {
        pub component_id: ComponentId,
        pub schema_type: SchemaComponentUpdate,
        pub user_handle: *const Worker_ComponentUpdateHandle,

        // NOTE: `user_handle` is borrowing data owned by the parent object, but it's a
        // type-erased pointer that may be null, so we just mark that we're borrowing
        // *something*.
        pub _marker: PhantomData<&'a ()>,
    }

    impl<'a> From<&'a Worker_ComponentUpdate> for ComponentUpdate<'a> {
        fn from(update: &Worker_ComponentUpdate) -> Self {
            ComponentUpdate {
                component_id: update.component_id,
                schema_type: SchemaComponentUpdate {
                    component_id: update.component_id,
                    internal: update.schema_type,
                },
                user_handle: update.user_handle,
                _marker: PhantomData,
            }
        }
    }

    #[derive(Debug)]
    pub struct CommandRequest<'a> {
        pub component_id: ComponentId,
        pub schema_type: SchemaCommandRequest,
        pub user_handle: *const Worker_CommandRequestHandle,

        // NOTE: `user_handle` is borrowing data owned by the parent object, but it's a
        // type-erased pointer that may be null, so we just mark that we're borrowing
        // *something*.
        pub _marker: PhantomData<&'a ()>,
    }

    impl<'a> From<&'a Worker_CommandRequest> for CommandRequest<'a> {
        fn from(request: &Worker_CommandRequest) -> Self {
            CommandRequest {
                component_id: request.component_id,
                schema_type: SchemaCommandRequest {
                    component_id: request.component_id,
                    internal: request.schema_type,
                },
                user_handle: request.user_handle,
                _marker: PhantomData,
            }
        }
    }

    #[derive(Debug)]
    pub struct CommandResponse<'a> {
        pub component_id: ComponentId,
        pub schema_type: SchemaCommandResponse,
        pub user_handle: *const Worker_CommandResponseHandle,

        // NOTE: `user_handle` is borrowing data owned by the parent object, but it's a
        // type-erased pointer that may be null, so we just mark that we're borrowing
        // *something*.
        pub _marker: PhantomData<&'a ()>,
    }

    impl<'a> From<&'a Worker_CommandResponse> for CommandResponse<'a> {
        fn from(response: &Worker_CommandResponse) -> Self {
            CommandResponse {
                component_id: response.component_id,
                schema_type: SchemaCommandResponse {
                    component_id: response.component_id,
                    internal: response.schema_type,
                },
                user_handle: response.user_handle,
                _marker: PhantomData,
            }
        }
    }
}

// A data structure which represents all known component types. Used to generate an array of vtables to pass
// to the connection object.
#[derive(Default)]
pub struct ComponentDatabase {
    component_vtables: Vec<Worker_ComponentVtable>,
}

impl ComponentDatabase {
    pub fn new() -> Self {
        ComponentDatabase {
            component_vtables: Vec::new(),
        }
    }

    pub fn add_component<C: Component>(mut self) -> Self {
        self.component_vtables.push(create_component_vtable::<C>());
        self
    }

    pub(crate) fn to_worker_sdk(&self) -> *const Worker_ComponentVtable {
        self.component_vtables.as_ptr()
    }

    pub(crate) fn len(&self) -> usize {
        self.component_vtables.len()
    }
}

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

// Vtable implementation functions.
fn create_component_vtable<C: Component>() -> Worker_ComponentVtable {
    Worker_ComponentVtable {
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
    let schema_data = schema::SchemaComponentData {
        component_id: C::ID,
        internal: data,
    };
    let deserialized_result = C::from_data(&schema_data);
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
        *data = schema_data.internal;
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
    let schema_update = schema::SchemaComponentUpdate {
        component_id: C::ID,
        internal: update,
    };
    let deserialized_result = C::from_update(&schema_update);
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
        *update = schema_update.internal;
    } else {
        *update = ptr::null_mut();
    }
}

unsafe extern "C" fn vtable_command_request_free<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) {
    handle_free::<C>(handle)
}

unsafe extern "C" fn vtable_command_request_copy<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) -> *mut raw::c_void {
    handle_copy::<C>(handle)
}

unsafe extern "C" fn vtable_command_request_deserialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    request: *mut Schema_CommandRequest,
    handle_out: *mut *mut Worker_CommandRequestHandle,
) -> u8 {
    let schema_request = schema::SchemaCommandRequest {
        component_id: C::ID,
        internal: request,
    };
    let deserialized_result = C::from_request(&schema_request);
    if let Ok(deserialized_request) = deserialized_result {
        *handle_out = handle_allocate(deserialized_request);
        1
    } else {
        0
    }
}

unsafe extern "C" fn vtable_command_request_serialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    request: *mut *mut Schema_CommandRequest,
) {
    let data = &*(handle as *const _);
    let schema_result = C::to_request(data);
    if let Ok(schema_request) = schema_result {
        *request = schema_request.internal;
    } else {
        *request = ptr::null_mut();
    }
}

unsafe extern "C" fn vtable_command_response_free<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) {
    handle_free::<C>(handle)
}

unsafe extern "C" fn vtable_command_response_copy<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) -> *mut raw::c_void {
    handle_copy::<C>(handle)
}

unsafe extern "C" fn vtable_command_response_deserialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    response: *mut Schema_CommandResponse,
    handle_out: *mut *mut Worker_CommandRequestHandle,
) -> u8 {
    let schema_response = schema::SchemaCommandResponse {
        component_id: C::ID,
        internal: response,
    };
    let deserialized_result = C::from_response(&schema_response);
    if let Ok(deserialized_response) = deserialized_result {
        *handle_out = handle_allocate(deserialized_response);
        1
    } else {
        0
    }
}

unsafe extern "C" fn vtable_command_response_serialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    response: *mut *mut Schema_CommandResponse,
) {
    let data = &*(handle as *const _);
    let schema_result = C::to_response(data);
    if let Ok(schema_response) = schema_result {
        *response = schema_response.internal;
    } else {
        *response = ptr::null_mut();
    }
}
