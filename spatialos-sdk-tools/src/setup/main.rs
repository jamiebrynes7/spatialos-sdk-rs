use fs_extra::dir::{self, CopyOptions};
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tap::*;

use structopt::StructOpt;

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

fn main() {
    let Opt { spatial_lib_dir, schema_dir } = Opt::from_args();

    let spatial_lib_dir = spatial_lib_dir
        .or_else(|| std::env::var("SPATIAL_LIB_DIR").map(Into::into).ok())
        .expect("--spatial_lib_dir argument must be specified or the SPATIAL_LIB_DIR environment variable must be set");

    // Determine the paths the the schema compiler and protoc relative the the lib
    // dir path.
    let schema_compiler_path = spatial_lib_dir.join("schema-compiler/schema_compiler");
    let protoc_path = spatial_lib_dir.join("schema-compiler/protoc");

    // Calculate the various output directories relative to `out_dir`.
    let bin_path = schema_dir.join("bin");
    let tmp_path = schema_dir.join("tmp");

    // Create the output directories if they don't already exist.
    fs::create_dir_all(&bin_path).expect("Failed to create bin");
    fs::create_dir_all(&tmp_path).expect("Failed to create tmp");

    // Copy the contents of the schema-compiler/proto dir into the temp dir.
    let proto_dir_glob = spatial_lib_dir.join("schema-compiler/proto/*");
    for entry in glob::glob(proto_dir_glob.to_str().unwrap())
        .unwrap()
        .filter_map(Result::ok)
    {
        dir::copy(
            &entry,
            &tmp_path,
            &CopyOptions {
                overwrite: true,
                ..CopyOptions::new()
            },
        )
        .expect("Failed to copy contents of schema-compiler/proto");
    }

    // Run the schema compiler for each of the schema files in std-lib/improbable.
    let schema_glob = spatial_lib_dir.join("std-lib/improbable/*.schema");
    let schema_path_arg =
        OsString::from("--schema_path=").tap(|arg| arg.push(&spatial_lib_dir.join("std-lib")));
    let proto_out_arg = OsString::from("--proto_out=").tap(|arg| arg.push(&tmp_path));
    for entry in glob::glob(schema_glob.to_str().unwrap())
        .unwrap()
        .filter_map(Result::ok)
    {
        Command::new(&schema_compiler_path)
            .arg(&schema_path_arg)
            .arg(&proto_out_arg)
            .arg("--load_all_schema_on_schema_path")
            .arg(&entry)
            .status()
            .expect("Failed to compile schema :'(");
    }

    // Run protoc on all the generated proto files.
    let proto_glob = tmp_path.join("**/*.proto");
    let proto_path_arg = OsString::from("--proto_path=").tap(|arg| arg.push(&tmp_path));
    let descriptor_out_arg = OsString::from("--descriptor_set_out=")
        .tap(|arg| arg.push(&bin_path.join("schema.descriptor")));
    for entry in glob::glob(proto_glob.to_str().unwrap())
        .unwrap()
        .filter_map(Result::ok)
    {
        let mut command = Command::new(&protoc_path);
        command
            .arg(&proto_path_arg)
            .arg(&descriptor_out_arg)
            .arg("--include_imports")
            .arg(&entry)
            .status()
            .expect("Failed to run protoc");
    }

    fs::remove_dir_all(&tmp_path).expect("Failed to remove temp dir");
}
