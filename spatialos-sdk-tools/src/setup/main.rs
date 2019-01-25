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
        schema_paths,
        output_dir,
    } = Opt::from_args();

    let output_dir = normalize(output_dir);

    let spatial_lib_dir = spatial_lib_dir
        .or_else(|| std::env::var("SPATIAL_LIB_DIR").map(Into::into).ok())
        .ok_or("--spatial-lib-dir argument must be specified or the SPATIAL_LIB_DIR environment variable must be set")?;

    // Determine the paths the the schema compiler and protoc relative the the lib
    // dir path.
    let schema_compiler_path = normalize(spatial_lib_dir.join("schema-compiler/schema_compiler"));
    let protoc_path = normalize(spatial_lib_dir.join("schema-compiler/protoc"));
    let std_lib_path = normalize(spatial_lib_dir.join("std-lib"));

    // Calculate the various output directories relative to `output_dir`.
    let bin_path = output_dir.join("bin");
    let tmp_path = output_dir.join("tmp");
    let bundle_json_path = bin_path.join("bundle.json");

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
    let schema_path_arg = OsString::from("--schema_path=").tap(|arg| arg.push(&std_lib_path));
    let proto_out_arg = OsString::from("--proto_out=").tap(|arg| arg.push(&tmp_path));
    let bundle_json_arg = OsString::from("--bundle_json_out=").tap(|arg| arg.push(&bundle_json_path));
    let descriptor_out_arg = OsString::from("--descriptor_set_out=")
        .tap(|arg| arg.push(normalize(bin_path.join("schema.descriptor"))));
    let mut command = Command::new(&schema_compiler_path);
    command
        .arg(&schema_path_arg)
        .arg(&proto_out_arg)
        .arg(&bundle_json_arg)
        .arg(&descriptor_out_arg)
        .arg("--load_all_schema_on_schema_path");

    for schema_path in &schema_paths {
        let arg = OsString::from("--schema_path=").tap(|arg| arg.push(schema_path));
        command.arg(&arg);
    }

    // Add all schema files in the std lib.
    let schema_glob = std_lib_path.join("improbable/*.schema");
    for entry in glob::glob(schema_glob.to_str().unwrap())?.filter_map(Result::ok) {
        command.arg(&entry);
    }

    // Add all user-provided schemas.
    for schema_path in &schema_paths {
        let schema_glob = schema_path.join("**/*.schema");
        for entry in glob::glob(schema_glob.to_str().unwrap())?.filter_map(Result::ok) {
            command.arg(&entry);
        }
    }

    command
        .status()
        .map_err(|_| "Failed to compile schema files")?;

    // Run protoc on all the generated proto files.
    let proto_path_arg = OsString::from("--proto_path=").tap(|arg| arg.push(&tmp_path));
    let mut command = Command::new(&protoc_path);
    command
        .arg(&proto_path_arg)
        .arg(&descriptor_out_arg)
        .arg("--include_imports");
    let proto_glob = tmp_path.join("**/*.proto");
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

    #[structopt(long = "schema-path", short = "s", parse(from_os_str))]
    schema_paths: Vec<PathBuf>,

    /// The path the output directory for the project.
    #[structopt(parse(from_os_str))]
    output_dir: PathBuf,
}

// HACK: Normalizes the separators in `path`.
//
// This is necessary in order to be robust on Windows. Currently,
// schema_compiler and protoc get tripped up if you have paths with mixed path
// separators (i.e. mixing `\` and `/`). This function normalizes paths to use
// the same separators everywhere, ensuring that we can be robust regardless of
// how the user specifies their paths.
fn normalize<P: AsRef<std::path::Path>>(path: P) -> PathBuf {
    path.as_ref().components().collect()
}
