use crate::opt::*;
use cargo_metadata::MetadataCommand;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

mod local;
mod opt;

fn main() {
    let opt = Opt::from_args();
    match &opt.command {
        Command::Codegen => {
            let config = Config::load().expect("Failed to load config");
            codegen(&config).expect("Failed to run codegen");
        }

        Command::Local(local) => match local {
            Local::Launch(launch) => local::launch(&opt, local, launch),
        },

        Command::Generate { command } => match command {
            Generate::ComponentId => {
                println!("Component ID: {}", generate_component_id());
            }
        },
    }
}

/// Generates a random, valid component ID.
///
/// Component IDs are `i32` values that must be:
///
/// * Greater than 100.
/// * Less than 536,870,911.
/// * Not in the range 190,000 to 199999.
fn generate_component_id() -> i32 {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    loop {
        let num = rng.gen();
        if num > 100 && (num < 190_000 || num > 199_999) && num < 536_870_911 {
            return num;
        }
    }
}

/// Project configuration stored in the `Cargo.toml` file at project's root.
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
struct Config {
    /// The list of worker projects to be built.
    ///
    /// If empty, the root project is assumed to contain all workers.
    workers: Vec<String>,

    /// The file to use as output for code generation.
    ///
    /// Defaults to `src/generated.rs`.
    codegen_out: String,

    /// The directory containing schema files for the project.
    ///
    /// Defaults to `./schema`.
    schema_dir: String,

    /// The directory where built workers are put.
    ///
    /// Defaults to `./build`.
    build_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            workers: vec![".".into()],
            codegen_out: "src/generated.rs".into(),
            schema_dir: "./schema".into(),
            build_dir: "./build".into(),
        }
    }
}

impl Config {
    fn load() -> Result<Self, String> {
        // Run `cargo metadata` to get the metadata for all packages in the workspace.
        let metadata = MetadataCommand::new()
            .no_deps()
            .exec()
            .expect("Failed to get cargo metadata");

        // Find the package corresponding to the root of the workspace.
        let manifest_path = metadata.workspace_root.join("Cargo.toml");
        let package = metadata
            .packages
            .iter()
            .find(|package| package.manifest_path == manifest_path)
            .expect("No root package found???");

        // Get configuration info from the crate metadata.
        Ok(package
            .metadata
            .get("spatialos")
            .and_then(|val| serde_json::from_value(val.clone()).ok())
            .unwrap_or_default())
    }
}

fn codegen(config: &Config) -> Result<(), String> {
    // TODO: Use the spatialos-sdk-tools crate directly rather than invoking the CLI.
    let codegen_out = PathBuf::from(&config.schema_dir).join("bin");
    let status = process::Command::new("setup")
        .arg("-s")
        .arg(&config.schema_dir)
        .arg("-c")
        .arg(&config.codegen_out)
        .arg("-o")
        .arg(&codegen_out)
        .status()
        .expect("Failed to run setup script");

    if !status.success() {
        return Err("Failed to run codegen".into());
    }

    Ok(())
}
