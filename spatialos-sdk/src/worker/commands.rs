use worker::query::EntityQuery;
use worker::EntityId;

#[derive(Debug)]
pub struct IncomingCommandRequest {}

#[derive(Debug)]
pub struct OutgoingCommandRequest {}

// =============================== World Commands =============================== //
#[derive(Debug)]
pub struct ReserveEntityIdsRequest(pub u32);

#[derive(Debug)]
pub struct CreateEntityRequest {}

#[derive(Debug)]
pub struct DeleteEntityRequest(pub EntityId);

#[derive(Debug)]
pub struct EntityQueryRequest(pub EntityQuery);
