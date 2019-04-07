// Conditionally re-export the correct version of download_cli dependent on platform.
#[cfg(target_os = "linux")]
pub use self::linux::*;
#[cfg(any(target_os = "windows", target_os = "macos"))]
pub use self::win_osx::*;

use crate::config::Config;
use crate::opt::DownloadSdk;
use log::*;
use std::{
    fs,
    path::{Path, PathBuf},
    process,
};

enum SpatialWorkerSdkPackage {
    CApiWin,
    CApiMac,
    CApiLinux,
}

impl SpatialWorkerSdkPackage {
    fn get_package_name(&self) -> &str {
        match self {
            SpatialWorkerSdkPackage::CApiWin => "c-static-x86_64-msvc_mt-win32",
            SpatialWorkerSdkPackage::CApiMac => "c-static-x86_64-clang_libcpp-macos",
            SpatialWorkerSdkPackage::CApiLinux => "c-static-x86_64-gcc_libstdcpp_pic-linux",
        }
    }

    fn get_relative_target_directory(&self) -> &str {
        match self {
            SpatialWorkerSdkPackage::CApiWin => "win",
            SpatialWorkerSdkPackage::CApiMac => "macos",
            SpatialWorkerSdkPackage::CApiLinux => "linux",
        }
    }
}

enum SpatialToolsPackage {
    SchemaCompilerWin,
    SchemaCompilerMac,
    SchemaCompilerLinux,
    SnapshotConverterWin,
    SnapshotConverterMac,
    SnapshotConverterLinux,
}

impl SpatialToolsPackage {
    fn get_package_name(&self) -> &str {
        match self {
            SpatialToolsPackage::SchemaCompilerWin => "schema_compiler-x86_64-win32",
            SpatialToolsPackage::SchemaCompilerMac => "schema_compiler-x86_64-macos",
            SpatialToolsPackage::SchemaCompilerLinux => "schema_compiler-x86_64-linux",
            SpatialToolsPackage::SnapshotConverterWin => "snapshot_converter-x86_64-win32",
            SpatialToolsPackage::SnapshotConverterMac => "snapshot_converter-x86_64-macos",
            SpatialToolsPackage::SnapshotConverterLinux => "snapshot_converter-x86_64-linux",
        }
    }

    fn get_relative_target_directory(&self) -> &str {
        match self {
            SpatialToolsPackage::SchemaCompilerWin
            | SpatialToolsPackage::SchemaCompilerMac
            | SpatialToolsPackage::SchemaCompilerLinux => "schema_compiler",
            SpatialToolsPackage::SnapshotConverterWin
            | SpatialToolsPackage::SnapshotConverterMac
            | SpatialToolsPackage::SnapshotConverterLinux => "snapshot_converter",
        }
    }
}

enum SpatialPackageSource {
    WorkerSdk(SpatialWorkerSdkPackage),
    Tools(SpatialToolsPackage),
    Schema,
}

impl SpatialPackageSource {
    fn get_package_name(&self) -> Vec<&str> {
        match self {
            SpatialPackageSource::WorkerSdk(package) => {
                vec!["worker_sdk", package.get_package_name()]
            }
            SpatialPackageSource::Tools(package) => vec!["tools", package.get_package_name()],
            SpatialPackageSource::Schema => vec!["schema", "standard_library"],
        }
    }

    fn get_relative_target_directory(&self) -> &str {
        match self {
            SpatialPackageSource::WorkerSdk(package) => package.get_relative_target_directory(),
            SpatialPackageSource::Tools(package) => package.get_relative_target_directory(),
            SpatialPackageSource::Schema => "std-lib",
        }
    }
}

// TODO: Allow users to specify which ones of these want? Linux is always required.
static COMMON_PACKAGES: &'static [SpatialPackageSource] = &[
    SpatialPackageSource::WorkerSdk(SpatialWorkerSdkPackage::CApiLinux),
    SpatialPackageSource::WorkerSdk(SpatialWorkerSdkPackage::CApiWin),
    SpatialPackageSource::WorkerSdk(SpatialWorkerSdkPackage::CApiMac),
    SpatialPackageSource::Schema,
];

#[cfg(target_os = "linux")]
static PLATFORM_PACKAGES: &'static [SpatialPackageSource] = &[
    SpatialPackageSource::Tools(SpatialToolsPackage::SchemaCompilerLinux),
    SpatialPackageSource::Tools(SpatialToolsPackage::SnapshotConverterLinux),
];

#[cfg(target_os = "windows")]
static PLATFORM_PACKAGES: &'static [SpatialPackageSource] = &[
    SpatialPackageSource::Tools(SpatialToolsPackage::SchemaCompilerWin),
    SpatialPackageSource::Tools(SpatialToolsPackage::SnapshotConverterWin),
];

#[cfg(target_os = "macos")]
static PLATFORM_PACKAGES: &'static [SpatialPackageSource] = &[
    SpatialPackageSource::Tools(SpatialToolsPackage::SchemaCompilerMac),
    SpatialPackageSource::Tools(SpatialToolsPackage::SnapshotConverterMac),
];

pub fn download_sdk(config: Result<Config, Box<dyn std::error::Error>>, options: &DownloadSdk) -> Result<(), Box<dyn std::error::Error>> {
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
        download_package(package, &spatial_sdk_version, &spatial_lib_dir)?;
    }

    for package in PLATFORM_PACKAGES {
        download_package(package, &spatial_sdk_version, &spatial_lib_dir)?;
    }

    Ok(())
}

fn download_package(
    package_source: &SpatialPackageSource,
    sdk_version: &str,
    spatial_lib_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Downloading {}",
        package_source.get_package_name().join(" ")
    );

    let mut output_path = PathBuf::new();
    output_path.push(spatial_lib_dir);
    output_path.push(package_source.get_relative_target_directory());

    let mut args = vec!["package", "retrieve"];

    args.extend(package_source.get_package_name());
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
        return Err("Failed to download package.")?;
    }

    Ok(())
}

#[cfg(target_os = "linux")]
mod linux {
    pub fn download_cli() -> Result<(), Box<dyn std::error::Error>> {
        Err("Linux installer is unsupported. Follow the instructions here to install the Spatial CLI: https://docs.improbable.io/reference/latest/shared/setup/linux".to_owned())?
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
mod win_osx {
    use log::*;
    use reqwest::get;
    use std::{fs::File, io::copy, path::Path, path::PathBuf, process};
    use tempfile;

    #[cfg(target_os = "windows")]
    const DOWNLOAD_LOCATION: &str =
        "https://console.improbable.io/installer/download/stable/latest/win";
    #[cfg(target_os = "macos")]
    const DOWNLOAD_LOCATION: &str =
        "https://console.improbable.io/installer/download/stable/latest/mac";

    pub fn download_cli() -> Result<(), Box<dyn std::error::Error>> {
        // Create a temporary directory. When this is dropped the directory is deleted.
        let tmp_dir = tempfile::TempDir::new()?;
        let installer_path = get_installer(tmp_dir.path())?;

        // Invoke the executable and wait for it to exit.
        let result = process::Command::new(installer_path).status()?;

        if !result.success() {
            return Err("Installer returned a non-zero exit code.".to_owned())?;
        }

        Ok(())
    }

    fn get_installer(directory: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
        // Download the installer.
        trace!("GET request to {}", DOWNLOAD_LOCATION);
        let mut response = get(DOWNLOAD_LOCATION)?;

        let (mut dest, path) = {
            let fname = response
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
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
}
