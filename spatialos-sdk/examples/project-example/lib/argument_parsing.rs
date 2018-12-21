extern crate clap;

use self::clap::{App, Arg, ArgMatches, SubCommand};

use spatialos_sdk::worker::locator::{LocatorCredentials, LocatorParameters};
use spatialos_sdk::worker::parameters::{ConnectionParameters, ProtocolLoggingParameters};

static RECEPTIONIST_HOST: &str = "127.0.0.1";
static RECEPTIONIST_PORT: u16 = 7777;

static RECEPTIONIST_SUBCOMMAND: &str = "receptionist";
static LOCATOR_SUBCOMMAND: &str = "locator";

static CONNECT_POLL_ARG: &str = "connect_with_poll";
static EXTERNAL_IP_ARG: &str = "use_external_ip";
static LOCATOR_TOKEN_ARG: &str = "locator_token";
static PROJECT_NAME_ARG: &str = "project_name";
static WORKER_TYPE_ARG: &str = "worker_type";

pub enum ConnectionType {
    Receptionist(String, u16),
    Locator(LocatorParameters),
}

pub struct WorkerConfiguration {
    pub connection_params: ConnectionParameters,
    pub connection_type: ConnectionType,
    pub connect_with_poll: bool,
}

// Gets the configuration of the worker.
pub fn get_worker_configuration() -> WorkerConfiguration {
    let polling_connection_arg = Arg::with_name(CONNECT_POLL_ARG)
        .long(CONNECT_POLL_ARG)
        .short("p")
        .help("Uses the polling connect rather than the blocking connect.");

    let worker_type_arg = Arg::with_name(WORKER_TYPE_ARG)
        .long(WORKER_TYPE_ARG)
        .short("w")
        .value_name("WORKER_TYPE")
        .required(true)
        .help("The worker type to use when connecting. Either: RustClient or RustServer.");

    let matches = App::new("SpatialOS Rust SDK Example Worker")
        .author("Jamie Brynes <jamiebrynes7@gmail.com>")
        .about("An example usage of the SpatialOS Rust SDK.")
        .subcommand(
            SubCommand::with_name(RECEPTIONIST_SUBCOMMAND)
                .about("Connect via receptionist.")
                .arg(&polling_connection_arg)
                .arg(&worker_type_arg)
                .arg(
                    Arg::with_name(EXTERNAL_IP_ARG)
                        .long(EXTERNAL_IP_ARG)
                        .short("e")
                        .help("Connect using external IP"),
                ),
        )
        .subcommand(
            SubCommand::with_name(LOCATOR_SUBCOMMAND)
                .about("Connect via locator.")
                .arg(&polling_connection_arg)
                .arg(&worker_type_arg)
                .arg(
                    Arg::with_name(LOCATOR_TOKEN_ARG)
                        .long(LOCATOR_TOKEN_ARG)
                        .short("t")
                        .value_name("LOCATOR_TOKEN")
                        .help("Locator login token.")
                        .required(true),
                )
                .arg(
                    Arg::with_name(PROJECT_NAME_ARG)
                        .long(PROJECT_NAME_ARG)
                        .short("n")
                        .takes_value(true)
                        .value_name("SPATIALOS_PROJECT_NAME")
                        .help("The SpatialOS project to use in the Locator flow.")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(sub_matches) = matches.subcommand_matches(LOCATOR_SUBCOMMAND) {
        let locator_params = LocatorParameters::new(
            sub_matches
                .value_of(PROJECT_NAME_ARG)
                .expect("No project name found.")
                .to_string(),
            LocatorCredentials::LoginToken(
                sub_matches
                    .value_of(LOCATOR_TOKEN_ARG)
                    .expect("No locator token found.")
                    .to_string(),
            ),
        );

        let params = ConnectionParameters::new(get_worker_type(sub_matches))
            .using_tcp()
            .using_external_ip();

        return WorkerConfiguration {
            connection_params: params,
            connect_with_poll: sub_matches.is_present(CONNECT_POLL_ARG),
            connection_type: ConnectionType::Locator(locator_params),
        };
    }

    if let Some(sub_matches) = matches.subcommand_matches(RECEPTIONIST_SUBCOMMAND) {
        let mut params = ConnectionParameters::new(get_worker_type(sub_matches)).using_tcp();

        if sub_matches.is_present(EXTERNAL_IP_ARG) {
            params.network.use_external_ip = true;
        }

        return WorkerConfiguration {
            connection_params: params,
            connect_with_poll: sub_matches.is_present(CONNECT_POLL_ARG),
            connection_type: ConnectionType::Receptionist(
                RECEPTIONIST_HOST.to_string(),
                RECEPTIONIST_PORT,
            ),
        };
    }

    panic!("Please select one of the subcommands.")
}

fn get_worker_type<'a>(matches: &'a ArgMatches) -> &'a str {
    let worker_type = matches
        .value_of(WORKER_TYPE_ARG)
        .expect("No worker type found.");

    match worker_type {
        "RustClient" => {}
        "RustWorker" => {}
        _ => panic!("Unknown worker type: {}", worker_type),
    };

    worker_type
}
