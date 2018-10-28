extern crate zip;

use std::fs;
use std::io;
use std::path;
use std::process;

fn main() {
    let sdk_version = "13.3.0";

    download_and_unpack(
        SpatialPackageSource::WorkerSdk,
        "c-static-x86_64-msvc_md-win32",
        sdk_version,
        "dependencies/win",
    ).expect("Could not download package");
    download_and_unpack(
        SpatialPackageSource::WorkerSdk,
        "c-static-x86_64-clang_libcpp-macos",
        sdk_version,
        "dependencies/macos",
    ).expect("Could not download package");
    download_and_unpack(
        SpatialPackageSource::WorkerSdk,
        "c-static-x86_64-gcc_libstdcpp_pic-linux",
        sdk_version,
        "dependencies/linux",
    ).expect("Could not download package");
}

enum SpatialPackageSource {
    WorkerSdk,
    Tools,
    Schema,
}

impl SpatialPackageSource {
    fn to_str(&self) -> &str {
        use SpatialPackageSource::*;

        match *self {
            WorkerSdk => "worker_sdk",
            Tools => "tools",
            Schema => "schema",
        }
    }
}

/// Downloads and unpacks a Spatial package into a specified directory.
///
/// * package_source        - the source of the package
/// * package_name          - the name of the package
/// * sdk_version           - the Spatial SDK version
/// * target_directory      - the target directory to unpack to
///
/// * returns               - an error if the operation could not be completed, empty otherwise
fn download_and_unpack(
    package_source: SpatialPackageSource,
    package_name: &str,
    sdk_version: &str,
    target_directory: &str,
) -> Result<(), io::Error> {
    let current_dir = std::env::current_dir().expect("Could not find current working directory.");

    // Clean target directory.
    fs::remove_dir_all(target_directory)?;
    fs::create_dir_all(target_directory)?;

    // Create temporary directory.
    let mut tmp_dir = current_dir.clone();
    tmp_dir.push("tmp");
    fs::create_dir_all(&tmp_dir)?;

    let mut tmp_file = tmp_dir.clone();
    tmp_file.push(package_name);

    println!("Downloading {}.", package_name);
    download_package(
        package_source,
        package_name,
        sdk_version,
        tmp_file.to_str().unwrap(),
    );

    println!("Unpacking {} to {}.", package_name, target_directory);
    unpack_package(tmp_file.to_str().unwrap(), target_directory)?;

    // Clean temp directory.
    fs::remove_dir_all(&tmp_dir)?;

    Ok(())
}

/// Downloads a Spatial package through the spatial CLI.
///
/// * package_source    - the package source, i.e - worker_sdk, tools, schema.
fn download_package(
    package_source: SpatialPackageSource,
    package_name: &str,
    sdk_version: &str,
    target_file: &str,
) {
    let args = vec![
        "package",
        "retrieve",
        package_source.to_str(),
        package_name,
        sdk_version,
        target_file,
    ];

    let out = process::Command::new("spatial")
        .args(args)
        .output()
        .expect("Could not run spatial package retrieve");

    if !out.status.success() {
        let stdout = match String::from_utf8(out.stdout) {
            Ok(v) => v,
            Err(e) => panic!(
                "Could not decode stdout from spatial command with error: {}",
                e
            ),
        };

        let stderr = match String::from_utf8(out.stderr) {
            Ok(v) => v,
            Err(e) => panic!(
                "Could not decode stderr from spatial command with error: {}",
                e
            ),
        };

        panic!(
            "spatial package retrieve returned a non-zero error code.\n Stdout: {}\nStderr: {}",
            stdout, stderr
        );
    }
}

/// Unpacks a zip archive into a directory.
///
/// * target_package_path    - the absolute path of the package archive.
/// * target_directory       - the absolute path of the target directory.
///
/// * result                 - an IO error if the operation could not be completed, empty otherwise.
fn unpack_package(target_package_path: &str, target_directory: &str) -> Result<(), io::Error> {
    // Prepare target directory.
    fs::remove_dir_all(target_directory)?;
    fs::create_dir_all(target_directory)?;

    // Unpack zip archive
    let fname = path::Path::new(target_package_path);
    let file = fs::File::open(&fname)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        let mut output_path = path::PathBuf::new();
        output_path.push(target_directory);
        output_path.push(file.sanitized_name());

        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&output_path)?;
        } else {
            if let Some(p) = output_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }

            let mut outfile = fs::File::create(&output_path)?;
            io::copy(&mut file, &mut outfile)?;

            #[cfg(any(unix))]
            {
                use std::os::unix::fs::PermissionsExt;

                let metadata = outfile.metadata()?;
                let mut permissions = metadata.permissions();
                // TODO: Check if is binary before setting executable?
                permissions.set_mode(0o774);
            }
        }
    }

    Ok(())
}
