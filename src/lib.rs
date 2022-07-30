use regex::Regex;
use std::path::Path;
use std::fs::{remove_file, write};
use std::{error::Error, ffi::OsStr};
use reqwest::header::USER_AGENT;
use reqwest::blocking;
use mt_logger::*;
mod unzip;

fn retrieve_mod(mod_url: &String) -> Result<blocking::Response, Box<dyn Error>> {
    let client = blocking::Client::new();
    client.get(mod_url).header(USER_AGENT, "hyper/0.5.2").send().expect("Something went wrong pinging the download.");
    let mod_http = client.get(mod_url).header(USER_AGENT, "hyper/0.5.2").send();
    let resp;
    match mod_http {
        Ok(res) => resp = res,
        Err(err) => {
            mt_log!(Level::Info, "{} with error: {}", mod_url, err);
            return Err(format!("{} with error: {}", mod_url, err).into());
        }
    }
    match resp.status().as_u16() {
        200..=299 => mt_log!(Level::Info, "Successful request to {}", mod_url),
        _ => {
            mt_log!(Level::Error, "Unsuccessful request to {}", mod_url);
            return Err(format!("Unsuccessful request to {}", mod_url).into());
        }
    }
    match resp.headers().get("content-type") {
        Some(cont_type) => mt_log!(Level::Info, "Content type {:?}", cont_type),
        None => {
            mt_log!(Level::Error, "No content type for {}", mod_url);
            return Err(format!("Unsuccessful request to {}", mod_url).into());
        }
    }
    Ok(resp)
}

pub fn get_mod(mod_details: (&String, &String, &Option<String>)) -> Result<(), Box<dyn Error>> {
    let (mod_path, mod_url, file_name) = mod_details;
    let mut resp;
    let mut attempts = 5;
    loop {
        match retrieve_mod(mod_url) {
            Ok(res) => {resp = res; break},
            Err(_error) => {attempts -= 1;}
        };
        if attempts < 0 {
            mt_log!(Level::Error, "Failed to retrieve mod {}", &mod_url);
            return Err("Failed to retrieve mod".to_string().into());
        }
    }
    let ret = parse_filename(resp.headers());
    let mut buf: Vec<u8> = vec![];
    match resp.copy_to(&mut buf) {
        Ok(_) => mt_log!(Level::Info, "Successfully parsed file {} into bytes", &mod_url),
        Err(err) => {
            mt_log!(Level::Error, "Couldn't parse file {} into bytes with err {}", &mod_url, &err);
            return Err(format!("Couldn't parse file {} into bytes with err {}", &mod_url, &err).into())
        }
    }
    mt_flush!().unwrap();
    let mut perm_file_name;
    match ret {
        Some(name) => perm_file_name = name,
        None => {
            match file_name {
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
    let path_ref = &format!("{}/{}", mod_path, perm_file_name)[..];
    let path = Path::new(path_ref);
    let parent_path = Path::new(mod_path);
    match write(path, buf) {
        Ok(_) => mt_log!(Level::Info, "Successfully wrote file {}", &mod_url),
        Err(err) => {
            mt_log!(Level::Error, "Couldn't write file {} with err {}", &mod_url, &err);
            return Err(format!("Couldn't write file {} with err {}", &mod_url, &err).into())
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
        mt_log!(Level::Error, "ERROR: no acceptable extension for filename detected at url: {}, skipping .{:?}...", mod_url, path.extension().unwrap());
        return Err(format!("ERROR: no acceptable extension for filename detected at url: {}, skipping .{:?}...", mod_url, path.extension().unwrap()).into());
    }
    mt_flush!().unwrap();
    Ok(())
}

fn parse_filename (headers: &reqwest::header::HeaderMap) -> Option<String>{
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
