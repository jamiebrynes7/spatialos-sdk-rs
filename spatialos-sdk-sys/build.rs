#[macro_use]
extern crate lazy_static;

use std::env;
use std::path::Path;

#[cfg(windows)]
lazy_static! {
    static ref LIBS: Vec<&'static str> = vec![
        "improbable_worker",
        "RakNetLibStatic",
        "ssl",
        "zlibstatic",
    ];
}

#[cfg(unix)]
lazy_static! {
    static ref LIBS: Vec<&'static str> = vec![
        "improbable_worker",
        "RakNetLibStatic",
        "ssl",
        "z",
    ];
}

#[cfg(target_os = "linux")]
static PACKAGE_DIR: &str = "linux";
#[cfg(target_os = "macos")]
static PACKAGE_DIR: &str = "macos";
#[cfg(target_os = "windows")]
static PACKAGE_DIR: &str = "win";

fn main() {
    let lib_dir = match env::var("SPATIAL_LIB_DIR") {
        Ok(s) => s,
        Err(_) => panic!("SPATIAL_LIB_DIR environment variable not set."),
    };

    let package_dir = Path::new(&lib_dir).join(PACKAGE_DIR);

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
