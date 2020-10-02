use crate::utils::cstr_to_string;
use bitflags::bitflags;
use spatialos_sdk_sys::worker::*;
use std::ffi::{CStr, CString};

pub(crate) type ReleaseCallbackHandle = Box<dyn FnOnce()>;

bitflags! {
    pub struct LogCategory: u32 {
        const Receive         = 0x01;
        const Send            = 0x02;
        const NetworkStatus   = 0x04;
        const NetworkTraffic  = 0x08;
        const Login           = 0x10;
        const Api             = 0x20;
        const Parameters      = 0x40;
        const All             = 0x7f;
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum LogLevel {
    Debug = Worker_LogLevel_WORKER_LOG_LEVEL_DEBUG as u8,
    Info = Worker_LogLevel_WORKER_LOG_LEVEL_INFO as u8,
    Warn = Worker_LogLevel_WORKER_LOG_LEVEL_WARN as u8,
    Error = Worker_LogLevel_WORKER_LOG_LEVEL_ERROR as u8,
    Fatal = Worker_LogLevel_WORKER_LOG_LEVEL_FATAL as u8,
}

impl From<u8> for LogLevel {
    fn from(log_level: u8) -> Self {
        match log_level {
            1 => LogLevel::Debug,
            2 => LogLevel::Info,
            3 => LogLevel::Warn,
            4 => LogLevel::Error,
            5 => LogLevel::Fatal,
            _ => {
                eprintln!("Unknown log level: {}, returning Error.", log_level);
                LogLevel::Error
            }
        }
    }
}

pub type LogFilter = fn(categories: LogCategory, level: LogLevel) -> bool;

#[derive(Debug)]
pub enum LogFilterParameters {
    Simple(LogCategory, LogLevel),
    Callback(LogFilter),
}

impl LogFilterParameters {
    pub(crate) fn to_worker_sdk(
        &self,
    ) -> (Worker_LogFilterParameters, Option<ReleaseCallbackHandle>) {
        match self {
            LogFilterParameters::Simple(category, level) => (
                Worker_LogFilterParameters {
                    categories: category.bits,
                    level: *level as u8,
                    ..Default::default()
                },
                None,
            ),
            LogFilterParameters::Callback(filter) => {
                let callback_ptr = Box::into_raw(Box::new(*filter)) as *mut ::std::os::raw::c_void;

                (
                    Worker_LogFilterParameters {
                        callback: Some(worker_log_filter_callback),
                        user_data: callback_ptr,
                        ..Default::default()
                    },
                    Some(Box::new(move || unsafe {
                        Box::from_raw(callback_ptr as *mut LogFilter);
                    })),
                )
            }
        }
    }
}

impl Default for LogFilterParameters {
    fn default() -> Self {
        LogFilterParameters::Simple(LogCategory::All, LogLevel::Error)
    }
}

#[no_mangle]
unsafe extern "C" fn worker_log_filter_callback(
    user_data: *mut ::std::os::raw::c_void,
    categories: u32,
    level: Worker_LogLevel,
) -> u8 {
    // We ensure that user_data is non_null only if there is a valid callback. So we cannot
    // enter this callback without having valid user_data.
    let callback = &mut *(user_data as *mut LogFilter);
    callback(
        LogCategory::from_bits_unchecked(categories),
        ::std::mem::transmute(level as u8),
    ) as u8
}

pub struct LogData {
    pub timestamp: String,
    pub categories: LogCategory,
    pub log_level: LogLevel,
    pub content: String,
}

#[derive(Debug)]
pub struct RotatingLogFileParameters {
    pub log_prefix: CString,
    pub max_log_files: u32,
    pub max_log_file_size_bytes: u32,
}

impl RotatingLogFileParameters {
    /// Sets the prefix string to be used for log file names.
    ///
    /// # Panics
    ///
    /// This will panic if `prefix` contains a 0 byte. This is a requirement imposed
    /// by the underlying SpatialOS API.
    pub fn set_prefix<T: AsRef<str>>(&mut self, prefix: T) {
        self.log_prefix = CString::new(prefix.as_ref()).expect("`prefix` contained a null byte.");
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

    /// Converts the rotating log parameters into the equivalent C API type.
    ///
    /// # Safety
    ///
    /// The returned `Worker_RotatingLogFileParameters` borrows data owned by `self`,
    /// and therefore must not outlive `self`.
    pub(crate) fn to_worker_sdk(&self) -> Worker_RotatingLogFileParameters {
        Worker_RotatingLogFileParameters {
            log_prefix: self.log_prefix.as_ptr(),
            max_log_files: self.max_log_files,
            max_log_file_size_bytes: self.max_log_file_size_bytes,
        }
    }
}

impl Default for RotatingLogFileParameters {
    fn default() -> Self {
        RotatingLogFileParameters {
            log_prefix: CStr::from_bytes_with_nul(&WORKER_DEFAULTS_LOG_PREFIX[..])
                .unwrap()
                .to_owned(),
            max_log_files: 0,
            max_log_file_size_bytes: 0,
        }
    }
}

pub type LogCallback = fn(message: LogData);

#[no_mangle]
unsafe extern "C" fn worker_log_callback(
    user_data: *mut ::std::os::raw::c_void,
    message: *const Worker_LogData,
) {
    let callback: &mut LogCallback = &mut *(user_data as *mut LogCallback);
    callback(LogData {
        timestamp: cstr_to_string((*message).timestamp),
        categories: LogCategory::from_bits_unchecked((*message).categories),
        log_level: ::std::mem::transmute((*message).log_level),
        content: cstr_to_string((*message).content),
    });
}

#[derive(Debug)]
pub enum LogsinkType {
    RotatingFile(RotatingLogFileParameters),
    Callback(LogCallback),
    Stdout,
    StdoutAnsi,
    Stderr,
    StderrAnsi,
}

impl LogsinkType {
    pub(crate) fn worker_sdk_type(&self) -> i32 {
        match self {
            LogsinkType::RotatingFile(_) => Worker_LogsinkType_WORKER_LOGSINK_TYPE_ROTATING_FILE,
            LogsinkType::Callback(_) => Worker_LogsinkType_WORKER_LOGSINK_TYPE_CALLBACK,
            LogsinkType::Stdout => Worker_LogsinkType_WORKER_LOGSINK_TYPE_STDOUT,
            LogsinkType::StdoutAnsi => Worker_LogsinkType_WORKER_LOGSINK_TYPE_STDOUT_ANSI,
            LogsinkType::Stderr => Worker_LogsinkType_WORKER_LOGSINK_TYPE_STDERR,
            LogsinkType::StderrAnsi => Worker_LogsinkType_WORKER_LOGSINK_TYPE_STDERR_ANSI,
        }
    }
}

#[derive(Debug)]
pub struct LogsinkParameters {
    pub filter_parameters: LogFilterParameters,
    pub logsink_type: LogsinkType,
}

impl LogsinkParameters {
    /// Converts the rotating log parameters into the equivalent C API type.
    ///
    /// # Safety
    ///
    /// The returned `Worker_LogsinkParameters` borrows data owned by `self`,
    /// and therefore must not outlive `self`.
    pub fn to_worker_sdk(&self) -> (Worker_LogsinkParameters, Vec<ReleaseCallbackHandle>) {
        let rotating_logfile_parameters = match self.logsink_type {
            LogsinkType::RotatingFile(ref params) => params.to_worker_sdk(),
            _ => Worker_RotatingLogFileParameters {
                log_prefix: WORKER_DEFAULTS_LOG_PREFIX.as_ptr() as *const i8,
                max_log_files: 0,
                max_log_file_size_bytes: 0,
            },
        };

        let (log_callback_parameters, release_log_callback_handle) = match self.logsink_type {
            LogsinkType::Callback(callback) => {
                let callback_ptr = Box::into_raw(Box::new(callback)) as *mut ::std::os::raw::c_void;

                (
                    Worker_LogCallbackParameters {
                        log_callback: Some(worker_log_callback),
                        user_data: callback_ptr,
                    },
                    Some(Box::new(move || unsafe {
                        Box::from_raw(callback_ptr as *mut LogCallback);
                    }) as ReleaseCallbackHandle),
                )
            }
            _ => Default::default(),
        };

        let (filter_parameters, release_filter_callback_handle) =
            self.filter_parameters.to_worker_sdk();

        let release_handle_funcs = release_log_callback_handle
            .into_iter()
            .chain(release_filter_callback_handle.into_iter())
            .collect();

        (
            Worker_LogsinkParameters {
                logsink_type: self.logsink_type.worker_sdk_type() as u8,
                filter_parameters,
                rotating_logfile_parameters,
                log_callback_parameters,
            },
            release_handle_funcs,
        )
    }
}

impl Default for LogsinkParameters {
    fn default() -> Self {
        LogsinkParameters {
            logsink_type: LogsinkType::Stdout,
            filter_parameters: LogFilterParameters::default(),
        }
    }
}
