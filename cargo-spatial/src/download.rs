// Conditionally re-export the correct version of download_cli dependent on platform.
#[cfg(target_os = "linux")]
pub use self::linux::*;
#[cfg(target_os = "macos")]
pub use self::macos::*;
#[cfg(target_os = "windows")]
pub use self::windows::*;

use crate::{config::Config, opt::DownloadSdk};
use log::*;
use reqwest::get;
use std::fs::File;
use std::io::copy;
use std::{
    fs,
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

pub fn download_sdk(
    config: Result<Config, Box<dyn std::error::Error>>,
    options: &DownloadSdk,
) -> Result<(), Box<dyn std::error::Error>> {
    let spatial_lib_dir = match config {
        Ok(ref config) => config.spatial_lib_dir().ok_or("spatial_lib_dir value must be set in the config, or the SPATIAL_LIB_DIR environment variable must be set")?,
        Err(_) => ::std::env::var("SPATIAL_LIB_DIR")?
    };

    let spatial_sdk_version = match options.sdk_version {
        Some(ref version) => version.clone(),
        None => config?.spatial_sdk_version,
    };

    info!("Downloading packages into: {}", spatial_lib_dir);

    // Clean existing directory.
    if Path::new(&spatial_lib_dir).exists() {
        fs::remove_dir_all(&spatial_lib_dir)?;
    }

    fs::create_dir_all(&spatial_lib_dir)?;
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
) -> Result<(), Box<dyn std::error::Error>> {
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

    let process = process::Command::new("spatial").args(args).output()?;

    if !process.status.success() {
        let stdout = String::from_utf8(process.stdout)?;
        let stderr = String::from_utf8(process.stderr)?;
        trace!("{}", stdout);
        trace!("{}", stderr);
        return Err("Failed to download package.".into());
    }

    Ok(())
}

fn get_installer(
    download_url: &str,
    directory: &Path,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Download the installer.
    trace!("GET request to {}", download_url);
    let mut response = get(download_url)?;

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
