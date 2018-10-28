use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    let mut libs = get_platform_libs(&target);
    
    let package_dir = if target.contains("windows") {
        "dependencies/win/lib"
    } else if target.contains("apple") {
        "dependencies/macos/lib"
    } else if target.contains("linux") {
        "dependencies/linux/lib"
    } else {
        panic!("Unsupported build platform: {}", target);
    };

    println!("cargo:rustc-link-search={}", package_dir);

    for lib in libs {
        println!("cargo:rustc-link-lib=static={}", lib)
    }

    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}

fn get_platform_libs(target: &String) -> Vec<&str>{
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
