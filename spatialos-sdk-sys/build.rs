use cargo_spatial_package_downloader::{
    download_package, SpatialPackageSource, SpatialWorkerSdkPackage,
};
use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

#[cfg(windows)]
const LIBS: [&str; 4] = ["improbable_worker", "RakNetLibStatic", "ssl", "zlibstatic"];
#[cfg(unix)]
const LIBS: [&str; 4] = ["improbable_worker", "RakNetLibStatic", "ssl", "z"];

#[cfg(target_os = "linux")]
static PACKAGE_DIR: &str = "linux";
#[cfg(target_os = "macos")]
static PACKAGE_DIR: &str = "macos";
#[cfg(target_os = "windows")]
static PACKAGE_DIR: &str = "win";

#[cfg(target_os = "linux")]
static PACKAGE: SpatialPackageSource =
    SpatialPackageSource::WorkerSdk(SpatialWorkerSdkPackage::CApiLinux);
#[cfg(target_os = "macos")]
static PACKAGE: SpatialPackageSource =
    SpatialPackageSource::WorkerSdk(SpatialWorkerSdkPackage::CApiMac);
#[cfg(target_os = "windows")]
static PACKAGE: SpatialPackageSource =
    SpatialPackageSource::WorkerSdk(SpatialWorkerSdkPackage::CApiWin);

fn main() -> Result<(), Box<dyn Error>> {
    let lib_dir = match env::var("SPATIAL_LIB_DIR") {
        Ok(s) => s,
        Err(_) => download_libs()?,
    };

    let package_dir = Path::new(&lib_dir).join(PACKAGE_DIR);

    println!("cargo:rustc-link-search={}", package_dir.to_str().unwrap());

    link_libs();

    Ok(())
}

fn link_libs() {
    for lib in LIBS.iter() {
        println!("cargo:rustc-link-lib=static={}", lib)
    }

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=dylib=gdi32");
        println!("cargo:rustc-link-lib=dylib=user32");
    }
}

fn download_libs() -> Result<String, Box<dyn Error>> {
    let mut target_dir_path = PathBuf::from(env::var("OUT_DIR")?);
    target_dir_path.push("spatial-libs");

    let target_dir = target_dir_path.to_str().unwrap().to_owned();

    if target_dir_path.exists() {
        return Ok(target_dir);
    }

    let version = env::var("CARGO_PKG_VERSION")?;

    download_package(PACKAGE, &version, &target_dir)?;
    Ok(target_dir)
}
