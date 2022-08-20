use mt_logger::*;
use std::{fs, path::Path, error::Error};

pub fn generate_dir<P: AsRef<Path> + std::fmt::Debug + ?Sized>(pathway: &P) -> Result<(), Box<dyn Error>> {
    if pathway.as_ref().exists() {
        return Ok(())
    }
    match fs::create_dir_all(pathway) {
        Ok(_) => {
            mt_log!(Level::Info, "Path {:?} created", pathway);
            Ok(())
        },
        Err(_) => {
            mt_log!(Level::Error, "Path {:?} failed to be created", pathway);
            Err(format!("Path {:?} failed to be created", pathway).into())
        },
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::generate_dir;

    #[test]
    fn test_creation() {
        let pathway = Path::new(r"C:\Users\joshu\Documents\programming\mod-downloader\test_folder\RVSN");
        generate_dir(r"C:\Users\joshu\Documents\programming\mod-downloader\test_folder\RVSN").expect("Failed to create dir");
        assert!(pathway.exists(), "Pathway does not exist")
    }
}