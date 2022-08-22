use std::error::Error;
use std::path::Path;
use std::fs::metadata;
use std::time::SystemTime;
use chrono::DateTime;
use hyper::HeaderMap;
use reqwest::header::USER_AGENT;
use reqwest::blocking;
use mt_logger::*;

use super::mod_details::ModDetails;

pub fn retrieve_mod(mod_details: &ModDetails) -> Result<blocking::Response, Box<dyn Error>> {
    let client = blocking::Client::new();
    let mod_headers = client.head(&mod_details.url).header(USER_AGENT, "hyper/0.5.2").send().unwrap();
    check_if_up_to_date(&mod_headers.headers(), &mod_details.mod_path);
    let mod_http = client.get(&mod_details.url).header(USER_AGENT, "hyper/0.5.2").send();
    let resp;
    match mod_http {
        Ok(res) => resp = res,
        Err(err) => {
            mt_log!(Level::Info, "{} with error: {}", &mod_details.url, err);
            return Err(format!("{} with error: {}", &mod_details.url, err).into());
        }
    }
    match resp.status().as_u16() {
        200..=299 => mt_log!(Level::Info, "Successful request to {}", &mod_details.url),
        _ => {
            mt_log!(Level::Error, "Unsuccessful request to {}", &mod_details.url);
            return Err(format!("Unsuccessful request to {}", &mod_details.url).into());
        }
    }
    match resp.headers().get("content-type") {
        Some(cont_type) => mt_log!(Level::Info, "Content type {:?}", cont_type),
        None => {
            mt_log!(Level::Error, "No content type for {}", &mod_details.url);
            return Err(format!("Unsuccessful request to {}", &mod_details.url).into());
        }
    }
    Ok(resp)
}

pub fn check_if_up_to_date<P: AsRef<Path> + std::fmt::Debug + ?Sized>(headers: &HeaderMap, file: &P) -> bool {
    let last_modded_header = match headers.get("last-modified") {
        Some(last) => last.to_str().unwrap(),
        None => return false,
    };
    let rfc_2822 = DateTime::parse_from_rfc2822(last_modded_header).unwrap();
    let mut file_last: SystemTime = SystemTime::UNIX_EPOCH;
    if file.as_ref().exists() {
        file_last = match metadata(file) {
            Ok(meta) => {
                match meta.modified() {
                    Ok(modified) => modified,
                    Err(_) => SystemTime::UNIX_EPOCH,
                }
            }
            Err(_) => SystemTime::UNIX_EPOCH,
        }
    }
    if file_last >= (SystemTime::from(rfc_2822)) {
        return true
    }
    false
}