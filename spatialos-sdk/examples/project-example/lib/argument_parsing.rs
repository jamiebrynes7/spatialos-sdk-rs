extern crate clap;

use self::clap::{App, Arg};

use spatialos_sdk::worker::parameters::{ConnectionParameters, ProtocolLoggingParameters};
use spatialos_sdk::worker::locator::{LocatorCredentials, LocatorParameters};

static RECEPTIONIST_HOST: &str = "127.0.0.1";
static RECEPTIONIST_PORT: u16 = 7777;
static WORKER_TYPE: &str = "RustWorker";

pub enum ConnectionType {
    Receptionist(String, u16),
    Locator(LocatorParameters)
}

pub struct WorkerConfiguration {
    pub connection_params: ConnectionParameters,
    pub connection_type: ConnectionType,
    pub connect_with_poll: bool,
}

// Gets the configuration of the worker.
pub fn get_worker_configuration() -> WorkerConfiguration {
    let matches = App::new("SpatialOS Rust SDK Example Worker")
        .author("Jamie Brynes <jamiebrynes7@gmail.com>")
        .about("An example usage of the SpatialOS Rust SDK.")
        .arg(
            Arg::with_name("local_receptionist")
                .long("local_receptionist")
                .takes_value(false)
                .help("Connect using a local receptionist flow.")
                .conflicts_with("locator")
        ).arg(
        Arg::with_name("locator")
            .long("locator")
            .takes_value(true)
            .value_name("LOCATOR_TOKEN")
            .help("Connect using a locator flow.")
            .requires("project_name")
    ).arg(
        Arg::with_name("connect_with_poll")
            .long("connect_with_poll")
            .short("p")
            .help("Uses the polling connect rather than the blocking connect.")
    ).arg(
        Arg::with_name("project_name")
            .long("project_name")
            .takes_value(true)
            .value_name("SPATIALOS_PROJECT_NAME")
            .help("The SpatialOS project to use in the Locator flow.")
    ).get_matches();

    let params = ConnectionParameters::new(WORKER_TYPE).using_tcp();
    if let Some(locator_token) = matches.value_of("locator") {
        let locator_params = LocatorParameters {
            project_name: matches.value_of("project_name")
                .expect("No project name found")
                .to_string(),
            credentials: LocatorCredentials::LoginToken(locator_token.to_owned()),
            logging: ProtocolLoggingParameters::default(),
            enable_logging: false
        };
        return WorkerConfiguration {
            connection_params: params,
            connect_with_poll: matches.is_present("connect_with_poll"),
            connection_type: ConnectionType::Locator(locator_params)
        };
    }

    if matches.is_present("local_receptionist") {
        return WorkerConfiguration {
            connection_params: params,
            connect_with_poll: matches.is_present("connect_with_poll"),
            connection_type: ConnectionType::Receptionist(RECEPTIONIST_HOST.to_string(), RECEPTIONIST_PORT)
        };
    }

    panic!("Received a CLI configuration that I don't know how to parse.");
}