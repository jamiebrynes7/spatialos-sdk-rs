use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Project configuration stored in the `Cargo.toml` file at project's root.
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// The version of the SpatialOS SDK to build against.
    ///
    /// Defaults to the latest supported version.
    pub spatial_sdk_version: String,

    /// The runtime version to use for local launches.
    ///
    /// Defaults to the latest version pinned by this `cargo-spatial` version.
    pub runtime_version: String,

    /// The list of worker projects to be built.
    ///
    /// If empty, the root project is assumed to contain all workers.
    pub workers: Vec<String>,

    /// The file to use as output for code generation.
    ///
    /// Defaults to `src/generated.rs`.
    pub codegen_out: String,

    /// The directories containing schema files for the project.
    ///
    /// Defaults to `./schema`.
    pub schema_paths: Vec<String>,

    /// The directory where built workers are put.
    ///
    /// Defaults to `./build`.
    pub build_dir: String,

    /// The build profile to use when building local workers.
    ///
    /// Defaults to `BuildProfile::Debug`.
    pub local_build_profile: BuildProfile,

    /// The directory where the SpatialOS SDK should be downloaded.
    ///
    /// If not specified, the SPATIAL_LIB_DIR environment variable will be used
    /// instead.
    spatial_lib_dir: Option<String>,

    /// The directory to use as output for the schema compiler.
    ///
    /// The built schema descriptor and bundle file will be put here. Defaults to
    /// `build_dir`/schema if not specified.
    schema_build_dir: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            spatial_sdk_version: "14.8.0".into(),
            runtime_version: "14.5.4".into(),
            workers: vec![".".into()],
            codegen_out: "src/generated.rs".into(),
            schema_paths: vec![],
            build_dir: "./build".into(),
            schema_build_dir: None,
            spatial_lib_dir: None,
            local_build_profile: BuildProfile::Debug,
        }
    }
}

impl Config {
    /// Attempts to load the project configuration from a `Spatial.toml` file.
    pub fn load() -> Result<Self> {
        // TODO: Traverse up the directory hierarchy until a `Spatial.toml` file is
        // found or the root directory is reached.
        let mut contents = String::new();
        File::open("Spatial.toml")
            .context("Could not find a `Spatial.toml` in current directory")?
            .read_to_string(&mut contents)
            .context("Failed to read contents of Spatial.toml")?;

        toml::from_str(&contents).context("Failed to deserialize Spatial.toml")
    }

    /// Returns the path to the output directory to be used for schema compilation.
    pub fn schema_build_dir(&self) -> String {
        self.schema_build_dir
            .clone()
            .unwrap_or_else(|| self.build_dir.clone() + "/schema")
    }

    /// Returns the absolute path to the spatial SDK directory, or `None` if the path hasn't
    /// been configured.
    pub fn spatial_lib_dir(&self) -> Option<String> {
        self.spatial_lib_dir
            .clone()
            .or_else(|| ::std::env::var("SPATIAL_LIB_DIR").ok())
            .map(|ref path| self.resolve_path(path))
    }

    /// Resolves the unresolved path into an absolute path.
    fn resolve_path(&self, unresolved_path: &str) -> String {
        let path = Path::new(unresolved_path);

        if path.is_absolute() {
            return path.to_str().unwrap().to_owned();
        }

        let mut cwd = ::std::env::current_dir().unwrap();
        cwd.push(path);

        cwd.to_str().unwrap().to_owned()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BuildProfile {
    Debug,
    Release,
}
