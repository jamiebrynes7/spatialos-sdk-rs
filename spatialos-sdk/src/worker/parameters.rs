use crate::worker::logging::{LogsinkParameters, ReleaseCallbackHandle};
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
    pub logsinks: Vec<LogsinkParameters>,
    pub enable_logging_at_startup: bool,
    pub enable_dynamic_components: bool,
    pub thread_affinity: ThreadAffinityParameters,
}

impl ConnectionParameters {
    pub fn new<T: AsRef<str>>(worker_type: T) -> Self {
        let mut params = ConnectionParameters::default();
        params.worker_type =
            CString::new(worker_type.as_ref()).expect("`worker_type` contains a null byte");
        params
    }

    pub fn with_logsink(mut self, logsink_params: LogsinkParameters) -> Self {
        self.logsinks.push(logsink_params);
        self
    }

    pub fn using_kcp(self) -> Self {
        self.using_kcp_with_params(ModularKcpNetworkParameters::default())
    }

    pub fn using_kcp_with_params(mut self, params: ModularKcpNetworkParameters) -> Self {
        self.network.protocol = ProtocolType::Kcp(params);
        self
    }

    pub fn using_tcp(self) -> Self {
        self.using_tcp_with_params(ModularTcpNetworkParameters::default())
    }

    pub fn using_tcp_with_params(mut self, params: ModularTcpNetworkParameters) -> Self {
        self.network.protocol = ProtocolType::Tcp(params);
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

    pub(crate) fn flatten(
        &self,
    ) -> (
        IntermediateConnectionParameters<'_>,
        Vec<ReleaseCallbackHandle>,
    ) {
        let protocol = match &self.network.protocol {
            ProtocolType::Kcp(params) => IntermediateProtocolType::Kcp(params.to_worker_sdk()),
            ProtocolType::Tcp(params) => IntermediateProtocolType::Tcp(params.to_worker_sdk()),
        };

        let (logsinks, release_callbacks): (Vec<_>, Vec<_>) = self
            .logsinks
            .iter()
            .map(LogsinkParameters::to_worker_sdk)
            .unzip();

        (
            IntermediateConnectionParameters {
                params: self,
                protocol,
                logsinks,
            },
            release_callbacks.into_iter().flatten().collect(),
        )
    }
}

impl Default for ConnectionParameters {
    fn default() -> Self {
        ConnectionParameters {
            worker_type: CString::new("").unwrap(),
            network: NetworkParameters::default(),
            send_queue_capacity: WORKER_DEFAULTS_SEND_QUEUE_CAPACITY,
            receive_queue_capacity: WORKER_DEFAULTS_RECEIVE_QUEUE_CAPACITY,
            log_message_queue_capacity: WORKER_DEFAULTS_LOG_MESSAGE_QUEUE_CAPACITY,
            built_in_metrics_report_period_millis:
                WORKER_DEFAULTS_BUILT_IN_METRICS_REPORT_PERIOD_MILLIS,
            logsinks: vec![],
            enable_logging_at_startup: false,
            enable_dynamic_components: WORKER_DEFAULTS_ENABLE_DYNAMIC_COMPONENTS != 0,
            thread_affinity: ThreadAffinityParameters::default(),
        }
    }
}

pub enum ProtocolType {
    Kcp(ModularKcpNetworkParameters),
    Tcp(ModularTcpNetworkParameters),
}

pub struct NetworkParameters {
    pub use_external_ip: bool,
    pub protocol: ProtocolType,
    pub connection_timeout_millis: u64,
    pub default_command_timeout_millis: u32,
}

impl NetworkParameters {
    pub fn default() -> Self {
        NetworkParameters {
            use_external_ip: false,
            protocol: ProtocolType::Kcp(ModularKcpNetworkParameters::default()),
            connection_timeout_millis: u64::from(WORKER_DEFAULTS_CONNECTION_TIMEOUT_MILLIS),
            default_command_timeout_millis: WORKER_DEFAULTS_DEFAULT_COMMAND_TIMEOUT_MILLIS,
        }
    }
}

pub struct ModularKcpNetworkParameters {
    pub security_type: SecurityType,
    pub multiplex_level: u8,
    pub downstream_kcp: KcpTransportParameters,
    pub upstream_kcp: KcpTransportParameters,
    pub downstream_erasure_codec: Option<ErasureCodecParameters>,
    pub upstream_erasure_codec: Option<ErasureCodecParameters>,
    pub downstream_heartbeat: Option<HeartbeatParameters>,
    pub upstream_heartbeat: Option<HeartbeatParameters>,
    pub downstream_compression: Option<CompressionParameters>,
    pub upstream_compression: Option<CompressionParameters>,
    pub flow_control: Option<FlowControlParameters>,
}

impl ModularKcpNetworkParameters {
    pub fn default() -> Self {
        ModularKcpNetworkParameters {
            security_type: SecurityType::Insecure,
            multiplex_level: WORKER_DEFAULTS_KCP_MULTIPLEX_LEVEL as u8,
            downstream_kcp: KcpTransportParameters::default(),
            upstream_kcp: KcpTransportParameters::default(),
            downstream_erasure_codec: None,
            upstream_erasure_codec: None,
            downstream_heartbeat: None,
            upstream_heartbeat: None,
            downstream_compression: None,
            upstream_compression: None,
            flow_control: Some(FlowControlParameters::default()),
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_ModularKcpNetworkParameters {
        Worker_ModularKcpNetworkParameters {
            security_type: self.security_type as u8,
            multiplex_level: self.multiplex_level,
            downstream_kcp: self.downstream_kcp.to_worker_sdk(),
            upstream_kcp: self.upstream_kcp.to_worker_sdk(),
            downstream_erasure_codec: match &self.downstream_erasure_codec {
                Some(x) => &x.to_worker_sdk(),
                None => ::std::ptr::null(),
            },
            upstream_erasure_codec: match &self.upstream_erasure_codec {
                Some(x) => &x.to_worker_sdk(),
                None => ::std::ptr::null(),
            },
            downstream_heartbeat: match &self.downstream_heartbeat {
                Some(x) => &x.to_worker_sdk(),
                None => ::std::ptr::null(),
            },
            upstream_heartbeat: match &self.upstream_heartbeat {
                Some(x) => &x.to_worker_sdk(),
                None => ::std::ptr::null(),
            },
            downstream_compression: match &self.downstream_compression {
                Some(x) => &x.to_worker_sdk(),
                None => ::std::ptr::null(),
            },
            upstream_compression: match &self.upstream_compression {
                Some(x) => &x.to_worker_sdk(),
                None => ::std::ptr::null(),
            },
            flow_control: match &self.flow_control {
                Some(x) => &x.to_worker_sdk(),
                None => ::std::ptr::null(),
            },
        }
    }
}

pub struct KcpTransportParameters {
    pub flush_interval_millis: u32,
    pub fast_retransmission: bool,
    pub early_retransmission: bool,
    pub disable_congestion_control: bool,
    pub min_rto_millis: u32,
}

impl KcpTransportParameters {
    pub fn default() -> Self {
        KcpTransportParameters {
            flush_interval_millis: WORKER_DEFAULTS_KCP_FLUSH_INTERVAL_MILLIS,
            fast_retransmission: WORKER_DEFAULTS_KCP_FAST_RETRANSMISSION != 0,
            early_retransmission: WORKER_DEFAULTS_KCP_EARLY_RETRANSMISSION != 0,
            disable_congestion_control: WORKER_DEFAULTS_KCP_DISABLE_CONGESTION_CONTROL != 0,
            min_rto_millis: WORKER_DEFAULTS_KCP_MIN_RTO_MILLIS,
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_KcpTransportParameters {
        Worker_KcpTransportParameters {
            flush_interval_millis: self.flush_interval_millis,
            fast_retransmission: self.fast_retransmission as u8,
            early_retransmission: self.early_retransmission as u8,
            disable_congestion_control: self.disable_congestion_control as u8,
            min_rto_millis: self.min_rto_millis,
        }
    }
}

// Tcp

pub struct ModularTcpNetworkParameters {
    pub security_type: SecurityType,
    pub multiplex_level: u8,
    pub downstream_tcp: TcpTransportParameters,
    pub upstream_tcp: TcpTransportParameters,
    pub downstream_heartbeat: Option<HeartbeatParameters>,
    pub upstream_heartbeat: Option<HeartbeatParameters>,
    pub downstream_compression: Option<CompressionParameters>,
    pub upstream_compression: Option<CompressionParameters>,
    pub flow_control: Option<FlowControlParameters>,
}

impl ModularTcpNetworkParameters {
    pub fn default() -> Self {
        ModularTcpNetworkParameters {
            security_type: SecurityType::Insecure,
            multiplex_level: WORKER_DEFAULTS_MODULAR_TCP_MULTIPLEX_LEVEL as u8,
            downstream_tcp: TcpTransportParameters::default(),
            upstream_tcp: TcpTransportParameters::default(),
            downstream_heartbeat: None,
            upstream_heartbeat: None,
            downstream_compression: None,
            upstream_compression: None,
            flow_control: Some(FlowControlParameters::default()),
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_ModularTcpNetworkParameters {
        Worker_ModularTcpNetworkParameters {
            security_type: self.security_type as u8,
            multiplex_level: self.multiplex_level,
            downstream_tcp: self.downstream_tcp.to_worker_sdk(),
            upstream_tcp: self.upstream_tcp.to_worker_sdk(),
            downstream_heartbeat: match &self.downstream_heartbeat {
                Some(x) => &x.to_worker_sdk(),
                None => ::std::ptr::null(),
            },
            upstream_heartbeat: match &self.upstream_heartbeat {
                Some(x) => &x.to_worker_sdk(),
                None => ::std::ptr::null(),
            },
            downstream_compression: match &self.downstream_compression {
                Some(x) => &x.to_worker_sdk(),
                None => ::std::ptr::null(),
            },
            upstream_compression: match &self.upstream_compression {
                Some(x) => &x.to_worker_sdk(),
                None => ::std::ptr::null(),
            },
            flow_control: match &self.flow_control {
                Some(x) => &x.to_worker_sdk(),
                None => ::std::ptr::null(),
            },
        }
    }
}

pub struct TcpTransportParameters {
    pub flush_delay_millis: u32,
}

impl TcpTransportParameters {
    pub fn default() -> Self {
        TcpTransportParameters {
            flush_delay_millis: WORKER_DEFAULTS_TCP_FLUSH_DELAY_MILLIS,
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_TcpTransportParameters {
        Worker_TcpTransportParameters {
            flush_delay_millis: self.flush_delay_millis,
        }
    }
}

// Network parameters

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum SecurityType {
    Insecure = Worker_NetworkSecurityType_WORKER_NETWORK_SECURITY_TYPE_INSECURE as u8,
    DTLS = Worker_NetworkSecurityType_WORKER_NETWORK_SECURITY_TYPE_DTLS as u8,
}

impl Default for SecurityType {
    fn default() -> Self {
        SecurityType::Insecure
    }
}

pub struct CompressionParameters {}

impl CompressionParameters {
    pub(crate) fn to_worker_sdk(&self) -> Worker_CompressionParameters {
        Worker_CompressionParameters { place_holder: 0 }
    }
}

pub struct FlowControlParameters {
    pub downstream_window_size_bytes: u32,
    pub upstream_window_size_bytes: u32,
}

impl FlowControlParameters {
    pub fn default() -> Self {
        FlowControlParameters {
            downstream_window_size_bytes: WORKER_DEFAULTS_FLOW_CONTROL_DOWNSTREAM_WINDOW_SIZE_BYTES,
            upstream_window_size_bytes: WORKER_DEFAULTS_FLOW_CONTROL_UPSTREAM_WINDOW_SIZE_BYTES,
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_FlowControlParameters {
        Worker_FlowControlParameters {
            downstream_window_size_bytes: self.downstream_window_size_bytes,
            upstream_window_size_bytes: self.upstream_window_size_bytes,
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

    pub(crate) fn to_worker_sdk(&self) -> Worker_ErasureCodecParameters {
        Worker_ErasureCodecParameters {
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

    pub(crate) fn to_worker_sdk(&self) -> Worker_HeartbeatParameters {
        Worker_HeartbeatParameters {
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

/// Helper struct for converting `ConnectionParameters` into `Worker_ConnectionParameters`.
pub(crate) struct IntermediateConnectionParameters<'a> {
    params: &'a ConnectionParameters,
    protocol: IntermediateProtocolType,
    logsinks: Vec<Worker_LogsinkParameters>,
}

impl<'a> IntermediateConnectionParameters<'a> {
    pub(crate) fn as_raw(
        &self,
        default_vtable: &Worker_ComponentVtable,
    ) -> Worker_ConnectionParameters {
        let partial_network_params = Worker_NetworkParameters {
            use_external_ip: self.params.network.use_external_ip as u8,
            connection_timeout_millis: self.params.network.connection_timeout_millis,
            default_command_timeout_millis: self.params.network.default_command_timeout_millis,
            ..Worker_NetworkParameters::default()
        };

        let network = match &self.protocol {
            IntermediateProtocolType::Kcp(kcp) => Worker_NetworkParameters {
                connection_type:
                    Worker_NetworkConnectionType_WORKER_NETWORK_CONNECTION_TYPE_MODULAR_KCP as u8,
                modular_kcp: *kcp,
                ..partial_network_params
            },
            IntermediateProtocolType::Tcp(tcp) => Worker_NetworkParameters {
                connection_type:
                    Worker_NetworkConnectionType_WORKER_NETWORK_CONNECTION_TYPE_MODULAR_TCP as u8,
                modular_tcp: *tcp,
                ..partial_network_params
            },
        };

        Worker_ConnectionParameters {
            worker_type: self.params.worker_type.as_ptr(),
            network,
            send_queue_capacity: self.params.send_queue_capacity,
            receive_queue_capacity: self.params.receive_queue_capacity,
            log_message_queue_capacity: self.params.log_message_queue_capacity,
            built_in_metrics_report_period_millis: self
                .params
                .built_in_metrics_report_period_millis,
            protocol_logging: Worker_ProtocolLoggingParameters::default(),
            enable_protocol_logging_at_startup: 0,
            logsink_count: self.logsinks.len() as u32,
            logsinks: self.logsinks.as_ptr(),
            enable_logging_at_startup: self.params.enable_logging_at_startup as u8,
            enable_dynamic_components: self.params.enable_dynamic_components as u8,
            thread_affinity: self.params.thread_affinity.to_worker_sdk(),
            component_vtable_count: 0,
            component_vtables: ptr::null(),
            default_component_vtable: default_vtable,
        }
    }
}

enum IntermediateProtocolType {
    Kcp(Worker_ModularKcpNetworkParameters),
    Tcp(Worker_ModularTcpNetworkParameters),
}
