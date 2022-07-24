use std::error::Error;
use std::process::{Command, Stdio};

pub fn unzip(archive_name: &str, path: &str) -> Result<(), Box<dyn Error>> {
    if cfg!(target_os = "windows") {
        Command::new("7za")
                .arg("x")
                .arg(archive_name)
                .arg("-aoa")
                .arg("-o".to_owned() + path)
                .stdout(Stdio::null())
                .status()
                .expect("failed to decompress archive")
    } else {
        Command::new("7z")
                .arg("x")
                .arg(archive_name)
                .arg("-aoa")
                .arg("-o".to_owned() + path)
                .stdout(Stdio::null())
                .status()
                .expect("failed to decompress archive")
    };
    Ok(())
}