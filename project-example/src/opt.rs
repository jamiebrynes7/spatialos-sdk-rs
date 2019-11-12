use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "project-example",
    about = "A SpatialOS worker written in Rust."
)]
pub struct Opt {
    #[structopt(name = "WORKER_TYPE", long, short = "w")]
    pub worker_type: String,

    #[structopt(name = "WORKER_ID", long, short = "i")]
    pub worker_id: Option<String>,

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
        #[structopt(name = "PLAYER_IDENTITY_TOKEN", short = "p")]
        player_identity_token: String,
        #[structopt(name = "LOGIN_TOKEN", long, short = "t")]
        login_token: String,
    },

    #[structopt(name = "dev-auth")]
    DevelopmentAuthentication {
        #[structopt(name = "DEV_AUTH_TOKEN", long, short = "t")]
        dev_auth_token: String,
    },
}
