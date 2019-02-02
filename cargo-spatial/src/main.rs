use cargo_spatial::{codegen, config::Config, local, opt::*};
use simplelog::*;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();

    // Initialize the logger.
    let verbosity = if opt.verbose {
        LevelFilter::Trace
    } else {
        LevelFilter::Warn
    };
    SimpleLogger::init(verbosity, Default::default()).expect("Failed to setup logger");

    match &opt.command {
        Command::Codegen => {
            let config = Config::load().expect("Failed to load config");
            codegen::run_codegen(&config).expect("Failed to run codegen");
        }

        Command::Local(local) => match local {
            Local::Launch(launch) => local::launch(&opt, local, launch),
        },

        Command::Generate { command } => match command {
            Generate::ComponentId => {
                println!("Component ID: {}", cargo_spatial::generate_component_id());
            }
        },
    }
}
