use crate::{Command, Opt};
use futures::{Async, Future};
use spatialos_sdk::worker::{
    connection::{WorkerConnection, WorkerConnectionFuture},
    constants::{LOCATOR_HOSTNAME, LOCATOR_PORT, RECEPTIONIST_PORT},
    locator::{
        Locator, LocatorCredentials, LocatorParameters, LoginTokensRequest,
        PlayerIdentityTokenRequest,
    },
    parameters::ConnectionParameters,
};
use uuid::Uuid;

const POLL_NUM_ATTEMPTS: u32 = 5;
const POLL_TIME_BETWEEN_ATTEMPTS_MILLIS: u64 = 3000;

pub fn get_connection(opt: Opt) -> Result<WorkerConnection, String> {
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
            let params = ConnectionParameters::new(worker_type)
                .using_tcp()
                .using_external_ip(connect_with_external_ip)
                .enable_internal_serialization();
            WorkerConnection::connect_receptionist_async(
                &worker_id,
                &host.unwrap_or_else(|| "127.0.0.1".into()),
                port.unwrap_or(RECEPTIONIST_PORT),
                &params,
            )
        }

        Command::Locator {
            token,
            project_name,
        } => {
            let params = LocatorParameters::new(LocatorCredentials::login_token(token))
                .with_project_name(project_name);
            let locator = Locator::new(LOCATOR_HOSTNAME, LOCATOR_PORT, &params);
            let deployment = get_deployment(&locator)?;
            WorkerConnection::connect_locator_and_queue_async(
                &locator,
                &deployment,
                &ConnectionParameters::new(worker_type)
                    .using_tcp()
                    .using_external_ip(true)
                    .enable_internal_serialization(),
                queue_status_callback,
            )
        }
        Command::DevelopmentAuthentication { dev_auth_token } => {
            let mut request = PlayerIdentityTokenRequest::new(dev_auth_token, "player-id")
                .with_display_name("My Player");
            let future = Locator::create_development_player_identity_token(
                LOCATOR_HOSTNAME,
                LOCATOR_PORT,
                &mut request,
            );

            let pit = future.wait()?;
            let mut request =
                LoginTokensRequest::new(pit.player_identity_token.as_str(), worker_type.as_str());
            let future = Locator::create_development_login_tokens(
                LOCATOR_HOSTNAME,
                LOCATOR_PORT,
                &mut request,
            );

            let response = future.wait()?;

            if response.login_tokens.is_empty() {
                return Err("No login tokens retrieved".to_owned());
            }

            let token = &response.login_tokens[0];
            let credentials = LocatorCredentials::player_identity(
                pit.player_identity_token.as_str(),
                token.login_token.as_str(),
            );
            let locator = Locator::new(
                LOCATOR_HOSTNAME,
                LOCATOR_PORT,
                &LocatorParameters::new(credentials),
            );

            WorkerConnection::connect_locator_async(
                &locator,
                &ConnectionParameters::new(worker_type)
                    .using_tcp()
                    .using_external_ip(true)
                    .enable_internal_serialization(),
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
