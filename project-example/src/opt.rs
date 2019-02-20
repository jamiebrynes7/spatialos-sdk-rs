use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "project-example",
    about = "A SpatialOS worker written in Rust."
)]
pub struct Opt {
    #[structopt(name = "WORKER_TYPE", long = "worker-type", short = "w")]
    pub worker_type: String,

    #[structopt(name = "WORKER_ID", long = "worker-id", short = "i")]
    pub worker_id: Option<String>,

    #[structopt(name = "POLLING_CONNECTION", long = "polling-connection", short = "p")]
    pub connect_with_poll: bool,

    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "receptionist")]
    Receptionist {
        #[structopt(long, short)]
        connect_with_external_ip: bool,

        #[structopt(long, short)]
        host: Option<String>,

        #[structopt(long, short)]
        port: Option<u16>,
    },

    #[structopt(name = "locator")]
    Locator {
        #[structopt(name = "LOCATOR_TOKEN", long = "locator-token", short = "t")]
        token: String,

        #[structopt(name = "PROJECT_NAME", long = "project-name", short = "n")]
        project_name: String,
    },

    #[structopt(name = "dev-auth")]
    DevelopmentAuthentication {
        #[structopt(name = "DEV_AUTH_TOKEN", long = "dev-auth-token", short = "t")]
        dev_auth_token: String,
    },
}
