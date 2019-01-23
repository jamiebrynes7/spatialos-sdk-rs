use crate::worker::internal::schema;
use spatialos_sdk_sys::worker;
use std::borrow::Borrow;
use std::os::raw;
use std::rc::Rc;
use std::{
    alloc::{self, Layout},
    mem, ptr,
};

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

    fn component_id() -> ComponentId;

    fn from_data(data: &schema::SchemaComponentData) -> Result<Self, String>;
    fn from_update(update: &schema::SchemaComponentUpdate) -> Result<Self::Update, String>;
    fn from_request(request: &schema::SchemaCommandRequest)
        -> Result<Self::CommandRequest, String>;
    fn from_response(
        response: &schema::SchemaCommandResponse,
    ) -> Result<Self::CommandResponse, String>;

    fn to_data(data: &Self) -> Result<schema::SchemaComponentData, String>;
    fn to_update(data: &Self::Update) -> Result<schema::SchemaComponentUpdate, String>;
    fn to_request(data: &Self::CommandRequest) -> Result<schema::SchemaCommandRequest, String>;
    fn to_response(data: &Self::CommandResponse) -> Result<schema::SchemaCommandResponse, String>;
}

// Internal untyped component data objects.
pub mod internal {
    use crate::worker::internal::schema::*;
    use spatialos_sdk_sys::worker::*;

    use crate::worker::component::ComponentId;

    #[derive(Debug)]
    pub struct ComponentData {
        pub component_id: ComponentId,
        pub schema_type: SchemaComponentData,
        pub user_handle: *mut Worker_ComponentDataHandle,
    }

    impl From<&Worker_ComponentData> for ComponentData {
        fn from(data: &Worker_ComponentData) -> Self {
            ComponentData {
                component_id: data.component_id,
                schema_type: SchemaComponentData {
                    component_id: data.component_id,
                    internal: data.schema_type,
                },
                user_handle: data.user_handle,
            }
        }
    }

    #[derive(Debug)]
    pub struct ComponentUpdate {
        pub component_id: ComponentId,
        pub schema_type: SchemaComponentUpdate,
        pub user_handle: *mut Worker_ComponentUpdateHandle,
    }

    impl From<&Worker_ComponentUpdate> for ComponentUpdate {
        fn from(update: &Worker_ComponentUpdate) -> Self {
            ComponentUpdate {
                component_id: update.component_id,
                schema_type: SchemaComponentUpdate {
                    component_id: update.component_id,
                    internal: update.schema_type,
                },
                user_handle: update.user_handle,
            }
        }
    }

    #[derive(Debug)]
    pub struct CommandRequest {
        pub component_id: ComponentId,
        pub schema_type: SchemaCommandRequest,
        pub user_handle: *mut Worker_CommandRequestHandle,
    }

    impl From<&Worker_CommandRequest> for CommandRequest {
        fn from(request: &Worker_CommandRequest) -> Self {
            CommandRequest {
                component_id: request.component_id,
                schema_type: SchemaCommandRequest {
                    component_id: request.component_id,
                    internal: request.schema_type,
                },
                user_handle: request.user_handle,
            }
        }
    }

    #[derive(Debug)]
    pub struct CommandResponse {
        pub component_id: ComponentId,
        pub schema_type: SchemaCommandResponse,
        pub user_handle: *mut Worker_CommandResponseHandle,
    }

    impl From<&Worker_CommandResponse> for CommandResponse {
        fn from(response: &Worker_CommandResponse) -> Self {
            CommandResponse {
                component_id: response.component_id,
                schema_type: SchemaCommandResponse {
                    component_id: response.component_id,
                    internal: response.schema_type,
                },
                user_handle: response.user_handle,
            }
        }
    }
}

// A data structure which represents all known component types. Used to generate an array of vtables to pass
// to the connection object.
pub struct ComponentDatabase {
    component_vtables: Vec<worker::Worker_ComponentVtable>,
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

    pub(crate) fn to_worker_sdk(&self) -> *const worker::Worker_ComponentVtable {
        self.component_vtables.as_ptr()
    }

    pub(crate) fn len(&self) -> usize {
        self.component_vtables.len()
    }
}

// An object which contains a reference counted instance to T.
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

pub fn get_component_data<C: Component>(data: &internal::ComponentData) -> &C {
    unsafe {
        let client_handle_ptr: *mut ClientHandle<C> = mem::transmute(data.user_handle);
        (*client_handle_ptr).data.borrow()
    }
}

pub fn get_component_update<C: Component>(update: &internal::ComponentUpdate) -> &C::Update {
    unsafe {
        let client_handle_ptr: *mut ClientHandle<C::Update> = mem::transmute(update.user_handle);
        (*client_handle_ptr).data.borrow()
    }
}

// Vtable implementation functions.
fn create_component_vtable<C: Component>() -> worker::Worker_ComponentVtable {
    worker::Worker_ComponentVtable {
        component_id: C::component_id(),
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
    ClientHandle::<C>::handle_free(handle)
}

unsafe extern "C" fn vtable_component_data_copy<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) -> *mut raw::c_void {
    ClientHandle::<C>::handle_copy(handle)
}

unsafe extern "C" fn vtable_component_data_deserialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    data: *mut worker::Schema_ComponentData,
    handle_out: *mut *mut worker::Worker_ComponentDataHandle,
) -> u8 {
    let schema_data = schema::SchemaComponentData {
        component_id: C::component_id(),
        internal: data,
    };
    let deserialized_result = C::from_data(&schema_data);
    if let Ok(deserialized_data) = deserialized_result {
        *handle_out = ClientHandle::<C>::handle_allocate(deserialized_data);
        1
    } else {
        0
    }
}

unsafe extern "C" fn vtable_component_data_serialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    data: *mut *mut worker::Schema_ComponentData,
) {
    let client_handle_ptr: *mut ClientHandle<C> = mem::transmute(handle);
    let schema_result = C::to_data((*client_handle_ptr).data.borrow());
    if let Ok(schema_data) = schema_result {
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
    ClientHandle::<C::Update>::handle_free(handle)
}

unsafe extern "C" fn vtable_component_update_copy<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) -> *mut raw::c_void {
    ClientHandle::<C::Update>::handle_copy(handle)
}

unsafe extern "C" fn vtable_component_update_deserialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    update: *mut worker::Schema_ComponentUpdate,
    handle_out: *mut *mut worker::Worker_ComponentUpdateHandle,
) -> u8 {
    let schema_update = schema::SchemaComponentUpdate {
        component_id: C::component_id(),
        internal: update,
    };
    let deserialized_result = C::from_update(&schema_update);
    if let Ok(deserialized_update) = deserialized_result {
        *handle_out = ClientHandle::<C::Update>::handle_allocate(deserialized_update);
        1
    } else {
        0
    }
}

unsafe extern "C" fn vtable_component_update_serialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    update: *mut *mut worker::Schema_ComponentUpdate,
) {
    let client_handle_ptr: *mut ClientHandle<C::Update> = mem::transmute(handle);
    let schema_result = C::to_update((*client_handle_ptr).data.borrow());
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
    ClientHandle::<C::CommandRequest>::handle_free(handle)
}

unsafe extern "C" fn vtable_command_request_copy<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) -> *mut raw::c_void {
    ClientHandle::<C::CommandRequest>::handle_copy(handle)
}

unsafe extern "C" fn vtable_command_request_deserialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    request: *mut worker::Schema_CommandRequest,
    handle_out: *mut *mut worker::Worker_CommandRequestHandle,
) -> u8 {
    let schema_request = schema::SchemaCommandRequest {
        component_id: C::component_id(),
        internal: request,
    };
    let deserialized_result = C::from_request(&schema_request);
    if let Ok(deserialized_request) = deserialized_result {
        *handle_out = ClientHandle::<C::CommandRequest>::handle_allocate(deserialized_request);
        1
    } else {
        0
    }
}

unsafe extern "C" fn vtable_command_request_serialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    request: *mut *mut worker::Schema_CommandRequest,
) {
    let client_handle_ptr: *mut ClientHandle<C::CommandRequest> = mem::transmute(handle);
    let schema_result = C::to_request((*client_handle_ptr).data.borrow());
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
    ClientHandle::<C::CommandResponse>::handle_free(handle)
}

unsafe extern "C" fn vtable_command_response_copy<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
) -> *mut raw::c_void {
    ClientHandle::<C::CommandResponse>::handle_copy(handle)
}

unsafe extern "C" fn vtable_command_response_deserialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    response: *mut worker::Schema_CommandResponse,
    handle_out: *mut *mut worker::Worker_CommandRequestHandle,
) -> u8 {
    let schema_response = schema::SchemaCommandResponse {
        component_id: C::component_id(),
        internal: response,
    };
    let deserialized_result = C::from_response(&schema_response);
    if let Ok(deserialized_response) = deserialized_result {
        *handle_out = ClientHandle::<C::CommandResponse>::handle_allocate(deserialized_response);
        1
    } else {
        0
    }
}

unsafe extern "C" fn vtable_command_response_serialize<C: Component>(
    _: u32,
    _: *mut raw::c_void,
    handle: *mut raw::c_void,
    response: *mut *mut worker::Schema_CommandResponse,
) {
    let client_handle_ptr: *mut ClientHandle<C::CommandResponse> = mem::transmute(handle);
    let schema_result = C::to_response((*client_handle_ptr).data.borrow());
    if let Ok(schema_response) = schema_result {
        *response = schema_response.internal;
    } else {
        *response = ptr::null_mut();
    }
}
