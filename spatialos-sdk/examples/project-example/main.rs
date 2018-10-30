extern crate spatialos_sdk;

use spatialos_sdk::worker::core::connection::WorkerConnection;
use spatialos_sdk::worker::core::parameters;

fn main() {
    println!("Entered program");
    let mut connection_parameters = parameters::ReceptionistConnectionParameters {
        hostname: "127.0.0.1".to_owned(),
        port: 7777,
        connection_params: parameters::ConnectionParameters::default(),
    };

    connection_parameters.connection_params.worker_type = "RustWorker".to_owned();

    match get_connection_block(&connection_parameters) {
        Ok(_) => println!("Connected successful with block."),
        Err(e) => println!("Failed to connect with block: \n{}", e),
    }

    match get_connection_poll(&connection_parameters) {
        Ok(_) => println!("Connected successful with poll."),
        Err(e) => println!("Failed to connect with poll: \n{}", e),
    }
}

fn get_connection_block(
    params: &parameters::ReceptionistConnectionParameters,
) -> Result<WorkerConnection, String> {
    let future = WorkerConnection::connect_receptionist_async("test-worker", params);
    future.get()
}

/*
fn get_connection_poll(params: &parameters::ReceptionistConnectionParameters) -> Result<WorkerConnection, String> {
    const NUM_ATTEMPTS: u8 = 3;
    const TIME_BETWEEN_ATTEMPTS_MILLIS: u64 = 1000;
    
    let future = WorkerConnection::connect_receptionist_async(
        "test-worker",
        params,
    );
    
    let mut res: Option<WorkerConnection> = None;
    for _ in 0..NUM_ATTEMPTS {
        if let Some(c) = future.poll(100) {
            res = Some(c);
            break;
        }
        ::std::thread::sleep(::std::time::Duration::from_millis(TIME_BETWEEN_ATTEMPTS_MILLIS));
    }
    
    match res {
        Some(c) => Ok(c),
        None => Err("Max connection attempts failed.".to_owned())
    }
}
*/
