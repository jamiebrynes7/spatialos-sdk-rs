use crate::config::{BuildProfile, Config};
use crate::opt::*;
use std::ffi::OsString;
use std::path::*;
use std::process;
use tap::*;

/// Prepares and launches a local deployment.
///
/// Before launching the deployment, this will first run code generation and build
/// workers in the project. Assumes that the current working directory is the root
/// directory of the project, i.e. the directory that has the `Spatial.toml` file.
pub fn launch(config: &Config, launch: &LocalLaunch) -> Result<(), Box<dyn std::error::Error>> {
    assert!(
        crate::current_dir_is_root(),
        "Current directory should be the project root"
    );

    // Run codegen and such.
    crate::codegen::run_codegen(&config)?;

    // Use `cargo install` to build workers and copy the exectuables to the build
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
                .map_err(|err| format!("Failed to build worker binaries: {}", err))?;

            if !status.success() {
                return Err("An error occurred while building workers")?;
            }
        }
    }

    // Run `spatial alpha local launch` with any user-specified flags.
    let mut command = process::Command::new("spatial");
    command.args(&["alpha", "local", "launch"]);
    if let Some(launch_config) = &launch.launch_config {
        let arg = OsString::from("--launch_config=").tap(|arg| arg.push(launch_config));
        command.arg(arg);
    }
    command
        .status()
        .map_err(|err| format!("Failed to run `spatial local launch`: {}", err))?;

    Ok(())
}
