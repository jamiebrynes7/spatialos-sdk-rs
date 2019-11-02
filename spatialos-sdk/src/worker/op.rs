#![allow(non_upper_case_globals)]

use crate::worker::{
    commands::*,
    component::{self, *},
    entity::Entity,
    internal::utils::*,
    metrics::Metrics,
    schema::{
        SchemaCommandRequest, SchemaCommandResponse, SchemaComponentData, SchemaComponentUpdate,
    },
    {Authority, EntityId, LogLevel, RequestId},
};
use spatialos_sdk_sys::worker::*;
use std::{collections::HashMap, slice};

pub struct OpList {
    raw: *mut Worker_OpList,
}

impl OpList {
    pub(crate) fn new(raw: *mut Worker_OpList) -> Self {
        assert!(!raw.is_null());
        OpList { raw }
    }

    /// Returns an iterator over the list.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use spatialos_sdk::worker::connection::*;
    /// # let connection: WorkerConnection = unimplemented!();
    /// for op in connection.get_op_list(0).iter() {
    ///     // Process `op`.
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_> {
        self.into_iter()
    }

    /// Returns the number of ops in the list.
    fn len(&self) -> usize {
        self.raw().op_count as usize
    }

    /// Returns a reference to the raw `Worker_OpList`.
    ///
    /// This is a simple helper to reduce the `unsafe` boilerplate in all the
    /// places where we need to access data on the raw op list.
    fn raw(&self) -> &Worker_OpList {
        unsafe { &*self.raw }
    }
}

impl<'a> IntoIterator for &'a OpList {
    type Item = WorkerOp<'a>;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let slice = unsafe { slice::from_raw_parts(self.raw().ops, self.len()) };
        Iter { iter: slice.iter() }
    }
}

impl Drop for OpList {
    fn drop(&mut self) {
        assert!(!self.raw.is_null());
        unsafe {
            Worker_OpList_Destroy(self.raw);
        }
    }
}

pub struct Iter<'a> {
    iter: slice::Iter<'a, Worker_Op>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = WorkerOp<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(WorkerOp::from)
    }
}

#[derive(Debug)]
pub enum StatusCode<T> {
    Success(T),
    Timeout(String),
    NotFound(String),
    AuthorityLost(String),
    PermissionDenied(String),
    ApplicationError(String),
    InternalError(String),
}

#[derive(Debug)]
pub enum WorkerOp<'a> {
    Disconnect(DisconnectOp),
    FlagUpdate(FlagUpdateOp),
    LogMessage(LogMessageOp),
    Metrics(MetricsOp),
    CriticalSection(CriticalSectionOp),
    AddEntity(AddEntityOp),
    RemoveEntity(RemoveEntityOp),
    AddComponent(AddComponentOp<'a>),
    RemoveComponent(RemoveComponentOp),
    ComponentUpdate(ComponentUpdateOp<'a>),
    AuthorityChange(AuthorityChangeOp),
    CommandRequest(CommandRequestOp<'a>),
    CommandResponse(CommandResponseOp<'a>),
    ReserveEntityIdsResponse(ReserveEntityIdsResponseOp),
    CreateEntityResponse(CreateEntityResponseOp),
    DeleteEntityResponse(DeleteEntityResponseOp),
    EntityQueryResponse(EntityQueryResponseOp),
}

impl<'a> From<&'a Worker_Op> for WorkerOp<'a> {
    fn from(op: &'a Worker_Op) -> Self {
        unsafe {
            let erased_op = &op.op;
            let op_type = Worker_StatusCode::from(op.op_type);
            match op_type {
                Worker_OpType_WORKER_OP_TYPE_DISCONNECT => {
                    let op = erased_op.disconnect;
                    let disconnect_op = DisconnectOp {
                        reason: cstr_to_string(op.reason),
                    };
                    WorkerOp::Disconnect(disconnect_op)
                }
                Worker_OpType_WORKER_OP_TYPE_FLAG_UPDATE => {
                    let op = erased_op.flag_update;
                    let flag_update_op = FlagUpdateOp {
                        name: cstr_to_string(op.name),
                        value: cstr_to_string(op.value),
                    };
                    WorkerOp::FlagUpdate(flag_update_op)
                }
                Worker_OpType_WORKER_OP_TYPE_LOG_MESSAGE => {
                    let op = erased_op.log_message;
                    let log_message_op = LogMessageOp {
                        message: cstr_to_string(op.message),
                        log_level: LogLevel::from(op.level),
                    };
                    WorkerOp::LogMessage(log_message_op)
                }
                Worker_OpType_WORKER_OP_TYPE_METRICS => {
                    let op = erased_op.metrics;
                    let metrics_op = MetricsOp {
                        metrics: Metrics::from(&op.metrics),
                    };
                    WorkerOp::Metrics(metrics_op)
                }
                Worker_OpType_WORKER_OP_TYPE_CRITICAL_SECTION => {
                    let op = erased_op.critical_section;
                    let critical_section_op = CriticalSectionOp {
                        in_critical_section: op.in_critical_section != 0,
                    };
                    WorkerOp::CriticalSection(critical_section_op)
                }
                Worker_OpType_WORKER_OP_TYPE_ADD_ENTITY => {
                    let op = erased_op.add_entity;
                    let add_entity_op = AddEntityOp {
                        entity_id: EntityId::new(op.entity_id),
                    };
                    WorkerOp::AddEntity(add_entity_op)
                }
                Worker_OpType_WORKER_OP_TYPE_REMOVE_ENTITY => {
                    let op = erased_op.remove_entity;
                    let remove_entity_op = RemoveEntityOp {
                        entity_id: EntityId::new(op.entity_id),
                    };
                    WorkerOp::RemoveEntity(remove_entity_op)
                }
                Worker_OpType_WORKER_OP_TYPE_ADD_COMPONENT => {
                    let op = &erased_op.add_component;
                    let add_component_op = AddComponentOp {
                        entity_id: EntityId::new(op.entity_id),
                        component_id: op.data.component_id,
                        component_data: internal::ComponentData::from(&op.data),
                    };
                    WorkerOp::AddComponent(add_component_op)
                }
                Worker_OpType_WORKER_OP_TYPE_REMOVE_COMPONENT => {
                    let op = erased_op.remove_component;
                    let remove_component_op = RemoveComponentOp {
                        entity_id: EntityId::new(op.entity_id),
                        component_id: op.component_id,
                    };
                    WorkerOp::RemoveComponent(remove_component_op)
                }
                Worker_OpType_WORKER_OP_TYPE_AUTHORITY_CHANGE => {
                    let op = erased_op.authority_change;
                    let authority_change_op = AuthorityChangeOp {
                        entity_id: EntityId::new(op.entity_id),
                        component_id: op.component_id,
                        authority: Authority::from(op.authority),
                    };
                    WorkerOp::AuthorityChange(authority_change_op)
                }
                Worker_OpType_WORKER_OP_TYPE_COMPONENT_UPDATE => {
                    let op = &erased_op.component_update;
                    let component_update_op = ComponentUpdateOp {
                        entity_id: EntityId::new(op.entity_id),
                        component_id: op.update.component_id,
                        component_update: internal::ComponentUpdate::from(&op.update),
                    };
                    WorkerOp::ComponentUpdate(component_update_op)
                }
                Worker_OpType_WORKER_OP_TYPE_COMMAND_REQUEST => {
                    let op = &erased_op.command_request;
                    let attribute_set = cstr_array_to_vec_string(
                        op.caller_attribute_set.attributes,
                        op.caller_attribute_set.attribute_count,
                    );

                    let command_request_op = CommandRequestOp {
                        request_id: RequestId::new(op.request_id),
                        entity_id: EntityId::new(op.entity_id),
                        timeout_millis: op.timeout_millis,
                        caller_worker_id: cstr_to_string(op.caller_worker_id),
                        caller_attribute_set: attribute_set,
                        component_id: op.request.component_id,
                        request: internal::CommandRequest::from(&op.request),
                    };
                    WorkerOp::CommandRequest(command_request_op)
                }
                Worker_OpType_WORKER_OP_TYPE_COMMAND_RESPONSE => {
                    let op = &erased_op.command_response;
                    let status_code = match Worker_StatusCode::from(op.status_code) {
                        Worker_StatusCode_WORKER_STATUS_CODE_SUCCESS => {
                            StatusCode::Success(CommandResponse {
                                response: internal::CommandResponse::from(&op.response),
                            })
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_TIMEOUT => {
                            StatusCode::Timeout(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_NOT_FOUND => {
                            StatusCode::NotFound(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_AUTHORITY_LOST => {
                            StatusCode::AuthorityLost(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_PERMISSION_DENIED => {
                            StatusCode::PermissionDenied(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_APPLICATION_ERROR => {
                            StatusCode::ApplicationError(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_INTERNAL_ERROR => {
                            StatusCode::InternalError(cstr_to_string(op.message))
                        }
                        _ => panic!(
                            "Unknown command response status code received: {}",
                            op.status_code
                        ),
                    };

                    let command_response_op = CommandResponseOp {
                        entity_id: EntityId::new(op.entity_id),
                        request_id: RequestId::new(op.request_id),
                        component_id: op.response.component_id,
                        response: status_code,
                    };
                    WorkerOp::CommandResponse(command_response_op)
                }
                Worker_OpType_WORKER_OP_TYPE_RESERVE_ENTITY_IDS_RESPONSE => {
                    let op = erased_op.reserve_entity_ids_response;
                    let status_code = match Worker_StatusCode::from(op.status_code) {
                        Worker_StatusCode_WORKER_STATUS_CODE_SUCCESS => StatusCode::Success(
                            ReservedEntityIdRange::new(op.first_entity_id, op.number_of_entity_ids),
                        ),
                        Worker_StatusCode_WORKER_STATUS_CODE_TIMEOUT => {
                            StatusCode::Timeout(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_NOT_FOUND => {
                            StatusCode::NotFound(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_AUTHORITY_LOST => {
                            StatusCode::AuthorityLost(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_PERMISSION_DENIED => {
                            StatusCode::PermissionDenied(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_APPLICATION_ERROR => {
                            StatusCode::ApplicationError(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_INTERNAL_ERROR => {
                            StatusCode::InternalError(cstr_to_string(op.message))
                        }
                        _ => panic!(
                            "Unknown command response status code received: {}",
                            op.status_code
                        ),
                    };

                    let reserve_entity_ids_response_op = ReserveEntityIdsResponseOp {
                        request_id: RequestId::new(op.request_id),
                        status_code,
                    };
                    WorkerOp::ReserveEntityIdsResponse(reserve_entity_ids_response_op)
                }
                Worker_OpType_WORKER_OP_TYPE_CREATE_ENTITY_RESPONSE => {
                    let op = erased_op.create_entity_response;
                    let status_code = match Worker_StatusCode::from(op.status_code) {
                        Worker_StatusCode_WORKER_STATUS_CODE_SUCCESS => {
                            StatusCode::Success(EntityId::new(op.entity_id))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_TIMEOUT => {
                            StatusCode::Timeout(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_NOT_FOUND => {
                            StatusCode::NotFound(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_AUTHORITY_LOST => {
                            StatusCode::AuthorityLost(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_PERMISSION_DENIED => {
                            StatusCode::PermissionDenied(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_APPLICATION_ERROR => {
                            StatusCode::ApplicationError(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_INTERNAL_ERROR => {
                            StatusCode::InternalError(cstr_to_string(op.message))
                        }
                        _ => panic!(
                            "Unknown command response status code received: {}",
                            op.status_code
                        ),
                    };

                    let create_entity_response_op = CreateEntityResponseOp {
                        request_id: RequestId::new(op.request_id),
                        status_code,
                    };
                    WorkerOp::CreateEntityResponse(create_entity_response_op)
                }
                Worker_OpType_WORKER_OP_TYPE_DELETE_ENTITY_RESPONSE => {
                    let op = erased_op.delete_entity_response;
                    let status_code = match Worker_StatusCode::from(op.status_code) {
                        Worker_StatusCode_WORKER_STATUS_CODE_SUCCESS => StatusCode::Success(()),
                        Worker_StatusCode_WORKER_STATUS_CODE_TIMEOUT => {
                            StatusCode::Timeout(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_NOT_FOUND => {
                            StatusCode::NotFound(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_AUTHORITY_LOST => {
                            StatusCode::AuthorityLost(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_PERMISSION_DENIED => {
                            StatusCode::PermissionDenied(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_APPLICATION_ERROR => {
                            StatusCode::ApplicationError(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_INTERNAL_ERROR => {
                            StatusCode::InternalError(cstr_to_string(op.message))
                        }
                        _ => panic!(
                            "Unknown command response status code received: {}",
                            op.status_code
                        ),
                    };

                    let delete_entity_response_op = DeleteEntityResponseOp {
                        request_id: RequestId::new(op.request_id),
                        entity_id: EntityId::new(op.entity_id),
                        status_code,
                    };
                    WorkerOp::DeleteEntityResponse(delete_entity_response_op)
                }
                Worker_OpType_WORKER_OP_TYPE_ENTITY_QUERY_RESPONSE => {
                    let op = erased_op.entity_query_response;
                    let status_code = match Worker_StatusCode::from(op.status_code) {
                        Worker_StatusCode_WORKER_STATUS_CODE_SUCCESS => {
                            if op.results.is_null() {
                                // Is count type.
                                StatusCode::Success(QueryResponse::Result(op.result_count))
                            } else {
                                let mut entities = HashMap::new();
                                let raw_entities =
                                    slice::from_raw_parts(op.results, op.result_count as usize);

                                for raw_entity in raw_entities {
                                    entities.insert(
                                        EntityId::new(raw_entity.entity_id),
                                        Entity::from_worker_sdk(raw_entity).unwrap(),
                                    );
                                }

                                StatusCode::Success(QueryResponse::Snapshot(entities))
                            }
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_TIMEOUT => {
                            StatusCode::Timeout(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_NOT_FOUND => {
                            StatusCode::NotFound(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_AUTHORITY_LOST => {
                            StatusCode::AuthorityLost(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_PERMISSION_DENIED => {
                            StatusCode::PermissionDenied(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_APPLICATION_ERROR => {
                            StatusCode::ApplicationError(cstr_to_string(op.message))
                        }
                        Worker_StatusCode_WORKER_STATUS_CODE_INTERNAL_ERROR => {
                            StatusCode::InternalError(cstr_to_string(op.message))
                        }
                        _ => panic!(
                            "Unknown command response status code received: {}",
                            op.status_code
                        ),
                    };

                    let entity_query_response_op = EntityQueryResponseOp {
                        request_id: RequestId::new(op.request_id),
                        status_code,
                    };

                    WorkerOp::EntityQueryResponse(entity_query_response_op)
                }
                _ => panic!("Unknown op code received: {}", op_type),
            }
        }
    }
}

#[derive(Debug)]
pub struct DisconnectOp {
    pub reason: String,
}

#[derive(Debug)]
pub struct FlagUpdateOp {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct LogMessageOp {
    pub message: String,
    pub log_level: LogLevel,
}

#[derive(Debug)]
pub struct MetricsOp {
    pub metrics: Metrics,
}

#[derive(Debug)]
pub struct CriticalSectionOp {
    pub in_critical_section: bool,
}

#[derive(Debug)]
pub struct AddEntityOp {
    pub entity_id: EntityId,
}

#[derive(Debug)]
pub struct RemoveEntityOp {
    pub entity_id: EntityId,
}

#[derive(Debug)]
pub struct ReserveEntityIdsResponseOp {
    pub request_id: RequestId<ReserveEntityIdsRequest>,
    pub status_code: StatusCode<ReservedEntityIdRange>,
}

// TODO: When https://doc.rust-lang.org/std/iter/trait.Step.html is stabilized - replace this
//       with std::ops::Range<EntityId> and implement Step for EntityId.
#[derive(Debug)]
pub struct ReservedEntityIdRange {
    current: i64,
    consumed: u32,
    reserved: u32,
}

impl ReservedEntityIdRange {
    pub(crate) fn new(first: i64, number: u32) -> Self {
        ReservedEntityIdRange {
            current: first,
            consumed: 0,
            reserved: number,
        }
    }
}

impl Iterator for ReservedEntityIdRange {
    type Item = EntityId;

    fn next(&mut self) -> Option<EntityId> {
        if self.consumed == self.reserved {
            return None;
        }

        self.consumed += 1;
        let res = Some(EntityId::new(self.current));
        self.current += 1;

        res
    }
}

#[derive(Debug)]
pub struct CreateEntityResponseOp {
    pub request_id: RequestId<CreateEntityRequest>,
    pub status_code: StatusCode<EntityId>,
}

#[derive(Debug)]
pub struct DeleteEntityResponseOp {
    pub request_id: RequestId<DeleteEntityRequest>,
    pub entity_id: EntityId,
    pub status_code: StatusCode<()>,
}

#[derive(Debug)]
pub enum QueryResponse {
    Snapshot(HashMap<EntityId, Entity>),
    Result(u32),
}

#[derive(Debug)]
pub struct EntityQueryResponseOp {
    pub request_id: RequestId<EntityQueryRequest>,
    pub status_code: StatusCode<QueryResponse>,
}

#[derive(Debug)]
pub struct AddComponentOp<'a> {
    pub entity_id: EntityId,
    pub component_id: ComponentId,
    component_data: component::internal::ComponentData<'a>,
}

impl<'a> AddComponentOp<'a> {
    pub fn get<C: Component>(&self) -> Option<&C> {
        if C::ID == self.component_data.component_id {
            // TODO: Deserialize schema_type if user_handle is null.
            Some(unsafe { &*(self.component_data.user_handle as *const _) })
        } else {
            None
        }
    }

    fn schema(&self) -> &SchemaComponentData {
        &self.component_data.schema_type
    }
}

#[derive(Debug)]
pub struct RemoveComponentOp {
    pub entity_id: EntityId,
    pub component_id: ComponentId,
}

#[derive(Debug)]
pub struct AuthorityChangeOp {
    pub entity_id: EntityId,
    pub component_id: ComponentId,
    pub authority: Authority,
}

#[derive(Debug)]
pub struct ComponentUpdateOp<'a> {
    pub entity_id: EntityId,
    pub component_id: ComponentId,
    pub component_update: component::internal::ComponentUpdate<'a>,
}

impl<'a> ComponentUpdateOp<'a> {
    pub fn get<C: Component>(&self) -> Option<&C::Update> {
        // TODO: Deserialize schema_type if user_handle is null.
        if C::ID == self.component_update.component_id
            && !self.component_update.user_handle.is_null()
        {
            Some(unsafe { &*(self.component_update.user_handle as *const _) })
        } else {
            None
        }
    }

    fn schema(&self) -> &SchemaComponentUpdate {
        &self.component_update.schema_type
    }
}

#[derive(Debug)]
pub struct CommandRequestOp<'a> {
    pub request_id: RequestId<IncomingCommandRequest>,
    pub entity_id: EntityId,
    pub timeout_millis: u32,
    pub caller_worker_id: String,
    pub caller_attribute_set: Vec<String>,
    pub component_id: ComponentId,
    request: component::internal::CommandRequest<'a>,
}

impl<'a> CommandRequestOp<'a> {
    pub fn get<C: Component>(&self) -> Option<&C::CommandRequest> {
        // TODO: Deserialize schema_type if user_handle is null.
        if C::ID == self.component_id && !self.request.user_handle.is_null() {
            Some(unsafe { &*(self.request.user_handle as *const _) })
        } else {
            None
        }
    }

    fn schema(&self) -> &SchemaCommandRequest {
        &self.request.schema_type
    }
}

#[derive(Debug)]
pub struct CommandResponseOp<'a> {
    pub request_id: RequestId<OutgoingCommandRequest>,
    pub entity_id: EntityId,
    pub component_id: ComponentId,
    pub response: StatusCode<CommandResponse<'a>>,
}

#[derive(Debug)]
pub struct CommandResponse<'a> {
    response: component::internal::CommandResponse<'a>,
}

impl<'a> CommandResponse<'a> {
    pub fn get<C: Component>(&self) -> Option<&C::CommandResponse> {
        // TODO: Deserialize schema_type if user_handle is null.
        if C::ID == self.response.component_id && !self.response.user_handle.is_null() {
            Some(unsafe { &*(self.response.user_handle as *const _) })
        } else {
            None
        }
    }

    fn schema(&self) -> &SchemaCommandResponse {
        &self.response.schema_type
    }
}

#[cfg(test)]
mod test {
    use super::ReservedEntityIdRange;

    #[test]
    fn reserved_entity_id_range_iterator_contains_correct_count() {
        let range = ReservedEntityIdRange::new(10, 54);
        assert_eq!(54, range.count());
    }

    #[test]
    fn reserved_entity_id_range_iterator_returns_sequential_ids() {
        let mut current_id: i64 = 1;
        let range = ReservedEntityIdRange::new(current_id, 10);

        for entity_id in range {
            assert_eq!(current_id, entity_id.id);
            current_id += 1;
        }
    }
}
