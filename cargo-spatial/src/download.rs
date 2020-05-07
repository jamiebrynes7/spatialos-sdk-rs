// Conditionally re-export the correct version of download_cli dependent on platform.
#[cfg(target_os = "linux")]
pub use self::linux::*;
#[cfg(target_os = "macos")]
pub use self::macos::*;
#[cfg(target_os = "windows")]
pub use self::windows::*;

use crate::{config::Config, errors::Error, opt::DownloadSdk};
use cargo_spatial_package_downloader::{
    download_package, SpatialPackageSource, SpatialSchemaPackage, SpatialToolsPackage,
    SpatialWorkerSdkPackage,
};
use log::*;
use std::{
    fmt::{Display, Formatter},
    fs,
    fs::File,
    io::copy,
    path::{Path, PathBuf},
};

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
) -> Result<(), Error<ErrorKind>> {
    let spatial_lib_dir = match config {
        Ok(ref config) => config.spatial_lib_dir(),
        Err(_) => ::std::env::var("SPATIAL_LIB_DIR").ok()
    }.ok_or(Error {
        kind: ErrorKind::BadConfig,
        msg: "'spatial_lib_dir' value must be set in the config, or the 'SPATIAL_LIB_DIR' environment variable must be set".into(),
        inner: None
    })?;

    let spatial_sdk_version = options
        .sdk_version
        .as_ref()
        .map_or_else(
            || config.map(|c| c.spatial_sdk_version),
            |ref v| Ok((*v).to_string()),
        )
        .map_err(|e| {
            Error {
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
        fs::remove_dir_all(&spatial_lib_dir).map_err(|e| Error {
            kind: ErrorKind::IO,
            msg: format!("Failed to remove directory {}.", &spatial_lib_dir),
            inner: Some(Box::new(e)),
        })?;
    }

    fs::create_dir_all(&spatial_lib_dir).map_err(|e| Error {
        kind: ErrorKind::IO,
        msg: format!("Failed to create directory {}.", &spatial_lib_dir),
        inner: Some(Box::new(e)),
    })?;
    trace!("Spatial lib directory cleaned.");

    for package in COMMON_PACKAGES {
        download_package(*package, &spatial_sdk_version, &spatial_lib_dir).map_err(|e| Error {
            kind: ErrorKind::FailedDownload,
            msg: "Failed to download package".to_owned(),
            inner: Some(Box::new(e)),
        })?;
    }

    for package in PLATFORM_PACKAGES {
        download_package(*package, &spatial_sdk_version, &spatial_lib_dir).map_err(|e| Error {
            kind: ErrorKind::FailedDownload,
            msg: "Failed to download package".to_owned(),
            inner: Some(Box::new(e)),
        })?;
    }

    if options.with_test_schema {
        download_package(
            SpatialPackageSource::Schema(SpatialSchemaPackage::ExhaustiveTestSchema),
            &spatial_sdk_version,
            &spatial_lib_dir,
        )
        .map_err(|e| Error {
            kind: ErrorKind::FailedDownload,
            msg: "Failed to download package".to_owned(),
            inner: Some(Box::new(e)),
        })?;
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
