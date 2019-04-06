use cargo_spatial::{codegen, config::Config, local, opt::*};
use log::*;
use simplelog::*;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    // Initialize the logger.
    let verbosity = if opt.verbose {
        LevelFilter::Trace
    } else {
        LevelFilter::Warn
    };
    SimpleLogger::init(verbosity, Default::default()).expect("Failed to setup logger");

    let config = Config::load()?;
    trace!("Loaded config: {:#?}", config);

    // Perform the operation selected by the user.
    match &opt.command {
        Command::Codegen => codegen::run_codegen(&config)?,

        Command::Local(local) => match local {
            Local::Launch(launch) => local::launch(&config, launch)?,
        },

        Command::Generate { command } => match command {
            Generate::ComponentId => {
                println!("Component ID: {}", cargo_spatial::generate_component_id());
            }
        },

        Command::Download {command} => match command {
            Download::Cli => println!("Download CLI"),
            Download::Sdk => println!("Download SDK")
        }
    }

    Ok(())
}
