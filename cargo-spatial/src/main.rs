use anyhow::Result;
use cargo_spatial::{codegen, config::Config, download, local, opt::*};
use log::*;
use simplelog::*;
use structopt::StructOpt;

fn main() -> Result<()> {
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
        Command::Codegen => codegen::run_codegen(&Config::load()?)?,

        Command::Local(local) => match local {
            Local::Launch(launch) => local::launch(&Config::load()?, launch)?,
        },

        Command::Generate { command } => match command {
            Generate::ComponentId => {
                println!("Component ID: {}", cargo_spatial::generate_component_id());
            }
        },

        Command::Download { command } => match command {
            Download::Cli => download::download_cli()?,
            Download::Sdk(options) => download::download_sdk(Config::load(), options)?,
        },
    }

    Ok(())
}
