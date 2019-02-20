use crate::worker::{component::ComponentDatabase, vtable};
use spatialos_sdk_sys::worker::*;
use std::{
    ffi::{CStr, CString},
    ptr,
};

pub struct ConnectionParameters {
    pub worker_type: CString,
    pub network: NetworkParameters,
    pub send_queue_capacity: u32,
    pub receive_queue_capacity: u32,
    pub log_message_queue_capacity: u32,
    pub built_in_metrics_report_period_millis: u32,
    pub protocol_logging: ProtocolLoggingParameters,
    pub enable_protocol_logging_at_startup: bool,
    pub thread_affinity: ThreadAffinityParameters,
    pub components: ComponentDatabase,
}

impl ConnectionParameters {
    pub fn new<T: AsRef<str>>(worker_type: T, components: ComponentDatabase) -> Self {
        let mut params = ConnectionParameters::default(components);
        params.worker_type =
            CString::new(worker_type.as_ref()).expect("`worker_type` contains a null byte");
        params
    }

    pub fn with_protocol_logging<T: AsRef<str>>(mut self, log_prefix: T) -> Self {
        self.enable_protocol_logging_at_startup = true;
        self.protocol_logging.log_prefix =
            CString::new(log_prefix.as_ref()).expect("`log_prefix` contained a null byte");
        self
    }

    pub fn using_tcp(self) -> Self {
        self.using_tcp_with_params(TcpNetworkParameters::default())
    }

    pub fn using_tcp_with_params(mut self, params: TcpNetworkParameters) -> Self {
        self.network.protocol_params = ProtocolType::Tcp(params);
        self
    }

    pub fn using_raknet(self) -> Self {
        self.using_raknet_with_params(RakNetNetworkParameters::default())
    }

    pub fn using_raknet_with_params(mut self, params: RakNetNetworkParameters) -> Self {
        self.network.protocol_params = ProtocolType::RakNet(params);
        self
    }

    pub fn using_kcp(self) -> Self {
        self.using_kcp_with_params(KcpNetworkParameters::default())
    }

    pub fn using_kcp_with_params(mut self, params: KcpNetworkParameters) -> Self {
        self.network.protocol_params = ProtocolType::Kcp(params);
        self
    }

    pub fn using_external_ip(mut self, use_external_ip: bool) -> Self {
        self.network.use_external_ip = use_external_ip;
        self
    }

    pub fn with_connection_timeout(mut self, timeout_millis: u64) -> Self {
        self.network.connection_timeout_millis = timeout_millis;
        self
    }

    pub fn default(components: ComponentDatabase) -> Self {
        ConnectionParameters {
            worker_type: CString::new("").unwrap(),
            network: NetworkParameters::default(),
            send_queue_capacity: WORKER_DEFAULTS_SEND_QUEUE_CAPACITY,
            receive_queue_capacity: WORKER_DEFAULTS_RECEIVE_QUEUE_CAPACITY,
            log_message_queue_capacity: WORKER_DEFAULTS_LOG_MESSAGE_QUEUE_CAPACITY,
            built_in_metrics_report_period_millis:
                WORKER_DEFAULTS_BUILT_IN_METRICS_REPORT_PERIOD_MILLIS,
            protocol_logging: ProtocolLoggingParameters::default(),
            enable_protocol_logging_at_startup: false,
            thread_affinity: ThreadAffinityParameters::default(),
            components,
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_ConnectionParameters {
        Worker_ConnectionParameters {
            worker_type: self.worker_type.as_ptr(),
            network: self.network.to_worker_sdk(),
            send_queue_capacity: self.send_queue_capacity,
            receive_queue_capacity: self.receive_queue_capacity,
            log_message_queue_capacity: self.log_message_queue_capacity,
            built_in_metrics_report_period_millis: self.built_in_metrics_report_period_millis,
            protocol_logging: self.protocol_logging.to_worker_sdk(),
            enable_protocol_logging_at_startup: self.enable_protocol_logging_at_startup as u8,
            thread_affinity: self.thread_affinity.to_worker_sdk(),
            component_vtable_count: self.components.len() as u32,
            component_vtables: self.components.to_worker_sdk(),
            default_component_vtable: ptr::null(),
        }
    }
}

pub enum ProtocolType {
    Tcp(TcpNetworkParameters),
    RakNet(RakNetNetworkParameters),
    Kcp(KcpNetworkParameters),
}

impl ProtocolType {
    fn to_worker_sdk(
        &self,
    ) -> (
        u8,
        Worker_RakNetNetworkParameters,
        Worker_TcpNetworkParameters,
        Worker_Alpha_KcpNetworkParameters,
    ) {
        match self {
            ProtocolType::Tcp(params) => {
                let tcp_params = params.to_worker_sdk();
                let raknet_params = RakNetNetworkParameters::default().to_worker_sdk();
                let kcp_params = KcpNetworkParameters::default().to_worker_sdk();
                (
                    Worker_NetworkConnectionType_WORKER_NETWORK_CONNECTION_TYPE_TCP as u8,
                    raknet_params,
                    tcp_params,
                    kcp_params,
                )
            }
            ProtocolType::RakNet(params) => {
                let tcp_params = TcpNetworkParameters::default().to_worker_sdk();
                let raknet_params = params.to_worker_sdk();
                let kcp_params = KcpNetworkParameters::default().to_worker_sdk();
                (
                    Worker_NetworkConnectionType_WORKER_NETWORK_CONNECTION_TYPE_RAKNET as u8,
                    raknet_params,
                    tcp_params,
                    kcp_params,
                )
            }
            ProtocolType::Kcp(params) => {
                let tcp_params = TcpNetworkParameters::default().to_worker_sdk();
                let raknet_params = RakNetNetworkParameters::default().to_worker_sdk();
                let kcp_params = params.to_worker_sdk();
                (
                    Worker_NetworkConnectionType_WORKER_NETWORK_CONNECTION_TYPE_RAKNET as u8,
                    raknet_params,
                    tcp_params,
                    kcp_params,
                )
            }
        }
    }
}

pub struct NetworkParameters {
    pub use_external_ip: bool,
    pub protocol_params: ProtocolType,
    pub connection_timeout_millis: u64,
    pub default_command_timeout_millis: u32,
}

impl NetworkParameters {
    pub fn default() -> Self {
        NetworkParameters {
            use_external_ip: false,
            protocol_params: ProtocolType::Tcp(TcpNetworkParameters::default()),
            connection_timeout_millis: u64::from(WORKER_DEFAULTS_CONNECTION_TIMEOUT_MILLIS),
            default_command_timeout_millis: WORKER_DEFAULTS_DEFAULT_COMMAND_TIMEOUT_MILLIS,
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_NetworkParameters {
        let (protocol_type, raknet_params, tcp_params, kcp_params) =
            self.protocol_params.to_worker_sdk();
        Worker_NetworkParameters {
            use_external_ip: self.use_external_ip as u8,
            connection_type: protocol_type,
            raknet: raknet_params,
            tcp: tcp_params,
            kcp: kcp_params,
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

pub struct KcpNetworkParameters {
    pub fast_transmission: bool,
    pub early_retransmission: bool,
    pub non_concessional_flow_control: bool,
    pub multiplex_level: u32,
    pub update_interval_millis: u32,
    pub min_rto_millis: u32,
    pub window_size: u32,
    pub erasure_codec: Option<ErasureCodecParameters>,
    pub heartbeat_params: HeartbeatParameters,
}

impl KcpNetworkParameters {
    pub fn default() -> Self {
        KcpNetworkParameters {
            fast_transmission: WORKER_DEFAULTS_KCP_FAST_RETRANSMISSION != 0,
            early_retransmission: WORKER_DEFAULTS_KCP_EARLY_RETRANSMISSION != 0,
            non_concessional_flow_control: WORKER_DEFAULTS_KCP_NON_CONCESSIONAL_FLOW_CONTROL != 0,
            multiplex_level: WORKER_DEFAULTS_KCP_MULTIPLEX_LEVEL,
            update_interval_millis: WORKER_DEFAULTS_KCP_UPDATE_INTERVAL_MILLIS,
            min_rto_millis: WORKER_DEFAULTS_KCP_MIN_RTO_MILLIS,
            window_size: WORKER_DEFAULTS_KCP_WINDOW_SIZE,
            erasure_codec: if WORKER_DEFAULTS_KCP_ENABLE_ERASURE_CODEC != 0 {
                Some(ErasureCodecParameters::default())
            } else {
                None
            },
            heartbeat_params: HeartbeatParameters::default(),
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_Alpha_KcpNetworkParameters {
        Worker_Alpha_KcpNetworkParameters {
            fast_retransmission: self.fast_transmission as u8,
            early_retransmission: self.early_retransmission as u8,
            non_concessional_flow_control: self.non_concessional_flow_control as u8,
            multiplex_level: self.multiplex_level,
            update_interval_millis: self.update_interval_millis,
            min_rto_millis: self.min_rto_millis,
            window_size: self.window_size,
            enable_erasure_codec: self.erasure_codec.is_some() as u8,
            erasure_codec: self
                .erasure_codec
                .as_ref()
                .unwrap_or(&ErasureCodecParameters::default())
                .to_worker_sdk(),
            heartbeat: self.heartbeat_params.to_worker_sdk(),
        }
    }
}

pub struct ErasureCodecParameters {
    pub original_packet_count: u8,
    pub recovery_packet_count: u8,
    pub window_size: u8,
}

impl ErasureCodecParameters {
    pub fn default() -> Self {
        ErasureCodecParameters {
            original_packet_count: WORKER_DEFAULTS_ERASURE_CODEC_ORIGINAL_PACKET_COUNT as u8,
            recovery_packet_count: WORKER_DEFAULTS_ERASURE_CODEC_RECOVERY_PACKET_COUNT as u8,
            window_size: WORKER_DEFAULTS_ERASURE_CODEC_WINDOW_SIZE as u8,
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_Alpha_ErasureCodecParameters {
        Worker_Alpha_ErasureCodecParameters {
            original_packet_count: self.original_packet_count,
            recovery_packet_count: self.recovery_packet_count,
            window_size: self.window_size,
        }
    }
}

pub struct HeartbeatParameters {
    pub interval_millis: u64,
    pub timeout_millis: u64,
}

impl HeartbeatParameters {
    pub fn default() -> Self {
        HeartbeatParameters {
            interval_millis: u64::from(WORKER_DEFAULTS_HEARTBEAT_INTERVAL_MILLIS),
            timeout_millis: u64::from(WORKER_DEFAULTS_HEARTBEAT_TIMEOUT_MILLIS),
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_Alpha_HeartbeatParameters {
        Worker_Alpha_HeartbeatParameters {
            interval_millis: self.interval_millis,
            timeout_millis: self.timeout_millis,
        }
    }
}

/// Parameters for configuring protocol logging. If enabled, logs all protocol
/// messages sent and received.
///
/// Note that all parameters are kept private and the struct can only be initialized
/// with default values in order to make it possible to add new parameters without a
/// breaking change.
///
/// If you would like to use a method-chaining style when initializing the parameters,
/// the [tap] crate is recommended. The examples below demonstrate this.
///
/// # Parameters
///
/// * `log_prefx: WORKER_DEFAULTS_LOG_PREFIX` - Log file names are prefixed with
///   this prefix, are numbered, and have the extension `.log`.
/// * `max_log_files: WORKER_DEFAULTS_MAX_LOG_FILES` - Maximum number of log files
///   to keep. Note that logs from any previous protocol logging sessions will be
///   overwritten.
/// * `max_log_file_size: WORKER_DEFAULTS_MAX_LOG_FILE_SIZE_BYTES` - Once the size
///   of a log file reaches this size, a new log file is created.
///
/// # Examples
///
/// ```
/// use spatialos_sdk::worker::parameters::ProtocolLoggingParameters;
/// use tap::*;
///
/// let params = ProtocolLoggingParameters::new()
///     .tap(|params| params.set_prefix("log-prefix-"))
///     .tap(|params| params.set_max_log_files(10));
/// ```
#[derive(Debug, Clone)]
pub struct ProtocolLoggingParameters {
    log_prefix: CString,
    max_log_files: u32,
    max_log_file_size_bytes: u32,
}

impl ProtocolLoggingParameters {
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the prefix string to be used for log file names.
    ///
    /// # Panics
    ///
    /// This will panic if `prefix` contains a 0 byte. This is a requirement imposed
    /// by the underlying SpatialOS API.
    pub fn set_prefix<T: AsRef<str>>(&mut self, prefix: T) {
        self.log_prefix = CString::new(prefix.as_ref()).expect("`prefix` contained a null byte");
    }

    /// Sets the maximum number of log files to keep.
    pub fn set_max_log_files(&mut self, max_log_files: u32) {
        self.max_log_files = max_log_files;
    }

    /// Sets the maximum size in bytes that a single log file can be.
    ///
    /// Once an individual log file exceeds this size, a new file will be created.
    pub fn set_max_log_file_size(&mut self, max_file_size: u32) {
        self.max_log_file_size_bytes = max_file_size;
    }

    /// Converts the logging parameters into the equivalent C API type.
    ///
    /// # Safety
    ///
    /// The returned `Worker_ProtocolLoggingParameters` borrows data owned by `self`,
    /// and therefore must not outlive `self`.
    pub(crate) fn to_worker_sdk(&self) -> Worker_ProtocolLoggingParameters {
        Worker_ProtocolLoggingParameters {
            log_prefix: self.log_prefix.as_ptr(),
            max_log_files: self.max_log_files,
            max_log_file_size_bytes: self.max_log_file_size_bytes,
        }
    }
}

impl Default for ProtocolLoggingParameters {
    fn default() -> Self {
        ProtocolLoggingParameters {
            log_prefix: CStr::from_bytes_with_nul(&WORKER_DEFAULTS_LOG_PREFIX[..])
                .unwrap()
                .to_owned(),
            max_log_files: WORKER_DEFAULTS_MAX_LOG_FILES,
            max_log_file_size_bytes: WORKER_DEFAULTS_MAX_LOG_FILE_SIZE_BYTES,
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
