use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap();

    let mut libs = get_platform_libs(&target);

    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let package_dir = if target.contains("windows") {
        "dependencies/win/lib"
    } else if target.contains("apple") {
        "dependencies/macos/lib"
    } else if target.contains("linux") {
        "dependencies/linux/lib"
    } else {
        panic!("Unsupported build platform: {}", target);
    };
    
    let package_dir = Path::new(&cargo_dir).join(package_dir);

    println!("cargo:rustc-link-search={}", package_dir.to_str().unwrap());

    for lib in libs {
        println!("cargo:rustc-link-lib=static={}", lib)
    }

    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}

fn get_platform_libs(target: &String) -> Vec<&str> {
    if target.contains("windows") {
        vec![
            "worker",
            "grpc++",
            "grpc",
            "gpr",
            "libprotobuf",
            "RakNetLibStatic",
            "ssl",
            "zlibstatic",
        ]
    } else {
        vec![
            "worker",
            "grpc++",
            "grpc",
            "gpr",
            "protobuf",
            "RakNetLibStatic",
            "ssl",
            "z",
        ]
    }
}
