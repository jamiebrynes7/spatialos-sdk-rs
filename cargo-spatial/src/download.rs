use reqwest::get;
use tempdir::TempDir;
use std::fs::File;
use std::io::copy;
use std::process;
use std::path::PathBuf;
use std::path::Path;

const DOWNLOAD_LOCATION: &str = "https://console.improbable.io/installer/download/stable/latest/win";

// TODO: logger
pub fn download_cli() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory. When this is dropped the directory is deleted.
    let tmp_dir = TempDir::new("spatialinstaller")?;
    let installer_path = get_installer(tmp_dir.path())?;

    // Invoke the executable and wait for it to exit.
    let mut command = process::Command::new(installer_path);

    // TODO: Log stdout and stderr to logger.
    let result = command.status()?;

    if !result.success() {
        return Err("Installer returned a non-zero exit code.".to_owned())?;
    }

    Ok(())
}

fn get_installer(directory: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Download the installer.
    let mut response = get(DOWNLOAD_LOCATION)?;

    let (mut dest, path) = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = directory.join(fname);
        println!("will be located under: '{:?}'", fname);
        (File::create(fname.clone())?, fname)
    };

    // Copy the data in the response to the temporary file.
    copy(&mut response, &mut dest)?;

    Ok(path)
}