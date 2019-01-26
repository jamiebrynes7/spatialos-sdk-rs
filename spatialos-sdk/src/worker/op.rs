#![allow(non_upper_case_globals)]

use std::collections::HashMap;
use std::slice;

use crate::worker::commands::*;
use crate::worker::component::{self, *};
use crate::worker::entity_snapshot::EntitySnapshot;
use crate::worker::metrics::Metrics;
use crate::worker::{Authority, EntityId, LogLevel, RequestId};

use crate::worker::internal::utils::*;
use spatialos_sdk_sys::worker::*;

pub struct OpList {
    ops: Vec<WorkerOp>,
    internal_ptr: *mut Worker_OpList,
}

impl OpList {
    pub(crate) fn new(raw_ops_list_ptr: *mut Worker_OpList) -> Self {
        unsafe {
            let worker_op_list = *raw_ops_list_ptr;
            let ops = slice::from_raw_parts(worker_op_list.ops, worker_op_list.op_count as usize)
                .iter()
                .map(WorkerOp::from)
                .collect::<Vec<WorkerOp>>();

            OpList {
                ops,
                internal_ptr: raw_ops_list_ptr,
            }
        }
    }
}

impl<'a> IntoIterator for &'a OpList {
    type Item = &'a WorkerOp;
    type IntoIter = slice::Iter<'a, WorkerOp>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.ops.iter()
    }
}

impl Drop for OpList {
    fn drop(&mut self) {
        assert!(!self.internal_ptr.is_null());
        unsafe { Worker_OpList_Destroy(self.internal_ptr) }
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
pub enum WorkerOp {
    Disconnect(DisconnectOp),
    FlagUpdate(FlagUpdateOp),
    LogMessage(LogMessageOp),
    Metrics(MetricsOp),
    CriticalSection(CriticalSectionOp),
    AddEntity(AddEntityOp),
    RemoveEntity(RemoveEntityOp),
    AddComponent(AddComponentOp),
    RemoveComponent(RemoveComponentOp),
    ComponentUpdate(ComponentUpdateOp),
    AuthorityChange(AuthorityChangeOp),
    CommandRequest(CommandRequestOp),
    CommandResponse(CommandResponseOp),
    ReserveEntityIdsResponse(ReserveEntityIdsResponseOp),
    CreateEntityResponse(CreateEntityResponseOp),
    DeleteEntityResponse(DeleteEntityResponseOp),
    EntityQueryResponse(EntityQueryResponseOp),
}

impl From<&Worker_Op> for WorkerOp {
    fn from(op: &Worker_Op) -> Self {
        unsafe {
            let erased_op = op.__bindgen_anon_1;
            let op_type = u32::from(op.op_type);
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
                    let op = erased_op.add_component;
                    let add_component_op = AddComponentOp {
                        entity_id: EntityId::new(op.entity_id),
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
                    let op = erased_op.component_update;
                    let component_update_op = ComponentUpdateOp {
                        entity_id: EntityId::new(op.entity_id),
                        component_update: internal::ComponentUpdate::from(&op.update),
                    };
                    WorkerOp::ComponentUpdate(component_update_op)
                }
                Worker_OpType_WORKER_OP_TYPE_COMMAND_REQUEST => {
                    let op = erased_op.command_request;
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
                        request: internal::CommandRequest::from(&op.request),
                    };
                    WorkerOp::CommandRequest(command_request_op)
                }
                Worker_OpType_WORKER_OP_TYPE_COMMAND_RESPONSE => {
                    let op = erased_op.command_response;
                    let status_code = match u32::from(op.status_code) {
                        Worker_StatusCode_WORKER_STATUS_CODE_SUCCESS => {
                            StatusCode::Success(internal::CommandResponse::from(&op.response))
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
                        status_code,
                    };
                    WorkerOp::CommandResponse(command_response_op)
                }
                Worker_OpType_WORKER_OP_TYPE_RESERVE_ENTITY_IDS_RESPONSE => {
                    let op = erased_op.reserve_entity_ids_response;
                    let status_code = match u32::from(op.status_code) {
                        Worker_StatusCode_WORKER_STATUS_CODE_SUCCESS => {
                            StatusCode::Success(EntityId::new(op.first_entity_id))
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

                    let reserve_entity_ids_response_op = ReserveEntityIdsResponseOp {
                        request_id: RequestId::new(op.request_id),
                        number_of_entity_ids: op.number_of_entity_ids,
                        status_code,
                    };
                    WorkerOp::ReserveEntityIdsResponse(reserve_entity_ids_response_op)
                }
                Worker_OpType_WORKER_OP_TYPE_CREATE_ENTITY_RESPONSE => {
                    let op = erased_op.create_entity_response;
                    let status_code = match u32::from(op.status_code) {
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
                    let status_code = match u32::from(op.status_code) {
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
                    let status_code = match u32::from(op.status_code) {
                        Worker_StatusCode_WORKER_STATUS_CODE_SUCCESS => {
                            if op.results.is_null() {
                                // Is count type.
                                StatusCode::Success(QueryResponse::Result(op.result_count))
                            } else {
                                // TODO: Deseralise data? Do something with the snapshot.
                                StatusCode::Success(QueryResponse::Snapshot(HashMap::new()))
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
    pub status_code: StatusCode<EntityId>,
    pub number_of_entity_ids: u32,
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
    Snapshot(HashMap<EntityId, EntitySnapshot>),
    Result(u32),
}

#[derive(Debug)]
pub struct EntityQueryResponseOp {
    pub request_id: RequestId<EntityQueryRequest>,
    pub status_code: StatusCode<QueryResponse>,
}

#[derive(Debug)]
pub struct AddComponentOp {
    pub entity_id: EntityId,
    component_data: component::internal::ComponentData,
}

impl AddComponentOp {
    pub fn component_id(&self) -> ComponentId {
        self.component_data.component_id
    }

    pub fn get<C: Component>(&self) -> Option<&C> {
        if C::component_id() == self.component_id() {
            Some(unsafe {
                &*(self.component_data.user_handle as *const _)
            })
        } else {
            None
        }
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
pub struct ComponentUpdateOp {
    pub entity_id: EntityId,
    pub component_update: component::internal::ComponentUpdate,
}

impl ComponentUpdateOp {
    pub fn component_id(&self) -> ComponentId {
        self.component_update.component_id
    }

    pub fn get<C: Component>(&self) -> Option<&C::Update> {
        if C::component_id() == self.component_id() {
            Some(unsafe {
                &*(self.component_update.user_handle as *const _)
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct CommandRequestOp {
    pub request_id: RequestId<IncomingCommandRequest>,
    pub entity_id: EntityId,
    pub timeout_millis: u32,
    pub caller_worker_id: String,
    pub caller_attribute_set: Vec<String>,
    pub request: component::internal::CommandRequest,
}

#[derive(Debug)]
pub struct CommandResponseOp {
    pub request_id: RequestId<OutgoingCommandRequest>,
    pub entity_id: EntityId,
    pub status_code: StatusCode<component::internal::CommandResponse>,
}
