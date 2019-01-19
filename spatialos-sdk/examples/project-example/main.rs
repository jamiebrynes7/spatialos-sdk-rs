extern crate spatialos_sdk;

mod lib;
use crate::lib::{get_connection, get_worker_configuration, CommandType};

use spatialos_sdk::worker::commands::{
    DeleteEntityRequest, EntityQueryRequest, ReserveEntityIdsRequest,
};
use spatialos_sdk::worker::connection::{Connection, WorkerConnection};
use spatialos_sdk::worker::metrics::{HistogramMetric, Metrics};
use spatialos_sdk::worker::query::{EntityQuery, QueryConstraint, ResultType};
use spatialos_sdk::worker::{EntityId, InterestOverride, LogLevel};

fn main() {
    println!("Entered program");

    let config = match get_worker_configuration() {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };

    let config = match config {
        CommandType::Worker(config) => config,
        CommandType::Setup { spatial_lib_dir, out_dir } => {
            do_setup(spatial_lib_dir, out_dir);
            return;
        }
    };

    let worker_connection = match get_connection(config) {
        Ok(c) => c,
        Err(e) => panic!("{}", e),
    };

    println!("Connected as: {}", worker_connection.get_worker_id());

    exercise_connection_code_paths(worker_connection);
}

fn exercise_connection_code_paths(mut c: WorkerConnection) {
    c.send_log_message(LogLevel::Info, "main", "Connected successfully!", None);
    print_worker_attributes(&c);
    check_for_flag(&c, "my-flag");

    let _ = c.get_op_list(0);
    c.send_reserve_entity_ids_request(ReserveEntityIdsRequest(1), None);
    c.send_delete_entity_request(DeleteEntityRequest(EntityId::new(1)), None);
    // TODO: Send create entity command
    send_query(&mut c);

    let interested = vec![
        InterestOverride::new(1, true),
        InterestOverride::new(100, false),
    ];
    c.send_component_interest(EntityId::new(1), &interested);
    c.send_authority_loss_imminent_acknowledgement(EntityId::new(1), 1337);

    send_metrics(&mut c);
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

fn do_setup(spatial_lib_dir: std::path::PathBuf, out_dir: std::path::PathBuf) {
    use std::fs;
    use fs_extra::dir::{self, CopyOptions};

    // Determine the paths the the schema compiler and protoc relative the the lib
    // dir path.
    let schema_compiler_path = spatial_lib_dir.join("schema-compiler/schema_compiler");
    let protoc_path = spatial_lib_dir.join("schema-compiler/protoc");

    // Calculate the various output directories relative to `out_dir`.
    let bin_path = out_dir.join("spatialos/schema/bin");
    let tmp_path = out_dir.join("tmp");

    // Create the output directories if they don't already exist.
    fs::create_dir_all(&bin_path).expect("Failed to crate spatialos/schema/bin");
    fs::create_dir_all(&tmp_path).expect("Failed to create tmp");

    dir::copy(spatial_lib_dir.join("schema-compiler/proto"), &tmp_path, &CopyOptions::new()).expect("Failed to copy contents of schema-compiler/proto");
    // let proto_glob = spatial_lib_dir.join("schema-compiler/proto/*");
    // for entry in glob::glob(proto_glob.to_str().unwrap()).unwrap().filter_map(Result::ok) {
    //     dbg!(entry);
    //     fs_extra::dir::copy(entry, )
    // }
}
