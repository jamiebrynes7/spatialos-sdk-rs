use crate::lib::{get_connection, get_worker_configuration};
use generated_code::example::Example;
use spatialos_sdk::worker::commands::{
    DeleteEntityRequest, EntityQueryRequest, ReserveEntityIdsRequest,
};
use spatialos_sdk::worker::component::{Component, ComponentDatabase};
use spatialos_sdk::worker::connection::{Connection, WorkerConnection};
use spatialos_sdk::worker::entity::Entity;
use spatialos_sdk::worker::metrics::{HistogramMetric, Metrics};
use spatialos_sdk::worker::op::WorkerOp;
use spatialos_sdk::worker::query::{EntityQuery, QueryConstraint, ResultType};
use spatialos_sdk::worker::{EntityId, InterestOverride, LogLevel};

mod generated_code;
mod lib;

use generated_code::example;

fn main() {
    println!("Entered program");

    let components = ComponentDatabase::new()
        .add_component::<generated_code::example::Example>()
        .add_component::<generated_code::improbable::Position>();
    let config = match get_worker_configuration(components) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };
    let mut worker_connection = match get_connection(config) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };

    println!("Connected as: {}", worker_connection.get_worker_id());

    exercise_connection_code_paths(&mut worker_connection);
    logic_loop(&mut worker_connection);
}

fn logic_loop(c: &mut WorkerConnection) {
    let mut counter = 0;

    loop {
        let ops = c.get_op_list(0);

        // Process ops.
        for op in &ops {
            println!("Received op: {:?}", op);
            match op {
                WorkerOp::AddComponent(add_component) => match add_component.component_id {
                    example::Example::ID => {
                        let component_data = add_component.get::<Example>().unwrap();
                        println!("Received Example data: {:?}", component_data);
                    }
                    id => println!("Received unknown component: {}", id),
                },
                WorkerOp::ComponentUpdate(update) => match update.component_id {
                    example::Example::ID => {
                        let component_update = update.get::<Example>();
                        println!("Received Example update: {:?}", component_update)
                    }
                    id => println!("Received unknown component: {}", id),
                },
                _ => {}
            }
        }

        ::std::thread::sleep(::std::time::Duration::from_millis(500));

        if counter % 20 == 0 {
            println!("Sending reserve entity ids request");
            c.send_reserve_entity_ids_request(ReserveEntityIdsRequest(1), None);
        }
        counter += 1;
    }
}

fn exercise_connection_code_paths(c: &mut WorkerConnection) {
    c.send_log_message(LogLevel::Info, "main", "Connected successfully!", None);
    print_worker_attributes(&c);
    check_for_flag(&c, "my-flag");

    let _ = c.get_op_list(0);
    c.send_reserve_entity_ids_request(ReserveEntityIdsRequest(1), None);
    c.send_delete_entity_request(DeleteEntityRequest(EntityId::new(1)), None);
    // TODO: Send create entity command
    send_query(c);

    let interested = vec![
        InterestOverride::new(1, true),
        InterestOverride::new(100, false),
    ];
    c.send_component_interest(EntityId::new(1), &interested);
    c.send_authority_loss_imminent_acknowledgement(EntityId::new(1), 1337);

    send_metrics(c);
    c.set_protocol_logging_enabled(false);

    let mut entity = Entity::new();
    entity.add(generated_code::improbable::Position {
        coords: generated_code::improbable::Coordinates {
            x: 10.0,
            y: 12.0,
            z: 0.0,
        },
    });
    let create_request_id = c.send_create_entity_request(entity, None, None);
    dbg!(create_request_id);

    println!("Testing completed");
}

fn print_worker_attributes(connection: &WorkerConnection) {
    let attrs = connection.get_worker_attributes();
    println!("The worker has the following attributes: ");
    for attr in attrs {
        println!("{}", attr)
    }
}

fn check_for_flag(connection: &WorkerConnection, flag_name: &str) {
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
