use crate::config::Config;
use log::*;
use spatialos_sdk_code_generator::{generator, schema_bundle};
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::*;
use std::process::Command;
use tap::*;

/// Performs code generation for the project described by `config`.
///
/// Assumes that the current working directory is the root directory of the project,
/// i.e. the directory that has the `Spatial.toml` file.
pub fn run_codegen(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    assert!(
        crate::current_dir_is_root(),
        "Current directory should be the project root"
    );

    // Ensure that the path to the Spatial SDK has been specified.
    let spatial_lib_dir = config.spatial_lib_dir()
        .map(normalize)
        .ok_or("spatial_lib_dir value must be set in the config, or the SPATIAL_LIB_DIR environment variable must be set")?;

    // Determine the paths the the schema compiler and protoc relative the the lib
    // dir path.
    let schema_compiler_path = normalize(spatial_lib_dir.join("schema-compiler/schema_compiler"));
    let std_lib_path = normalize(spatial_lib_dir.join("std-lib"));

    // Calculate the various output directories relative to `output_dir`.
    let output_dir = normalize(config.schema_build_dir());
    let bundle_json_path = output_dir.join("bundle.json");

    // Create the output directory if it doesn't already exist.
    fs::create_dir_all(&output_dir)
        .map_err(|_| format!("Failed to create {}", output_dir.display()))?;
    trace!("Created schema output dir: {}", output_dir.display());

    // Prepare initial flags for the schema compiler.
    let schema_path_arg = OsString::from("--schema_path=").tap(|arg| arg.push(&std_lib_path));
    let bundle_json_arg =
        OsString::from("--bundle_json_out=").tap(|arg| arg.push(&bundle_json_path));
    let descriptor_out_arg = OsString::from("--descriptor_set_out=")
        .tap(|arg| arg.push(normalize(output_dir.join("schema.descriptor"))));

    // Run the schema compiler for all schema files in the project.
    //
    // This will generated the schema descriptor file that SpatialOS loads directly, as
    // well as the schema bundle file that's used for code generation. All schema files
    // in the project are included, as well as the schema files in the standard schema
    // library
    let mut command = Command::new(&schema_compiler_path);
    command
        .arg(&schema_path_arg)
        .arg(&bundle_json_arg)
        .arg(&descriptor_out_arg)
        .arg("--load_all_schema_on_schema_path");

    // Add all the root schema paths.
    for schema_path in &config.schema_paths {
        let arg = OsString::from("--schema_path=").tap(|arg| arg.push(normalize(schema_path)));
        command.arg(&arg);
    }

    // Add all schema files in the std lib.
    add_schemas(&std_lib_path.join("improbable"), &mut command);

    // Add all user-provided schemas.
    for schema_path in &config.schema_paths {
        add_schemas(schema_path, &mut command);
    }

    trace!("{:#?}", command);
    let status = command
        .status()
        .map_err(|_| "Failed to compile schema files")?;

    if !status.success() {
        return Err("Failed to run schema compilation")?;
    }

    // Load bundle.json, which describes the schema definitions for all components.
    let mut input_file = File::open(&bundle_json_path).map_err(|_| "Failed to open bundle.json")?;
    let mut contents = String::new();
    input_file
        .read_to_string(&mut contents)
        .map_err(|_| "Failed to read contents of bundle.json")?;

    // Run code generation.
    let bundle = schema_bundle::load_bundle(&contents)
        .map_err(|_| "Failed to parse contents of bundle.json")?;
    let generated_file = generator::generate_code(bundle);

    // Write the generated code to the output file.
    File::create(&config.codegen_out)
        .map_err(|_| "Unable to create codegen output file")?
        .write_all(generated_file.as_bytes())
        .map_err(|_| "Failed to write generated code to file")?;

    Ok(())
}

/// HACK: Normalizes the separators in `path`.
///
/// This is necessary in order to be robust on Windows, as well as work around
/// some idiosyncrasies with schema_compiler and protoc. Currently,
/// schema_compiler and protoc get tripped up if you have paths with mixed path
/// separators (i.e. mixing '\' and '/'). This function normalizes paths to use
/// the same separators everywhere, ensuring that we can be robust regardless of
/// how the user specifies their paths. It also removes any current dir segments
/// ("./"), as they can trip up schema_compiler and protoc as well.
///
/// Improbable has noted that they are aware of these issues and will fix them
/// at some point in the future.
fn normalize<P: AsRef<std::path::Path>>(path: P) -> PathBuf {
    path.as_ref()
        .components()
        .filter(|&comp| comp != Component::CurDir)
        .collect()
}

/// Recursively searches `path` for `.schema` files and adds them to `command`.
fn add_schemas<P: AsRef<Path>>(path: P, command: &mut Command) {
    let schema_glob = path.as_ref().join("**/*.schema");
    for entry in glob::glob(schema_glob.to_str().unwrap())
        .unwrap()
        .filter_map(Result::ok)
    {
        command.arg(&entry);
    }
}
