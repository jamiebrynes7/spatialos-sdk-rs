pub mod internal;

pub mod commands;
pub mod component;
pub mod connection;
pub mod entity;
pub mod entity_snapshot;
pub mod locator;
pub mod metrics;
pub mod op;
pub mod parameters;
pub mod query;
pub mod snapshot;
pub mod vtable;

use component::ComponentId;
use spatialos_sdk_sys::worker::Worker_InterestOverride;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct EntityId {
    pub id: i64,
}

impl EntityId {
    pub fn new(id: i64) -> EntityId {
        EntityId { id }
    }

    pub fn is_valid(self) -> bool {
        self.id > 0
    }

    pub fn to_string(self) -> String {
        format!("EntityId: {}", self.id)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialOrd, Ord)]
pub struct RequestId<T> {
    id: u32,
    _type: PhantomData<T>,
}

// SAFE: `RequestId<T>` is a type-safe wrapper around a single integer value, and is
// completely thread-safe. The manual impls here are only needed to ensure that the
// implementation isn't bounded on `T: Send`/`T: Sync`.
unsafe impl<T> Send for RequestId<T> {}
unsafe impl<T> Sync for RequestId<T> {}

impl<T> RequestId<T> {
    pub fn new(id: u32) -> RequestId<T> {
        RequestId {
            id,
            _type: PhantomData,
        }
    }
}

impl<T> PartialEq for RequestId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Hash for RequestId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Authority {
    Authoritative,
    AuthorityLossImminent,
    NotAuthoritative,
}

impl Authority {
    pub fn has_authority(self) -> bool {
        self != Authority::NotAuthoritative
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
    fn to_worker_sdk(self) -> u8 {
        match self {
            LogLevel::Debug => 1,
            LogLevel::Info => 2,
            LogLevel::Warn => 3,
            LogLevel::Error => 4,
            LogLevel::Fatal => 5,
        }
    }
}

// TODO: Replace with TryFrom when it stabilises: https://github.com/rust-lang/rust/issues/33417
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

pub struct InterestOverride {
    pub component_id: ComponentId,
    pub is_interested: bool,
}

impl InterestOverride {
    pub fn new(component_id: ComponentId, is_interested: bool) -> Self {
        InterestOverride {
            component_id,
            is_interested,
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> Worker_InterestOverride {
        Worker_InterestOverride {
            is_interested: self.is_interested as u8,
            component_id: self.component_id,
        }
    }
}
