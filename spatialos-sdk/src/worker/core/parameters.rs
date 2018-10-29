use std::ffi::CString;
use std::ptr;

use worker::core::vtable;
use worker::internal::bindings::*;

pub enum ConnectionType {
    TCP,
    RakNet,
}

impl ConnectionType {
    fn to_u8(&self) -> u8 {
        match self {
            ConnectionType::TCP => {
                Worker_NetworkConnectionType_WORKER_NETWORK_CONNECTION_TYPE_TCP as u8
            }
            ConnectionType::RakNet => {
                Worker_NetworkConnectionType_WORKER_NETWORK_CONNECTION_TYPE_RAKNET as u8
            }
        }
    }
}

pub struct ReceptionistConnectionParameters {
    pub hostname: String,
    pub port: u16,
    pub connection_params: ConnectionParameters,
}

pub struct LocatorConnectionParameters {}

pub struct ConnectionParameters {
    pub worker_type: String,
    pub network: NetworkParameters,
    pub send_queue_capacity: u32,
    pub receive_queue_capacity: u32,
    pub log_message_queue_capacity: u32,
    pub built_in_metrics_report_period_millis: u32,
    pub protocol_logging: ProtocolLoggingParameters,
    pub enable_protocol_logging_at_startup: bool,
    pub thread_affinity: ThreadAffinityParameters,

    native_worker_type: Option<CString>,
}

impl ConnectionParameters {
    pub fn default() -> Self {
        ConnectionParameters {
            worker_type: "".to_owned(),
            network: NetworkParameters::default(),
            send_queue_capacity: WORKER_DEFAULTS_SEND_QUEUE_CAPACITY,
            receive_queue_capacity: WORKER_DEFAULTS_RECEIVE_QUEUE_CAPACITY,
            log_message_queue_capacity: WORKER_DEFAULTS_LOG_MESSAGE_QUEUE_CAPACITY,
            built_in_metrics_report_period_millis:
                WORKER_DEFAULTS_BUILT_IN_METRICS_REPORT_PERIOD_MILLIS,
            protocol_logging: ProtocolLoggingParameters::default(),
            enable_protocol_logging_at_startup: false,
            thread_affinity: ThreadAffinityParameters::default(),

            native_worker_type: None,
        }
    }

    pub(crate) fn to_worker_sdk(&mut self) -> Worker_ConnectionParameters {
        let worker_type_cstr = CString::new(self.worker_type.clone())
            .expect("Received 0 byte in supplied worker_type.");
        let ptr = worker_type_cstr.as_ptr();
        self.native_worker_type = Some(worker_type_cstr);
        Worker_ConnectionParameters {
            worker_type: ptr,
            network: self.network.to_worker_sdk(),
            send_queue_capacity: self.send_queue_capacity,
            receive_queue_capacity: self.receive_queue_capacity,
            log_message_queue_capacity: self.log_message_queue_capacity,
            built_in_metrics_report_period_millis: self.built_in_metrics_report_period_millis,
            protocol_logging: self.protocol_logging.to_worker_sdk(),
            enable_protocol_logging_at_startup: self.enable_protocol_logging_at_startup as u8,
            thread_affinity: self.thread_affinity.to_worker_sdk(),
            component_vtable_count: 0,
            component_vtables: ptr::null(),
            default_component_vtable: &vtable::PASSTHROUGH_VTABLE,
        }
    }
}

pub struct NetworkParameters {
    pub use_external_ip: bool,
    pub connection_type: ConnectionType,
    pub raknet: RakNetNetworkParameters,
    pub tcp: TcpNetworkParameters,
    pub connection_timeout_millis: u64,
    pub default_command_timeout_millis: u32,
}

impl NetworkParameters {
    pub fn default() -> Self {
        NetworkParameters {
            use_external_ip: false,
            connection_type: ConnectionType::TCP,
            raknet: RakNetNetworkParameters::default(),
            tcp: TcpNetworkParameters::default(),
            connection_timeout_millis: WORKER_DEFAULTS_CONNECTION_TIMEOUT_MILLIS as u64,
            default_command_timeout_millis: WORKER_DEFAULTS_DEFAULT_COMMAND_TIMEOUT_MILLIS,
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_NetworkParameters {
        Worker_NetworkParameters {
            use_external_ip: self.use_external_ip as u8,
            connection_type: self.connection_type.to_u8(),
            raknet: self.raknet.to_worker_sdk(),
            tcp: self.tcp.to_worker_sdk(),
            connection_timeout_millis: self.connection_timeout_millis,
            default_command_timeout_millis: self.default_command_timeout_millis,
        }
    }
}

pub struct RakNetNetworkParameters {
    pub heartbeat_timeout_millis: u32,
}

impl RakNetNetworkParameters {
    pub fn default() -> Self {
        RakNetNetworkParameters {
            heartbeat_timeout_millis: WORKER_DEFAULTS_RAKNET_HEARTBEAT_TIMEOUT_MILLIS,
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_RakNetNetworkParameters {
        Worker_RakNetNetworkParameters {
            heartbeat_timeout_millis: self.heartbeat_timeout_millis,
        }
    }
}

pub struct TcpNetworkParameters {
    pub multiplex_level: u8,
    pub send_buffer_size: u32,
    pub receive_buffer_size: u32,
    pub no_delay: bool,
}

impl TcpNetworkParameters {
    pub fn default() -> Self {
        TcpNetworkParameters {
            multiplex_level: WORKER_DEFAULTS_TCP_MULTIPLEX_LEVEL as u8,
            send_buffer_size: WORKER_DEFAULTS_TCP_SEND_BUFFER_SIZE,
            receive_buffer_size: WORKER_DEFAULTS_TCP_RECEIVE_BUFFER_SIZE,
            no_delay: WORKER_DEFAULTS_TCP_NO_DELAY != 0,
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_TcpNetworkParameters {
        Worker_TcpNetworkParameters {
            multiplex_level: self.multiplex_level,
            send_buffer_size: self.send_buffer_size,
            receive_buffer_size: self.receive_buffer_size,
            no_delay: self.no_delay as u8,
        }
    }
}

pub struct ProtocolLoggingParameters {
    pub log_prefix: String,
    pub max_log_files: u32,
    pub max_log_file_size_bytes: u32,
}

impl ProtocolLoggingParameters {
    pub fn default() -> Self {
        ProtocolLoggingParameters {
            log_prefix: "".to_owned(),
            max_log_files: WORKER_DEFAULTS_MAX_LOG_FILES,
            max_log_file_size_bytes: WORKER_DEFAULTS_MAX_LOG_FILE_SIZE_BYTES,
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_ProtocolLoggingParameters {
        let log_prefix_cstr =
            CString::new(self.log_prefix.clone()).expect("Received 0 byte in supplied log prefix.");

        Worker_ProtocolLoggingParameters {
            log_prefix: log_prefix_cstr.as_ptr(),
            max_log_files: self.max_log_files,
            max_log_file_size_bytes: self.max_log_file_size_bytes,
        }
    }
}

pub struct ThreadAffinityParameters {
    pub receive_threads_affinity_mask: u64,
    pub send_threads_affinity_mask: u64,
    pub temporary_threads_affinity_mask: u64,
}

impl ThreadAffinityParameters {
    pub fn default() -> Self {
        ThreadAffinityParameters {
            receive_threads_affinity_mask: 0,
            send_threads_affinity_mask: 0,
            temporary_threads_affinity_mask: 0,
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_ThreadAffinityParameters {
        Worker_ThreadAffinityParameters {
            receive_threads_affinity_mask: self.receive_threads_affinity_mask,
            send_threads_affinity_mask: self.send_threads_affinity_mask,
            temporary_threads_affinity_mask: self.temporary_threads_affinity_mask,
        }
    }
}

pub struct CommandParameters {
    allow_short_circuit: bool,
}

impl CommandParameters {
    const SHORT_CIRCUIT: CommandParameters = CommandParameters {
        allow_short_circuit: true,
    };

    const DEFAULT: CommandParameters = CommandParameters {
        allow_short_circuit: false,
    };

    pub(crate) fn to_worker_sdk(&self) -> Worker_CommandParameters {
        Worker_CommandParameters {
            allow_short_circuit: self.allow_short_circuit as u8,
        }
    }
}

pub struct SnapshotParameters {}

impl SnapshotParameters {
    pub(crate) fn to_worker_sdk(&self) -> Worker_SnapshotParameters {
        Worker_SnapshotParameters {
            component_vtable_count: 0,
            component_vtables: ::std::ptr::null(),
            default_component_vtable: &vtable::PASSTHROUGH_VTABLE,
        }
    }
}
