use crate::ptr::MutPtr;
use crate::worker::{
    commands::*,
    component::*,
    locator::*,
    metrics::Metrics,
    op::OpList,
    parameters::ConnectionParameters,
    utils::cstr_to_string,
    worker_future::{WorkerFuture, WorkerSdkFuture},
    {EntityId, InterestOverride, LogLevel, RequestId},
};
use spatialos_sdk_sys::worker::*;
use std::{
    error::Error,
    ffi::{CStr, CString, NulError},
    fmt::{Display, Formatter},
    ptr,
    result::Result,
};

pub type ConnectionStatus = Result<(), ConnectionStatusError>;

#[derive(Clone, PartialOrd, PartialEq, Eq, Debug)]
pub struct ConnectionStatusError {
    pub code: ConnectionStatusErrorCode,
    pub detail: String,
}

#[derive(Clone, PartialOrd, PartialEq, Eq, Debug)]
pub enum ConnectionStatusErrorCode {
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
    Unknown,
}

impl From<i32> for ConnectionStatusErrorCode {
    fn from(code: i32) -> Self {
        match code {
            Worker_ConnectionStatusCode_WORKER_CONNECTION_STATUS_CODE_INTERNAL_ERROR => {
                ConnectionStatusErrorCode::InternalError
            }
            Worker_ConnectionStatusCode_WORKER_CONNECTION_STATUS_CODE_INVALID_ARGUMENT => {
                ConnectionStatusErrorCode::InvalidArgument
            }
            Worker_ConnectionStatusCode_WORKER_CONNECTION_STATUS_CODE_NETWORK_ERROR => {
                ConnectionStatusErrorCode::NetworkError
            }
            Worker_ConnectionStatusCode_WORKER_CONNECTION_STATUS_CODE_TIMEOUT => {
                ConnectionStatusErrorCode::Timeout
            }
            Worker_ConnectionStatusCode_WORKER_CONNECTION_STATUS_CODE_CANCELLED => {
                ConnectionStatusErrorCode::Cancelled
            }
            Worker_ConnectionStatusCode_WORKER_CONNECTION_STATUS_CODE_REJECTED => {
                ConnectionStatusErrorCode::Rejected
            }
            Worker_ConnectionStatusCode_WORKER_CONNECTION_STATUS_CODE_PLAYER_IDENTITY_TOKEN_EXPIRED => {
                ConnectionStatusErrorCode::PlayerIdentityTokenExpired
            }
            Worker_ConnectionStatusCode_WORKER_CONNECTION_STATUS_CODE_LOGIN_TOKEN_EXPIRED => {
                ConnectionStatusErrorCode::LoginTokenExpired
            }
            Worker_ConnectionStatusCode_WORKER_CONNECTION_STATUS_CODE_CAPACITY_EXCEEDED => {
                ConnectionStatusErrorCode::CapacityExceeded
            }
            Worker_ConnectionStatusCode_WORKER_CONNECTION_STATUS_CODE_RATE_EXCEEDED => {
                ConnectionStatusErrorCode::RateExceeded
            }
            Worker_ConnectionStatusCode_WORKER_CONNECTION_STATUS_CODE_SERVER_SHUTDOWN => {
                ConnectionStatusErrorCode::ServerShutdown
            }
            _ => ConnectionStatusErrorCode::Unknown,
        }
    }
}

impl Display for ConnectionStatusError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.code {
            ConnectionStatusErrorCode::InternalError => {
                f.write_fmt(format_args!("Internal error: {}", self.detail))
            }
            ConnectionStatusErrorCode::InvalidArgument => {
                f.write_fmt(format_args!("Invalid argument: {}", self.detail))
            }
            ConnectionStatusErrorCode::NetworkError => {
                f.write_fmt(format_args!("Network error: {}", self.detail))
            }
            ConnectionStatusErrorCode::Timeout => {
                f.write_fmt(format_args!("Timeout: {}", self.detail))
            }
            ConnectionStatusErrorCode::Cancelled => {
                f.write_fmt(format_args!("Cancelled: {}", self.detail))
            }
            ConnectionStatusErrorCode::Rejected => {
                f.write_fmt(format_args!("Rejected: {}", self.detail))
            }
            ConnectionStatusErrorCode::PlayerIdentityTokenExpired => f.write_fmt(format_args!(
                "Player identity token expired: {}",
                self.detail
            )),
            ConnectionStatusErrorCode::LoginTokenExpired => {
                f.write_fmt(format_args!("Login token expired: {}", self.detail))
            }
            ConnectionStatusErrorCode::CapacityExceeded => {
                f.write_fmt(format_args!("Capacity exceeded: {}", self.detail))
            }
            ConnectionStatusErrorCode::RateExceeded => {
                f.write_fmt(format_args!("Rate exceeded: {}", self.detail))
            }
            ConnectionStatusErrorCode::ServerShutdown => {
                f.write_fmt(format_args!("Server shutdown: {}", self.detail))
            }
            ConnectionStatusErrorCode::Unknown => {
                f.write_fmt(format_args!("Unknown error: {}", self.detail))
            }
        }
    }
}

impl Error for ConnectionStatusError {}

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
    ) -> RequestId;
    fn send_create_entity_request(
        &mut self,
        payload: CreateEntityRequest,
        timeout_millis: Option<u32>,
    ) -> RequestId;
    fn send_delete_entity_request(
        &mut self,
        payload: DeleteEntityRequest,
        timeout_millis: Option<u32>,
    ) -> RequestId;
    fn send_entity_query_request(
        &mut self,
        payload: EntityQueryRequest,
        timeout_millis: Option<u32>,
    ) -> RequestId;

    fn send_command_request<T: Into<CommandRequest>>(
        &mut self,
        entity_id: EntityId,
        request: T,
        timeout_millis: Option<u32>,
        params: CommandParameters,
    ) -> RequestId;

    fn send_command_response<T: Into<CommandResponse>>(
        &mut self,
        request_id: RequestId,
        response: T,
    );

    fn send_command_failure(
        &mut self,
        request_id: RequestId,
        message: &str,
    ) -> Result<(), NulError>;

    fn send_component_update<T: Into<ComponentUpdate>>(
        &mut self,
        entity_id: EntityId,
        update: T,
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

    fn flush(&mut self);

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

    pub fn connect_receptionist(
        worker_id: &str,
        hostname: &str,
        port: u16,
        params: ConnectionParameters,
    ) -> WorkerFuture<WorkerConnectionFuture> {
        let hostname_cstr = CString::new(hostname).expect("Received 0 byte in supplied hostname.");
        let worker_id_cstr =
            CString::new(worker_id).expect("Received 0 byte in supplied Worker ID");

        let future =
            WorkerConnectionFuture::Receptionist(hostname_cstr, worker_id_cstr, port, params);

        WorkerFuture::new(future)
    }

    pub fn connect_locator(
        locator: Locator,
        params: ConnectionParameters,
    ) -> WorkerFuture<WorkerConnectionFuture> {
        WorkerFuture::new(WorkerConnectionFuture::Locator(locator, params))
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
    ) -> RequestId {
        unsafe {
            let timeout = match timeout_millis {
                Some(c) => &c,
                None => ptr::null(),
            };
            RequestId::new(Worker_Connection_SendReserveEntityIdsRequest(
                self.connection_ptr.get(),
                payload.0,
                timeout,
            ))
        }
    }

    fn send_create_entity_request(
        &mut self,
        payload: CreateEntityRequest,
        timeout_millis: Option<u32>,
    ) -> RequestId {
        let timeout = match timeout_millis {
            Some(c) => &c,
            None => ptr::null(),
        };
        let entity_id = match payload.1 {
            Some(e) => &e.id,
            None => ptr::null(),
        };
        let mut component_data = payload.0.into_raw();
        unsafe {
            RequestId::new(Worker_Connection_SendCreateEntityRequest(
                self.connection_ptr.get(),
                component_data.len() as _,
                component_data.as_mut_ptr(),
                entity_id,
                timeout,
            ))
        }
    }

    fn send_delete_entity_request(
        &mut self,
        payload: DeleteEntityRequest,
        timeout_millis: Option<u32>,
    ) -> RequestId {
        unsafe {
            let timeout = match timeout_millis {
                Some(c) => &c,
                None => ptr::null(),
            };
            RequestId::new(Worker_Connection_SendDeleteEntityRequest(
                self.connection_ptr.get(),
                payload.0.id,
                timeout,
            ))
        }
    }

    fn send_entity_query_request(
        &mut self,
        payload: EntityQueryRequest,
        timeout_millis: Option<u32>,
    ) -> RequestId {
        unsafe {
            let timeout = match timeout_millis {
                Some(c) => &c,
                None => ptr::null(),
            };

            let worker_query = payload.0.to_worker_sdk();
            RequestId::new(Worker_Connection_SendEntityQueryRequest(
                self.connection_ptr.get(),
                &worker_query.query,
                timeout,
            ))
        }
    }

    fn send_command_request<T: Into<CommandRequest>>(
        &mut self,
        entity_id: EntityId,
        request: T,
        timeout_millis: Option<u32>,
        params: CommandParameters,
    ) -> RequestId {
        let timeout = match timeout_millis {
            Some(c) => &c,
            None => ptr::null(),
        };

        let command = request.into();

        let mut command_request = Worker_CommandRequest {
            reserved: ptr::null_mut(),
            component_id: command.component_id,
            command_index: command.command_index,
            schema_type: command.schema_data.into_raw(),
            user_handle: ptr::null_mut(),
        };

        unsafe {
            RequestId::new(Worker_Connection_SendCommandRequest(
                self.connection_ptr.get(),
                entity_id.id,
                &mut command_request,
                timeout,
                &params.to_worker_sdk(),
            ))
        }
    }

    fn send_command_response<T: Into<CommandResponse>>(
        &mut self,
        request_id: RequestId,
        response: T,
    ) {
        let command = response.into();

        let mut raw_response = Worker_CommandResponse {
            reserved: ptr::null_mut(),
            component_id: command.component_id,
            command_index: command.command_index,
            schema_type: command.schema_data.into_raw(),
            user_handle: ptr::null_mut(),
        };

        unsafe {
            Worker_Connection_SendCommandResponse(
                self.connection_ptr.get(),
                request_id.id,
                &mut raw_response,
            );
        }
    }

    fn send_command_failure(
        &mut self,
        request_id: RequestId,
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

    fn send_component_update<T: Into<ComponentUpdate>>(
        &mut self,
        entity_id: EntityId,
        update: T,
        parameters: UpdateParameters,
    ) {
        let update = update.into();
        let mut component_update = Worker_ComponentUpdate {
            reserved: ptr::null_mut(),
            component_id: update.component_id,
            schema_type: update.schema_data.into_raw(),
            user_handle: ptr::null_mut(),
        };

        let params = parameters.to_worker_sdk();
        unsafe {
            Worker_Connection_SendComponentUpdate(
                self.connection_ptr.get(),
                entity_id.id,
                &mut component_update,
                &params,
            );
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

    fn flush(&mut self) {
        assert!(!self.connection_ptr.is_null());
        unsafe { Worker_Connection_Alpha_Flush(self.connection_ptr.get()) }
    }

    fn set_protocol_logging_enabled(&mut self, enabled: bool) {
        assert!(!self.connection_ptr.is_null());

        unsafe {
            Worker_Connection_SetProtocolLoggingEnabled(self.connection_ptr.get(), enabled as u8);
        }
    }

    fn get_connection_status(&mut self) -> ConnectionStatus {
        let ptr = self.connection_ptr.get();

        let code = unsafe { Worker_Connection_GetConnectionStatusCode(ptr) } as i32;

        if code == Worker_ConnectionStatusCode_WORKER_CONNECTION_STATUS_CODE_SUCCESS {
            return Ok(());
        }

        let detail =
            cstr_to_string(unsafe { Worker_Connection_GetConnectionStatusDetailString(ptr) });

        Err(ConnectionStatusError {
            code: ConnectionStatusErrorCode::from(code),
            detail,
        })
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

pub enum WorkerConnectionFuture {
    Receptionist(CString, CString, u16, ConnectionParameters),
    Locator(Locator, ConnectionParameters),
}

unsafe impl Send for WorkerConnectionFuture {}

impl WorkerSdkFuture for WorkerConnectionFuture {
    type RawPointer = Worker_ConnectionFuture;
    type Output = Result<WorkerConnection, ConnectionStatusError>;

    fn start(&self) -> *mut Self::RawPointer {
        let default_vtable = Default::default();
        match &self {
            WorkerConnectionFuture::Receptionist(hostname, worker_id, port, params) => {
                let params = params.flatten();
                unsafe {
                    Worker_ConnectAsync(
                        hostname.as_ptr(),
                        *port,
                        worker_id.as_ptr(),
                        &params.as_raw(&default_vtable),
                    )
                }
            }
            WorkerConnectionFuture::Locator(locator, params) => {
                let params = params.flatten();
                unsafe {
                    Worker_Locator_ConnectAsync(locator.locator, &params.as_raw(&default_vtable))
                }
            }
        }
    }

    unsafe fn get(
        ptr: *mut Worker_ConnectionFuture,
    ) -> Result<WorkerConnection, ConnectionStatusError> {
        let connection_ptr = Worker_ConnectionFuture_Get(ptr, ptr::null());
        assert!(!connection_ptr.is_null());

        let mut connection = WorkerConnection::new(connection_ptr);
        let status = connection.get_connection_status();
        status.map(|()| connection)
    }

    unsafe fn destroy(ptr: *mut Worker_ConnectionFuture) {
        Worker_ConnectionFuture_Destroy(ptr);
    }
}
