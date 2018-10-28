use worker::query::EntityQuery;
use worker::EntityId;

pub struct IncomingCommandRequest {}

pub struct OutgoingCommandRequest {}

// =============================== World Commands =============================== //
pub struct ReserveEntityIdsRequest(pub u32);

pub struct CreateEntityRequest {}

pub struct DeleteEntityRequest(pub EntityId);

pub struct EntityQueryRequest(pub EntityQuery);
