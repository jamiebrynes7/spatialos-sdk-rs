use futures::{Async, Future};

use crate::lib::{Command, Opt};
use spatialos_sdk::worker::{
    component::ComponentDatabase,
    connection::{WorkerConnection, WorkerConnectionFuture},
    locator::{Locator, LocatorCredentials, LocatorParameters},
    parameters::ConnectionParameters,
};
use uuid::Uuid;

static LOCATOR_HOSTNAME: &str = "locator.improbable.io";

static POLL_NUM_ATTEMPTS: u32 = 5;
static POLL_TIME_BETWEEN_ATTEMPTS_MILLIS: u64 = 3000;

pub fn get_connection(opt: Opt, components: ComponentDatabase) -> Result<WorkerConnection, String> {
    let Opt {
        worker_type,
        worker_id,
        connect_with_poll,
        command,
    } = opt;

    let worker_id = worker_id.unwrap_or_else(|| format!("{}-{}", &worker_type, Uuid::new_v4()));
    let mut future = match command {
        Command::Receptionist {
            host,
            port,
            connect_with_external_ip,
        } => {
            let params = ConnectionParameters::new(worker_type, components)
                .using_tcp()
                .using_external_ip(connect_with_external_ip);
            WorkerConnection::connect_receptionist_async(
                &worker_id,
                &host.unwrap_or_else(|| "127.0.0.1".into()),
                port.unwrap_or(7777),
                &params,
            )
        }

        Command::Locator {
            token,
            project_name,
        } => {
            let params =
                LocatorParameters::new(project_name, LocatorCredentials::LoginToken(token));
            let locator = Locator::new(LOCATOR_HOSTNAME, &params);
            let deployment = get_deployment(&locator)?;
            WorkerConnection::connect_locator_async(
                &locator,
                &deployment,
                &ConnectionParameters::new(worker_type, components)
                    .using_tcp()
                    .using_external_ip(true),
                queue_status_callback,
            )
        }
    };

    if connect_with_poll {
        get_connection_poll(&mut future)
    } else {
        future.wait()
    }
}

fn queue_status_callback(_queue_status: &Result<u32, String>) -> bool {
    true
}

fn get_deployment(locator: &Locator) -> Result<String, String> {
    let deployment_list_future = locator.get_deployment_list_async();
    let deployment_list = deployment_list_future.wait()?;

    if deployment_list.is_empty() {
        return Err("No deployments could be found!".to_owned());
    }

    Ok(deployment_list[0].deployment_name.clone())
}

fn get_connection_poll(future: &mut WorkerConnectionFuture) -> Result<WorkerConnection, String> {
    for _ in 0..POLL_NUM_ATTEMPTS {
        println!("Attempting to poll.");
        match future.poll() {
            Ok(res) => {
                if let Async::Ready(conn) = res {
                    return Ok(conn);
                }
            }
            Err(s) => return Err(s),
        };

        ::std::thread::sleep(::std::time::Duration::from_millis(
            POLL_TIME_BETWEEN_ATTEMPTS_MILLIS,
        ));
    }

    Err("Max connection attempts failed.".to_owned())
}
