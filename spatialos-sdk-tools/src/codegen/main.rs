use log::*;
use simplelog::*;
use spatialos_sdk_code_generator::{generator, schema_bundle};
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::*;
use std::process::Command;
use structopt::StructOpt;
use tap::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Opt {
        spatial_lib_dir,
        schema_paths,
        codegen,
        verbose,
        output_dir,
    } = Opt::from_args();

    // Initialize the logger.
    let verbosity = if verbose {
        LevelFilter::Trace
    } else {
        LevelFilter::Warn
    };
    SimpleLogger::init(verbosity, Default::default()).expect("Failed to setup logger");

    let output_dir = normalize(output_dir);

    let spatial_lib_dir = spatial_lib_dir
        .or_else(|| std::env::var("SPATIAL_LIB_DIR").map(Into::into).ok())
        .ok_or("--spatial-lib-dir argument must be specified or the SPATIAL_LIB_DIR environment variable must be set")?;

    // Determine the paths the the schema compiler and protoc relative the the lib
    // dir path.
    let schema_compiler_path = normalize(spatial_lib_dir.join("schema-compiler/schema_compiler"));
    let std_lib_path = normalize(spatial_lib_dir.join("std-lib"));

    // Calculate the various output directories relative to `output_dir`.
    let bundle_json_path = output_dir.join("bundle.json");

    // Create the output directories if they don't already exist.
    fs::create_dir_all(&output_dir)
        .map_err(|_| format!("Failed to create {}", output_dir.display()))?;

    // Run the schema compiler for each of the schema files in std-lib/improbable.
    let schema_path_arg = OsString::from("--schema_path=").tap(|arg| arg.push(&std_lib_path));
    let bundle_json_arg =
        OsString::from("--bundle_json_out=").tap(|arg| arg.push(&bundle_json_path));
    let descriptor_out_arg = OsString::from("--descriptor_set_out=")
        .tap(|arg| arg.push(normalize(output_dir.join("schema.descriptor"))));
    let mut command = Command::new(&schema_compiler_path);
    command
        .arg(&schema_path_arg)
        .arg(&bundle_json_arg)
        .arg(&descriptor_out_arg)
        .arg("--load_all_schema_on_schema_path");

    for schema_path in &schema_paths {
        let arg = OsString::from("--schema_path=").tap(|arg| arg.push(normalize(schema_path)));
        command.arg(&arg);
    }

    // Add all schema files in the std lib.
    add_schemas(&std_lib_path.join("improbable"), &mut command);

    // Add all user-provided schemas.
    for schema_path in &schema_paths {
        add_schemas(schema_path, &mut command);
    }

    trace!("{:#?}", command);
    command
        .status()
        .map_err(|_| "Failed to compile schema files")?;

    // If the user specified the `--codegen` flag, run code generation with the bundle file
    // that we just generated.
    if let Some(codegen_out_path) = codegen {
        let mut input_file =
            File::open(&bundle_json_path).expect("Unable to open the test schema bundle.");
        let mut contents = String::new();
        input_file
            .read_to_string(&mut contents)
            .expect("Unable to read the test schema bundle");
        let generated_file =
            generator::generate_code(schema_bundle::load_bundle(&contents).unwrap());
        let mut output_file = File::create(codegen_out_path).unwrap();
        output_file.write_all(generated_file.as_bytes()).unwrap();
    }

    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cargo-spatial-codegen",
    rename_all = "kebab-case",
    about = "Perform schema compilation and code generation for a Rust SpatialOS project."
)]
struct Opt {
    /// The path to your local installation of the SpatialOS SDK
    ///
    /// If not specified, uses the SPATIAL_OS_DIR environment variable instead. Will fail
    /// with an error if neither is set.
    #[structopt(long, short = "l", parse(from_os_str))]
    spatial_lib_dir: Option<PathBuf>,

    /// A directory to search for schema files
    ///
    /// The directory will be searched recursively for all .schema files. Any schema
    /// files found will be included in compilation. Can be specified multiple times,
    /// e.g. `setup -s foo/schemas -s bar/schemas -o schemas/bin`.
    #[structopt(long = "schema-path", short = "s", parse(from_os_str))]
    schema_paths: Vec<PathBuf>,

    /// Display detailed log output
    #[structopt(long, short)]
    verbose: bool,

    /// Perform code generation and put the output in the specified file
    ///
    /// If not specified, will not perform code generation.
    #[structopt(long, short = "c", parse(from_os_str))]
    codegen: Option<PathBuf>,

    /// The path the output directory for the project
    #[structopt(long, short, parse(from_os_str))]
    output_dir: PathBuf,
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
fn add_schemas(path: &PathBuf, command: &mut Command) {
    let schema_glob = path.join("**/*.schema");
    for entry in glob::glob(schema_glob.to_str().unwrap())
        .unwrap()
        .filter_map(Result::ok)
    {
        command.arg(&entry);
    }
}
