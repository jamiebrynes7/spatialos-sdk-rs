use log::*;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::process;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpatialWorkerSdkPackage {
    CHeaders,
    CApiWin,
    CApiMac,
    CApiLinux,
}

impl SpatialWorkerSdkPackage {
    fn package_name(self) -> &'static str {
        match self {
            SpatialWorkerSdkPackage::CHeaders => "c_headers",
            SpatialWorkerSdkPackage::CApiWin => "c-static-x86_64-vc141_mt-win32",
            SpatialWorkerSdkPackage::CApiMac => "c-static-x86_64-clang-macos",
            SpatialWorkerSdkPackage::CApiLinux => "c-static-x86_64-gcc510_pic-linux",
        }
    }

    fn relative_target_directory(self) -> &'static str {
        match self {
            SpatialWorkerSdkPackage::CHeaders => "headers",
            SpatialWorkerSdkPackage::CApiWin => "win",
            SpatialWorkerSdkPackage::CApiMac => "macos",
            SpatialWorkerSdkPackage::CApiLinux => "linux",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpatialToolsPackage {
    SchemaCompilerWin,
    SchemaCompilerMac,
    SchemaCompilerLinux,
    SnapshotConverterWin,
    SnapshotConverterMac,
    SnapshotConverterLinux,
}

impl SpatialToolsPackage {
    fn package_name(self) -> &'static str {
        match self {
            SpatialToolsPackage::SchemaCompilerWin => "schema_compiler-x86_64-win32",
            SpatialToolsPackage::SchemaCompilerMac => "schema_compiler-x86_64-macos",
            SpatialToolsPackage::SchemaCompilerLinux => "schema_compiler-x86_64-linux",
            SpatialToolsPackage::SnapshotConverterWin => "snapshot_converter-x86_64-win32",
            SpatialToolsPackage::SnapshotConverterMac => "snapshot_converter-x86_64-macos",
            SpatialToolsPackage::SnapshotConverterLinux => "snapshot_converter-x86_64-linux",
        }
    }

    fn relative_target_directory(self) -> &'static str {
        match self {
            SpatialToolsPackage::SchemaCompilerWin
            | SpatialToolsPackage::SchemaCompilerMac
            | SpatialToolsPackage::SchemaCompilerLinux => "schema-compiler",
            SpatialToolsPackage::SnapshotConverterWin
            | SpatialToolsPackage::SnapshotConverterMac
            | SpatialToolsPackage::SnapshotConverterLinux => "snapshot-converter",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpatialSchemaPackage {
    StandardLibrary,
    ExhaustiveTestSchema,
}

impl SpatialSchemaPackage {
    fn package_name(self) -> &'static str {
        match self {
            SpatialSchemaPackage::StandardLibrary => "standard_library",
            SpatialSchemaPackage::ExhaustiveTestSchema => "test_schema_library",
        }
    }

    fn relative_target_directory(self) -> &'static str {
        match self {
            SpatialSchemaPackage::StandardLibrary => "std-lib",
            SpatialSchemaPackage::ExhaustiveTestSchema => "test-schema",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpatialPackageSource {
    WorkerSdk(SpatialWorkerSdkPackage),
    Tools(SpatialToolsPackage),
    Schema(SpatialSchemaPackage),
}

impl SpatialPackageSource {
    fn package_name(self) -> Vec<&'static str> {
        match self {
            SpatialPackageSource::WorkerSdk(package) => vec!["worker_sdk", package.package_name()],
            SpatialPackageSource::Tools(package) => vec!["tools", package.package_name()],
            SpatialPackageSource::Schema(package) => vec!["schema", package.package_name()],
        }
    }

    fn relative_target_directory(self) -> &'static str {
        match self {
            SpatialPackageSource::WorkerSdk(package) => package.relative_target_directory(),
            SpatialPackageSource::Tools(package) => package.relative_target_directory(),
            SpatialPackageSource::Schema(package) => package.relative_target_directory(),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub msg: String,
    pub inner: Option<Box<dyn std::error::Error>>,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut msg = self.msg.clone();

        if let Some(ref inner) = self.inner {
            msg = format!("{}\nInner error: {}", msg, inner);
        }

        f.write_str(&msg)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.as_ref().map(|e| e.as_ref())
    }
}

pub fn download_package(
    package_source: SpatialPackageSource,
    sdk_version: &str,
    spatial_lib_dir: &str,
) -> Result<(), Error> {
    info!("Downloading {}", package_source.package_name().join(" "));

    let mut output_path = PathBuf::new();
    output_path.push(spatial_lib_dir);
    output_path.push(package_source.relative_target_directory());

    let mut args = vec!["package", "retrieve"];

    args.extend(package_source.package_name());
    args.push(sdk_version);
    args.push(output_path.to_str().unwrap());
    args.push("--unzip");

    trace!("Running spatial command with arguments: {:?}", args);

    let process = process::Command::new("spatial")
        .args(args)
        .output()
        .map_err(|e| Error {
            msg: "Failed to run 'spatial'.".into(),
            inner: Some(Box::new(e)),
        })?;

    if !process.status.success() {
        if let Ok(stdout) = String::from_utf8(process.stdout) {
            trace!("{}", stdout);
        } else {
            warn!("Could not read 'spatial' standard out.");
        }

        return Err(match String::from_utf8(process.stderr) {
            Ok(err) => Error {
                msg: err,
                inner: None,
            },
            Err(e) => Error {
                msg: "Package download failed and stderr is not utf-8 compliant.".into(),
                inner: Some(Box::new(e)),
            },
        });
    }

    Ok(())
}
