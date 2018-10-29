#[macro_use]
extern crate lazy_static;

use std::env;
use std::path::{Path, PathBuf};

#[cfg(windows)]
lazy_static! {
    static ref LIBS: Vec<&'static str> = vec![
            "worker",
            "grpc++",
            "grpc",
            "gpr",
            "libprotobuf",
            "RakNetLibStatic",
            "ssl",
            "zlibstatic",
        ];
}

#[cfg(unix)]
lazy_static! {
    static ref LIBS: Vec<&'static str> = vec![
            "worker",
            "grpc++",
            "grpc",
            "gpr",
            "protobuf",
            "RakNetLibStatic",
            "ssl",
            "z",
        ];
}

#[cfg(target_os = "linux")]
static PACKAGE_DIR: &str = "dependencies/linux/lib";
#[cfg(target_os = "macos")]
static PACKAGE_DIR: &str = "dependencies/macos/lib";
#[cfg(target_os = "windows")]
static PACKAGE_DIR: &str = "dependencies/win/lib";

fn main() {
    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let package_dir = Path::new(&cargo_dir).join(PACKAGE_DIR);

    println!("cargo:rustc-link-search={}", package_dir.to_str().unwrap());

    for lib in LIBS.iter() {
        println!("cargo:rustc-link-lib=static={}", lib)
    }
    
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=dylib=c++");
    
    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
