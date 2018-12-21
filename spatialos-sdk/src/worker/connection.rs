use spatialos_sdk_sys::worker::*;
use std::ffi::{CStr, CString};
use std::ptr;

use crate::worker::commands::*;
use crate::worker::component::ComponentUpdate;
use crate::worker::locator::*;
use crate::worker::metrics::Metrics;
use crate::worker::op::{DisconnectOp, OpList, WorkerOp};
use crate::worker::parameters::{CommandParameters, ConnectionParameters};
use crate::worker::{EntityId, InterestOverride, LogLevel, RequestId};

/// Connection trait to allow for mocking the connection.
pub trait Connection {
    fn send_log_message<T: Into<Vec<u8>>, U: Into<Vec<u8>>>(
        &mut self,
        level: LogLevel,
        logger_name: T,
        message: U,
        entity_id: Option<EntityId>,
    );
    fn send_metrics(&mut self, metrics: &Metrics);

    fn send_reserve_entity_ids_request(
        &mut self,
        payload: ReserveEntityIdsRequest,
        timeout_millis: Option<u32>,
    ) -> RequestId<ReserveEntityIdsRequest>;
    fn send_create_entity_request(
        &mut self,
        payload: CreateEntityRequest,
        timeout_millis: Option<u32>,
    ) -> RequestId<CreateEntityRequest>;
    fn send_delete_entity_request(
        &mut self,
        payload: DeleteEntityRequest,
        timeout_millis: Option<u32>,
    ) -> RequestId<DeleteEntityRequest>;
    fn send_entity_query_request(
        &mut self,
        payload: EntityQueryRequest,
        timeout_millis: Option<u32>,
    ) -> RequestId<EntityQueryRequest>;

    fn send_command_request(
        &mut self,
        entity_id: EntityId,
        request: CommandRequest,
        timeout_millis: Option<u32>,
        command_parameters: CommandParameters,
    ) -> RequestId<OutgoingCommandRequest>;

    fn send_command_response(
        &mut self,
        request_id: RequestId<IncomingCommandRequest>,
        response: CommandResponse,
    );

    fn send_command_failure(
        &mut self,
        request_id: RequestId<IncomingCommandRequest>,
        message: &str,
    );

    fn send_component_update(&mut self, entity_id: EntityId, component_update: ComponentUpdate);
    fn send_component_interest(
        &mut self,
        entity_id: EntityId,
        interest_overrides: &Vec<InterestOverride>,
    );

    fn send_authority_loss_imminent_acknowledgement(
        &mut self,
        entity_id: EntityId,
        component_id: u32,
    );

    fn set_protocol_logging_enabled(&mut self, enabled: bool);
    fn is_connected(&self) -> bool;
    fn get_worker_id(&self) -> &str;
    fn get_worker_attributes(&self) -> Vec<String>;
    fn get_worker_flag<T: Into<Vec<u8>>>(&self, name: T) -> Option<String>;

    fn get_op_list(&mut self, timeout_millis: u32) -> OpList;
}

pub struct WorkerConnection {
    connection_ptr: *mut Worker_Connection,
    worker_id: String,
}

impl WorkerConnection {
    pub(crate) fn new(connection_ptr: *mut Worker_Connection) -> Self {
        unsafe {
            let worker_id = Worker_Connection_GetWorkerId(connection_ptr);
            let cstr = CStr::from_ptr(worker_id);

            WorkerConnection {
                connection_ptr,
                worker_id: cstr.to_string_lossy().to_string(),
            }
        }
    }

    pub fn connect_receptionist_async(
        worker_id: &str,
        hostname: &str,
        port: u16,
        params: &ConnectionParameters,
    ) -> WorkerConnectionFuture {
        let hostname_cstr = CString::new(hostname).expect("Received 0 byte in supplied hostname.");
        let worker_id_cstr =
            CString::new(worker_id).expect("Received 0 byte in supplied Worker ID");
        let conn_params = params.to_worker_sdk();
        let future_ptr = unsafe {
            Worker_ConnectAsync(
                hostname_cstr.as_ptr(),
                port,
                worker_id_cstr.as_ptr(),
                &conn_params.native_struct,
            )
        };
        assert!(!future_ptr.is_null());
        WorkerConnectionFuture::new(future_ptr)
    }

    pub fn connect_locator_async(
        locator: &Locator,
        deployment_name: &str,
        params: &ConnectionParameters,
        callback: QueueStatusCallback,
    ) -> WorkerConnectionFuture {
        let deployment_name_cstr = CString::new(deployment_name).unwrap();
        let connection_params = params.to_worker_sdk();

        let callback = Box::new(callback);
        let callback_ptr = Box::into_raw(callback) as *mut ::std::os::raw::c_void;

        unsafe {
            let ptr = Worker_Locator_ConnectAsync(
                locator.locator,
                deployment_name_cstr.as_ptr(),
                &connection_params.native_struct,
                callback_ptr,
                Some(queue_status_callback_handler),
            );
            WorkerConnectionFuture {
                future_ptr: ptr,
                was_consumed: false,
                queue_status_callback: Some(callback_ptr),
            }
        }
    }

    pub fn get_disconnect_reason(&self, op_list: &OpList) -> Option<String> {
        op_list
            .ops
            .iter()
            .filter_map(|op| match op {
                WorkerOp::Disconnect(op) => Some(op.reason.clone()),
                _ => None,
            })
            .next()
    }
}

impl Connection for WorkerConnection {
    fn send_log_message<T: Into<Vec<u8>>, U: Into<Vec<u8>>>(
        &mut self,
        level: LogLevel,
        logger_name: T,
        message: U,
        entity_id: Option<EntityId>,
    ) {
        assert!(!self.connection_ptr.is_null());

        let logger_name = CString::new(logger_name).unwrap();
        let message = CString::new(message).unwrap();

        unsafe {
            let log_message = Worker_LogMessage {
                level: level.to_worker_sdk(),
                logger_name: logger_name.as_ptr(),
                message: message.as_ptr(),
                entity_id: match entity_id {
                    Some(e) => &e.id,
                    None => ptr::null(),
                },
            };

            Worker_Connection_SendLogMessage(self.connection_ptr, &log_message);
        }
    }

    fn send_metrics(&mut self, metrics: &Metrics) {
        assert!(!self.connection_ptr.is_null());
        let worker_metrics = metrics.to_worker_sdk();
        unsafe { Worker_Connection_SendMetrics(self.connection_ptr, &worker_metrics.metrics) }
    }

    fn send_reserve_entity_ids_request(
        &mut self,
        payload: ReserveEntityIdsRequest,
        timeout_millis: Option<u32>,
    ) -> RequestId<ReserveEntityIdsRequest> {
        unsafe {
            let timeout = match timeout_millis {
                Some(c) => &c,
                None => ptr::null(),
            };
            let id = Worker_Connection_SendReserveEntityIdsRequest(
                self.connection_ptr,
                payload.0,
                timeout,
            );
            RequestId::new(id)
        }
    }

    fn send_create_entity_request(
        &mut self,
        _payload: CreateEntityRequest,
        _timeout_millis: Option<u32>,
    ) -> RequestId<CreateEntityRequest> {
        unimplemented!()
    }

    fn send_delete_entity_request(
        &mut self,
        payload: DeleteEntityRequest,
        timeout_millis: Option<u32>,
    ) -> RequestId<DeleteEntityRequest> {
        unsafe {
            let timeout = match timeout_millis {
                Some(c) => &c,
                None => ptr::null(),
            };
            let id = Worker_Connection_SendDeleteEntityRequest(
                self.connection_ptr,
                payload.0.id,
                timeout,
            );
            RequestId::new(id)
        }
    }

    fn send_entity_query_request(
        &mut self,
        payload: EntityQueryRequest,
        timeout_millis: Option<u32>,
    ) -> RequestId<EntityQueryRequest> {
        unsafe {
            let timeout = match timeout_millis {
                Some(c) => &c,
                None => ptr::null(),
            };

            let worker_query = payload.0.to_worker_sdk();
            let id = Worker_Connection_SendEntityQueryRequest(
                self.connection_ptr,
                &worker_query.query,
                timeout,
            );
            RequestId::new(id)
        }
    }

    fn send_command_request(
        &mut self,
        _entity_id: EntityId,
        _request: CommandRequest,
        _timeout_millis: Option<u32>,
        _command_parameters: CommandParameters,
    ) -> RequestId<OutgoingCommandRequest> {
        unimplemented!()
    }

    fn send_command_response(
        &mut self,
        _request_id: RequestId<IncomingCommandRequest>,
        _response: CommandResponse,
    ) {
        unimplemented!()
    }

    fn send_command_failure(
        &mut self,
        _request_id: RequestId<IncomingCommandRequest>,
        _message: &str,
    ) {
        unimplemented!()
    }

    fn send_component_update(&mut self, _entity_id: EntityId, _component_update: ComponentUpdate) {
        unimplemented!()
    }

    fn send_component_interest(
        &mut self,
        entity_id: EntityId,
        interest_overrides: &Vec<InterestOverride>,
    ) {
        assert!(!self.connection_ptr.is_null());
        let worker_sdk_overrides = interest_overrides
            .iter()
            .map(|x| x.to_woker_sdk())
            .collect::<Vec<Worker_InterestOverride>>();

        unsafe {
            Worker_Connection_SendComponentInterest(
                self.connection_ptr,
                entity_id.id,
                worker_sdk_overrides.as_ptr(),
                worker_sdk_overrides.len() as u32,
            );
        }
    }

    fn send_authority_loss_imminent_acknowledgement(
        &mut self,
        entity_id: EntityId,
        component_id: u32,
    ) {
        assert!(!self.connection_ptr.is_null());

        unsafe {
            Worker_Connection_SendAuthorityLossImminentAcknowledgement(
                self.connection_ptr,
                entity_id.id,
                component_id,
            );
        }
    }

    fn set_protocol_logging_enabled(&mut self, enabled: bool) {
        assert!(!self.connection_ptr.is_null());

        unsafe {
            Worker_Connection_SetProtocolLoggingEnabled(self.connection_ptr, enabled as u8);
        }
    }

    fn is_connected(&self) -> bool {
        assert!(!self.connection_ptr.is_null());
        (unsafe { Worker_Connection_IsConnected(self.connection_ptr) } != 0)
    }

    fn get_worker_id(&self) -> &str {
        &self.worker_id
    }

    fn get_worker_attributes(&self) -> Vec<String> {
        assert!(!self.connection_ptr.is_null());
        unsafe {
            let sdk_attr = Worker_Connection_GetWorkerAttributes(self.connection_ptr);
            ::std::slice::from_raw_parts(
                (*sdk_attr).attributes,
                (*sdk_attr).attribute_count as usize,
            )
            .iter()
            .map(|s| CStr::from_ptr(*s).to_string_lossy().to_string())
            .collect()
        }
    }

    fn get_worker_flag<T: Into<Vec<u8>>>(&self, name: T) -> Option<String> {
        let flag_name = CString::new(name).unwrap();

        extern "C" fn worker_flag_handler(
            user_data: *mut ::std::os::raw::c_void,
            value: *const ::std::os::raw::c_char,
        ) {
            unsafe {
                if value.is_null() {
                    return;
                }
                let data: &mut Option<String> = &mut *(user_data as *mut Option<String>);
                let str = CStr::from_ptr(value).to_string_lossy().to_string();
                *data = Some(str);
            }
        };

        let mut data: Option<String> = None;
        unsafe {
            Worker_Connection_GetFlag(
                self.connection_ptr,
                flag_name.as_ptr(),
                (&mut data as *mut Option<String>) as *mut ::std::os::raw::c_void,
                Some(worker_flag_handler),
            );
        }

        data
    }

    fn get_op_list(&mut self, timeout_millis: u32) -> OpList {
        assert!(!self.connection_ptr.is_null());
        let raw_op_list =
            unsafe { Worker_Connection_GetOpList(self.connection_ptr, timeout_millis) };
        assert!(!raw_op_list.is_null());
        OpList::new(raw_op_list)
    }
}

impl Drop for WorkerConnection {
    fn drop(&mut self) {
        assert!(!self.connection_ptr.is_null());
        unsafe { Worker_Connection_Destroy(self.connection_ptr) };
    }
}

pub struct WorkerConnectionFuture {
    future_ptr: *mut Worker_ConnectionFuture,
    was_consumed: bool,
    queue_status_callback: Option<*mut ::std::os::raw::c_void>,
}

impl WorkerConnectionFuture {
    pub(crate) fn new(ptr: *mut Worker_ConnectionFuture) -> Self {
        WorkerConnectionFuture {
            future_ptr: ptr,
            was_consumed: false,
            queue_status_callback: None,
        }
    }

    pub fn get(&mut self) -> Result<WorkerConnection, String> {
        if self.was_consumed {
            return Err("WorkerConnectionFuture has already been consumed.".to_owned());
        }

        assert!(!self.future_ptr.is_null());
        let connection_ptr = unsafe { Worker_ConnectionFuture_Get(self.future_ptr, ptr::null()) };
        assert!(!connection_ptr.is_null());

        self.was_consumed = true;
        let mut worker_connection = WorkerConnection::new(connection_ptr);

        if worker_connection.is_connected() {
            Ok(worker_connection)
        } else {
            let op_list = worker_connection.get_op_list(0);
            let disconnect_reason = worker_connection.get_disconnect_reason(&op_list);
            match disconnect_reason {
                Some(v) => Err(v),
                None => Err("No disconnect op found in ops list.".to_owned()),
            }
        }
    }

    pub fn poll(&mut self, timeout_millis: u32) -> Option<Result<WorkerConnection, String>> {
        if self.was_consumed {
            return Some(Err(
                "WorkerConnectionFuture has already been consumed.".to_owned()
            ));
        }

        assert!(!self.future_ptr.is_null());
        let connection_ptr =
            unsafe { Worker_ConnectionFuture_Get(self.future_ptr, &timeout_millis) };

        if connection_ptr.is_null() {
            // The get operation timed out.
            None
        } else {
            // Connection future has returned - either a valid connection or a failed connection.
            let mut worker_connection = WorkerConnection::new(connection_ptr);
            self.was_consumed = true;
            match worker_connection.is_connected() {
                true => Some(Ok(worker_connection)),
                false => {
                    let op_list = worker_connection.get_op_list(0); // Segfaults!
                    let disconnect_reason = worker_connection.get_disconnect_reason(&op_list);

                    match disconnect_reason {
                        Some(v) => Some(Err(v)),
                        None => Some(Err("No disconnect op found in ops list.".to_owned())),
                    }
                }
            }
        }
    }
}

impl Drop for WorkerConnectionFuture {
    fn drop(&mut self) {
        assert!(!self.future_ptr.is_null());
        unsafe {
            Worker_ConnectionFuture_Destroy(self.future_ptr);

            match self.queue_status_callback {
                Some(ptr) => {
                    // Drop the callback
                    let _callback = Box::from_raw(ptr as *mut QueueStatusCallback);
                }
                None => {}
            }
        };
    }
}
