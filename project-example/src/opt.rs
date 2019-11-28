use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "project-example",
    about = "A SpatialOS worker written in Rust."
)]
pub struct Opt {
    #[structopt(long, short = "w")]
    pub worker_type: String,

    #[structopt(long, short = "i")]
    pub worker_id: Option<String>,

    #[structopt(long = "polling-connection", short = "p")]
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
        #[structopt(short = "p")]
        player_identity_token: String,
        #[structopt(long, short = "t")]
        login_token: String,
    },

    #[structopt(name = "dev-auth")]
    DevelopmentAuthentication {
        #[structopt(long, short = "t")]
        dev_auth_token: String,
    },
}
