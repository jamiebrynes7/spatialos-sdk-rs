extern crate spatialos_sdk;
extern crate uuid;

use spatialos_sdk::worker::core::commands::ReserveEntityIdsRequest;
use spatialos_sdk::worker::core::connection::{Connection, WorkerConnection};
use spatialos_sdk::worker::core::parameters;
use spatialos_sdk::worker::core::query::{EntityQuery, ResultType, QueryConstraint};
use spatialos_sdk::worker::core::LogLevel;

use uuid::Uuid;
use spatialos_sdk::worker::core::commands::EntityQueryRequest;

fn main() {
    println!("Entered program");
    let mut connection_parameters = parameters::ReceptionistConnectionParameters {
        hostname: "127.0.0.1".to_owned(),
        port: 7777,
        connection_params: parameters::ConnectionParameters::default(),
    };
    connection_parameters.connection_params.worker_type = "RustWorker".to_owned();

    let worker_id = get_worker_id();

    let mut worker_connection = match get_connection_block(&connection_parameters, &worker_id) {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect with block: \n{}", e),
    };

    worker_connection.send_log_message(LogLevel::Info, "main", "Connected successfully!", None);

    logic_loop(worker_connection);

    /*
    match get_connection_poll(&connection_parameters) {
        Ok(_) => println!("Connected successful with poll."),
        Err(e) => println!("Failed to connect with poll: \n{}", e),
    }
    */
}

fn logic_loop(mut c: WorkerConnection) {
    let mut counter = 0;

    loop {
        let ops = c.get_op_list(0);
        c.send_log_message(
            LogLevel::Info,
            "loop",
            &format!("Received {} ops", ops.ops.len()),
            None,
        );
        ::std::thread::sleep(::std::time::Duration::from_millis(500));

        if counter % 20 == 0 {
            println!("Sending reserve entity ids request");
            c.send_reserve_entity_ids_request(ReserveEntityIdsRequest(1), None);
            println!("Sending query");
            send_query(&mut c);
        }
        counter += 1;
    }
}

fn get_worker_id() -> String {
    let worker_uuid = Uuid::new_v4();
    let mut worker_id = String::from("RustWorker-");
    worker_id.push_str(&worker_uuid.to_string());

    worker_id
}

fn get_connection_block(
    params: &parameters::ReceptionistConnectionParameters,
    worker_id: &str,
) -> Result<WorkerConnection, String> {
    let mut future = WorkerConnection::connect_receptionist_async(worker_id, params);
    future.get()
}

fn get_connection_poll(
    params: &parameters::ReceptionistConnectionParameters,
) -> Result<WorkerConnection, String> {
    const NUM_ATTEMPTS: u8 = 3;
    const TIME_BETWEEN_ATTEMPTS_MILLIS: u64 = 1000;

    let mut future = WorkerConnection::connect_receptionist_async("test-worker", params);

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
    let c6 = QueryConstraint::Sphere(0.0,0.0,0.0,0.0);

    let or = QueryConstraint::Or(vec![c1,c2]);
    let not = QueryConstraint::Not(Box::new(c3));
    let and_not = QueryConstraint::And(vec![c6, not]);

    let final_constraint = QueryConstraint::And(vec![or, and_not, c4, c5]);
    let query = EntityQuery {
        constraint: final_constraint,
        result_type: ResultType::Count
    };

    c.send_entity_query_request(EntityQueryRequest(query), None);
}