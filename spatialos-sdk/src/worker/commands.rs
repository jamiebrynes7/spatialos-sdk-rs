use crate::worker::entity::Entity;
use crate::worker::query::EntityQuery;
use crate::worker::EntityId;
use spatialos_sdk_sys::worker::Worker_CommandParameters;

/// Additional parameters for sending command requests.
///
/// Additional parameters passed to [`Connection::send_command_request`]. Note that all
/// parameters are kept private and the struct can only be initialized with default
/// values in order to make it possible to add new parameters without a breaking change.
///
/// If you would like to use a method-chaining style when initializing the parameters,
/// the [tap] crate is recommended. The examples below demonstrate this.
///
/// # Parameters
///
/// * `allow_short_circuit: false` allow the command to skip being routed through SpatialOS if
///   the worker is sending a command to itself. This avoids a round trip, but
///   SpatialOS cannot guarantee that the command will be fully delivered and executed.
///   [See the docs for more information.][short-circuit]
///
/// # Examples
///
/// ```
/// use spatialos_sdk::worker::commands::CommandParameters;
/// use tap::*;
///
/// let params = CommandParameters::new()
///     .tap(CommandParameters::allow_short_circuit);
/// ```
///
/// [short-circuit]: https://docs.improbable.io/reference/14.1/shared/design/commands#component-commands
/// [tap]: https://crates.io/crates/tap
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommandParameters {
    allow_short_circuit: bool,
}

impl CommandParameters {
    /// Creates a new `CommandParameters` with default values for all parameters.
    pub fn new() -> CommandParameters {
        Default::default()
    }

    /// Sets the `allow_short_circuit` parameter to `true`.
    pub fn allow_short_circuit(&mut self) {
        self.allow_short_circuit = true;
    }

    pub(crate) fn to_worker_sdk(self) -> Worker_CommandParameters {
        Worker_CommandParameters {
            allow_short_circuit: self.allow_short_circuit as u8,
        }
    }
}

// =============================== World Commands =============================== //
#[derive(Debug)]
pub struct ReserveEntityIdsRequest(pub u32);

#[derive(Debug)]
pub struct CreateEntityRequest(pub Entity, pub Option<EntityId>);

#[derive(Debug)]
pub struct DeleteEntityRequest(pub EntityId);

#[derive(Debug)]
pub struct EntityQueryRequest(pub EntityQuery);
