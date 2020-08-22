pub mod commands;
pub mod component;
pub mod connection;
pub mod entity;
pub mod entity_builder;
pub mod locator;
pub mod logging;
pub mod metrics;
pub mod op;
pub mod parameters;
pub mod query;
pub mod schema;
pub mod snapshot;
pub(crate) mod utils;
pub mod worker_future;

use component::ComponentId;
use spatialos_sdk_sys::worker::Worker_InterestOverride;
use std::fmt::{Display, Error, Formatter};

// NOTE: This must be `repr(transparent)` in order for it to be ABI-compatible with
// the C API, which uses a raw `i64` to represent an entity ID. See the comment on
// the `impl_primitive_field!` macro for more details.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
#[repr(transparent)]
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
}

impl Display for EntityId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Entity ID {}", self.id)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestId {
    pub id: i64,
}

impl RequestId {
    pub fn new(id: i64) -> Self {
        RequestId { id }
    }
}

impl Display for RequestId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Request ID {}", self.id)
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

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
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

pub mod constants {
    pub const LOCATOR_HOSTNAME: &str = "locator.improbable.io";
    pub const LOCATOR_PORT: u16 = 444;
    pub const RECEPTIONIST_PORT: u16 = 7777;
}
