//! Support for vtable-based serialization of schema data.
//!
//! # Error Handling
//!
//! We return 1, indicating that serialization succeeded, regardless of whether or
//! not an error occurred during serialization. This ensures that the worker SDK
//! will pass any error back to the user, allowing for application-specific handling
//! of errors.

use crate::worker::{
    component::*,
    handle::{self, RawHandle},
    schema::*,
};
use spatialos_sdk_sys::worker::*;
use std::{collections::HashMap, os::raw, ptr};

inventory::collect!(VTable);

lazy_static::lazy_static! {
    pub(crate) static ref DATABASE: ComponentDatabase = {

        let mut raw_vtables = Vec::new();
        let mut vtables = HashMap::new();

        for table in inventory::iter::<VTable>.into_iter() {
            raw_vtables.push(table.worker_vtable);
            vtables.insert(table.component_id(), table.clone());
        }

        ComponentDatabase {
            vtables,
            raw_vtables
        }
    };
}

#[derive(Clone, Debug)]
pub(crate) struct ComponentDatabase {
    vtables: HashMap<ComponentId, VTable>,
    raw_vtables: Vec<Worker_ComponentVtable>,
}

impl ComponentDatabase {
    pub(crate) fn has_vtable(&self, id: ComponentId) -> bool {
        self.vtables.contains_key(&id)
    }

    pub(crate) fn get_vtable(&self, id: ComponentId) -> Option<&VTable> {
        self.vtables.get(&id)
    }

    pub(crate) fn to_worker_sdk(&self) -> *const Worker_ComponentVtable {
        self.raw_vtables.as_ptr()
    }

    pub(crate) fn len(&self) -> usize {
        self.vtables.len()
    }
}

unsafe impl Sync for ComponentDatabase {}
unsafe impl Send for ComponentDatabase {}

#[derive(Clone, Debug)]
pub struct VTable {
    pub(crate) worker_vtable: Worker_ComponentVtable,
}

impl VTable {
    pub fn new<C: Component>() -> Self {
        VTable {
            worker_vtable: Worker_ComponentVtable {
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
                component_data_free: Some(component_data_free::<C>),
                component_data_copy: Some(component_data_copy::<C>),
                component_data_deserialize: Some(component_data_deserialize::<C>),
                component_data_serialize: Some(component_data_serialize::<C>),
                component_update_free: Some(component_update_free::<C>),
                component_update_copy: Some(component_update_copy::<C>),
                component_update_deserialize: Some(component_update_deserialize::<C>),
                component_update_serialize: Some(vtable_component_update_serialize::<C>),
            },
        }
    }

    pub fn component_id(&self) -> ComponentId {
        self.worker_vtable.component_id
    }
}

pub const PASSTHROUGH_VTABLE: Worker_ComponentVtable = Worker_ComponentVtable {
    component_id: 0,
    user_data: ptr::null_mut(),
    command_request_free: None,
    command_request_copy: None,
    command_request_deserialize: None,
    command_request_serialize: None,
    command_response_free: None,
    command_response_copy: None,
    command_response_deserialize: None,
    command_response_serialize: None,
    component_data_free: None,
    component_data_copy: None,
    component_data_deserialize: None,
    component_data_serialize: None,
    component_update_free: None,
    component_update_copy: None,
    component_update_deserialize: None,
    component_update_serialize: None,
};

unsafe extern "C" fn component_data_free<C: Component>(
    component_id: ComponentId,
    _user_data: *mut raw::c_void,
    handle: RawHandle,
) {
    assert_eq!(
        C::ID,
        component_id,
        "Mismatched component ID in vtable function",
    );

    handle::drop_raw::<C>(handle)
}

unsafe extern "C" fn component_data_copy<C: Component>(
    component_id: ComponentId,
    _user_data: *mut raw::c_void,
    handle: RawHandle,
) -> RawHandle {
    assert_eq!(
        C::ID,
        component_id,
        "Mismatched component ID in vtable function",
    );

    handle::clone_raw::<C>(handle)
}

unsafe extern "C" fn component_data_deserialize<C: Component>(
    component_id: ComponentId,
    _user_data: *mut raw::c_void,
    data: *mut Schema_ComponentData,
    handle_out: *mut RawHandle,
) -> u8 {
    assert_eq!(
        C::ID,
        component_id,
        "Mismatched component ID in vtable function",
    );

    let schema_data = SchemaComponentData::from_raw(data);
    let result = schema_data.deserialize::<C>();
    *handle_out = handle::allocate_raw(result);

    // NOTE: See module documentation for why we always return 1, even if serialization
    // failed.
    1
}

unsafe extern "C" fn component_data_serialize<C: Component>(
    component_id: ComponentId,
    _user_data: *mut raw::c_void,
    handle: RawHandle,
    data: *mut *mut Schema_ComponentData,
) {
    assert_eq!(
        C::ID,
        component_id,
        "Mismatched component ID in vtable function",
    );

    let client_data = handle::deref_raw::<C>(handle)
        .as_ref()
        .expect("Cannot serialize `Err`");
    *data = SchemaComponentData::from_component(client_data).into_raw();
}

unsafe extern "C" fn component_update_free<C: Component>(
    component_id: ComponentId,
    _user_data: *mut raw::c_void,
    handle: RawHandle,
) {
    assert_eq!(
        C::ID,
        component_id,
        "Mismatched component ID in vtable function",
    );

    handle::drop_raw::<C::Update>(handle)
}

unsafe extern "C" fn component_update_copy<C: Component>(
    component_id: ComponentId,
    _user_data: *mut raw::c_void,
    handle: RawHandle,
) -> RawHandle {
    assert_eq!(
        C::ID,
        component_id,
        "Mismatched component ID in vtable function",
    );

    handle::clone_raw::<C::Update>(handle)
}

unsafe extern "C" fn component_update_deserialize<C: Component>(
    component_id: ComponentId,
    _user_data: *mut raw::c_void,
    source: *mut Schema_ComponentUpdate,
    handle_out: *mut *mut Worker_ComponentUpdateHandle,
) -> u8 {
    assert_eq!(
        C::ID,
        component_id,
        "Mismatched component ID in vtable function",
    );

    let schema_update = SchemaComponentUpdate::from_raw(source);
    let result = schema_update.deserialize::<C>();
    *handle_out = handle::allocate_raw(result);

    // NOTE: See module documentation for why we always return 1, even if serialization
    // failed.
    1
}

unsafe extern "C" fn vtable_component_update_serialize<C: Component>(
    component_id: ComponentId,
    _user_data: *mut raw::c_void,
    handle: RawHandle,
    result: *mut *mut Schema_ComponentUpdate,
) {
    assert_eq!(
        C::ID,
        component_id,
        "Mismatched component ID in vtable function",
    );

    let data = handle::deref_raw::<C::Update>(handle)
        .as_ref()
        .expect("Cannot serialize `Err`");
    *result = SchemaComponentUpdate::from_update::<C::Update>(data).into_raw();
}

unsafe extern "C" fn vtable_command_request_free<C: Component>(
    component_id: ComponentId,
    _command_index: CommandIndex,
    _user_data: *mut raw::c_void,
    handle: RawHandle,
) {
    assert_eq!(
        C::ID,
        component_id,
        "Mismatched component ID in vtable function",
    );

    handle::drop_raw::<C::CommandRequest>(handle);
}

unsafe extern "C" fn vtable_command_request_copy<C: Component>(
    component_id: ComponentId,
    _command_index: CommandIndex,
    _user_data: *mut raw::c_void,
    handle: RawHandle,
) -> RawHandle {
    assert_eq!(
        C::ID,
        component_id,
        "Mismatched component ID in vtable function",
    );

    handle::clone_raw::<C::CommandRequest>(handle)
}

unsafe extern "C" fn vtable_command_request_deserialize<C: Component>(
    component_id: ComponentId,
    command_index: CommandIndex,
    _user_data: *mut raw::c_void,
    source: *mut Schema_CommandRequest,
    handle_out: *mut *mut Worker_CommandRequestHandle,
) -> u8 {
    assert_eq!(
        C::ID,
        component_id,
        "Mismatched component ID in vtable function",
    );

    let schema_request = SchemaCommandRequest::from_raw(source);
    let result = C::from_request(command_index, &schema_request);
    *handle_out = handle::allocate_raw(result);

    // NOTE: See module documentation for why we always return 1, even if serialization
    // failed.
    1
}

unsafe extern "C" fn vtable_command_request_serialize<C: Component>(
    component_id: ComponentId,
    _command_index: CommandIndex,
    _user_data: *mut raw::c_void,
    handle: RawHandle,
    request: *mut *mut Schema_CommandRequest,
) {
    assert_eq!(
        C::ID,
        component_id,
        "Mismatched component ID in vtable function",
    );

    let data = handle::deref_raw::<C::CommandRequest>(handle)
        .as_ref()
        .expect("Cannot serialize an `Err`");
    *request = C::to_request(data).into_raw();
}

unsafe extern "C" fn vtable_command_response_free<C: Component>(
    component_id: ComponentId,
    _command_index: CommandIndex,
    _user_data: *mut raw::c_void,
    handle: RawHandle,
) {
    assert_eq!(
        C::ID,
        component_id,
        "Mismatched component ID in vtable function",
    );

    handle::drop_raw::<C::CommandResponse>(handle);
}

unsafe extern "C" fn vtable_command_response_copy<C: Component>(
    component_id: ComponentId,
    _command_index: CommandIndex,
    _user_data: *mut raw::c_void,
    handle: RawHandle,
) -> RawHandle {
    assert_eq!(
        C::ID,
        component_id,
        "Mismatched component ID in vtable function",
    );

    handle::clone_raw::<C::CommandResponse>(handle)
}

unsafe extern "C" fn vtable_command_response_deserialize<C: Component>(
    component_id: ComponentId,
    command_index: CommandIndex,
    _user_data: *mut raw::c_void,
    source: *mut Schema_CommandResponse,
    handle_out: *mut *mut Worker_CommandRequestHandle,
) -> u8 {
    assert_eq!(
        C::ID,
        component_id,
        "Mismatched component ID in vtable function",
    );

    let schema_response = SchemaCommandResponse::from_raw(source);
    let result = C::from_response(command_index, &schema_response);
    *handle_out = handle::allocate_raw(result);

    1
}

unsafe extern "C" fn vtable_command_response_serialize<C: Component>(
    component_id: ComponentId,
    _command_index: CommandIndex,
    _user_data: *mut raw::c_void,
    handle: RawHandle,
    response: *mut *mut Schema_CommandResponse,
) {
    assert_eq!(
        C::ID,
        component_id,
        "Mismatched component ID in vtable function",
    );

    let data = handle::deref_raw::<C::CommandResponse>(handle)
        .as_ref()
        .expect("Cannot serialize an `Err`");
    *response = C::to_response(data).into_raw();
}
