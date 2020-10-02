use crate::config::{BuildProfile, Config};
use crate::format_arg;
use crate::opt::*;
use anyhow::{anyhow, Context, Result};
use serde::export::Formatter;
use std::fmt::Display;
use std::path::*;
use std::process;

#[derive(Debug)]
pub enum ErrorKind {
    Codegen,
    Build,
    Launch,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ErrorKind::Codegen => f.write_str("Codegen Error"),
            ErrorKind::Build => f.write_str("Build Error"),
            ErrorKind::Launch => f.write_str("Launch Error"),
        }
    }
}

/// Prepares and launches a local deployment.
///
/// Before launching the deployment, this will first run code generation and build
/// workers in the project. Assumes that the current working directory is the root
/// directory of the project, i.e. the directory that has the `Spatial.toml` file.
pub fn launch(config: &Config, launch: &LocalLaunch) -> Result<()> {
    if !crate::current_dir_is_root() {
        return Err(anyhow!("The current directory is not the project root."));
    }

    // Run codegen and such.
    crate::codegen::run_codegen(config).context("Failed to generate code.")?;

    // Use `cargo install` to build workers and copy the executables to the build
    // directory.
    //
    // TODO: Manually copy the built executables instead of using `cargo install`.
    // `cargo install` doesn't use the same build cache as normal builds, so it will
    // sometimes result in unnecessary recompilation, which can slow down launch times.
    if !launch.no_build {
        let build_profile = match config.local_build_profile {
            BuildProfile::Debug => "debug",
            BuildProfile::Release => "release",
        };
        let build_dir = PathBuf::from(&config.build_dir).join(build_profile);
        for worker_path in &config.workers {
            let mut command = process::Command::new("cargo");
            command
                .arg("install")
                .arg("--root")
                .arg(&build_dir)
                .arg("--force")
                .arg("--path")
                .arg(worker_path);

            if config.local_build_profile == BuildProfile::Debug {
                command.arg("--debug");
            }

            let status = command
                .status()
                .context("Failed to execute 'cargo install'")?;

            if !status.success() {
                return Err(anyhow!("Failed to build worker."));
            }
        }
    }

    // Run `spatial alpha local launch` with any user-specified flags.
    let mut command = process::Command::new("spatial");
    command.args(&[
        "alpha",
        "local",
        "launch",
        "--runtime_version",
        &config.runtime_version,
    ]);
    if let Some(launch_config) = &launch.launch_config {
        command.arg(&format_arg("launch_config", launch_config));
    }

    command.status().context("Failed to launch deployment.")?;

    Ok(())
}
