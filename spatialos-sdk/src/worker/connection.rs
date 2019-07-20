use crate::ptr::MutPtr;
use crate::worker::parameters::ProtocolType;
use crate::worker::{
    commands::*,
    component::{self, Component, UpdateParameters},
    entity::Entity,
    internal::utils::cstr_to_string,
    locator::*,
    metrics::Metrics,
    op::OpList,
    parameters::ConnectionParameters,
    {EntityId, InterestOverride, LogLevel, RequestId},
};
use futures::{Async, Future};
use spatialos_sdk_sys::worker::*;
use std::{
    ffi::{CStr, CString, NulError},
    ptr,
};

/// Information about the status of a worker connection or network request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionStatus {
    /// The status of the connection or request.
    pub code: ConnectionStatusCode,

    /// Detailed, human-readable description of the connection status.
    ///
    /// Will be "OK" if no error occurred.
    pub detail: String,
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Debug)]
pub enum ConnectionStatusCode {
    Success,
    InternalError,
    InvalidArgument,
    NetworkError,
    Timeout,
    Cancelled,
    Rejected,
    PlayerIdentityTokenExpired,
    LoginTokenExpired,
    CapacityExceeded,
    RateExceeded,
    ServerShutdown,
}

impl From<u8> for ConnectionStatusCode {
    fn from(value: u8) -> Self {
        match u32::from(value) {
            1 => ConnectionStatusCode::Success,
            2 => ConnectionStatusCode::InternalError,
            3 => ConnectionStatusCode::InvalidArgument,
            4 => ConnectionStatusCode::NetworkError,
            5 => ConnectionStatusCode::Timeout,
            6 => ConnectionStatusCode::Cancelled,
            7 => ConnectionStatusCode::Rejected,
            8 => ConnectionStatusCode::PlayerIdentityTokenExpired,
            9 => ConnectionStatusCode::LoginTokenExpired,
            10 => ConnectionStatusCode::CapacityExceeded,
            11 => ConnectionStatusCode::RateExceeded,
            12 => ConnectionStatusCode::ServerShutdown,
            _ => panic!(format!("Unknown connection status code: {}", value)),
        }
    }
}

/// Connection trait to allow for mocking the connection.
pub trait Connection {
    fn send_log_message(
        &mut self,
        level: LogLevel,
        logger_name: &str,
        message: &str,
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
        entity: Entity,
        entity_id: Option<EntityId>,
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

    fn send_command_request<C: Component>(
        &mut self,
        entity_id: EntityId,
        request: C::CommandRequest,
        timeout_millis: Option<u32>,
        params: CommandParameters,
    ) -> RequestId<OutgoingCommandRequest>;

    fn send_command_response<C: Component>(
        &mut self,
        request_id: RequestId<IncomingCommandRequest>,
        response: C::CommandResponse,
    );

    fn send_command_failure(
        &mut self,
        request_id: RequestId<IncomingCommandRequest>,
        message: &str,
    ) -> Result<(), NulError>;

    fn send_component_update<C: Component>(
        &mut self,
        entity_id: EntityId,
        update: C::Update,
        parameters: UpdateParameters,
    );

    fn send_component_interest(
        &mut self,
        entity_id: EntityId,
        interest_overrides: &[InterestOverride],
    );

    fn send_authority_loss_imminent_acknowledgement(
        &mut self,
        entity_id: EntityId,
        component_id: u32,
    );

    fn set_protocol_logging_enabled(&mut self, enabled: bool);

    fn get_connection_status(&mut self) -> ConnectionStatus;

    fn get_worker_flag(&mut self, name: &str) -> Option<String>;

    fn get_op_list(&mut self, timeout_millis: u32) -> OpList;

    fn get_worker_id(&self) -> &str;

    fn get_worker_attributes(&self) -> &[String];
}

pub struct WorkerConnection {
    // NOTE: The `Worker_Connection` pointer is wrapped in a `MutPtr` to ensure
    // that we only attempt to use the connection pointer in methods that take
    // `&mut self`. This enforces the thread-safety requirements of
    // `Worker_Connection`. See this forum post for more information:
    // https://forums.improbable.io/t/thread-safety-of-worker-connection-object/5358/2
    //
    // TODO: Replace the forum post URL with the actual relevant C API docs, once
    // the docs have been updated to clarify the thread-safety requirements of
    // the worker connection object.
    connection_ptr: MutPtr<Worker_Connection>,

    // Cached copies of static connection data. These are stored internally so that we can guarantee it will be safe to access this data through `&self`.
    id: String,
    attributes: Vec<String>,
}

#[allow(unused_assignments)]
fn do_connect<F>(connect: F, params: &ConnectionParameters) -> WorkerConnectionFuture
where
    F: FnOnce(&Worker_ConnectionParameters) -> WorkerConnectionFuture,
{
    let mut conn_params = params.to_worker_sdk();

    // conn_params.network will not have the protocol parameters setup (due to requiring pointers to things on the stack). Therefore, we do that here.
    let mut udp_kcp_params = Worker_Alpha_KcpParameters::default();
    let mut udp_erasure_codec_params = Worker_ErasureCodecParameters::default();
    let mut udp_heartbeat_params = Worker_HeartbeatParameters::default();
    let mut udp_flow_control_params = Worker_Alpha_FlowControlParameters::default();
    match &params.network.protocol {
        ProtocolType::Tcp(tcp_params) => {
            conn_params.network.connection_type =
                Worker_NetworkConnectionType_WORKER_NETWORK_CONNECTION_TYPE_TCP as u8;
            conn_params.network.tcp = tcp_params.to_worker_sdk();
        }
        ProtocolType::Udp(udp_params) => {
            conn_params.network.connection_type =
                Worker_NetworkConnectionType_WORKER_NETWORK_CONNECTION_TYPE_MODULAR_UDP as u8;
            conn_params.network.modular_udp.security_type =
                udp_params.security_type.to_worker_sdk();
            // These parameters should be zero-initialized (null pointers) if they are not matched here.
            if let Some(kcp_params) = &udp_params.kcp {
                udp_kcp_params = kcp_params.to_worker_sdk();
                conn_params.network.modular_udp.downstream_kcp = &udp_kcp_params;
                conn_params.network.modular_udp.upstream_kcp = &udp_kcp_params;
            }
            if let Some(erasure_codec_params) = &udp_params.erasure_codec {
                udp_erasure_codec_params = erasure_codec_params.to_worker_sdk();
                conn_params.network.modular_udp.downstream_erasure_codec =
                    &udp_erasure_codec_params;
                conn_params.network.modular_udp.upstream_erasure_codec = &udp_erasure_codec_params;
            }
            if let Some(heartbeat_params) = &udp_params.heartbeat {
                udp_heartbeat_params = heartbeat_params.to_worker_sdk();
                conn_params.network.modular_udp.downstream_heartbeat = &udp_heartbeat_params;
                conn_params.network.modular_udp.upstream_heartbeat = &udp_heartbeat_params;
            }
            if let Some(flow_control_params) = &udp_params.flow_control {
                udp_flow_control_params = flow_control_params.to_worker_sdk();
                conn_params.network.modular_udp.flow_control = &udp_flow_control_params;
            }
        }
    }

    connect(&conn_params)
}

impl WorkerConnection {
    pub(crate) fn new(connection_ptr: *mut Worker_Connection) -> Self {
        unsafe {
            let worker_id = Worker_Connection_GetWorkerId(connection_ptr);
            let cstr = CStr::from_ptr(worker_id);

            let sdk_attr = Worker_Connection_GetWorkerAttributes(connection_ptr);

            let attributes = if (*sdk_attr).attributes.is_null() {
                Vec::new()
            } else {
                ::std::slice::from_raw_parts(
                    (*sdk_attr).attributes,
                    (*sdk_attr).attribute_count as usize,
                )
                .iter()
                .map(|s| CStr::from_ptr(*s).to_string_lossy().to_string())
                .collect()
            };

            WorkerConnection {
                connection_ptr: MutPtr::new(connection_ptr),
                id: cstr.to_string_lossy().to_string(),
                attributes,
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
        do_connect(
            move |conn_params| {
                let future_ptr = unsafe {
                    Worker_ConnectAsync(
                        hostname_cstr.as_ptr(),
                        port,
                        worker_id_cstr.as_ptr(),
                        conn_params,
                    )
                };
                assert!(!future_ptr.is_null());
                WorkerConnectionFuture::new(future_ptr)
            },
            params,
        )
    }

    pub fn connect_locator_async(
        locator: &Locator,
        params: &ConnectionParameters,
    ) -> WorkerConnectionFuture {
        do_connect(
            move |conn_params| {
                let future_ptr =
                    unsafe { Worker_Locator_ConnectAsync(locator.locator, conn_params) };
                assert!(!future_ptr.is_null());
                WorkerConnectionFuture::new(future_ptr)
            },
            params,
        )
    }
}

impl Connection for WorkerConnection {
    fn send_log_message(
        &mut self,
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

            Worker_Connection_SendLogMessage(self.connection_ptr.get(), &log_message);
        }
    }

    fn send_metrics(&mut self, metrics: &Metrics) {
        assert!(!self.connection_ptr.is_null());
        let worker_metrics = metrics.to_worker_sdk();
        unsafe { Worker_Connection_SendMetrics(self.connection_ptr.get(), &worker_metrics.metrics) }
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
                self.connection_ptr.get(),
                payload.0,
                timeout,
            );
            RequestId::new(id)
        }
    }

    fn send_create_entity_request(
        &mut self,
        entity: Entity,
        entity_id: Option<EntityId>,
        timeout_millis: Option<u32>,
    ) -> RequestId<CreateEntityRequest> {
        let timeout = match timeout_millis {
            Some(c) => &c,
            None => ptr::null(),
        };
        let entity_id = match entity_id {
            Some(e) => &e.id,
            None => ptr::null(),
        };
        let mut component_data = entity.raw_component_data();
        let id = unsafe {
            Worker_Connection_SendCreateEntityRequest(
                self.connection_ptr.get(),
                component_data.components.len() as _,
                component_data.components.as_mut_ptr(),
                entity_id,
                timeout,
            )
        };

        RequestId::new(id)
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
                self.connection_ptr.get(),
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
                self.connection_ptr.get(),
                &worker_query.query,
                timeout,
            );
            RequestId::new(id)
        }
    }

    fn send_command_request<C: Component>(
        &mut self,
        entity_id: EntityId,
        request: C::CommandRequest,
        timeout_millis: Option<u32>,
        params: CommandParameters,
    ) -> RequestId<OutgoingCommandRequest> {
        let command_index = C::get_request_command_index(&request);

        let timeout = match timeout_millis {
            Some(c) => &c,
            None => ptr::null(),
        };

        let mut command_request = Worker_CommandRequest {
            reserved: ptr::null_mut(),
            component_id: C::ID,
            command_index,
            schema_type: ptr::null_mut(),
            user_handle: component::handle_allocate(request),
        };

        let request_id = unsafe {
            Worker_Connection_SendCommandRequest(
                self.connection_ptr.get(),
                entity_id.id,
                &mut command_request,
                timeout,
                &params.to_worker_sdk(),
            )
        };

        unsafe {
            component::handle_free::<C::CommandRequest>(command_request.user_handle);
        }

        RequestId::new(request_id)
    }

    fn send_command_response<C: Component>(
        &mut self,
        request_id: RequestId<IncomingCommandRequest>,
        response: C::CommandResponse,
    ) {
        unsafe {
            let mut raw_response = Worker_CommandResponse {
                reserved: ptr::null_mut(),
                component_id: C::ID,
                command_index: C::get_response_command_index(&response),
                schema_type: ptr::null_mut(),
                user_handle: component::handle_allocate(response),
            };

            Worker_Connection_SendCommandResponse(
                self.connection_ptr.get(),
                request_id.id,
                &mut raw_response,
            );

            component::handle_free::<C::CommandResponse>(raw_response.user_handle);
        }
    }

    fn send_command_failure(
        &mut self,
        request_id: RequestId<IncomingCommandRequest>,
        message: &str,
    ) -> Result<(), NulError> {
        let message = CString::new(message)?;
        unsafe {
            Worker_Connection_SendCommandFailure(
                self.connection_ptr.get(),
                request_id.id,
                message.as_ptr(),
            );
        }

        Ok(())
    }

    fn send_component_update<C: Component>(
        &mut self,
        entity_id: EntityId,
        update: C::Update,
        parameters: UpdateParameters,
    ) {
        let mut component_update = Worker_ComponentUpdate {
            reserved: ptr::null_mut(),
            component_id: C::ID,
            schema_type: ptr::null_mut(),
            user_handle: component::handle_allocate(update),
        };

        let params = parameters.to_worker_sdk();
        unsafe {
            Worker_Connection_SendComponentUpdate(
                self.connection_ptr.get(),
                entity_id.id,
                &mut component_update,
                &params,
            );

            component::handle_free::<C::Update>(component_update.user_handle);
        }
    }

    fn send_component_interest(
        &mut self,
        entity_id: EntityId,
        interest_overrides: &[InterestOverride],
    ) {
        assert!(!self.connection_ptr.is_null());
        let worker_sdk_overrides = interest_overrides
            .iter()
            .map(InterestOverride::to_worker_sdk)
            .collect::<Vec<Worker_InterestOverride>>();

        unsafe {
            Worker_Connection_SendComponentInterest(
                self.connection_ptr.get(),
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
                self.connection_ptr.get(),
                entity_id.id,
                component_id,
            );
        }
    }

    fn set_protocol_logging_enabled(&mut self, enabled: bool) {
        assert!(!self.connection_ptr.is_null());

        unsafe {
            Worker_Connection_SetProtocolLoggingEnabled(self.connection_ptr.get(), enabled as u8);
        }
    }

    fn get_connection_status(&mut self) -> ConnectionStatus {
        let ptr = self.connection_ptr.get();
        unsafe {
            let code = ConnectionStatusCode::from(Worker_Connection_GetConnectionStatusCode(ptr));
            let detail = cstr_to_string(Worker_Connection_GetConnectionStatusDetailString(ptr));
            ConnectionStatus { code, detail }
        }
    }

    fn get_worker_flag(&mut self, name: &str) -> Option<String> {
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
            Worker_Connection_GetWorkerFlag(
                self.connection_ptr.get(),
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
            unsafe { Worker_Connection_GetOpList(self.connection_ptr.get(), timeout_millis) };
        assert!(!raw_op_list.is_null());
        OpList::new(raw_op_list)
    }

    fn get_worker_id(&self) -> &str {
        &self.id
    }

    fn get_worker_attributes(&self) -> &[String] {
        &self.attributes
    }
}

impl Drop for WorkerConnection {
    fn drop(&mut self) {
        assert!(!self.connection_ptr.is_null());
        unsafe { Worker_Connection_Destroy(self.connection_ptr.get()) };
    }
}

// SAFE: The worker connection object is safe to send between threads.
unsafe impl Send for WorkerConnection {}
unsafe impl Sync for WorkerConnection {}

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
}

impl Drop for WorkerConnectionFuture {
    fn drop(&mut self) {
        assert!(!self.future_ptr.is_null());
        unsafe {
            Worker_ConnectionFuture_Destroy(self.future_ptr);
        };
    }
}

impl Future for WorkerConnectionFuture {
    type Item = WorkerConnection;
    type Error = String;

    fn poll(&mut self) -> Result<Async<<Self as Future>::Item>, <Self as Future>::Error> {
        if self.was_consumed {
            return Err("WorkerConnectionFuture has already been consumed.".to_owned());
        }

        assert!(!self.future_ptr.is_null());
        let connection_ptr = unsafe { Worker_ConnectionFuture_Get(self.future_ptr, &0) };

        if connection_ptr.is_null() {
            return Ok(Async::NotReady);
        }

        self.was_consumed = true;
        let mut connection = WorkerConnection::new(connection_ptr);

        let status = connection.get_connection_status();
        if status.code == ConnectionStatusCode::Success {
            return Ok(Async::Ready(connection));
        }

        Err(status.detail)
    }

    fn wait(self) -> Result<<Self as Future>::Item, <Self as Future>::Error>
    where
        Self: Sized,
    {
        if self.was_consumed {
            return Err("WorkerConnectionFuture has already been consumed.".to_owned());
        }

        assert!(!self.future_ptr.is_null());
        let connection_ptr = unsafe { Worker_ConnectionFuture_Get(self.future_ptr, ptr::null()) };
        let mut connection = WorkerConnection::new(connection_ptr);

        let status = connection.get_connection_status();
        if status.code == ConnectionStatusCode::Success {
            return Ok(connection);
        }

        Err(status.detail)
    }
}
