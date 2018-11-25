use spatialos_sdk_sys::worker::*;
use std::ptr;

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
