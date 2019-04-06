use cargo_spatial::{codegen, download, local, opt::*};
use log::*;
use simplelog::*;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    // Initialize the logger.
    let verbosity = if opt.verbose {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    };
    SimpleLogger::init(verbosity, Default::default()).expect("Failed to setup logger");

    // Perform the operation selected by the user.
    match &opt.command {
        Command::Codegen => codegen::run_codegen()?,

        Command::Local(local) => match local {
            Local::Launch(launch) => local::launch(launch)?,
        },

        Command::Generate { command } => match command {
            Generate::ComponentId => {
                println!("Component ID: {}", cargo_spatial::generate_component_id());
            }
        },

        Command::Download { command } => match command {
            Download::Cli => download::download_cli()?,
            Download::Sdk(options) => download::download_sdk(options)?,
        },
    }

    Ok(())
}
