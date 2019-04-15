use crate::{connection_handler::*, opt::*};
use generated::{example, improbable};
use rand::Rng;
use spatialos_sdk::worker::{
    commands::{EntityQueryRequest, ReserveEntityIdsRequest},
    component::{UpdateParameters},
    connection::{Connection, WorkerConnection},
    entity_builder::EntityBuilder,
    metrics::{HistogramMetric, Metrics},
    query::{EntityQuery, QueryConstraint, ResultType},
    view::{View, ViewQuery},
    {EntityId, InterestOverride, LogLevel},
};
use std::{
    f64,
    time::{Duration, SystemTime},
};
use structopt::StructOpt;
use tap::*;

mod connection_handler;
#[rustfmt::skip]
mod generated;
mod opt;

fn main() {
    let opt = Opt::from_args();
    let mut worker_connection = match get_connection(opt) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };

    println!("Connected as: {}", worker_connection.get_worker_id());

    exercise_connection_code_paths(&mut worker_connection);
    create_entities(&mut worker_connection, 5);
    logic_loop(&mut worker_connection);
}

struct RotatorQuery<'a> {
    pub id: EntityId,
    pub position: &'a improbable::Position,
    pub rotate: &'a example::Rotate,
}

impl<'a, 'b: 'a> ViewQuery<'b> for RotatorQuery<'a> {
    fn filter(view: &View, entity_id: &EntityId) -> bool {
        view.is_authoritative::<improbable::Position>(entity_id)
            && view.is_authoritative::<example::Rotate>(entity_id)
    }

    fn select(view: &'b View, entity_id: EntityId) -> RotatorQuery<'a> {
        RotatorQuery {
            id: entity_id.clone(),
            position: view.get_component::<improbable::Position>(&entity_id).unwrap(),
            rotate: view.get_component::<example::Rotate>(&entity_id).unwrap(),
        }
    }
}

fn logic_loop(c: &mut WorkerConnection) {
    let mut rng = rand::thread_rng();

    let mut builder = EntityBuilder::new(0.0, 0.0, 0.0, "rusty");

    builder.add_component(
        example::Rotate {
            angle: rng.gen_range(0.0, 2.0 * f64::consts::PI),
            radius: rng.gen_range(20.0, 100.0),
            center: improbable::Vector3d {
                x: rng.gen_range(-50.0, 50.0),
                y: 0.0,
                z: rng.gen_range(-50.0, 50.0),
            },
        },
        "rusty",
    );
    builder.set_metadata("Rotator", "rusty");
    builder.set_entity_acl_write_access("rusty");

    let entity = builder.build().unwrap();

    let create_request_id = c.send_create_entity_request(entity, None, None);
    println!("Create entity request ID: {:?}", create_request_id);


    let mut view = View::new();
    let update_params = UpdateParameters::default().tap(|params| params.allow_loopback());

    let mut fps_tracker = FpsTracker::new(10);
    let mut metrics = Metrics::new();

    loop {
        fps_tracker.record();
        view.process_ops(&c.get_op_list(0));

        for RotatorQuery {
            id,
            position: _,
            rotate,
        } in view.query::<RotatorQuery>()
        {
            c.send_component_update::<example::Rotate>(
                id.clone(),
                example::RotateUpdate {
                    angle: Some(rotate.angle + f64::consts::PI * 2.0 / 200.0),
                    ..Default::default()
                },
                update_params,
            );

            c.send_component_update::<improbable::Position>(
                id.clone(),
                improbable::PositionUpdate {
                    coords: Some(improbable::Coordinates {
                        x: rotate.angle.sin() * rotate.radius + rotate.center.x,
                        y: rotate.center.y,
                        z: rotate.angle.cos() * rotate.radius + rotate.center.z,
                    }),
                },
                update_params,
            );
        }

        let load = (60.0 - fps_tracker.get_fps()) / 30.0;
        metrics.load = Some(if load < 0.0 { 0.0 } else { load } as f64);
        c.send_metrics(&metrics);

        // Run the main loop at approximately 60 fps.
        ::std::thread::sleep(::std::time::Duration::from_millis(16));
    }
}

fn exercise_connection_code_paths(c: &mut WorkerConnection) {
    c.send_log_message(LogLevel::Info, "main", "Connected successfully!", None);
    print_worker_attributes(&c);
    check_for_flag(c, "my-flag");

    let _ = c.get_op_list(0);
    c.send_reserve_entity_ids_request(ReserveEntityIdsRequest(1), None);
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

fn create_entities(c: &mut WorkerConnection, number: u32) {
    let mut rng = rand::thread_rng();

    for _ in 0..number {
        let mut builder = EntityBuilder::new(0.0, 0.0, 0.0, "rusty");

        builder.add_component(example::Rotate {
            angle: rng.gen_range(0.0, 2.0 * f64::consts::PI),
            radius: rng.gen_range(20.0, 100.0),
            center: improbable::Vector3d {
                x: rng.gen_range(-50.0, 50.0),
                y: 0.0,
                z: rng.gen_range(-50.0, 50.0),
            }
        }, "rusty");

        builder.set_entity_acl_write_access("rusty");

        let entity = builder.build().unwrap();

        let create_request_id = c.send_create_entity_request(entity, None, None);
        println!("Create entity request ID: {:?}", create_request_id);
    }
}

struct FpsTracker {
    measurements: Vec<Duration>,
    max_measurements: usize,
    last: SystemTime,
}

impl FpsTracker {
    pub fn new(max: usize) -> Self {
        FpsTracker {
            measurements: Vec::new(),
            max_measurements: max,
            last: SystemTime::now(),
        }
    }

    pub fn record(&mut self) {
        let now = SystemTime::now();
        let diff = now.duration_since(self.last).expect("Error");
        self.measurements.push(diff);

        if self.measurements.len() > self.max_measurements {
            self.measurements.remove(0);
        }

        self.last = now;
    }

    pub fn get_fps(&self) -> f64 {
        if self.measurements.is_empty() {
            return 0.0;
        }

        let sum = self
            .measurements
            .iter()
            .map(|duration| 1.0 / (f64::from(duration.subsec_micros()) / 1_000_000.0))
            .fold(0.0, |sum, next| sum + next);

        sum / self.measurements.len() as f64
    }
}
