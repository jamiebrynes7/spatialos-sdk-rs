use spatialos_sdk::worker::core::schema;
use spatialos_sdk::worker::core::schema::SchemaField;
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId};
use spatialos_sdk::worker::internal::bindings;
use std::os::raw;
use std::rc::Rc;
use std::{alloc::{self, Layout}, mem, ptr};
use std::borrow::Borrow;

#[allow(dead_code)]
pub struct Example;

pub struct ExampleData {
    x: f32
}

pub struct ExampleUpdate {
    x: Option<f32>
}

impl ComponentMetaclass for Example {
    type Data = ExampleData;
    type Update = ExampleUpdate;
    fn component_id() -> ComponentId { 1000 }
}

impl ComponentUpdate<Example> for ExampleUpdate {
    fn to_data(self) -> ExampleData {
        ExampleData {
            x: self.x.unwrap()
        }
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
        Some(ExampleData { x: fields.field::<f32>(0).get_or_default() })
    }

    fn serialize_update(update: &ExampleUpdate) -> schema::SchemaComponentUpdate {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Example::component_id());
        let mut fields = serialized_update.fields_mut();
        if let Some(field_value) = update.x { fields.field::<f32>(0).add(field_value); }
        serialized_update
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Option<ExampleUpdate> {
        let fields = update.fields();
        Some(ExampleUpdate { x: fields.field::<f32>(0).get() })
    }

    // TODO: Command requests.
    // TODO: Command responses.

    // TODO: We can make this entirely generic on M: ComponentMetaclass (once the other vtable functions are generic),
    // so move it to SpatialOS SDK crate once that's done.
    fn create_internal_vtable() -> bindings::Worker_ComponentVtable {
        bindings::Worker_ComponentVtable {
            component_id: Self::component_id(),
            user_data: ptr::null_mut(),
            command_request_free: Some(Example_CommandRequest_Free),
            command_request_copy: Some(Example_CommandRequest_Copy),
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
            component_update_free: Some(ComponentUpdate_Free::<Example>),
            component_update_copy: Some(ComponentUpdate_Copy::<Example>),
            component_update_deserialize: Some(Example_ComponentUpdate_Deserialize),
            component_update_serialize: Some(Example_ComponentUpdate_Serialize)
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

// List of all possible command request and response types.
enum CommandDataTypes {
    // TODO: All possible command data types go here.
}

struct CommandState {
    command_id: u32,
    command_data: CommandDataTypes
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

unsafe fn do_deserialize_update<M: ComponentMetaclass, V: ComponentVtable<M>>(
    update: *mut bindings::Schema_ComponentUpdate,
    handle_out: *mut *mut bindings::Worker_ComponentUpdateHandle) -> bool {
    let schema_update = schema::SchemaComponentUpdate {
        component_id: M::component_id(),
        internal: update
    };
    let deserialize_result = V::deserialize_update(&schema_update);
    if let Some(deserialized_update) = deserialize_result {
        *handle_out = do_allocate::<M::Update>(deserialized_update);
        true
    } else {
        false
    }
}

unsafe fn do_serialize_update<M: ComponentMetaclass, V: ComponentVtable<M>>(
    handle: *mut raw::c_void,
    update: *mut *mut bindings::Schema_ComponentUpdate) {
    let client_handle_ptr: *mut ClientHandle<M::Update> = mem::transmute(handle);
    let schema_update = V::serialize_update((*client_handle_ptr).data.borrow());
    *update = schema_update.internal;
}
// TODO: Remaining component data.

unsafe extern "C" fn ComponentUpdate_Free<M: ComponentMetaclass>(_: u32, _: *mut raw::c_void, handle: *mut raw::c_void) {
    do_free::<M::Update>(handle)
}

unsafe extern "C" fn ComponentUpdate_Copy<M: ComponentMetaclass>(_: u32, _: *mut raw::c_void, handle: *mut raw::c_void) -> *mut raw::c_void {
    do_copy::<M::Update>(handle)
}

// TODO: Make these generic like above, then merge with do_ functions and move all these to SpatialOS crate (so we can
// make bindings::* "internal" again).
unsafe extern "C" fn Example_ComponentUpdate_Deserialize(
    _: u32, _: *mut raw::c_void,
    update: *mut bindings::Schema_ComponentUpdate,
    handle_out: *mut *mut bindings::Worker_ComponentUpdateHandle) -> u8 {
    if do_deserialize_update::<Example, Example>(update, handle_out) { 1 } else { 0 }
}

unsafe extern "C" fn Example_ComponentUpdate_Serialize(
    _: u32, _: *mut raw::c_void,
    handle: *mut raw::c_void,
    update: *mut *mut bindings::Schema_ComponentUpdate) {
    do_serialize_update::<Example, Example>(handle, update)
}

unsafe extern "C" fn Example_CommandRequest_Free(_: u32, _: *mut raw::c_void, handle: *mut raw::c_void) {
    do_free::<CommandState>(handle)
}

unsafe extern "C" fn Example_CommandRequest_Copy(_: u32, _: *mut raw::c_void, handle: *mut raw::c_void) -> *mut raw::c_void {
    do_copy::<CommandState>(handle)
}

// TODO: Remaining command request.
// TODO: Remaining command response.