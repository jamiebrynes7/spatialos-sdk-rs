use std::ffi::CString;

use worker::core::commands::*;
use worker::core::component::ComponentUpdate;
use worker::core::metrics::Metrics;
use worker::core::op::OpList;
use worker::core::parameters::{CommandParameters, ReceptionistConnectionParameters};
use worker::core::{EntityId, InterestOverride, LogLevel, RequestId};
use worker::internal::bindings::*;

/// Connection trait to allow for mocking the connection.
trait Connection {
    fn send_log_message(
        level: LogLevel,
        logger_name: &str,
        message: &str,
        entity_id: Option<EntityId>,
    );
    fn send_metrics(metrics: Metrics);

    fn send_reserve_entity_ids_request(payload: ReserveEntityIdsRequest);
    fn send_create_entity_request(payload: CreateEntityRequest);
    fn send_delete_entity_request(payload: DeleteEntityRequest);
    fn send_entity_query_request(payload: EntityQueryRequest);

    fn send_command_request(
        entity_id: EntityId,
        request: CommandRequest,
        command_parameters: CommandParameters,
    ) -> RequestId<OutgoingCommandRequest>;
    fn send_command_response(
        request_id: RequestId<IncomingCommandRequest>,
        response: CommandResponse,
    );
    fn send_command_failure(request_id: RequestId<IncomingCommandRequest>, message: &str);

    fn send_component_update(entity_id: EntityId, component_update: ComponentUpdate);
    fn send_component_interest(entity_id: EntityId, interest_overrides: &Vec<InterestOverride>);

    fn send_authority_loss_imminent_acknowledgement(entity_id: EntityId, component_id: u32);

    fn set_protocol_logging_enabled(enabled: bool);
    fn is_connected() -> bool;
    fn get_worker_id() -> String;
    fn get_worker_attributes() -> Vec<String>;
    fn get_worker_flag(name: &str) -> Option<String>;

    fn get_op_list(timeout_millis: u32) -> OpList;
}

pub struct WorkerConnection {
    connection_ptr: *mut Worker_Connection,
}

impl WorkerConnection {
    pub fn connect_receptionist_async(
        worker_id: &str,
        params: &mut ReceptionistConnectionParameters,
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
                &conn_params,
            )
        };
        assert!(!future_ptr.is_null());
        WorkerConnectionFuture { future_ptr }
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
}

impl WorkerConnectionFuture {
    pub(crate) fn new(ptr: *mut Worker_ConnectionFuture) -> Self {
        WorkerConnectionFuture { future_ptr: ptr }
    }

    pub fn get(&self, timeout_millis: u32) -> Option<WorkerConnection> {
        assert!(!self.future_ptr.is_null());
        let connection_ptr =
            unsafe { Worker_ConnectionFuture_Get(self.future_ptr, &timeout_millis) };

        if connection_ptr.is_null() {
            None
        } else {
            Some(WorkerConnection { connection_ptr })
        }
    }
}

impl Drop for WorkerConnectionFuture {
    fn drop(&mut self) {
        assert!(!self.future_ptr.is_null());
        unsafe { Worker_ConnectionFuture_Destroy(self.future_ptr) };
    }
}
