use crate::worker::schema::{self, SchemaObjectType};
use maybe_owned::MaybeOwned;
use spatialos_sdk_sys::worker::*;
use std::{collections::hash_map::HashMap, mem, os::raw, ptr, sync::Arc};

// Re-export inventory so generated code doesn't require the user to add inventory to their
// Cargo.toml
pub use inventory;

pub type ComponentId = u32;
pub type UserHandle = *mut raw::c_void;

/// A component type as defined in a schema file.
pub trait Component: SchemaObjectType {
    const ID: ComponentId;

    type Update: Update<Component = Self>;
}

pub trait Update: Sized {
    type Component: Component<Update = Self>;
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
#[derive(Debug, Clone, Default)]
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

/// Component data returned from the runtime.
#[derive(Debug)]
pub(crate) struct ComponentDataRef<'a> {
    pub component_id: ComponentId,
    pub schema_type: schema::ComponentDataRef<'a>,
    pub user_handle: *mut Worker_ComponentDataHandle,
}

impl<'a> ComponentDataRef<'a> {
    pub unsafe fn from_raw(data: &'a Worker_ComponentData) -> Self {
        Self {
            component_id: data.component_id,
            schema_type: schema::ComponentDataRef::from_raw(&*data.schema_type),
            user_handle: data.user_handle,
        }
    }

    pub fn get<C: Component>(&self) -> Option<MaybeOwned<'a, C>> {
        if C::ID != self.component_id {
            return None;
        }

        if !self.user_handle.is_null() {
            let component = unsafe { *(self.user_handle as *const _) };
            Some(MaybeOwned::Borrowed(component))
        } else {
            Some(MaybeOwned::Owned(self.schema_type.deserialize()))
        }
    }
}

// #[derive(Debug)]
// pub struct ComponentUpdate<'a> {
//     pub component_id: ComponentId,
//     pub schema_type: SchemaComponentUpdate<'a>,
//     pub user_handle: *const Worker_ComponentUpdateHandle,

//     // NOTE: `user_handle` is borrowing data owned by the parent object, but it's a
//     // type-erased pointer that may be null, so we just mark that we're borrowing
//     // *something*.
//     pub _marker: PhantomData<&'a ()>,
// }

// impl<'a> From<&'a Worker_ComponentUpdate> for ComponentUpdate<'a> {
//     fn from(update: &Worker_ComponentUpdate) -> Self {
//         ComponentUpdate {
//             component_id: update.component_id,
//             schema_type: unsafe { SchemaComponentUpdate::from_raw(update.schema_type) },
//             user_handle: update.user_handle,
//             _marker: PhantomData,
//         }
//     }
// }

// #[derive(Debug)]
// pub struct CommandRequest<'a> {
//     pub component_id: ComponentId,
//     pub schema_type: SchemaCommandRequest,
//     pub user_handle: *const Worker_CommandRequestHandle,

//     // NOTE: `user_handle` is borrowing data owned by the parent object, but it's a
//     // type-erased pointer that may be null, so we just mark that we're borrowing
//     // *something*.
//     pub _marker: PhantomData<&'a ()>,
// }

// impl<'a> From<&'a Worker_CommandRequest> for CommandRequest<'a> {
//     fn from(request: &Worker_CommandRequest) -> Self {
//         CommandRequest {
//             component_id: request.component_id,
//             schema_type: SchemaCommandRequest {
//                 component_id: request.component_id,
//                 internal: request.schema_type,
//             },
//             user_handle: request.user_handle,
//             _marker: PhantomData,
//         }
//     }
// }

// #[derive(Debug)]
// pub struct CommandResponse<'a> {
//     pub component_id: ComponentId,
//     pub schema_type: SchemaCommandResponse,
//     pub user_handle: *const Worker_CommandResponseHandle,

//     // NOTE: `user_handle` is borrowing data owned by the parent object, but it's a
//     // type-erased pointer that may be null, so we just mark that we're borrowing
//     // *something*.
//     pub _marker: PhantomData<&'a ()>,
// }

// impl<'a> From<&'a Worker_CommandResponse> for CommandResponse<'a> {
//     fn from(response: &Worker_CommandResponse) -> Self {
//         CommandResponse {
//             component_id: response.component_id,
//             schema_type: SchemaCommandResponse {
//                 component_id: response.component_id,
//                 internal: response.schema_type,
//             },
//             user_handle: response.user_handle,
//             _marker: PhantomData,
//         }
//     }
// }

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

pub(crate) fn handle_allocate<T>(data: T) -> UserHandle {
    Arc::into_raw(Arc::new(data)) as *mut _
}

pub(crate) unsafe fn handle_free<T>(handle: UserHandle) {
    let _ = Arc::<T>::from_raw(handle as *const _);
}

pub(crate) unsafe fn handle_copy<T>(handle: UserHandle) -> UserHandle {
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
    _component_id: u32,
    _user_data: *mut raw::c_void,
    handle: *mut raw::c_void,
) {
    handle_free::<C>(handle);
}

unsafe extern "C" fn vtable_component_data_copy<C: Component>(
    _component_id: u32,
    _user_data: *mut raw::c_void,
    handle: *mut raw::c_void,
) -> *mut raw::c_void {
    handle_copy::<C>(handle)
}

unsafe extern "C" fn vtable_component_data_deserialize<C: Component>(
    _component_id: u32,
    _user_data: *mut raw::c_void,
    schema_data: *mut Schema_ComponentData,
    handle_out: *mut *mut Worker_ComponentDataHandle,
) -> u8 {
    let schema_data = schema::ComponentDataRef::from_raw(&*schema_data);
    let component = schema_data.deserialize::<C>();
    *handle_out = handle_allocate(component);
    1
}

unsafe extern "C" fn vtable_component_data_serialize<C: Component>(
    _component_id: u32,
    _user_data: *mut raw::c_void,
    handle: *mut raw::c_void,
    data: *mut *mut Schema_ComponentData,
) {
    let component = &*(handle as *const C);
    *data = schema::ComponentData::new(component).into_raw();
}

unsafe extern "C" fn vtable_component_update_free<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) {
    handle_free::<C>(handle);
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
    unimplemented!()
    // let schema_update = SchemaComponentUpdate::from_raw(update);
    // let deserialized_result = C::from_update(&schema_update);
    // if let Ok(deserialized_update) = deserialized_result {
    //     *handle_out = handle_allocate(deserialized_update);
    //     1
    // } else {
    //     0
    // }
}

unsafe extern "C" fn vtable_component_update_serialize<C>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    update: *mut *mut Schema_ComponentUpdate,
) where
    C: Component,
{
    unimplemented!();
    // let data: &C::Update = &*(handle as *const _);
    // *update = SchemaComponentUpdate::new(data).into_raw();
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
    unimplemented!()
    // let schema_request = schema::SchemaCommandRequest {
    //     component_id: C::ID,
    //     internal: request,
    // };
    // let deserialized_result = C::from_request(&schema_request);
    // if let Ok(deserialized_request) = deserialized_result {
    //     *handle_out = handle_allocate(deserialized_request);
    //     1
    // } else {
    //     0
    // }
}

unsafe extern "C" fn vtable_command_request_serialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    request: *mut *mut Schema_CommandRequest,
) {
    unimplemented!();
    // let data = &*(handle as *const _);
    // let schema_result = C::to_request(data);
    // if let Ok(schema_request) = schema_result {
    //     *request = schema_request.internal;
    // } else {
    //     *request = ptr::null_mut();
    // }
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
    unimplemented!();
    // let schema_response = schema::SchemaCommandResponse {
    //     component_id: C::ID,
    //     internal: response,
    // };
    // let deserialized_result = C::from_response(&schema_response);
    // if let Ok(deserialized_response) = deserialized_result {
    //     *handle_out = handle_allocate(deserialized_response);
    //     1
    // } else {
    //     0
    // }
}

unsafe extern "C" fn vtable_command_response_serialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    response: *mut *mut Schema_CommandResponse,
) {
    unimplemented!();
    // let data = &*(handle as *const _);
    // let schema_result = C::to_response(data);
    // if let Ok(schema_response) = schema_result {
    //     *response = schema_response.internal;
    // } else {
    //     *response = ptr::null_mut();
    // }
}
