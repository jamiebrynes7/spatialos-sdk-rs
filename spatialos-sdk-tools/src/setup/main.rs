use fs_extra::dir::{self, CopyOptions};
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;
use tap::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Opt {
        spatial_lib_dir,
        schema_dir,
    } = Opt::from_args();

    let spatial_lib_dir = spatial_lib_dir
        .or_else(|| std::env::var("SPATIAL_LIB_DIR").map(Into::into).ok())
        .ok_or("--spatial-lib-dir argument must be specified or the SPATIAL_LIB_DIR environment variable must be set")?;

    // Determine the paths the the schema compiler and protoc relative the the lib
    // dir path.
    let schema_compiler_path = spatial_lib_dir.join("schema-compiler/schema_compiler");
    let protoc_path = spatial_lib_dir.join("schema-compiler/protoc");

    // Calculate the various output directories relative to `out_dir`.
    let bin_path = schema_dir.join("bin");
    let tmp_path = schema_dir.join("tmp");

    // Create the output directories if they don't already exist.
    fs::create_dir_all(&bin_path)
        .map_err(|_| format!("Failed to create {}", bin_path.display()))?;
    fs::create_dir_all(&tmp_path)
        .map_err(|_| format!("Failed to create {}", tmp_path.display()))?;

    // Copy the contents of the schema-compiler/proto dir into the temp dir.
    let proto_dir_glob = spatial_lib_dir.join("schema-compiler/proto/*");
    for entry in glob::glob(proto_dir_glob.to_str().unwrap())?.filter_map(Result::ok) {
        dir::copy(
            &entry,
            &tmp_path,
            &CopyOptions {
                overwrite: true,
                ..CopyOptions::new()
            },
        )
        .map_err(|_| {
            format!(
                "Failed to copy {} to {}",
                entry.display(),
                tmp_path.display()
            )
        })?;
    }

    // Run the schema compiler for each of the schema files in std-lib/improbable.
    let schema_path_arg =
        OsString::from("--schema_path=").tap(|arg| arg.push(&spatial_lib_dir.join("std-lib")));
    let proto_out_arg = OsString::from("--proto_out=").tap(|arg| arg.push(&tmp_path));
    let mut command = Command::new(&schema_compiler_path);
    command
        .arg(&schema_path_arg)
        .arg(&proto_out_arg)
        .arg("--load_all_schema_on_schema_path");

    let schema_glob = spatial_lib_dir.join("std-lib/improbable/*.schema");
    for entry in glob::glob(schema_glob.to_str().unwrap())?.filter_map(Result::ok) {
        command.arg(&entry);
    }

    command
        .status()
        .map_err(|_| "Failed to compile schema files")?;

    // Run protoc on all the generated proto files.
    let proto_glob = tmp_path.join("**/*.proto");
    let proto_path_arg = OsString::from("--proto_path=").tap(|arg| arg.push(&tmp_path));
    let descriptor_out_arg = OsString::from("--descriptor_set_out=")
        .tap(|arg| arg.push(&bin_path.join("schema.descriptor")));
    let mut command = Command::new(&protoc_path);
    command
        .arg(&proto_path_arg)
        .arg(&descriptor_out_arg)
        .arg("--include_imports");
    for entry in glob::glob(proto_glob.to_str().unwrap())?.filter_map(Result::ok) {
        command.arg(&entry);
    }

    command.status().map_err(|_| "Failed to run protoc")?;

    // Remove the temp directory once the setup process has finished.
    fs::remove_dir_all(&tmp_path).map_err(|_| "Failed to remove temp dir")?;

    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(name = "setup", about = "Perform setup for a Rust SpatialOS project.")]
struct Opt {
    /// The path to your local installation of the SpatialOS SDK. If not specified,
    /// the SPATIAL_OS_DIR environment variable instead.
    #[structopt(long = "spatial-lib-dir", short = "l", parse(from_os_str))]
    spatial_lib_dir: Option<PathBuf>,

    /// The path the schema directory for the project.
    #[structopt(parse(from_os_str))]
    schema_dir: PathBuf,
}
