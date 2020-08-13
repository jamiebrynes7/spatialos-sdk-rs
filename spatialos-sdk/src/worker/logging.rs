use crate::worker::utils::cstr_to_string;
use spatialos_sdk_sys::worker::*;
use std::ffi::CString;
use std::ptr;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum LogsinkType {
    RotatingFile = Worker_LogsinkType_WORKER_LOGSINK_TYPE_ROTATING_FILE as u8,
    Callback = Worker_LogsinkType_WORKER_LOGSINK_TYPE_CALLBACK as u8,
    Stdout = Worker_LogsinkType_WORKER_LOGSINK_TYPE_STDOUT as u8,
    StdoutANSI = Worker_LogsinkType_WORKER_LOGSINK_TYPE_STDOUT_ANSI as u8,
    Stderr = Worker_LogsinkType_WORKER_LOGSINK_TYPE_STDERR as u8,
    StderrANSI = Worker_LogsinkType_WORKER_LOGSINK_TYPE_STDERR_ANSI as u8,
}

bitflags! {
    pub struct LogCategory: u32 {
        const Receive         = 0x01;
        const Send            = 0x02;
        const NetworkStatus   = 0x04;
        const NetworkTraffic  = 0x08;
        const Login           = 0x10;
        const Api             = 0x20;
        const All             = 0x3f;
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum LogLevel {
    Debug = Worker_LogLevel_WORKER_LOG_LEVEL_DEBUG as u8,
    Info = Worker_LogLevel_WORKER_LOG_LEVEL_INFO as u8,
    Warn = Worker_LogLevel_WORKER_LOG_LEVEL_WARN as u8,
    Error = Worker_LogLevel_WORKER_LOG_LEVEL_ERROR as u8,
    Fatal = Worker_LogLevel_WORKER_LOG_LEVEL_FATAL as u8,
}

pub type LogFilter = fn(categories: LogCategory, level: LogLevel) -> bool;

pub struct LogFilterParameters {
    pub categories: LogCategory,
    pub level: LogLevel,
    pub callback: Option<LogFilter>,
}

impl LogFilterParameters {
    pub(crate) fn to_worker_sdk(&self) -> Worker_LogFilterParameters {
        // TODO: This currently leaks the callback pointer. This should live for the duration of the `Connection`.
        let callback_ptr = self
            .callback
            .map(Box::new)
            .map(Box::into_raw)
            .map(|ptr| ptr as *mut ::std::os::raw::c_void);

        Worker_LogFilterParameters {
            categories: self.categories.bits,
            level: self.level as u8,
            callback: self.callback.map(|_| worker_log_filter_callback as _),
            user_data: callback_ptr.unwrap_or(ptr::null_mut()),
        }
    }
}

impl Default for LogFilterParameters {
    fn default() -> Self {
        LogFilterParameters {
            categories: LogCategory::All,
            level: LogLevel::Error,
            callback: None,
        }
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

pub type LogCallback = fn(message: LogData);

#[derive(Default)]
pub struct LogCallbackParameters {
    pub log_callback: Option<LogCallback>,
}

impl LogCallbackParameters {
    pub(crate) fn to_worker_sdk(&self) -> Worker_LogCallbackParameters {
        // TODO: This currently leaks the callback pointer. This should live for the duration of the `Connection`.
        let callback_ptr = self
            .log_callback
            .map(Box::new)
            .map(Box::into_raw)
            .map(|ptr| ptr as *mut ::std::os::raw::c_void);

        Worker_LogCallbackParameters {
            log_callback: self.log_callback.map(|_| worker_log_callback as _),
            user_data: callback_ptr.unwrap_or(ptr::null_mut()),
        }
    }
}

#[no_mangle]
unsafe extern "C" fn worker_log_callback(
    user_data: *mut ::std::os::raw::c_void,
    message: *const Worker_LogData,
) {
    let params: &mut LogCallbackParameters = &mut *(user_data as *mut LogCallbackParameters);
    if let Some(callback) = params.log_callback {
        callback(LogData {
            timestamp: cstr_to_string((*message).timestamp),
            categories: LogCategory::from_bits_unchecked((*message).categories),
            log_level: ::std::mem::transmute((*message).log_level),
            content: cstr_to_string((*message).content),
        })
    };
}

pub struct RotatingLogFileParameters {
    pub log_prefix: CString,
    pub max_log_files: u32,
    pub max_log_file_size_bytes: u32,
}

impl RotatingLogFileParameters {
    pub fn default() -> Self {
        RotatingLogFileParameters {
            log_prefix: Default::default(),
            max_log_files: 0,
            max_log_file_size_bytes: 0,
        }
    }

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

pub struct LogsinkParameters {
    pub logsink_type: LogsinkType,
    pub filter_parameters: LogFilterParameters,
    pub rotating_logfile_parameters: RotatingLogFileParameters,
    pub log_callback_parameters: LogCallbackParameters,
}

impl LogsinkParameters {
    /// Converts the rotating log parameters into the equivalent C API type.
    ///
    /// # Safety
    ///
    /// The returned `Worker_RotatingLogFileParameters` borrows data owned by `self`,
    /// and therefore must not outlive `self`.
    pub fn to_worker_sdk(&self) -> Worker_LogsinkParameters {
        Worker_LogsinkParameters {
            logsink_type: self.logsink_type as u8,
            filter_parameters: self.filter_parameters.to_worker_sdk(),
            rotating_logfile_parameters: self.rotating_logfile_parameters.to_worker_sdk(),
            log_callback_parameters: self.log_callback_parameters.to_worker_sdk(),
        }
    }
}

impl Default for LogsinkParameters {
    fn default() -> Self {
        LogsinkParameters {
            logsink_type: LogsinkType::Stdout,
            filter_parameters: LogFilterParameters::default(),
            rotating_logfile_parameters: RotatingLogFileParameters::default(),
            log_callback_parameters: LogCallbackParameters { log_callback: None },
        }
    }
}
