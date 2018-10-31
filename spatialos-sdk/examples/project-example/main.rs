extern crate spatialos_sdk;
extern crate uuid;

use spatialos_sdk::worker::core::connection::{Connection, WorkerConnection};
use spatialos_sdk::worker::core::parameters;
use spatialos_sdk::worker::core::LogLevel;

use uuid::Uuid;

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
    loop {
        let ops = c.get_op_list(0);
        c.send_log_message(
            LogLevel::Info,
            "loop",
            &format!("Received {} ops", ops.ops.len()),
            None,
        );
        ::std::thread::sleep(::std::time::Duration::from_millis(500));
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
