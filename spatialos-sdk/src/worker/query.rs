use std::ptr;

use crate::worker::component::ComponentId;
use crate::worker::EntityId;

use spatialos_sdk_sys::worker::{
    Worker_AndConstraint, Worker_ComponentConstraint, Worker_Constraint,
    Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_AND,
    Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_COMPONENT,
    Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_ENTITY_ID,
    Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_NOT,
    Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_OR,
    Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_SPHERE, Worker_Constraint_Union,
    Worker_EntityIdConstraint, Worker_EntityQuery, Worker_NotConstraint, Worker_OrConstraint,
    Worker_SphereConstraint,
};

#[derive(Debug)]
pub enum ResultType {
    Count,
    Snapshot(Vec<ComponentId>),
}

impl ResultType {
    pub(crate) fn to_u8(&self) -> u8 {
        match self {
            ResultType::Count => 1,
            ResultType::Snapshot(_) => 2,
        }
    }
}

#[derive(Debug)]
pub struct EntityQuery {
    pub constraint: QueryConstraint,
    pub result_type: ResultType,
}

impl EntityQuery {
    pub fn new(constraint: QueryConstraint, result_type: ResultType) -> Self {
        EntityQuery {
            constraint,
            result_type,
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> WrappedEntityQuery {
        let (constraint, underlying_constraints) = self.constraint.to_worker_sdk();
        match &self.result_type {
            ResultType::Count => {
                let worker_entity_query = Worker_EntityQuery {
                    constraint,
                    result_type: self.result_type.to_u8(),
                    snapshot_result_type_component_id_count: 0,
                    snapshot_result_type_component_ids: ptr::null(),
                };

                WrappedEntityQuery {
                    query: worker_entity_query,
                    _ids: None,
                    _underlying_constraint_data: underlying_constraints,
                }
            }
            ResultType::Snapshot(ids) => {
                let worker_entity_query = Worker_EntityQuery {
                    constraint,
                    result_type: self.result_type.to_u8(),
                    snapshot_result_type_component_id_count: ids.len() as u32,
                    snapshot_result_type_component_ids: ids.as_ptr(),
                };

                WrappedEntityQuery {
                    query: worker_entity_query,
                    _ids: Some(ids.as_slice()),
                    _underlying_constraint_data: underlying_constraints,
                }
            }
        }
    }
}

pub(crate) struct WrappedEntityQuery<'a> {
    pub query: Worker_EntityQuery,
    _ids: Option<&'a [u32]>,
    _underlying_constraint_data: Box<[Worker_Constraint]>,
}

#[derive(Debug, Clone)]
pub enum QueryConstraint {
    EntityId(EntityId),
    Component(ComponentId),
    Sphere(f64, f64, f64, f64),

    And(Vec<QueryConstraint>),
    Or(Vec<QueryConstraint>),
    Not(Box<QueryConstraint>),
}

impl QueryConstraint {
    // The general strategy with this is to pre-allocate the memory required to store all elements
    // in the constraint tree. This means that we can insert points to indices in the array which
    // will contain the correct data. This also means that the array lifetime should live as long
    // as the struct passed down to the C layer.
    pub(crate) fn to_worker_sdk(&self) -> (Worker_Constraint, Box<[Worker_Constraint]>) {
        // First descend the tree and count how many constraints we need in the Vec.
        let size = self.constraint_len_recursive();

        // Allocate the vector required to store this data. Use a dummy placeholder for the data.
        let dummy_constraint = Worker_Constraint {
            constraint_type: Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_ENTITY_ID as u8,
            constraint: Worker_Constraint_Union {
                entity_id_constraint: Worker_EntityIdConstraint { entity_id: 0 },
            },
        };
        let underlying_data = vec![dummy_constraint; size as usize];
        let mut data = underlying_data.into_boxed_slice();

        // Now go down the tree again, this time creating pointers to the correct vector element.
        // Also fill the vector with the correct data as we go down the tree as well.
        let (constraint, _) = self.to_worker_sdk_recursive(&mut data, 0);

        // Return the constraint and the underlying data.
        (constraint, data)
    }

    fn to_worker_sdk_recursive(
        &self,
        underlying_data: &mut [Worker_Constraint],
        current_index: usize,
    ) -> (Worker_Constraint, usize) {
        match &self {
            QueryConstraint::EntityId(id) => {
                let constraint = Worker_Constraint {
                    constraint_type: Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_ENTITY_ID as u8,
                    constraint: Worker_Constraint_Union {
                        entity_id_constraint: Worker_EntityIdConstraint { entity_id: id.id },
                    },
                };

                (constraint, 0)
            }
            QueryConstraint::Component(component_id) => {
                let constraint = Worker_Constraint {
                    constraint_type: Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_COMPONENT as u8,
                    constraint: Worker_Constraint_Union {
                        component_constraint: Worker_ComponentConstraint {
                            component_id: *component_id,
                        },
                    },
                };

                (constraint, 0)
            }
            QueryConstraint::Sphere(x, y, z, radius) => {
                let constraint = Worker_Constraint {
                    constraint_type: Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_SPHERE as u8,
                    constraint: Worker_Constraint_Union {
                        sphere_constraint: Worker_SphereConstraint {
                            x: *x,
                            y: *y,
                            z: *z,
                            radius: *radius,
                        },
                    },
                };

                (constraint, 0)
            }
            QueryConstraint::And(constraints) => {
                let mut num_constraints_filled = constraints.len();
                let mut next_index = current_index;
                for constraint in constraints {
                    let (worker_constraint, elements_filled) = constraint.to_worker_sdk_recursive(
                        underlying_data,
                        current_index + num_constraints_filled,
                    );
                    num_constraints_filled += elements_filled;

                    underlying_data[next_index] = worker_constraint;
                    next_index += 1;
                }

                let constraint = Worker_Constraint {
                    constraint_type: Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_AND as u8,
                    constraint: Worker_Constraint_Union {
                        and_constraint: Worker_AndConstraint {
                            constraint_count: constraints.len() as u32,
                            constraints: &mut underlying_data[current_index]
                                as *mut Worker_Constraint,
                        },
                    },
                };

                (constraint, num_constraints_filled)
            }
            QueryConstraint::Or(constraints) => {
                let mut num_constraints_filled = constraints.len();
                let mut next_index = current_index;
                for constraint in constraints {
                    let (worker_constraint, elements_filled) = constraint.to_worker_sdk_recursive(
                        underlying_data,
                        current_index + num_constraints_filled,
                    );
                    num_constraints_filled += elements_filled;

                    underlying_data[next_index] = worker_constraint;
                    next_index += 1;
                }

                let constraint = Worker_Constraint {
                    constraint_type: Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_OR as u8,
                    constraint: Worker_Constraint_Union {
                        or_constraint: Worker_OrConstraint {
                            constraint_count: constraints.len() as u32,
                            constraints: &mut underlying_data[current_index]
                                as *mut Worker_Constraint,
                        },
                    },
                };

                (constraint, num_constraints_filled)
            }
            QueryConstraint::Not(constraint) => {
                let (worker_constraint, elements_filled) =
                    constraint.to_worker_sdk_recursive(underlying_data, current_index + 1);
                underlying_data[current_index] = worker_constraint;

                let constraint = Worker_Constraint {
                    constraint_type: Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_NOT as u8,
                    constraint: Worker_Constraint_Union {
                        not_constraint: Worker_NotConstraint {
                            constraint: &mut underlying_data[current_index]
                                as *mut Worker_Constraint,
                        },
                    },
                };

                (constraint, 1 + elements_filled)
            }
        }
    }

    fn constraint_len_recursive(&self) -> u32 {
        match &self {
            QueryConstraint::EntityId(_)
            | QueryConstraint::Component(_)
            | QueryConstraint::Sphere(..)
            | QueryConstraint::Not(_) => 1,
            QueryConstraint::And(constraints) | QueryConstraint::Or(constraints) => {
                constraints
                    .iter()
                    .map(QueryConstraint::constraint_len_recursive)
                    .sum::<u32>()
                    + 1
            }
        }
    }
}

// Since we are not doing floating point operations, this should be okay.
#[allow(clippy::float_cmp)]
#[cfg(test)]
mod test {
    use crate::worker::query::{EntityQuery, QueryConstraint, ResultType};
    use crate::worker::EntityId;
    use std::slice::from_raw_parts;

    use spatialos_sdk_sys::worker::{
        Worker_Constraint, Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_AND,
        Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_COMPONENT,
        Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_ENTITY_ID,
        Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_NOT,
        Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_OR,
        Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_SPHERE,
    };

    fn is_worker_query_valid(query: &EntityQuery) {
        let worker_query = query.to_worker_sdk();

        assert_eq!(query.result_type.to_u8(), worker_query.query.result_type);
        if let ResultType::Snapshot(ids) = &query.result_type {
            assert_eq!(
                ids.len() as u32,
                worker_query.query.snapshot_result_type_component_id_count
            );
            // TODO: Check the ids.
        }

        is_constraint_valid(&query.constraint, &worker_query.query.constraint);
    }

    fn is_constraint_valid(constraint: &QueryConstraint, worker_constraint: &Worker_Constraint) {
        match constraint {
            QueryConstraint::EntityId(id) => {
                assert_eq!(
                    Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_ENTITY_ID as u8,
                    worker_constraint.constraint_type
                );
                unsafe {
                    let bindgen_constraint = worker_constraint.constraint.entity_id_constraint;
                    assert_eq!(id.id, bindgen_constraint.entity_id);
                }
            }
            QueryConstraint::Component(component_id) => {
                assert_eq!(
                    Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_COMPONENT as u8,
                    worker_constraint.constraint_type
                );
                unsafe {
                    let bindgen_constraint = worker_constraint.constraint.component_constraint;
                    assert_eq!(*component_id, bindgen_constraint.component_id);
                }
            }
            QueryConstraint::Sphere(x, y, z, r) => {
                assert_eq!(
                    Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_SPHERE as u8,
                    worker_constraint.constraint_type
                );
                unsafe {
                    let bindgen_constraint = worker_constraint.constraint.sphere_constraint;
                    assert_eq!(*x, bindgen_constraint.x);
                    assert_eq!(*y, bindgen_constraint.y);
                    assert_eq!(*z, bindgen_constraint.z);
                    assert_eq!(*r, bindgen_constraint.radius);
                }
            }
            QueryConstraint::And(other_constraints) => {
                assert_eq!(
                    Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_AND as u8,
                    worker_constraint.constraint_type
                );
                unsafe {
                    let bindgen_constraint = worker_constraint.constraint.and_constraint;
                    assert_ne!(::std::ptr::null_mut(), bindgen_constraint.constraints);
                    assert_eq!(
                        other_constraints.len() as u32,
                        bindgen_constraint.constraint_count
                    );
                    let data = from_raw_parts(
                        bindgen_constraint.constraints,
                        bindgen_constraint.constraint_count as usize,
                    );
                    for i in 0..other_constraints.len() {
                        is_constraint_valid(&other_constraints[i], &data[i]);
                    }
                }
            }
            QueryConstraint::Or(other_constraints) => {
                assert_eq!(
                    Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_OR as u8,
                    worker_constraint.constraint_type
                );
                unsafe {
                    let bindgen_constraint = worker_constraint.constraint.or_constraint;
                    assert_ne!(::std::ptr::null_mut(), bindgen_constraint.constraints);
                    assert_eq!(
                        other_constraints.len() as u32,
                        bindgen_constraint.constraint_count
                    );
                    let data = from_raw_parts(
                        bindgen_constraint.constraints,
                        bindgen_constraint.constraint_count as usize,
                    );
                    for i in 0..other_constraints.len() {
                        is_constraint_valid(&other_constraints[i], &data[i]);
                    }
                }
            }
            QueryConstraint::Not(other_constraint) => {
                assert_eq!(
                    Worker_ConstraintType_WORKER_CONSTRAINT_TYPE_NOT as u8,
                    worker_constraint.constraint_type
                );
                unsafe {
                    let bindgen_constraint = worker_constraint.constraint.not_constraint;
                    assert_ne!(::std::ptr::null_mut(), bindgen_constraint.constraint);

                    is_constraint_valid(
                        other_constraint,
                        bindgen_constraint.constraint.as_ref().unwrap(),
                    );
                }
            }
        }
    }

    #[test]
    fn query_result_types_conversion() {
        let query = EntityQuery {
            constraint: QueryConstraint::Component(1),
            result_type: ResultType::Count,
        };

        is_worker_query_valid(&query);

        let query = EntityQuery {
            constraint: QueryConstraint::Component(1),
            result_type: ResultType::Snapshot(vec![0, 1, 2]),
        };

        is_worker_query_valid(&query);
    }

    #[test]
    fn exhaustive_simple_query_conversion() {
        let query = EntityQuery {
            constraint: QueryConstraint::Component(1),
            result_type: ResultType::Count,
        };

        is_worker_query_valid(&query);

        let query = EntityQuery {
            constraint: QueryConstraint::EntityId(EntityId::new(10)),
            result_type: ResultType::Count,
        };

        is_worker_query_valid(&query);

        let query = EntityQuery {
            constraint: QueryConstraint::Sphere(1.0, 1.0, 1.0, 1.0),
            result_type: ResultType::Count,
        };

        is_worker_query_valid(&query);
    }

    #[test]
    fn exhaustive_simple_nested_query_conversion() {
        let c1 = QueryConstraint::Component(1);
        let c2 = QueryConstraint::EntityId(EntityId::new(10));
        let constraints = vec![c1, c2];

        let query = EntityQuery {
            constraint: QueryConstraint::And(constraints.clone()),
            result_type: ResultType::Count,
        };

        is_worker_query_valid(&query);

        let query = EntityQuery {
            constraint: QueryConstraint::Or(constraints),
            result_type: ResultType::Count,
        };

        is_worker_query_valid(&query);

        let query = EntityQuery {
            constraint: QueryConstraint::Not(Box::new(QueryConstraint::EntityId(EntityId::new(
                10,
            )))),
            result_type: ResultType::Count,
        };

        is_worker_query_valid(&query);
    }

    #[test]
    fn complex_nested_query_conversion() {
        let c1 = QueryConstraint::Component(1);
        let c2 = QueryConstraint::EntityId(EntityId::new(10));
        let constraints = vec![c1, c2];

        let and = QueryConstraint::And(constraints.clone());
        let or = QueryConstraint::Or(vec![and, QueryConstraint::Component(5)]);

        let and = QueryConstraint::And(vec![
            or,
            QueryConstraint::Not(Box::new(QueryConstraint::Component(15))),
        ]);

        let query = EntityQuery {
            constraint: and,
            result_type: ResultType::Count,
        };

        is_worker_query_valid(&query);
    }
}
