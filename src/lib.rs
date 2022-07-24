// def get_mod(mod_details):
//     """Function to run in thread to handle each mod.

//     Threading function to download the mod using requests.
//     It then takes certain actions based on the file extension.
//     Args:
//         mod_details: a tuple holding the url, absolute path to save to,
//         and the optional filename. Filename is None
//         if it's not provided in the tuple.
//     """
//     # get the absolute path
//     mod_path = Path(mod_details[1])
//     # grab the file from the url
//     mod_http = requests.get(mod_details[0])
//     # seperate into content header and the actual file bytes
//     content = mod_http.content
//     condis = mod_http.headers['content-disposition']
//     # get the filename
//     filename = get_filename(condis)
//     # if there is no filename from the content header
//     if not filename:
//         # if there is no optional name either, print error, skip mod
//         if not mod_details[2]:
//             print("ERROR: no filename from http content, " +
//                   "must provide filename in csv")
//             return
//         else:
//             # else use optional name provided in the .csv
//             filename = mod_details[2]
//     # full path to save mod to
//     full_path = mod_path / filename
//     # get filename extension
//     file_extension = Path(filename).suffix
//     # write the mod file to the save path
//     with open(full_path, 'wb') as mod_file:
//         mod_file.write(content)
//     # if it's a simple script or package, just return, job done
//     if file_extension in (".ts4script", ".package"):
//         return
//     # if it's a zip or 7z, use the archive module to extract files in the zip
//     elif file_extension in (".zip", ".7z"):
//         Archive(full_path).extractall(mod_path)
//         # remove the original zip as it's no longer needed
//         os.remove(full_path)
//         return
//     # report unaccounted for file extension, but keep the download
//     else:
//         print("ERROR: Unaccounted for file extension")
//     return
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
    Ok(resp)
}

pub fn get_mod(mod_details: (&String, &String, &Option<String>)) -> Result<(), Box<dyn Error>> {
    let (mod_path, mod_url, file_name) = mod_details;
    let resp;
    let mut attempts = 5;
    loop {
        match retrieve_mod(mod_url) {
            Ok(res) => {resp = res; break},
            Err(_error) => {attempts -= 1;}
        };
        if attempts > -1 {
            mt_log!(Level::Error, "Failed to retrieve mod {}", &mod_url);
            return Err("Failed to retrieve mod".to_string().into());
        }
    }
    let ret = parse_filename(resp.headers());
    let content;
    match resp.bytes() {
        Ok(cont) => content = cont,
        Err(err) => {mt_log!(Level::Error, "Couldn't parse file {} into bytes with err {}, exiting...", &mod_url, &err);
                            return Err(format!("Couldn't parse file {} into bytes with err {}, exiting...", &mod_url, &err).into());}
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
    write(path, content).expect("Failed to write file, exiting...");
    if path.extension() == Some(OsStr::new("package")) || path.extension() == Some(OsStr::new("script")) {
        return Ok(());
    } else if path.extension() == Some(OsStr::new("zip")) {
        // let source = File::open(path).expect("Couldn't open the archive");
        // zip_extract::extract(&source, &parent_path, false).expect("Couldn't unzip archive...");
        unzip::unzip(path.to_str().unwrap(), parent_path.to_str().unwrap()).expect(format!("Failed to open archive {}", path.to_str().unwrap()).as_str());
        remove_file(path).expect("Failed to clean up zip file");
    } else {
        remove_file(path).expect("Failed to remove temp file");
        return Err(format!("ERROR: no acceptable extension for filename detected at url: {}, skipping .{:?}...", mod_url, path.extension().unwrap()).into());
    }
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
