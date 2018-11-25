extern crate uuid;
use self::uuid::Uuid;

use spatialos_sdk::worker::connection::WorkerConnection;
use spatialos_sdk::worker::locator::Locator;
use spatialos_sdk::worker::connection::WorkerConnectionFuture;

use lib::argument_parsing::WorkerConfiguration;
use lib::argument_parsing::ConnectionType;

static LOCATOR_HOSTNAME: &str = "locator.improbable.io";

static POLL_NUM_ATTEMPTS: u32 = 5;
static POLL_TIME_BETWEEN_ATTEMPTS_MILLIS: u64 = 3000;

pub fn get_connection(configuration: WorkerConfiguration) -> Result<WorkerConnection, String> {
    let worker_id = get_worker_id();

    let mut future = match configuration.connection_type {
        ConnectionType::Receptionist(host, port) => {
            WorkerConnection::connect_receptionist_async(&worker_id, &host, port, &configuration.connection_params)
        }
        ConnectionType::Locator(params) => {
            let locator = Locator::new(LOCATOR_HOSTNAME, &params);
            let deployment = get_deployment(&locator);
            WorkerConnection::connect_locator_async(&locator, &deployment, &configuration.connection_params, queue_status_callback)
        }
    };

    match configuration.connect_with_poll {
        true => get_connection_poll(&mut future),
        false => future.get()
    }
}

fn get_worker_id() -> String {
    let worker_uuid = Uuid::new_v4();
    let mut worker_id = String::from("RustWorker-");
    worker_id.push_str(&worker_uuid.to_string());
    worker_id
}

fn queue_status_callback(queue_status: Result<u32, String>) -> bool {
    true
}

fn get_deployment(locator: &Locator) -> String {
    let mut deployment_list_future = locator.get_deployment_list_async();
    let deployment_list = deployment_list_future.get();
    match deployment_list {
        Ok(deployments) => {
            if deployments.len() == 0 {
                panic!("No deployments could be found!");
            }

            deployments[0].deployment_name.clone()
        },
        Err(e) => panic!("{}", e)
    }
}

fn get_connection_poll(
    future: &mut WorkerConnectionFuture
) -> Result<WorkerConnection, String> {
    let mut res: Option<WorkerConnection> = None;
    let mut err: Option<String> = None;
    for _ in 0..POLL_NUM_ATTEMPTS {
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
            POLL_TIME_BETWEEN_ATTEMPTS_MILLIS,
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