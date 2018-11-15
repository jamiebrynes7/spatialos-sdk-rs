use spatialos_sdk::worker::core::schema;
use spatialos_sdk::worker::core::schema::SchemaField;
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId};
use spatialos_sdk::worker::internal::bindings;
use std::os::raw;
use std::rc::Rc;
use std::{alloc::{self, Layout}, mem, ptr};
use std::borrow::Borrow;

// CommandData.
pub struct CommandData {
    value: i32
}

// ExampleData (implicit).
pub struct ExampleData {
    x: f32
}

// Example.
#[allow(dead_code)]
pub struct Example;

pub struct ExampleUpdate {
    x: Option<f32>
}

impl ComponentMetaclass for Example {
    type Data = ExampleData;
    type Update = ExampleUpdate;
    type CommandRequest = ExampleCommandRequest;
    type CommandResponse = ExampleCommandResponse;
    fn component_id() -> ComponentId { 1000 }
}

impl ComponentUpdate<Example> for ExampleUpdate {
    fn to_data(self) -> ExampleData {
        ExampleData {
            x: self.x.unwrap()
        }
    }

    fn merge(&mut self, update: ExampleUpdate) {
        if let Some(x) = update.x { self.x = Some(x); }
    }
}

impl ComponentData<Example> for ExampleData {
    fn to_update(self) -> ExampleUpdate {
        ExampleUpdate {
            x: Some(self.x)
        }
    }

    fn merge(&mut self, update: ExampleUpdate) {
        if let Some(x) = update.x { self.x = x; }
    }
}

pub enum ExampleCommandRequest {
    TestCommand(CommandData), // index = 1
}

pub enum ExampleCommandResponse {
    TestCommand(CommandData), // index = 1
}

// TODO: #[derive(...)] this.
impl ComponentVtable<Example> for Example {
    fn serialize_data(data: &ExampleData) -> schema::SchemaComponentData {
        let mut serialized_data = schema::SchemaComponentData::new(Example::component_id());
        let mut fields = serialized_data.fields_mut();
        fields.field::<f32>(0).add(data.x);
        serialized_data
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Option<ExampleData> {
        let fields = data.fields();
        Some(ExampleData { x: fields.field::<f32>(1).get_or_default() })
    }

    fn serialize_update(update: &ExampleUpdate) -> schema::SchemaComponentUpdate {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Example::component_id());
        let mut fields = serialized_update.fields_mut();
        if let Some(field_value) = update.x { fields.field::<f32>(1).add(field_value); }
        serialized_update
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Option<ExampleUpdate> {
        let fields = update.fields();
        Some(ExampleUpdate { x: fields.field::<f32>(1).get() })
    }

    fn serialize_command_request(request: &ExampleCommandRequest) -> schema::SchemaCommandRequest {
        let command_index = match request {
            ExampleCommandRequest::TestCommand(_) => 1
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Example::component_id(), command_index);
        let mut object = serialized_request.object_mut();
        match request {
            ExampleCommandRequest::TestCommand(data) => {
                object.field::<i32>(1).add(data.value)
            }
        }
        serialized_request
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Option<ExampleCommandRequest> {
        let object = request.object();
        match request.command_index() {
            1 => {
                Some(ExampleCommandRequest::TestCommand(CommandData { value: object.field::<i32>(1).get_or_default() }))
            },
            _ => None
        }
    }

    fn serialize_command_response(response: &ExampleCommandResponse) -> schema::SchemaCommandResponse {
        let command_index = match response {
            ExampleCommandResponse::TestCommand(_) => 1
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Example::component_id(), command_index);
        let mut object = serialized_response.object_mut();
        match response {
            ExampleCommandResponse::TestCommand(data) => {
                object.field::<i32>(1).add(data.value)
            }
        }
        serialized_response
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Option<ExampleCommandResponse> {
        let object = response.object();
        match response.command_index() {
            1 => {
                Some(ExampleCommandResponse::TestCommand(CommandData { value: object.field::<i32>(1).get_or_default() }))
            },
            _ => None
        }
    }

    // TODO: We can make this entirely generic on M: ComponentMetaclass (once the other vtable functions are generic),
    // so move it to SpatialOS SDK crate once that's done.
    fn create_internal_vtable() -> bindings::Worker_ComponentVtable {
        bindings::Worker_ComponentVtable {
            component_id: Self::component_id(),
            user_data: ptr::null_mut(),
            command_request_free: Some(CommandRequest_Free::<Example>),
            command_request_copy: Some(CommandRequest_Copy::<Example>),
            command_request_deserialize: Some(CommandRequest_Deserialize::<Example, Example>),
            command_request_serialize: Some(CommandRequest_Serialize::<Example, Example>),
            command_response_free: Some(CommandResponse_Free::<Example>),
            command_response_copy: Some(CommandResponse_Copy::<Example>),
            command_response_deserialize: Some(CommandResponse_Deserialize::<Example, Example>),
            command_response_serialize: Some(CommandResponse_Serialize::<Example, Example>),
            component_data_free: Some(ComponentData_Free::<Example>),
            component_data_copy: Some(ComponentData_Copy::<Example>),
            component_data_deserialize: Some(ComponentData_Deserialize::<Example, Example>),
            component_data_serialize: Some(ComponentData_Serialize::<Example, Example>),
            component_update_free: Some(ComponentUpdate_Free::<Example>),
            component_update_copy: Some(ComponentUpdate_Copy::<Example>),
            component_update_deserialize: Some(ComponentUpdate_Deserialize::<Example, Example>),
            component_update_serialize: Some(ComponentUpdate_Serialize::<Example, Example>)
        }
    }
}

// TODO: Move this to where the do_ commands go to.
struct ClientHandle<T> {
    data: Rc<T>,
    // When Drop() is called, data should reduce its reference count.
}

impl<T> ClientHandle<T> {
    fn new(data: T) -> ClientHandle<T> {
        ClientHandle {
            data: Rc::new(data)
        }
    }

    fn copy(&self) -> ClientHandle<T> {
        ClientHandle {
            data: Rc::clone(&self.data)
        }
    }
}

// TODO: Move these do_ functions to the SpatialOS SDK crate.
unsafe fn do_free<T>(handle: *mut raw::c_void) {
    let client_handle_ptr: *mut ClientHandle<T> = mem::transmute(handle);

    // Call drop() on pointer value.
    client_handle_ptr.drop_in_place();

    // Deallocate memory.
    alloc::dealloc(mem::transmute(client_handle_ptr), Layout::new::<ClientHandle<T>>());
}

unsafe fn do_allocate<T>(data: T) -> *mut raw::c_void {
    // Allocate client handle and initialize with data.
    let new_client_handle: *mut ClientHandle<T> = mem::transmute(alloc::alloc(Layout::new::<ClientHandle<T>>()));
    ptr::write(new_client_handle, ClientHandle::<T>::new(data));

    // Return new pointer.
    mem::transmute(new_client_handle)
}

unsafe fn do_copy<T>(handle: *mut raw::c_void) -> *mut raw::c_void {
    let client_handle_ptr: *mut ClientHandle<T> = mem::transmute(handle);

    // Allocate client handle and initialize with copy of existing client handle
    let new_client_handle: *mut ClientHandle<T> = mem::transmute(alloc::alloc(Layout::new::<ClientHandle<T>>()));
    let client_handle: &ClientHandle<T> = &*client_handle_ptr;
    ptr::write(new_client_handle, client_handle.copy());

    // Return new pointer.
    mem::transmute(new_client_handle)
}

// TODO: merge with do_ functions and move all these to SpatialOS crate (so we can make bindings::* "internal" again).
unsafe extern "C" fn ComponentData_Free<M: ComponentMetaclass>(_: u32, _: *mut raw::c_void, handle: *mut raw::c_void) {
    do_free::<M::Data>(handle)
}

unsafe extern "C" fn ComponentData_Copy<M: ComponentMetaclass>(_: u32, _: *mut raw::c_void, handle: *mut raw::c_void) -> *mut raw::c_void {
    do_copy::<M::Data>(handle)
}

unsafe extern "C" fn ComponentData_Deserialize<M: ComponentMetaclass, V: ComponentVtable<M>>(
    _: u32, _: *mut raw::c_void,
    data: *mut bindings::Schema_ComponentData,
    handle_out: *mut *mut bindings::Worker_ComponentDataHandle) -> u8 {
    let schema_data = schema::SchemaComponentData {
        component_id: M::component_id(),
        internal: data
    };
    let deserialized_result = V::deserialize_data(&schema_data);
    if let Some(deserialized_data) = deserialized_result {
        *handle_out = do_allocate::<M::Data>(deserialized_data);
        1
    } else {
        0
    }
}

unsafe extern "C" fn ComponentData_Serialize<M: ComponentMetaclass, V: ComponentVtable<M>>(
    _: u32, _: *mut raw::c_void,
    handle: *mut raw::c_void,
    data: *mut *mut bindings::Schema_ComponentData) {
    let client_handle_ptr: *mut ClientHandle<M::Data> = mem::transmute(handle);
    let schema_data = V::serialize_data((*client_handle_ptr).data.borrow());
    *data = schema_data.internal;
}

unsafe extern "C" fn ComponentUpdate_Free<M: ComponentMetaclass>(_: u32, _: *mut raw::c_void, handle: *mut raw::c_void) {
    do_free::<M::Update>(handle)
}

unsafe extern "C" fn ComponentUpdate_Copy<M: ComponentMetaclass>(_: u32, _: *mut raw::c_void, handle: *mut raw::c_void) -> *mut raw::c_void {
    do_copy::<M::Update>(handle)
}

unsafe extern "C" fn ComponentUpdate_Deserialize<M: ComponentMetaclass, V: ComponentVtable<M>>(
    _: u32, _: *mut raw::c_void,
    update: *mut bindings::Schema_ComponentUpdate,
    handle_out: *mut *mut bindings::Worker_ComponentUpdateHandle) -> u8 {
    let schema_update = schema::SchemaComponentUpdate {
        component_id: M::component_id(),
        internal: update
    };
    let deserialized_result = V::deserialize_update(&schema_update);
    if let Some(deserialized_update) = deserialized_result {
        *handle_out = do_allocate::<M::Update>(deserialized_update);
        1
    } else {
        0
    }
}

unsafe extern "C" fn ComponentUpdate_Serialize<M: ComponentMetaclass, V: ComponentVtable<M>>(
    _: u32, _: *mut raw::c_void,
    handle: *mut raw::c_void,
    update: *mut *mut bindings::Schema_ComponentUpdate) {
    let client_handle_ptr: *mut ClientHandle<M::Update> = mem::transmute(handle);
    let schema_update = V::serialize_update((*client_handle_ptr).data.borrow());
    *update = schema_update.internal;
}

unsafe extern "C" fn CommandRequest_Free<M: ComponentMetaclass>(_: u32, _: *mut raw::c_void, handle: *mut raw::c_void) {
    do_free::<M::CommandRequest>(handle)
}

unsafe extern "C" fn CommandRequest_Copy<M: ComponentMetaclass>(_: u32, _: *mut raw::c_void, handle: *mut raw::c_void) -> *mut raw::c_void {
    do_copy::<M::CommandRequest>(handle)
}

unsafe extern "C" fn CommandRequest_Deserialize<M: ComponentMetaclass, V: ComponentVtable<M>>(
    _: u32, _: *mut raw::c_void,
    request: *mut bindings::Schema_CommandRequest,
    handle_out: *mut *mut bindings::Worker_CommandRequestHandle) -> u8 {

    let schema_request = schema::SchemaCommandRequest {
        component_id: M::component_id(),
        internal: request
    };
    let deserialized_result = V::deserialize_command_request(&schema_request);
    if let Some(deserialized_request) = deserialized_result {
        *handle_out = do_allocate::<M::CommandRequest>(deserialized_request);
        1
    } else {
        0
    }
}

unsafe extern "C" fn CommandRequest_Serialize<M: ComponentMetaclass, V: ComponentVtable<M>>(
    _: u32, _: *mut raw::c_void,
    handle: *mut raw::c_void,
    request: *mut *mut bindings::Schema_CommandRequest) {
    let client_handle_ptr: *mut ClientHandle<M::CommandRequest> = mem::transmute(handle);
    let schema_request = V::serialize_command_request((*client_handle_ptr).data.borrow());
    *request = schema_request.internal;
}

unsafe extern "C" fn CommandResponse_Free<M: ComponentMetaclass>(_: u32, _: *mut raw::c_void, handle: *mut raw::c_void) {
    do_free::<M::CommandResponse>(handle)
}

unsafe extern "C" fn CommandResponse_Copy<M: ComponentMetaclass>(_: u32, _: *mut raw::c_void, handle: *mut raw::c_void) -> *mut raw::c_void {
    do_copy::<M::CommandResponse>(handle)
}

unsafe extern "C" fn CommandResponse_Deserialize<M: ComponentMetaclass, V: ComponentVtable<M>>(
    _: u32, _: *mut raw::c_void,
    response: *mut bindings::Schema_CommandResponse,
    handle_out: *mut *mut bindings::Worker_CommandRequestHandle) -> u8 {

    let schema_response = schema::SchemaCommandResponse {
        component_id: M::component_id(),
        internal: response
    };
    let deserialized_result = V::deserialize_command_response(&schema_response);
    if let Some(deserialized_response) = deserialized_result {
        *handle_out = do_allocate::<M::CommandResponse>(deserialized_response);
        1
    } else {
        0
    }
}

unsafe extern "C" fn CommandResponse_Serialize<M: ComponentMetaclass, V: ComponentVtable<M>>(
    _: u32, _: *mut raw::c_void,
    handle: *mut raw::c_void,
    response: *mut *mut bindings::Schema_CommandResponse) {
    let client_handle_ptr: *mut ClientHandle<M::CommandResponse> = mem::transmute(handle);
    let schema_response = V::serialize_command_response((*client_handle_ptr).data.borrow());
    *response = schema_response.internal;
}
