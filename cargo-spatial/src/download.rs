// Conditionally re-export the correct version of download_cli dependent on platform.
#[cfg(target_os = "linux")]
pub use self::linux::*;
#[cfg(target_os = "macos")]
pub use self::macos::*;
#[cfg(target_os = "windows")]
pub use self::windows::*;

use crate::{config::Config, errors::WrappedError, opt::DownloadSdk};
use log::*;
use std::{
    fmt::{Display, Formatter},
    fs,
    fs::File,
    io::copy,
    path::{Path, PathBuf},
    process,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SpatialWorkerSdkPackage {
    CHeaders,
    CApiWin,
    CApiMac,
    CApiLinux,
}

impl SpatialWorkerSdkPackage {
    fn package_name(self) -> &'static str {
        match self {
            SpatialWorkerSdkPackage::CHeaders => "c_headers",
            SpatialWorkerSdkPackage::CApiWin => "c-static-x86_64-vc140_mt-win32",
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
enum SpatialToolsPackage {
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
enum SpatialSchemaPackage {
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
enum SpatialPackageSource {
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

// TODO: Allow users to specify which ones of these want? Linux is always required.
static COMMON_PACKAGES: &[SpatialPackageSource] = &[
    SpatialPackageSource::WorkerSdk(SpatialWorkerSdkPackage::CHeaders),
    SpatialPackageSource::WorkerSdk(SpatialWorkerSdkPackage::CApiLinux),
    SpatialPackageSource::WorkerSdk(SpatialWorkerSdkPackage::CApiWin),
    SpatialPackageSource::WorkerSdk(SpatialWorkerSdkPackage::CApiMac),
    SpatialPackageSource::Schema(SpatialSchemaPackage::StandardLibrary),
];

#[cfg(target_os = "linux")]
static PLATFORM_PACKAGES: &[SpatialPackageSource] = &[
    SpatialPackageSource::Tools(SpatialToolsPackage::SchemaCompilerLinux),
    SpatialPackageSource::Tools(SpatialToolsPackage::SnapshotConverterLinux),
];

#[cfg(target_os = "windows")]
static PLATFORM_PACKAGES: &[SpatialPackageSource] = &[
    SpatialPackageSource::Tools(SpatialToolsPackage::SchemaCompilerWin),
    SpatialPackageSource::Tools(SpatialToolsPackage::SnapshotConverterWin),
];

#[cfg(target_os = "macos")]
static PLATFORM_PACKAGES: &[SpatialPackageSource] = &[
    SpatialPackageSource::Tools(SpatialToolsPackage::SchemaCompilerMac),
    SpatialPackageSource::Tools(SpatialToolsPackage::SnapshotConverterMac),
];

#[derive(Debug)]
pub enum ErrorKind {
    IO,
    BadConfig,
    FailedDownload,
}
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ErrorKind::IO => f.write_str("IO Error"),
            ErrorKind::BadConfig => f.write_str("Bad Config"),
            ErrorKind::FailedDownload => f.write_str("Download Failed"),
        }
    }
}

pub fn download_sdk(
    config: Result<Config, Box<dyn std::error::Error>>,
    options: &DownloadSdk,
) -> Result<(), WrappedError<ErrorKind>> {
    let spatial_lib_dir = match config {
        Ok(ref config) => config.spatial_lib_dir(),
        Err(_) => ::std::env::var("SPATIAL_LIB_DIR").ok()
    }.ok_or(WrappedError {
        kind: ErrorKind::BadConfig,
        msg: "'spatial_lib_dir' value must be set in the config, or the 'SPATIAL_LIB_DIR' environment variable must be set".into(),
        inner: None
    })?;

    let spatial_sdk_version = options
        .sdk_version
        .as_ref()
        .map_or_else(
            || config.map(|c| c.spatial_sdk_version),
            |ref v| Ok(v.to_string()),
        )
        .map_err(|e| {
            WrappedError {
            kind: ErrorKind::BadConfig,
            msg:
            "'spatial_sdk_version' must be set in the config, or provided as a command line argument"
                .into(),
            inner: Some(e),
        }
        })?;

    info!("Downloading packages into: {}", spatial_lib_dir);

    // Clean existing directory.
    if Path::new(&spatial_lib_dir).exists() {
        fs::remove_dir_all(&spatial_lib_dir).map_err(|e| WrappedError {
            kind: ErrorKind::IO,
            msg: format!("Failed to remove directory {}.", &spatial_lib_dir),
            inner: Some(Box::new(e)),
        })?;
    }

    fs::create_dir_all(&spatial_lib_dir).map_err(|e| WrappedError {
        kind: ErrorKind::IO,
        msg: format!("Failed to create directory {}.", &spatial_lib_dir),
        inner: Some(Box::new(e)),
    })?;
    trace!("Spatial lib directory cleaned.");

    for package in COMMON_PACKAGES {
        download_package(*package, &spatial_sdk_version, &spatial_lib_dir)?;
    }

    for package in PLATFORM_PACKAGES {
        download_package(*package, &spatial_sdk_version, &spatial_lib_dir)?;
    }

    if options.with_test_schema {
        download_package(
            SpatialPackageSource::Schema(SpatialSchemaPackage::ExhaustiveTestSchema),
            &spatial_sdk_version,
            &spatial_lib_dir,
        )?;
    }

    Ok(())
}

fn download_package(
    package_source: SpatialPackageSource,
    sdk_version: &str,
    spatial_lib_dir: &str,
) -> Result<(), WrappedError<ErrorKind>> {
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
        .map_err(|e| WrappedError {
            kind: ErrorKind::FailedDownload,
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
            Ok(err) => WrappedError {
                kind: ErrorKind::FailedDownload,
                msg: err,
                inner: None,
            },
            Err(e) => WrappedError {
                kind: ErrorKind::FailedDownload,
                msg: "Package download failed and stderr is not utf-8 compliant.".into(),
                inner: Some(Box::new(e)),
            },
        });
    }

    Ok(())
}

fn get_installer(
    download_url: &str,
    directory: &Path,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Download the installer.
    trace!("GET request to {}", download_url);
    let mut response = reqwest::blocking::get(download_url)?;

    let (mut dest, path) = {
        let fname = response
            .url()
            .path_segments()
            .and_then(::std::iter::Iterator::last)
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        trace!("Downloading {}", fname);
        let fname = directory.join(fname);
        trace!("Creating temporary file at: {:?}", fname);
        (File::create(fname.clone())?, fname)
    };

    // Copy the data in the response to the temporary file.
    copy(&mut response, &mut dest)?;

    Ok(path)
}

#[cfg(target_os = "linux")]
mod linux {
    pub fn download_cli() -> Result<(), Box<dyn std::error::Error>> {
        Err("Linux installer is unsupported. Follow the instructions here to install the Spatial CLI: https://docs.improbable.io/reference/latest/shared/setup/linux".to_owned().into())
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use log::info;
    use std::process;
    use tempfile;

    const DOWNLOAD_LOCATION: &str =
        "https://console.improbable.io/installer/download/stable/latest/win";

    pub fn download_cli() -> Result<(), Box<dyn std::error::Error>> {
        // Create a temporary directory. When this is dropped the directory is deleted.
        let tmp_dir = tempfile::TempDir::new()?;
        info!("Downloading installer.");
        let installer_path = super::get_installer(DOWNLOAD_LOCATION, tmp_dir.path())?;

        info!("Executing installer.");
        // Invoke the executable and wait for it to exit.
        let result = process::Command::new(installer_path).status();

        match result {
            Ok(status) => {
                if !status.success() {
                    return Err("Installer returned a non-zero exit code.".to_owned().into());
                }

                Ok(())
            }
            Err(e) => {
                if let Some(code) = e.raw_os_error() {
                    if code == 740 {
                        return Err("Installer requires elevated permissions to run. Please rerun in a terminal with elevated permissions.".to_owned().into());
                    }
                }

                Err(e.into())
            }
        }
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use log::info;
    use std::process;
    use tempfile;

    const DOWNLOAD_LOCATION: &str =
        "https://console.improbable.io/installer/download/stable/latest/mac";

    pub fn download_cli() -> Result<(), Box<dyn std::error::Error>> {
        // Create a temporary directory. When this is dropped the directory is deleted.
        let tmp_dir = tempfile::TempDir::new()?;
        info!("Downloading installer.");
        let installer_path = super::get_installer(DOWNLOAD_LOCATION, tmp_dir.path())?;

        info!("Executing installer.");
        let status = process::Command::new("installer")
            .arg("-pkg")
            .arg(installer_path)
            .args(&["-target", "/"])
            .status()?;

        if !status.success() {
            return Err("Installer returned a non-zero exit code.".to_owned().into());
        }

        Ok(())
    }
}
