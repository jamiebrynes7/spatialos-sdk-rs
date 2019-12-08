use crate::config::Config;
use crate::format_arg;
use log::*;
use spatialos_sdk_code_generator::{generator, schema_bundle};
use std::{
    error::Error,
    fmt::{Display, Formatter},
    fs::{self, File},
    io::prelude::*,
    path::*,
    process::Command
};

#[derive(Debug)]
pub enum CodegenErrorKind {
    BadConfig,
    SchemaCompiler,
    InvalidBundle,
    IO,
}

impl Display for CodegenErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            CodegenErrorKind::BadConfig => f.write_str("Bad Config"),
            CodegenErrorKind::SchemaCompiler => f.write_str("Schema Compiler Error"),
            CodegenErrorKind::InvalidBundle => f.write_str("Invalid Schema Bundle"),
            CodegenErrorKind::IO => f.write_str("IO Error"),
        }
    }
}

#[derive(Debug)]
pub struct CodegenError {
    kind: CodegenErrorKind,
    msg: String,
    inner: Option<Box<dyn Error>>,
}

impl Display for CodegenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut msg = format!("{}: {}", self.kind, self.msg);

        if let Some(ref inner) = self.inner {
            msg = format!("{}\nInner error: {}", msg, inner);
        }

        f.write_str(&msg)
    }
}

impl Error for CodegenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.inner.as_ref().map(|e| e.as_ref())
    }
}

/// Performs code generation for the project described by `config`.
///
/// Assumes that the current working directory is the root directory of the project,
/// i.e. the directory that has the `Spatial.toml` file.
pub fn run_codegen(config: &Config) -> Result<(), CodegenError> {
    if !crate::current_dir_is_root() {
        return Err(CodegenError {
            msg: "Current directory should be the project root.".into(),
            kind: CodegenErrorKind::BadConfig,
            inner: None,
        });
    }

    // Ensure that the path to the Spatial SDK has been specified.
    let spatial_lib_dir = config.spatial_lib_dir()
        .map(PathBuf::from)
        .ok_or(CodegenError {
            msg: "spatial_lib_dir value must be set in the config, or the SPATIAL_LIB_DIR environment variable must be set.".into(),
            kind: CodegenErrorKind::BadConfig,
            inner: None})?;

    // Determine the paths the the schema compiler and protoc relative the the lib
    // dir path.
    let schema_compiler_path = spatial_lib_dir.join("schema-compiler/schema_compiler");
    let std_lib_path = spatial_lib_dir.join("std-lib");

    // Calculate the various output directories relative to `output_dir`.
    let output_dir = PathBuf::from(config.schema_build_dir());
    let bundle_json_path = output_dir.join("bundle.json");
    let schema_descriptor_path = output_dir.join("schema.descriptor");

    // Create the output directory if it doesn't already exist.
    fs::create_dir_all(&output_dir).map_err(|e| {
        let msg = format!("Failed to create {}", output_dir.display());
        CodegenError {
            msg,
            kind: CodegenErrorKind::IO,
            inner: Some(Box::new(e)),
        }
    })?;
    trace!("Created schema output dir: {}", output_dir.display());

    // Prepare initial flags for the schema compiler.
    let schema_path_arg = format_arg("schema_path", &std_lib_path);
    let bundle_json_arg = format_arg("bundle_json_out", &bundle_json_path);
    let descriptor_out_arg = format_arg("descriptor_set_out", &schema_descriptor_path);

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
        command.arg(&format_arg("schema_path", schema_path));
    }

    trace!("{:#?}", command);
    let status = command.status().map_err(|e| CodegenError {
        msg: "Failed to compile schema files".into(),
        kind: CodegenErrorKind::SchemaCompiler,
        inner: Some(Box::new(e)),
    })?;

    if !status.success() {
        return Err(CodegenError {
            msg: "Failed to run schema compilation".into(),
            kind: CodegenErrorKind::SchemaCompiler,
            inner: None,
        });
    }

    // Load bundle.json, which describes the schema definitions for all components.
    let mut input_file = File::open(&bundle_json_path).map_err(|e| CodegenError {
        msg: "Failed to open bundle.json".into(),
        kind: CodegenErrorKind::SchemaCompiler,
        inner: Some(Box::new(e)),
    })?;

    let mut contents = String::new();
    input_file
        .read_to_string(&mut contents)
        .map_err(|e| CodegenError {
            msg: "Failed to read contents of bundle.json".into(),
            kind: CodegenErrorKind::IO,
            inner: Some(Box::new(e)),
        })?;

    // Run code generation.
    let bundle = schema_bundle::load_bundle(&contents).map_err(|e| CodegenError {
        msg: "Failed to parse contents of bundle.json".into(),
        kind: CodegenErrorKind::InvalidBundle,
        inner: Some(Box::new(e)),
    })?;
    let generated_file = generator::generate_code(bundle);

    // Write the generated code to the output file.
    File::create(&config.codegen_out)
        .map_err(|e| CodegenError {
            msg: "Unable to create codegen output file".into(),
            kind: CodegenErrorKind::IO,
            inner: Some(Box::new(e)),
        })?
        .write_all(generated_file.as_bytes())
        .map_err(|e| CodegenError {
            msg: "Failed to write generated code to file".into(),
            kind: CodegenErrorKind::IO,
            inner: Some(Box::new(e)),
        })?;

    Ok(())
}
