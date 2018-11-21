pub mod commands;
pub mod component;
pub mod connection;
pub mod entity_snapshot;
pub mod metrics;
pub mod op;
pub mod parameters;
pub mod query;
pub mod snapshot;
pub mod vtable;

use std::marker::PhantomData;
use worker::internal::bindings::Worker_InterestOverride;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct EntityId {
    pub id: i64,
}

impl EntityId {
    pub fn new(id: i64) -> EntityId {
        EntityId { id }
    }

    pub fn is_valid(&self) -> bool {
        self.id > 0
    }

    pub fn to_string(&self) -> String {
        format!("EntityId: {}", self.id)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct RequestId<T> {
    pub id: u32,
    _type: PhantomData<*const T>,
}

impl<T> RequestId<T> {
    pub fn new(id: u32) -> RequestId<T> {
        RequestId {
            id,
            _type: PhantomData,
        }
    }

    pub fn to_string(&self) -> String {
        format!("RequestId: {}", self.id)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Authority {
    Authoritative,
    AuthorityLossImminent,
    NotAuthoritative,
}

impl Authority {
    pub fn has_authority(&self) -> bool {
        self != &Authority::NotAuthoritative
    }
}

impl From<u8> for Authority {
    fn from(auth: u8) -> Self {
        match auth {
            0 => Authority::NotAuthoritative,
            1 => Authority::Authoritative,
            2 => Authority::AuthorityLossImminent,
            _ => panic!("Unknown authority state: {}", auth),
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl LogLevel {
    fn to_worker_sdk(&self) -> u8 {
        match self {
            LogLevel::Debug => 1,
            LogLevel::Info => 2,
            LogLevel::Warn => 3,
            LogLevel::Error => 4,
            LogLevel::Fatal => 5,
            _ => panic!("Unknown log level"),
        }
    }
}

impl From<u8> for LogLevel {
    fn from(log_level: u8) -> Self {
        match log_level {
            1 => LogLevel::Debug,
            2 => LogLevel::Info,
            3 => LogLevel::Warn,
            4 => LogLevel::Error,
            5 => LogLevel::Fatal,
            _ => panic!("Unknown log level: {}", log_level),
        }
    }
}

pub struct InterestOverride {
    pub component_id: u32,
    pub is_interested: bool,
}

impl InterestOverride {
    pub(crate) fn to_woker_sdk(&self) -> Worker_InterestOverride {
        Worker_InterestOverride {
            is_interested: self.is_interested as u8,
            component_id: self.component_id,
        }
    }
}

impl From<InterestOverride> for Worker_InterestOverride {
    fn from(interest_override: InterestOverride) -> Self {
        Worker_InterestOverride {
            is_interested: interest_override.is_interested as u8,
            component_id: interest_override.component_id,
        }
    }
}
