use crate::worker::{component::DATABASE, vtable};
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
    use_internal_serialization: bool,
}

impl ConnectionParameters {
    pub fn new<T: AsRef<str>>(worker_type: T) -> Self {
        let mut params = ConnectionParameters::default();
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
        self.network.protocol = ProtocolType::Tcp(params);
        self
    }

    pub fn using_udp(self) -> Self {
        self.using_udp_with_params(UdpNetworkParameters::default())
    }

    pub fn using_udp_with_params(mut self, params: UdpNetworkParameters) -> Self {
        self.network.protocol = ProtocolType::Udp(params);
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

    pub fn enable_internal_serialization(mut self) -> Self {
        self.use_internal_serialization = true;
        self
    }

    pub fn default() -> Self {
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
            use_internal_serialization: false,
        }
    }

    pub(crate) fn flatten(&self) -> IntermediateConnectionParameters<'_> {
        let protocol = match &self.network.protocol {
            ProtocolType::Tcp(params) => IntermediateProtocolType::Tcp(params.to_worker_sdk()),
            ProtocolType::Udp(params) => IntermediateProtocolType::Udp {
                security_type: params.security_type.to_worker_sdk(),
                kcp: params.kcp.as_ref().map(KcpParameters::to_worker_sdk),
                erasure_codec: params.erasure_codec.as_ref().map(ErasureCodecParameters::to_worker_sdk),
                heartbeat: params.heartbeat.as_ref().map(HeartbeatParameters::to_worker_sdk),
                flow_control: params.flow_control.as_ref().map(FlowControlParameters::to_worker_sdk),
            }
        };

        IntermediateConnectionParameters {
            params: self,
            protocol,
        }
    }
}

pub enum ProtocolType {
    Tcp(TcpNetworkParameters),
    Udp(UdpNetworkParameters),
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
            protocol: ProtocolType::Tcp(TcpNetworkParameters::default()),
            connection_timeout_millis: u64::from(WORKER_DEFAULTS_CONNECTION_TIMEOUT_MILLIS),
            default_command_timeout_millis: WORKER_DEFAULTS_DEFAULT_COMMAND_TIMEOUT_MILLIS,
        }
    }
}

// TCP

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

// UDP

pub enum SecurityType {
    Insecure,
    DTLS,
}

impl SecurityType {
    pub fn default() -> Self {
        SecurityType::Insecure
    }

    pub(crate) fn to_worker_sdk(&self) -> u8 {
        (match self {
            SecurityType::Insecure => {
                Worker_NetworkSecurityType_WORKER_NETWORK_SECURITY_TYPE_INSECURE
            }
            SecurityType::DTLS => Worker_NetworkSecurityType_WORKER_NETWORK_SECURITY_TYPE_DTLS,
        }) as u8
    }
}

pub struct KcpParameters {
    pub fast_retransmission: bool,
    pub early_retransmission: bool,
    pub non_concessional_flow_control: bool,
    pub multiplex_level: u32,
    pub update_interval_millis: u32,
    pub min_rto_millis: u32,
}

impl KcpParameters {
    pub fn default() -> Self {
        KcpParameters {
            fast_retransmission: WORKER_DEFAULTS_KCP_FAST_RETRANSMISSION != 0,
            early_retransmission: WORKER_DEFAULTS_KCP_EARLY_RETRANSMISSION != 0,
            non_concessional_flow_control: WORKER_DEFAULTS_KCP_NON_CONCESSIONAL_FLOW_CONTROL != 0,
            multiplex_level: WORKER_DEFAULTS_KCP_MULTIPLEX_LEVEL,
            update_interval_millis: WORKER_DEFAULTS_KCP_UPDATE_INTERVAL_MILLIS,
            min_rto_millis: WORKER_DEFAULTS_KCP_MIN_RTO_MILLIS,
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_Alpha_KcpParameters {
        Worker_Alpha_KcpParameters {
            fast_retransmission: self.fast_retransmission as u8,
            early_retransmission: self.early_retransmission as u8,
            non_concessional_flow_control: self.non_concessional_flow_control as u8,
            multiplex_level: self.multiplex_level,
            update_interval_millis: self.update_interval_millis,
            min_rto_millis: self.min_rto_millis,
        }
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

    pub(crate) fn to_worker_sdk(&self) -> Worker_Alpha_FlowControlParameters {
        Worker_Alpha_FlowControlParameters {
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

pub struct UdpNetworkParameters {
    pub security_type: SecurityType,
    pub kcp: Option<KcpParameters>,
    pub erasure_codec: Option<ErasureCodecParameters>,
    pub heartbeat: Option<HeartbeatParameters>,
    pub flow_control: Option<FlowControlParameters>,
}

impl UdpNetworkParameters {
    pub fn default() -> Self {
        UdpNetworkParameters {
            security_type: SecurityType::Insecure,
            kcp: Some(KcpParameters::default()),
            erasure_codec: None,
            heartbeat: None,
            flow_control: Some(FlowControlParameters::default()),
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
}

impl<'a> IntermediateConnectionParameters<'a> {
    pub(crate) fn as_raw(&self) -> Worker_ConnectionParameters {
        let partial_network_params = Worker_NetworkParameters {
            use_external_ip: self.params.network.use_external_ip as u8,
            connection_timeout_millis: self.params.network.connection_timeout_millis,
            default_command_timeout_millis: self.params.network.default_command_timeout_millis,

            ..Worker_NetworkParameters::default()
        };

        let network = match &self.protocol {
            &IntermediateProtocolType::Tcp(tcp) => {
                Worker_NetworkParameters {
                    connection_type:
                        Worker_NetworkConnectionType_WORKER_NETWORK_CONNECTION_TYPE_TCP as u8,
                    tcp,

                    ..partial_network_params
                }
            }

            IntermediateProtocolType::Udp {
                security_type,
                kcp,
                erasure_codec,
                heartbeat,
                flow_control,
            } => {
                let kcp = kcp.as_ref().map(|param| param as *const _).unwrap_or(ptr::null());
                let erasure_codec = erasure_codec.as_ref().map(|param| param as *const _).unwrap_or(ptr::null());
                let heartbeat = heartbeat.as_ref().map(|param| param as *const _).unwrap_or(ptr::null());
                let flow_control = flow_control.as_ref().map(|param| param as *const _).unwrap_or(ptr::null());

                Worker_NetworkParameters {

                    connection_type:
                        Worker_NetworkConnectionType_WORKER_NETWORK_CONNECTION_TYPE_MODULAR_UDP as u8,

                    modular_udp: Worker_Alpha_ModularUdpNetworkParameters {
                        security_type: *security_type,

                        downstream_kcp: kcp,
                        upstream_kcp: kcp,

                        downstream_erasure_codec: erasure_codec,
                        upstream_erasure_codec: erasure_codec,

                        downstream_heartbeat: heartbeat,
                        upstream_heartbeat: heartbeat,

                        flow_control,
                    },

                    ..partial_network_params
                }
            }
        };

        Worker_ConnectionParameters {
            worker_type: self.params.worker_type.as_ptr(),
            network,
            send_queue_capacity: self.params.send_queue_capacity,
            receive_queue_capacity: self.params.receive_queue_capacity,
            log_message_queue_capacity: self.params.log_message_queue_capacity,
            built_in_metrics_report_period_millis: self.params.built_in_metrics_report_period_millis,
            protocol_logging: self.params.protocol_logging.to_worker_sdk(),
            enable_protocol_logging_at_startup: self.params.enable_protocol_logging_at_startup as u8,
            enable_dynamic_components: 0,
            thread_affinity: self.params.thread_affinity.to_worker_sdk(),

            component_vtable_count: if self.params.use_internal_serialization {
                DATABASE.len() as u32
            } else {
                0
            },

            component_vtables: if self.params.use_internal_serialization {
                DATABASE.to_worker_sdk()
            } else {
                ptr::null()
            },

            default_component_vtable: if self.params.use_internal_serialization {
                ptr::null()
            } else {
                &vtable::PASSTHROUGH_VTABLE
            },
        }
    }
}

enum IntermediateProtocolType {
    Tcp(Worker_TcpNetworkParameters),

    Udp {
        security_type: u8,
        kcp: Option<Worker_Alpha_KcpParameters>,
        erasure_codec: Option<Worker_ErasureCodecParameters>,
        heartbeat: Option<Worker_HeartbeatParameters>,
        flow_control: Option<Worker_Alpha_FlowControlParameters>,
    }
}
