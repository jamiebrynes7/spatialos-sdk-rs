use crate::{Command, Opt};
use spatialos_sdk::worker::{
    connection::WorkerConnection,
    constants::{LOCATOR_HOSTNAME, LOCATOR_PORT, RECEPTIONIST_PORT},
    locator::{
        Locator, LocatorParameters, LoginTokensRequest, PlayerIdentityCredentials,
        PlayerIdentityTokenRequest,
    },
    parameters::ConnectionParameters,
};
use std::error::Error;
use uuid::Uuid;

pub async fn get_connection(opt: Opt) -> Result<WorkerConnection, Box<dyn Error>> {
    let Opt {
        worker_type,
        worker_id,
        command,
    } = opt;

    let worker_id = worker_id.unwrap_or_else(|| format!("{}-{}", &worker_type, Uuid::new_v4()));
    let future = match command {
        Command::Receptionist {
            host,
            port,
            connect_with_external_ip,
        } => {
            let params = ConnectionParameters::new(worker_type)
                .using_udp()
                .using_external_ip(connect_with_external_ip);
            WorkerConnection::connect_receptionist(
                &worker_id,
                &host.unwrap_or_else(|| "127.0.0.1".into()),
                port.unwrap_or(RECEPTIONIST_PORT),
                params,
            )
        }

        Command::Locator {
            player_identity_token,
            login_token,
        } => {
            let locator = Locator::new(
                LOCATOR_HOSTNAME,
                LOCATOR_PORT,
                &LocatorParameters::new(PlayerIdentityCredentials::new(
                    player_identity_token,
                    login_token,
                )),
            );
            WorkerConnection::connect_locator(
                locator,
                ConnectionParameters::new(worker_type)
                    .using_tcp()
                    .using_external_ip(true),
            )
        }

        Command::DevelopmentAuthentication { dev_auth_token } => {
            let request = PlayerIdentityTokenRequest::new(dev_auth_token, "player-id")
                .with_display_name("My Player");
            let pit = Locator::create_development_player_identity_token(
                LOCATOR_HOSTNAME,
                LOCATOR_PORT,
                request,
            )
            .await?;

            let request = LoginTokensRequest::new(&pit.player_identity_token, &worker_type);
            let response =
                Locator::create_development_login_tokens(LOCATOR_HOSTNAME, LOCATOR_PORT, request)
                    .await?;

            if response.login_tokens.is_empty() {
                return Err("No login tokens retrieved".into());
            }

            let token = &response.login_tokens[0];
            let credentials =
                PlayerIdentityCredentials::new(&pit.player_identity_token, &token.login_token);
            let locator = Locator::new(
                LOCATOR_HOSTNAME,
                LOCATOR_PORT,
                &LocatorParameters::new(credentials),
            );

            WorkerConnection::connect_locator(
                locator,
                ConnectionParameters::new(worker_type)
                    .using_tcp()
                    .using_external_ip(true),
            )
        }
    };

    Ok(future.await?)
}
