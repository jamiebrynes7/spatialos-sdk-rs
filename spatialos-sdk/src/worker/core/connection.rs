use std::ffi::CString;
use std::ptr;

use worker::core::commands::*;
use worker::core::component::ComponentUpdate;
use worker::core::metrics::Metrics;
use worker::core::op::{DisconnectOp, OpList, WorkerOp};
use worker::core::parameters::{CommandParameters, ReceptionistConnectionParameters};
use worker::core::{EntityId, InterestOverride, LogLevel, RequestId};
use worker::internal::bindings::*;

/// Connection trait to allow for mocking the connection.
pub trait Connection {
    fn send_log_message(
        &self,
        level: LogLevel,
        logger_name: &str,
        message: &str,
        entity_id: Option<EntityId>,
    );
    fn send_metrics(&self, metrics: Metrics);

    fn send_reserve_entity_ids_request(&self, payload: ReserveEntityIdsRequest);
    fn send_create_entity_request(&self, payload: CreateEntityRequest);
    fn send_delete_entity_request(&self, payload: DeleteEntityRequest);
    fn send_entity_query_request(&self, payload: EntityQueryRequest);

    fn send_command_request(
        &self,
        entity_id: EntityId,
        request: CommandRequest,
        command_parameters: CommandParameters,
    ) -> RequestId<OutgoingCommandRequest>;
    fn send_command_response(
        &self,
        request_id: RequestId<IncomingCommandRequest>,
        response: CommandResponse,
    );
    fn send_command_failure(&self, request_id: RequestId<IncomingCommandRequest>, message: &str);

    fn send_component_update(&self, entity_id: EntityId, component_update: ComponentUpdate);
    fn send_component_interest(
        &self,
        entity_id: EntityId,
        interest_overrides: &Vec<InterestOverride>,
    );

    fn send_authority_loss_imminent_acknowledgement(&self, entity_id: EntityId, component_id: u32);

    fn set_protocol_logging_enabled(&self, enabled: bool);
    fn is_connected(&self) -> bool;
    fn get_worker_id(&self) -> String;
    fn get_worker_attributes(&self) -> Vec<String>;
    fn get_worker_flag(&self, name: &str) -> Option<String>;

    fn get_op_list(&self, timeout_millis: u32) -> OpList;
}

pub struct WorkerConnection {
    connection_ptr: *mut Worker_Connection,
}

impl WorkerConnection {
    pub fn connect_receptionist_async(
        worker_id: &str,
        params: &ReceptionistConnectionParameters,
    ) -> WorkerConnectionFuture {
        let hostname_cstr =
            CString::new(params.hostname.clone()).expect("Received 0 byte in supplied hostname.");
        let worker_id_cstr =
            CString::new(worker_id).expect("Received 0 byte in supplied Worker ID");
        let conn_params = params.connection_params.to_worker_sdk();
        let future_ptr = unsafe {
            Worker_ConnectAsync(
                hostname_cstr.as_ptr(),
                params.port,
                worker_id_cstr.as_ptr(),
                &conn_params.native_struct,
            )
        };
        assert!(!future_ptr.is_null());
        WorkerConnectionFuture::new(future_ptr)
    }

    pub fn get_disconnect_reason(&self, op_list: &OpList) -> Option<String> {
        op_list
            .ops
            .iter()
            .filter_map(|op| match op {
                WorkerOp::Disconnect(op) => Some(op.reason.clone()),
                _ => None,
            }).next()
    }
}

impl Connection for WorkerConnection {
    fn send_log_message(
        &self,
        level: LogLevel,
        logger_name: &str,
        message: &str,
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

    fn send_metrics(&self, metrics: Metrics) {
        unimplemented!()
    }

    fn send_reserve_entity_ids_request(&self, payload: ReserveEntityIdsRequest) {
        unimplemented!()
    }

    fn send_create_entity_request(&self, payload: CreateEntityRequest) {
        unimplemented!()
    }

    fn send_delete_entity_request(&self, payload: DeleteEntityRequest) {
        unimplemented!()
    }

    fn send_entity_query_request(&self, payload: EntityQueryRequest) {
        unimplemented!()
    }

    fn send_command_request(
        &self,
        entity_id: EntityId,
        request: CommandRequest,
        command_parameters: CommandParameters,
    ) -> RequestId<OutgoingCommandRequest> {
        unimplemented!()
    }

    fn send_command_response(
        &self,
        request_id: RequestId<IncomingCommandRequest>,
        response: CommandResponse,
    ) {
        unimplemented!()
    }

    fn send_command_failure(&self, request_id: RequestId<IncomingCommandRequest>, message: &str) {
        unimplemented!()
    }

    fn send_component_update(&self, entity_id: EntityId, component_update: ComponentUpdate) {
        unimplemented!()
    }

    fn send_component_interest(
        &self,
        entity_id: EntityId,
        interest_overrides: &Vec<InterestOverride>,
    ) {
        unimplemented!()
    }

    fn send_authority_loss_imminent_acknowledgement(&self, entity_id: EntityId, component_id: u32) {
        unimplemented!()
    }

    fn set_protocol_logging_enabled(&self, enabled: bool) {
        unimplemented!()
    }

    fn is_connected(&self) -> bool {
        assert!(!self.connection_ptr.is_null());
        (unsafe { Worker_Connection_IsConnected(self.connection_ptr) } != 0)
    }

    fn get_worker_id(&self) -> String {
        unimplemented!()
    }

    fn get_worker_attributes(&self) -> Vec<String> {
        unimplemented!()
    }

    fn get_worker_flag(&self, name: &str) -> Option<String> {
        unimplemented!()
    }

    fn get_op_list(&self, timeout_millis: u32) -> OpList {
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
}

impl WorkerConnectionFuture {
    pub(crate) fn new(ptr: *mut Worker_ConnectionFuture) -> Self {
        WorkerConnectionFuture {
            future_ptr: ptr,
            was_consumed: false,
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
        let worker_connection = WorkerConnection { connection_ptr };

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
            let worker_connection = WorkerConnection { connection_ptr };
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
        unsafe { Worker_ConnectionFuture_Destroy(self.future_ptr) };
    }
}
