extern crate uuid;
use self::uuid::Uuid;

use spatialos_sdk::worker::connection::WorkerConnection;
use spatialos_sdk::worker::connection::WorkerConnectionFuture;
use spatialos_sdk::worker::locator::Locator;

use lib::argument_parsing::ConnectionType;
use lib::argument_parsing::WorkerConfiguration;

static LOCATOR_HOSTNAME: &str = "locator.improbable.io";

static POLL_NUM_ATTEMPTS: u32 = 5;
static POLL_TIME_BETWEEN_ATTEMPTS_MILLIS: u64 = 3000;

pub fn get_connection(configuration: WorkerConfiguration) -> Result<WorkerConnection, String> {
    let worker_id = get_worker_id(&configuration);

    let mut future = match configuration.connection_type {
        ConnectionType::Receptionist(host, port) => WorkerConnection::connect_receptionist_async(
            &worker_id,
            &host,
            port,
            &configuration.connection_params,
        ),
        ConnectionType::Locator(params) => {
            let locator = Locator::new(LOCATOR_HOSTNAME, &params);
            let deployment = get_deployment(&locator)?;
            WorkerConnection::connect_locator_async(
                &locator,
                &deployment,
                &configuration.connection_params,
                queue_status_callback,
            )
        }
    };

    match configuration.connect_with_poll {
        true => get_connection_poll(&mut future),
        false => future.get(),
    }
}

fn get_worker_id(config: &WorkerConfiguration) -> String {
    let worker_uuid = Uuid::new_v4();
    format!(
        "{}-{}",
        config.connection_params.worker_type,
        worker_uuid.to_string()
    )
}

fn queue_status_callback(_queue_status: Result<u32, String>) -> bool {
    true
}

fn get_deployment(locator: &Locator) -> Result<String, String> {
    let mut deployment_list_future = locator.get_deployment_list_async();
    let deployment_list = deployment_list_future.get()?;

    if deployment_list.len() == 0 {
        return Err("No deployments could be found!".to_owned());
    }

    Ok(deployment_list[0].deployment_name.clone())
}

fn get_connection_poll(future: &mut WorkerConnectionFuture) -> Result<WorkerConnection, String> {
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
