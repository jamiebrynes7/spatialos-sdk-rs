extern crate spatialos_sdk;
extern crate uuid;

use spatialos_sdk::worker::commands::{
    DeleteEntityRequest, EntityQueryRequest, ReserveEntityIdsRequest,
};
use spatialos_sdk::worker::connection::{Connection, WorkerConnection};
use spatialos_sdk::worker::parameters;
use spatialos_sdk::worker::query::{EntityQuery, QueryConstraint, ResultType};
use spatialos_sdk::worker::{EntityId, InterestOverride, LogLevel};

use spatialos_sdk::worker::metrics::*;
use uuid::Uuid;

static HOST: &str = "127.0.0.1";
static PORT: u16 = 7777;

fn main() {
    println!("Entered program");

    let connection_params = parameters::ConnectionParameters::new("RustWorker").using_tcp();

    let worker_id = get_worker_id();

    let mut worker_connection = match get_connection_block(&connection_params, &worker_id) {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect with block: \n{}", e),
    };

    /*
    let mut worker_connection = match get_connection_poll(&connection_parameters, &worker_id) {
        Ok(c) => {
            println!("Connected successful with poll.");
            c
        },
        Err(e) => panic!("Failed to connect with poll: \n{}", e),
    };
    */

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
        InterestOverride {
            is_interested: true,
            component_id: 1,
        },
        InterestOverride {
            is_interested: false,
            component_id: 100,
        },
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

fn get_worker_id() -> String {
    let worker_uuid = Uuid::new_v4();
    let mut worker_id = String::from("RustWorker-");
    worker_id.push_str(&worker_uuid.to_string());

    worker_id
}

fn check_for_flag(connection: &WorkerConnection, flag_name: &str) {
    let flag = connection.get_worker_flag(flag_name);
    match flag {
        Some(f) => println!("Found flag value: {}", f),
        None => println!("Could not find flag value"),
    }
}

fn get_connection_block(
    params: &parameters::ConnectionParameters,
    worker_id: &str,
) -> Result<WorkerConnection, String> {
    let mut future = WorkerConnection::connect_receptionist_async(worker_id, HOST, PORT, params);
    future.get()
}

fn get_connection_poll(
    params: &parameters::ConnectionParameters,
    worker_id: &str,
) -> Result<WorkerConnection, String> {
    const NUM_ATTEMPTS: u8 = 3;
    const TIME_BETWEEN_ATTEMPTS_MILLIS: u64 = 1000;

    let mut future = WorkerConnection::connect_receptionist_async(worker_id, HOST, PORT, params);

    let mut res: Option<WorkerConnection> = None;
    let mut err: Option<String> = None;
    for _ in 0..NUM_ATTEMPTS {
        println!("Attempting to poll");
        match future.poll(100) {
            Some(r) => {
                match r {
                    Ok(c) => res = Some(c),
                    Err(e) => err = Some(e),
                };
                break;
            }
            None => {}
        };
        ::std::thread::sleep(::std::time::Duration::from_millis(
            TIME_BETWEEN_ATTEMPTS_MILLIS,
        ));
    }

    match err {
        Some(e) => Err(e),
        None => match res {
            Some(c) => Ok(c),
            None => Err("Max connection attempts failed.".to_owned()),
        },
    }
}

fn send_query(c: &mut WorkerConnection) {
    let c1 = QueryConstraint::Component(0);
    let c2 = QueryConstraint::Component(1);
    let c3 = QueryConstraint::Component(2);
    let c4 = QueryConstraint::Component(3);
    let c5 = QueryConstraint::Component(4);
    let c6 = QueryConstraint::Sphere(10.0, 10.0, 10.0, 250.0);

    let or = QueryConstraint::Or(vec![c1, c2]);
    let not = QueryConstraint::Not(Box::new(c3));
    let and_not = QueryConstraint::And(vec![c6, not]);

    let final_constraint = QueryConstraint::And(vec![or, and_not, c4, c5]);
    let query = EntityQuery {
        constraint: final_constraint,
        result_type: ResultType::Count,
    };

    c.send_entity_query_request(EntityQueryRequest(query), None);
}

fn send_metrics(c: &mut WorkerConnection) {
    let m = Metrics {
        load: Some(0.2),
        gauge_metrics: vec![
            GaugeMetric {
                key: "some metric".to_owned(),
                value: 0.15,
            },
            GaugeMetric {
                key: "another metric".to_owned(),
                value: 0.2,
            },
        ],
        histogram_metrics: vec![HistogramMetric {
            key: "yet another metric".to_owned(),
            sum: 2.0,
            buckets: vec![HistogramMetricBucket {
                upper_bound: 6.7,
                samples: 2,
            }],
        }],
    };

    c.send_metrics(&m);
}
