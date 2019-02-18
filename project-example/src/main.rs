use crate::{connection_handler::*, opt::*};
use generated::{example, improbable};
use spatialos_sdk::worker::{
    commands::{EntityQueryRequest, ReserveEntityIdsRequest},
    component::{Component, ComponentDatabase},
    connection::{Connection, WorkerConnection},
    entity::Entity,
    metrics::{HistogramMetric, Metrics},
    op::{StatusCode, WorkerOp},
    parameters::UpdateParameters,
    query::{EntityQuery, QueryConstraint, ResultType},
    {EntityId, InterestOverride, LogLevel},
};
use std::{
    collections::{BTreeMap, HashMap},
    f64,
};
use structopt::StructOpt;
use tap::*;

mod connection_handler;
mod generated;
mod opt;

fn main() {
    println!("Entered program");

    let components = ComponentDatabase::new()
        .add_component::<example::Example>()
        .add_component::<example::Rotate>()
        .add_component::<improbable::EntityAcl>()
        .add_component::<improbable::Persistence>()
        .add_component::<improbable::Metadata>()
        .add_component::<improbable::Interest>()
        .add_component::<improbable::Position>();

    let opt = Opt::from_args();
    let mut worker_connection = match get_connection(opt, components) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };

    println!("Connected as: {}", worker_connection.get_worker_id());

    exercise_connection_code_paths(&mut worker_connection);
    logic_loop(&mut worker_connection);
}

fn logic_loop(c: &mut WorkerConnection) {
    let mut world = HashMap::new();

    loop {
        let ops = c.get_op_list(0);

        // Process ops.
        for op in &ops {
            if let WorkerOp::Metrics(_) = op {
                println!("Received metrics.");
            } else {
                println!("Received op: {:?}", op);
            }

            match op {
                WorkerOp::AddEntity(add_entity_op) => {
                    world.insert(add_entity_op.entity_id, EntityState::default());
                }

                WorkerOp::RemoveEntity(remove_entity_op) => {
                    world.remove(&remove_entity_op.entity_id);
                }

                WorkerOp::AddComponent(add_component) => match add_component.component_id {
                    example::Rotate::ID => {
                        let rotate = add_component.get::<example::Rotate>().unwrap();
                        let entity_state = world
                            .get_mut(&add_component.entity_id)
                            .expect("Entity wasn't present in local world");
                        entity_state.rotate = Some(rotate.clone());
                    }
                    id => println!("Received unknown component: {}", id),
                },

                WorkerOp::AuthorityChange(authority_change) => {
                    if authority_change.component_id == example::Rotate::ID {
                        eprintln!(
                            "Authority change for {}: {:?}",
                            c.get_worker_id(),
                            authority_change
                        );
                        let state = world.get_mut(&authority_change.entity_id).unwrap();
                        state.has_authority = authority_change.authority.has_authority();
                    }
                }

                WorkerOp::ReserveEntityIdsResponse(response) => {
                    if let StatusCode::Success(response_data) = response.status_code {
                        let mut entity = Entity::new();
                        entity.add(improbable::Position {
                            coords: improbable::Coordinates {
                                x: 0.0,
                                y: 0.0,
                                z: 0.0,
                            },
                        });
                        entity.add(example::Rotate {
                            angle: 0.0,
                            radius: 10.0,
                            center_x: -7.0,
                            center_y: 0.0,
                            center_z: 13.0,
                        });
                        entity.add(improbable::EntityAcl {
                            read_acl: improbable::WorkerRequirementSet {
                                attribute_set: vec![improbable::WorkerAttributeSet {
                                    attribute: vec!["rusty".into()],
                                }],
                            },
                            component_write_acl: BTreeMap::new().tap(|writes| {
                                writes.insert(
                                    improbable::Position::ID,
                                    improbable::WorkerRequirementSet {
                                        attribute_set: vec![improbable::WorkerAttributeSet {
                                            attribute: vec!["rusty".into()],
                                        }],
                                    },
                                );
                                writes.insert(
                                    example::Rotate::ID,
                                    improbable::WorkerRequirementSet {
                                        attribute_set: vec![improbable::WorkerAttributeSet {
                                            attribute: vec!["rusty".into()],
                                        }],
                                    },
                                );
                            }),
                        });
                        let create_request_id = c.send_create_entity_request(
                            entity,
                            Some(response_data.first_entity_id),
                            None,
                        );
                        println!("Create entity request ID: {:?}", create_request_id);
                    }
                }
                WorkerOp::ComponentUpdate(update) => match update.component_id {
                    example::Example::ID => {
                        let component_update = update.get::<example::Example>();
                        println!("Received Example update: {:?}", component_update)
                    }
                    id => println!("Received unknown component: {}", id),
                },
                _ => {}
            }
        }

        // Update the rotation of all rotation components that we have control over.
        for (&entity_id, entity_state) in &mut world {
            // Ignore any entities where we don't have authority over the `Rotate` component.
            if !entity_state.has_authority {
                continue;
            }

            // Only update entities that have a `Rotate` component.
            if let Some(rotate) = &mut entity_state.rotate {
                rotate.angle += f64::consts::PI * 2.0 / 20.0;
                eprintln!("{} - {}", c.get_worker_id(), rotate.angle);

                c.send_component_update::<example::Rotate>(
                    entity_id,
                    example::RotateUpdate {
                        angle: Some(rotate.angle),
                        ..Default::default()
                    },
                    UpdateParameters { loopback: true },
                );

                c.send_component_update::<improbable::Position>(
                    entity_id,
                    improbable::PositionUpdate {
                        coords: Some(improbable::Coordinates {
                            x: rotate.angle.sin() * rotate.radius + rotate.center_x,
                            y: rotate.center_y,
                            z: rotate.angle.cos() * rotate.radius + rotate.center_z,
                        }),
                    },
                    UpdateParameters { loopback: true },
                );
            }

            println!(
                "Sending component update for improbable::Position to entity {:?}.",
                entity_id
            );
        }

        ::std::thread::sleep(::std::time::Duration::from_millis(500));
    }
}

fn exercise_connection_code_paths(c: &mut WorkerConnection) {
    c.send_log_message(LogLevel::Info, "main", "Connected successfully!", None);
    print_worker_attributes(&c);
    check_for_flag(c, "my-flag");

    let _ = c.get_op_list(0);
    c.send_reserve_entity_ids_request(ReserveEntityIdsRequest(1), None);
    //c.send_delete_entity_request(DeleteEntityRequest(EntityId::new(1)), None);
    send_query(c);

    let interested = vec![
        InterestOverride::new(1, true),
        InterestOverride::new(100, false),
    ];
    c.send_component_interest(EntityId::new(1), &interested);
    c.send_authority_loss_imminent_acknowledgement(EntityId::new(1), 1337);

    send_metrics(c);
    c.set_protocol_logging_enabled(false);

    println!("Testing completed");
}

fn print_worker_attributes(connection: &WorkerConnection) {
    let attrs = connection.get_worker_attributes();
    println!("The worker has the following attributes: ");
    for attr in attrs {
        println!("{}", attr)
    }
}

fn check_for_flag(connection: &mut WorkerConnection, flag_name: &str) {
    let flag = connection.get_worker_flag(flag_name);
    match flag {
        Some(f) => println!("Found flag value: {}", f),
        None => println!("Could not find flag value"),
    }
}

fn send_query(c: &mut WorkerConnection) {
    let query = EntityQuery::new(
        QueryConstraint::And(vec![
            QueryConstraint::Or(vec![
                QueryConstraint::Component(0),
                QueryConstraint::Component(1),
            ]),
            QueryConstraint::And(vec![
                QueryConstraint::Sphere(10.0, 10.0, 10.0, 250.0),
                QueryConstraint::Not(Box::new(QueryConstraint::Component(2))),
            ]),
            QueryConstraint::EntityId(EntityId::new(10)),
        ]),
        ResultType::Count,
    );

    c.send_entity_query_request(EntityQueryRequest(query), None);
}

fn send_metrics(c: &mut WorkerConnection) {
    let mut m = Metrics::new()
        .with_load(0.2)
        .with_gauge_metric("some_metric", 0.15)
        .with_histogram_metric("histogram_metric", HistogramMetric::new(&[6.7]));

    let gauge_metric = m.add_gauge_metric("another_metric").unwrap();
    *gauge_metric = 0.2;

    let histogram_metric = m
        .add_histogram_metric("another_histogram", &[0.1, 0.2, 0.3])
        .unwrap();
    histogram_metric.add_sample(1.0);
    histogram_metric.add_sample(0.5);

    c.send_metrics(&m);
}

#[derive(Debug, Default)]
struct EntityState {
    has_authority: bool,
    rotate: Option<example::Rotate>,
}
