use crate::worker::internal::schema;
use spatialos_sdk_sys::worker;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::os::raw;
use std::rc::Rc;
use std::{
    alloc::{self, Layout},
    mem, ptr,
};

pub type ComponentId = u32;

pub trait ComponentMetaclass {
    type Data;
    type Update;
    type CommandRequest;
    type CommandResponse;

    fn component_id() -> ComponentId;
}

pub trait ComponentUpdate<M: ComponentMetaclass> {
    fn merge(&mut self, update: M::Update);
}

pub trait ComponentData<M: ComponentMetaclass> {
    fn merge(&mut self, update: M::Update);
}

pub trait ComponentVtable<M: ComponentMetaclass> {
    fn serialize_data(update: &M::Data) -> Result<schema::SchemaComponentData, String>;
    fn deserialize_data(update: &schema::SchemaComponentData) -> Result<M::Data, String>;
    fn serialize_update(update: &M::Update) -> Result<schema::SchemaComponentUpdate, String>;
    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Result<M::Update, String>;
    fn serialize_command_request(
        request: &M::CommandRequest,
    ) -> Result<schema::SchemaCommandRequest, String>;
    fn deserialize_command_request(
        response: &schema::SchemaCommandRequest,
    ) -> Result<M::CommandRequest, String>;
    fn serialize_command_response(
        request: &M::CommandResponse,
    ) -> Result<schema::SchemaCommandResponse, String>;
    fn deserialize_command_response(
        response: &schema::SchemaCommandResponse,
    ) -> Result<M::CommandResponse, String>;
}

// A data structure which represents all known component types.
pub struct ComponentDatabase {
    component_vtables: Vec<worker::Worker_ComponentVtable>,
}

impl ComponentDatabase {
    pub fn new() -> Self {
        ComponentDatabase {
            component_vtables: Vec::new(),
        }
    }

    pub fn add_component<M: ComponentMetaclass, V: ComponentVtable<M>>(mut self) -> Self {
        self.component_vtables
            .push(create_component_vtable::<M, V>());
        self
    }

    pub(crate) fn to_worker_sdk(&self) -> *const worker::Worker_ComponentVtable {
        self.component_vtables.as_ptr()
    }

    pub(crate) fn len(&self) -> usize {
        self.component_vtables.len()
    }
}

// A trait that's implemented by a type which can be serialized and deserialized into a schema object.
pub trait TypeSerializer where Self: std::marker::Sized {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String>;
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String>;
}

// Client handles. An object which contains a reference counted instance to T.
struct ClientHandle<T> {
    data: Rc<T>,
    // When Drop() is called, data should reduce its reference count.
}

impl<T> ClientHandle<T> {
    fn new(data: T) -> ClientHandle<T> {
        ClientHandle {
            data: Rc::new(data),
        }
    }

    fn copy(&self) -> ClientHandle<T> {
        ClientHandle {
            data: Rc::clone(&self.data),
        }
    }

    // Vtable helper functions.
    unsafe fn handle_allocate(data: T) -> *mut raw::c_void {
        // Allocate client handle and initialize with data.
        let new_client_handle: *mut ClientHandle<T> =
            mem::transmute(alloc::alloc(Layout::new::<ClientHandle<T>>()));
        ptr::write(new_client_handle, ClientHandle::<T>::new(data));

        // Return new pointer.
        mem::transmute(new_client_handle)
    }

    unsafe fn handle_free(handle: *mut raw::c_void) {
        let client_handle_ptr: *mut ClientHandle<T> = mem::transmute(handle);

        // Call drop() on pointer value.
        client_handle_ptr.drop_in_place();

        // Deallocate memory.
        alloc::dealloc(
            mem::transmute(client_handle_ptr),
            Layout::new::<ClientHandle<T>>(),
        );
    }

    unsafe fn handle_copy(handle: *mut raw::c_void) -> *mut raw::c_void {
        let client_handle_ptr: *mut ClientHandle<T> = mem::transmute(handle);

        // Allocate client handle and initialize with copy of existing client handle
        let new_client_handle: *mut ClientHandle<T> =
            mem::transmute(alloc::alloc(Layout::new::<ClientHandle<T>>()));
        let client_handle: &ClientHandle<T> = &*client_handle_ptr;
        ptr::write(new_client_handle, client_handle.copy());

        // Return new pointer.
        mem::transmute(new_client_handle)
    }
}

pub fn get_component_data<M: ComponentMetaclass>(data: &internal::ComponentData) -> &M::Data {
    unsafe {
        let client_handle_ptr: *mut ClientHandle<M::Data> = mem::transmute(data.user_handle);
        (*client_handle_ptr).data.borrow()
    }
}

pub fn get_component_update<M: ComponentMetaclass>(
    update: &internal::ComponentUpdate,
) -> &M::Update {
    unsafe {
        let client_handle_ptr: *mut ClientHandle<M::Update> = mem::transmute(update.user_handle);
        (*client_handle_ptr).data.borrow()
    }
}

// Vtable implementation functions.
pub(crate) fn create_component_vtable<M: ComponentMetaclass, V: ComponentVtable<M>>(
) -> worker::Worker_ComponentVtable {
    worker::Worker_ComponentVtable {
        component_id: M::component_id(),
        user_data: ptr::null_mut(),
        command_request_free: Some(vtable_command_request_free::<M>),
        command_request_copy: Some(vtable_command_request_copy::<M>),
        command_request_deserialize: Some(vtable_command_request_deserialize::<M, V>),
        command_request_serialize: Some(vtable_command_request_serialize::<M, V>),
        command_response_free: Some(vtable_command_response_free::<M>),
        command_response_copy: Some(vtable_command_response_copy::<M>),
        command_response_deserialize: Some(vtable_command_response_deserialize::<M, V>),
        command_response_serialize: Some(vtable_command_response_serialize::<M, V>),
        component_data_free: Some(vtable_component_data_free::<M>),
        component_data_copy: Some(vtable_component_data_copy::<M>),
        component_data_deserialize: Some(vtable_component_data_deserialize::<M, V>),
        component_data_serialize: Some(vtable_component_data_serialize::<M, V>),
        component_update_free: Some(vtable_component_update_free::<M>),
        component_update_copy: Some(vtable_component_update_copy::<M>),
        component_update_deserialize: Some(vtable_component_update_deserialize::<M, V>),
        component_update_serialize: Some(vtable_component_update_serialize::<M, V>),
    }
}

unsafe extern "C" fn vtable_component_data_free<M: ComponentMetaclass>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) {
    ClientHandle::<M::Data>::handle_free(handle)
}

unsafe extern "C" fn vtable_component_data_copy<M: ComponentMetaclass>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) -> *mut raw::c_void {
    ClientHandle::<M::Data>::handle_copy(handle)
}

unsafe extern "C" fn vtable_component_data_deserialize<
    M: ComponentMetaclass,
    V: ComponentVtable<M>,
>(
    _: u32,
    _: *mut raw::c_void,
    data: *mut worker::Schema_ComponentData,
    handle_out: *mut *mut worker::Worker_ComponentDataHandle,
) -> u8 {
    let schema_data = schema::SchemaComponentData {
        component_id: M::component_id(),
        internal: data,
    };
    let deserialized_result = V::deserialize_data(&schema_data);
    if let Ok(deserialized_data) = deserialized_result {
        *handle_out = ClientHandle::<M::Data>::handle_allocate(deserialized_data);
        1
    } else {
        0
    }
}

unsafe extern "C" fn vtable_component_data_serialize<
    M: ComponentMetaclass,
    V: ComponentVtable<M>,
>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    data: *mut *mut worker::Schema_ComponentData,
) {
    let client_handle_ptr: *mut ClientHandle<M::Data> = mem::transmute(handle);
    let schema_result = V::serialize_data((*client_handle_ptr).data.borrow());
    if let Ok(schema_data) = schema_result {
        *data = schema_data.internal;
    } else {
        *data = ptr::null_mut();
    }
}

unsafe extern "C" fn vtable_component_update_free<M: ComponentMetaclass>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) {
    ClientHandle::<M::Update>::handle_free(handle)
}

unsafe extern "C" fn vtable_component_update_copy<M: ComponentMetaclass>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) -> *mut raw::c_void {
    ClientHandle::<M::Update>::handle_copy(handle)
}

unsafe extern "C" fn vtable_component_update_deserialize<
    M: ComponentMetaclass,
    V: ComponentVtable<M>,
>(
    _: u32,
    _: *mut raw::c_void,
    update: *mut worker::Schema_ComponentUpdate,
    handle_out: *mut *mut worker::Worker_ComponentUpdateHandle,
) -> u8 {
    let schema_update = schema::SchemaComponentUpdate {
        component_id: M::component_id(),
        internal: update,
    };
    let deserialized_result = V::deserialize_update(&schema_update);
    if let Ok(deserialized_update) = deserialized_result {
        *handle_out = ClientHandle::<M::Update>::handle_allocate(deserialized_update);
        1
    } else {
        0
    }
}

unsafe extern "C" fn vtable_component_update_serialize<
    M: ComponentMetaclass,
    V: ComponentVtable<M>,
>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    update: *mut *mut worker::Schema_ComponentUpdate,
) {
    let client_handle_ptr: *mut ClientHandle<M::Update> = mem::transmute(handle);
    let schema_result = V::serialize_update((*client_handle_ptr).data.borrow());
    if let Ok(schema_update) = schema_result {
        *update = schema_update.internal;
    } else {
        *update = ptr::null_mut();
    }
}

unsafe extern "C" fn vtable_command_request_free<M: ComponentMetaclass>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) {
    ClientHandle::<M::CommandRequest>::handle_free(handle)
}

unsafe extern "C" fn vtable_command_request_copy<M: ComponentMetaclass>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) -> *mut raw::c_void {
    ClientHandle::<M::CommandRequest>::handle_copy(handle)
}

unsafe extern "C" fn vtable_command_request_deserialize<
    M: ComponentMetaclass,
    V: ComponentVtable<M>,
>(
    _: u32,
    _: *mut raw::c_void,
    request: *mut worker::Schema_CommandRequest,
    handle_out: *mut *mut worker::Worker_CommandRequestHandle,
) -> u8 {
    let schema_request = schema::SchemaCommandRequest {
        component_id: M::component_id(),
        internal: request,
    };
    let deserialized_result = V::deserialize_command_request(&schema_request);
    if let Ok(deserialized_request) = deserialized_result {
        *handle_out = ClientHandle::<M::CommandRequest>::handle_allocate(deserialized_request);
        1
    } else {
        0
    }
}

unsafe extern "C" fn vtable_command_request_serialize<
    M: ComponentMetaclass,
    V: ComponentVtable<M>,
>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    request: *mut *mut worker::Schema_CommandRequest,
) {
    let client_handle_ptr: *mut ClientHandle<M::CommandRequest> = mem::transmute(handle);
    let schema_result = V::serialize_command_request((*client_handle_ptr).data.borrow());
    if let Ok(schema_request) = schema_result {
        *request = schema_request.internal;
    } else {
        *request = ptr::null_mut();
    }
}

unsafe extern "C" fn vtable_command_response_free<M: ComponentMetaclass>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) {
    ClientHandle::<M::CommandResponse>::handle_free(handle)
}

unsafe extern "C" fn vtable_command_response_copy<M: ComponentMetaclass>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) -> *mut raw::c_void {
    ClientHandle::<M::CommandResponse>::handle_copy(handle)
}

unsafe extern "C" fn vtable_command_response_deserialize<
    M: ComponentMetaclass,
    V: ComponentVtable<M>,
>(
    _: u32,
    _: *mut raw::c_void,
    response: *mut worker::Schema_CommandResponse,
    handle_out: *mut *mut worker::Worker_CommandRequestHandle,
) -> u8 {
    let schema_response = schema::SchemaCommandResponse {
        component_id: M::component_id(),
        internal: response,
    };
    let deserialized_result = V::deserialize_command_response(&schema_response);
    if let Ok(deserialized_response) = deserialized_result {
        *handle_out = ClientHandle::<M::CommandResponse>::handle_allocate(deserialized_response);
        1
    } else {
        0
    }
}

unsafe extern "C" fn vtable_command_response_serialize<
    M: ComponentMetaclass,
    V: ComponentVtable<M>,
>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    response: *mut *mut worker::Schema_CommandResponse,
) {
    let client_handle_ptr: *mut ClientHandle<M::CommandResponse> = mem::transmute(handle);
    let schema_result = V::serialize_command_response((*client_handle_ptr).data.borrow());
    if let Ok(schema_response) = schema_result {
        *response = schema_response.internal;
    } else {
        *response = ptr::null_mut();
    }
}

pub mod internal {
    use spatialos_sdk_sys::worker::{
        Schema_CommandRequest, Schema_CommandResponse, Schema_ComponentData,
        Schema_ComponentUpdate, Worker_CommandRequest, Worker_CommandRequestHandle,
        Worker_CommandResponse, Worker_CommandResponseHandle, Worker_ComponentData,
        Worker_ComponentDataHandle, Worker_ComponentUpdate, Worker_ComponentUpdateHandle,
    };

    use crate::worker::ComponentId;

    // TODO: Wrap Schema_ComponentData
    #[derive(Debug)]
    pub struct ComponentData {
        pub component_id: ComponentId,
        pub schema_type: *mut Schema_ComponentData,
        pub user_handle: *mut Worker_ComponentDataHandle,
    }

    impl From<&Worker_ComponentData> for ComponentData {
        fn from(data: &Worker_ComponentData) -> Self {
            ComponentData {
                component_id: data.component_id,
                schema_type: data.schema_type,
                user_handle: data.user_handle,
            }
        }
    }

    // TODO: Wrap Schema_ComponentUpdate
    #[derive(Debug)]
    pub struct ComponentUpdate {
        pub component_id: ComponentId,
        pub schema_type: *mut Schema_ComponentUpdate,
        pub user_handle: *mut Worker_ComponentUpdateHandle,
    }

    impl From<&Worker_ComponentUpdate> for ComponentUpdate {
        fn from(update: &Worker_ComponentUpdate) -> Self {
            ComponentUpdate {
                component_id: update.component_id,
                schema_type: update.schema_type,
                user_handle: update.user_handle,
            }
        }
    }

    // TODO: Wrap Schema_CommandRequest
    #[derive(Debug)]
    pub struct CommandRequest {
        pub component_id: u32,
        pub schema_type: *mut Schema_CommandRequest,
    }

    impl From<&Worker_CommandRequest> for CommandRequest {
        fn from(request: &Worker_CommandRequest) -> Self {
            CommandRequest {
                component_id: request.component_id,
                schema_type: request.schema_type,
            }
        }
    }

    // TODO: Wrap Schema_CommandResponse
    #[derive(Debug)]
    pub struct CommandResponse {
        pub component_id: u32,
        pub schema_type: *mut Schema_CommandResponse,
    }

    impl From<&Worker_CommandResponse> for CommandResponse {
        fn from(response: &Worker_CommandResponse) -> Self {
            CommandResponse {
                component_id: response.component_id,
                schema_type: response.schema_type,
            }
        }
    }
}
