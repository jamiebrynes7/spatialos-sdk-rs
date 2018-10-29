extern crate spatialos_sdk;

use spatialos_sdk::worker::core::connection;
use spatialos_sdk::worker::core::parameters;

fn main() {
    println!("Entered program");
    let mut connection_parameters = parameters::ReceptionistConnectionParameters {
        hostname: "127.0.0.1".to_owned(),
        port: 7777,
        connection_params: parameters::ConnectionParameters::default(),
    };

    connection_parameters.connection_params.worker_type = "RustWorker".to_owned();

    let future = connection::WorkerConnection::connect_receptionist_async(
        "test-worker",
        &mut connection_parameters,
    );
    let connection = match future.get(0) {
        Some(c) => {
            println!("Connected!");
            c
        }
        None => panic!("Could not connect to the SpatialOS runtime"),
    };
}
