use spatialos_sdk_sys::worker::*;
use std::{ptr, mem};
use std::os::raw;
use crate::worker::schema::{SchemaCommandResponse, DataPointer, SchemaComponentData, SchemaComponentUpdate, SchemaCommandRequest};
use std::sync::Arc;
use crate::worker::component::{Component, ComponentId};
use std::collections::HashMap;

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
    raw_vtables: Vec<Worker_ComponentVtable>
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
    pub worker_vtable: Worker_ComponentVtable,
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
    let schema_data = SchemaComponentData::from_raw(data);
    match schema_data.deserialize::<C>() {
        Ok(deserialized_data) => {
            *handle_out = handle_allocate(deserialized_data);
            1
        }

        // TODO: How should we handle errors occurring during vtable serialization? We could
        // probably store the whole `Result` in the user handle and pass it to the user when
        // they try to retrieve the result.
        Err(..) => {
            *handle_out = ptr::null_mut();
            0
        }
    }
}

unsafe extern "C" fn vtable_component_data_serialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    data: *mut *mut Schema_ComponentData,
) {
    let client_data = &*handle.cast::<C>();
    *data = SchemaComponentData::from_component(client_data).into_raw();
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
    match schema_update.deserialize::<C>() {
        Ok(deserialized_update) => {
            *handle_out = handle_allocate(deserialized_update);
            1
        }

        // TODO: How should we handle errors occurring during vtable serialization? We could
        // probably store the whole `Result` in the user handle and pass it to the user when
        // they try to retrieve the result.
        Err(..) => {
            *handle_out = ptr::null_mut();
            0
        }
    }
}

unsafe extern "C" fn vtable_component_update_serialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    result: *mut *mut Schema_ComponentUpdate,
) {
    let data: &C::Update = &*(handle as *const _);
    *result = SchemaComponentUpdate::from_update(data).into_raw();
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
    *request = C::to_request(data).into_raw();
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
    *response = C::to_response(data).into_raw();
}