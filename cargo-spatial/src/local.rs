use crate::config::Config;
use crate::opt::*;
use std::ffi::OsString;
use std::path::*;
use std::process;
use tap::*;

/// Builds workers and then runs `spatial local launch`.
pub fn launch(_opt: &Opt, _local: &Local, launch: &LocalLaunch) {
    let config = Config::load().expect("Failed to load configuration");

    // Run codegen and such.
    crate::codegen::run_codegen(&config).expect("Failed to run codegen");

    // Use `cargo install` to build workers and copy the exectuables to the build
    // directory.
    //
    // TODO: Manually copy the built executables instead of using `cargo install`.
    // `cargo install` doesn't use the same build cache as normal builds, so it will
    // sometimes result in unnecessary recompilation, which can slow down launch times.
    if !launch.no_build {
        let build_dir = PathBuf::from(&config.build_dir).join("debug");
        for worker_path in config.workers {
            let status = process::Command::new("cargo")
                .arg("install")
                .arg("--root")
                .arg(&build_dir)
                .arg("--debug")
                .arg("--force")
                .arg("--path")
                .arg(&worker_path)
                .status()
                .expect("Failed to build worker bin");

            if !status.success() {
                return;
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
    command.status().expect("Failed to run spatial");
}
