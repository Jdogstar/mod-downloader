use std::error::Error;
use std::process::{Command, Stdio};
use mt_logger::*;

pub fn unzip(archive_name: &str, path: &str) -> Result<(), Box<dyn Error>> {
    mt_log!(Level::Info, "Attempting to unzip {} at {}", archive_name, path);
    let unzipped = Command::new("7z")
            .arg("x")
            .arg(archive_name)
            .arg("-aoa")
            .arg("-o".to_owned() + path)
            .stdout(Stdio::null())
            .status();
    match unzipped {
        Ok(_) => {
            mt_log!(Level::Info, "Successfully unzipped archive {} at {}", archive_name, path);
            Ok(())
        },
        Err(err) => {
            mt_log!(Level::Error, "Unsuccessfully attemtped to unzipped archive {} at {}: {}", archive_name, path, err);
            Err(format!("Unsuccessfully attemtped to unzipped archive {} at {}: {}", archive_name, path, err).into())
        }
    }
}