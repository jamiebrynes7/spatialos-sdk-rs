extern crate cargo_spatial;

use std::path::Path;

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

fn main() {

    cargo_spatial::download::download_sdk_version("./dependecies/".to_string(), "14.1.0".to_string(), false).ok();
    let lib_dir  = "./dependecies";

    let package_dir = Path::new(&lib_dir).join(PACKAGE_DIR).canonicalize().ok().unwrap();
    

    println!("cargo:rustc-link-search={}", package_dir.to_str().unwrap());

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
