use mods::get_mod::retrieve_mod;
use mods::mod_details::ModDetails;
use pathways::generate_dir;
use regex::Regex;
use std::path::Path;
use std::fs::{remove_file, write};
use std::{error::Error, ffi::OsStr};
use mt_logger::*;
mod unzip;
mod pathways;
pub mod mods;

pub fn get_mods(mod_details: &ModDetails) -> Result<(), Box<dyn Error>> {
    let mut resp;
    let mut attempts = 5;
    loop {
        match retrieve_mod(mod_details) {
            Ok(res) => {resp = res; break},
            Err(_error) => {attempts -= 1;}
        };
        if attempts < 0 {
            mt_log!(Level::Error, "Failed to retrieve mod {}", mod_details.url);
            return Err("Failed to retrieve mod".to_string().into());
        }
    }
    let ret = parse_filename(resp.headers());
    let mut buf: Vec<u8> = vec![];
    match resp.copy_to(&mut buf) {
        Ok(_) => mt_log!(Level::Info, "Successfully parsed file {} into bytes", mod_details.url),
        Err(err) => {
            mt_log!(Level::Error, "Couldn't parse file {} into bytes with err {}", mod_details.url, &err);
            return Err(format!("Couldn't parse file {} into bytes with err {}", mod_details.url, &err).into())
        }
    }
    mt_flush!().unwrap();
    let mut perm_file_name;
    match ret {
        Some(name) => perm_file_name = name,
        None => {
            match &mod_details.default_name {
                Some(name) => perm_file_name = name.to_string(),
                None =>  {
                    println!("ERROR: no filename from http content, must provide filename in config");
                    return Err(format!("ERROR: no filename from http content, must provide filename in config").into());
                }
            }
        }
    }
    if perm_file_name.contains(" ") {
        let words_in_path: Vec<&str> = perm_file_name.split_whitespace().collect();
        perm_file_name = words_in_path.join("_")
    }
    let path_ref = &format!("{}/{}", mod_details.mod_path, perm_file_name)[..];
    let path = Path::new(path_ref);
    let parent_path = Path::new(&mod_details.mod_path);
    generate_dir(parent_path).unwrap();
    match write(path, buf) {
        Ok(_) => mt_log!(Level::Info, "Successfully wrote file {}", mod_details.url),
        Err(err) => {
            mt_log!(Level::Error, "Couldn't write file {} with err {}", mod_details.url, &err);
            return Err(format!("Couldn't write file {} with err {}", mod_details.url, &err).into())
        }
    }
    if path.extension() == Some(OsStr::new("package")) || path.extension() == Some(OsStr::new("script")) {
        return Ok(());
    } else if path.extension() == Some(OsStr::new("zip")) || path.extension() == Some(OsStr::new("7z")) {
        mt_log!(Level::Info, "Archive detected, moving to unzip {:?} at {:?}", path, parent_path);
        unzip::unzip(path.to_str().unwrap(), parent_path.to_str().unwrap()).expect(format!("Failed to open archive {}", path.to_str().unwrap()).as_str());
        remove_file(path).expect("Failed to clean up zip file");
    } else {
        remove_file(path).expect("Failed to remove temp file");
        mt_log!(Level::Error, "ERROR: no acceptable extension for filename detected at url: {}, skipping .{:?}...", mod_details.url, path.extension().unwrap());
        return Err(format!("ERROR: no acceptable extension for filename detected at url: {}, skipping .{:?}...", mod_details.url, path.extension().unwrap()).into());
    }
    mt_flush!().unwrap();
    Ok(())
}

fn parse_filename(headers: &reqwest::header::HeaderMap) -> Option<String>{
    let re = Regex::new("filename=\"*([^\"]+)\"").unwrap();
    match headers.get("content-disposition") {
        Some(file_header) => {
            let file_txt = String::from(file_header.to_str().unwrap());
            let mat;
            match re.captures(&file_txt[..]) {
                Some(matching) => mat = matching,
                None => return None
            }
            match mat.get(1) {
                Some(group) => Some(String::from(group.as_str())),
                None => None,
            }
        },
        None => None,
    }
    
}
