// Conditionally re-export the correct version of download_cli dependent on platform.
#[cfg(target_os = "linux")]
pub use self::linux::*;
#[cfg(target_os = "macos")]
pub use self::macos::*;
#[cfg(target_os = "windows")]
pub use self::windows::*;

use crate::{config::Config, opt::DownloadSdk};
use anyhow::{anyhow, Context, Result};
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

pub fn download_sdk(config: Result<Config>, options: &DownloadSdk) -> Result<()> {
    let spatial_lib_dir = match config {
        Ok(ref config) => config.spatial_lib_dir(),
        Err(_) => ::std::env::var("SPATIAL_LIB_DIR").ok()
    }.ok_or_else(||anyhow!("'spatial_lib_dir' value must be set in the config, or the 'SPATIAL_LIB_DIR' environment variable must be set"))?;

    let spatial_sdk_version = options
        .sdk_version
        .as_ref()
        .map_or_else(
            || config.map(|c| c.spatial_sdk_version),
            |ref v| Ok((*v).to_string()),
        )
        .context("'spatial_sdk_version' must be set in the config, or provided as a command line argument")?;

    info!("Downloading packages into: {}", spatial_lib_dir);

    // Clean existing directory.
    if Path::new(&spatial_lib_dir).exists() {
        fs::remove_dir_all(&spatial_lib_dir)
            .with_context(|| format!("Failed to remove directory '{}'", &spatial_lib_dir))?;
    }

    fs::create_dir_all(&spatial_lib_dir)
        .with_context(|| format!("Failed to create directory: '{}'", &spatial_lib_dir))?;
    trace!("Spatial lib directory cleaned.");

    for package in COMMON_PACKAGES.iter().chain(PLATFORM_PACKAGES) {
        download_package(*package, &spatial_sdk_version, &spatial_lib_dir).with_context(|| {
            format!(
                "Failed to download package: {:?} @ version: '{}' to '{}'",
                package, &spatial_sdk_version, &spatial_lib_dir
            )
        })?;
    }

    if options.with_test_schema {
        let package = SpatialPackageSource::Schema(SpatialSchemaPackage::ExhaustiveTestSchema);
        download_package(package, &spatial_sdk_version, &spatial_lib_dir).with_context(|| {
            format!(
                "Failed to download package: {:?} @ version: '{}' to '{}'",
                package, &spatial_sdk_version, &spatial_lib_dir
            )
        })?;
    }

    Ok(())
}

fn get_installer(download_url: &str, directory: &Path) -> Result<PathBuf> {
    // Download the installer.
    trace!("GET request to {}", download_url);
    let mut response = reqwest::blocking::get(download_url)
        .with_context(|| format!("Failed to complete GET request to: '{}'", download_url))?;

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

        let file = File::create(fname.clone())
            .with_context(|| format!("Failed to create file: '{}'", fname.display()));
        (file?, fname)
    };

    // Copy the data in the response to the temporary file.
    copy(&mut response, &mut dest)
        .with_context(|| format!("Failed to copy response data to '{}'", path.display()))?;

    Ok(path)
}

#[cfg(target_os = "linux")]
mod linux {
    use anyhow::{anyhow, Result};

    pub fn download_cli() -> Result<()> {
        Err(anyhow!("Linux installer is unsupported. Follow the instructions here to install the Spatial CLI: https://docs.improbable.io/reference/latest/shared/setup/linux"))
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use anyhow::{anyhow, Context, Result};
    use log::info;
    use std::process;
    use tempfile;

    const DOWNLOAD_LOCATION: &str =
        "https://console.improbable.io/installer/download/stable/latest/win";

    pub fn download_cli() -> Result<()> {
        // Create a temporary directory. When this is dropped the directory is deleted.
        let tmp_dir = tempfile::TempDir::new().context("Failed to create temporary directory")?;
        info!("Downloading installer.");
        let installer_path = super::get_installer(DOWNLOAD_LOCATION, tmp_dir.path())
            .context("Failed to download installer.")?;

        info!("Executing installer.");
        // Invoke the executable and wait for it to exit.
        let status = process::Command::new(installer_path).status().map_err(|e| {
            if let Some(code) = e.raw_os_error() {
                if code == 740 {
                    return anyhow!("Installer requires elevated permissions to run. Please rerun in a terminal with elevated permissions.");
                }
            }

            e.into()
        }).context("Failed to invoke installer.")?;

        if !status.success() {
            return Err(anyhow!(
                "Installer returned a non-zero exit code: '{}'",
                status.code().unwrap()
            ));
        }

        Ok(())
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use anyhow::{anyhow, Context, Result};
    use log::info;
    use std::process;
    use tempfile;

    const DOWNLOAD_LOCATION: &str =
        "https://console.improbable.io/installer/download/stable/latest/mac";

    pub fn download_cli() -> Result<()> {
        // Create a temporary directory. When this is dropped the directory is deleted.
        let tmp_dir = tempfile::TempDir::new().context("Failed to create temporary directory")?;
        info!("Downloading installer.");
        let installer_path = super::get_installer(DOWNLOAD_LOCATION, tmp_dir.path())
            .context("Failed to download installer.")?;

        info!("Executing installer.");
        let status = process::Command::new("installer")
            .arg("-pkg")
            .arg(installer_path)
            .args(&["-target", "/"])
            .status()
            .context("Failed to run installer")?;

        if !status.success() {
            return Err(anyhow!(
                "Installer returned a non-zero exit code: '{}'",
                status.code().unwrap()
            ));
        }

        Ok(())
    }
}
