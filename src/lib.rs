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
use compress_tools::*;
use std::fs::{File, remove_file, write};
use std::ffi::OsStr;
use reqwest::header::USER_AGENT;
use reqwest::Client;

pub async fn get_mod(mod_details: (&String, &String, &Option<String>)) -> () {
    let (mod_path, mod_url, file_name) = mod_details;
    let client = Client::new();
    client.get(mod_url).header(USER_AGENT, "hyper/0.5.2").send().await.expect("Something went wrong pinging download.");
    let mod_http = client.get(mod_url).header(USER_AGENT, "hyper/0.5.2").send().await;
    let resp;
    match mod_http {
        Ok(res) => resp = res,
        Err(err) => {
            println!("{} with error: {}\nSkipping...", mod_url, err);
            return
        }
    }
    let ret = parse_filename(resp.headers()).await;
    let content = resp.bytes().await.expect("Couldn't parse file into bytes, exiting...");
    let perm_file_name;
    match ret {
        Some(name) => perm_file_name = name,
        None => {
            match file_name {
                Some(name) => perm_file_name = name.to_string(),
                None =>  {
                    println!("ERROR: no filename from http content, must provide filename in config");
                    return
                }
            }
        }
    }
    let path_ref = &format!("{}/{}", mod_path, perm_file_name)[..];
    let path = Path::new(path_ref);
    let parent_path = Path::new(mod_path);
    write(path, content).expect("Failed to write file, exiting...");
    if path.extension() == Some(OsStr::new("package")) || path.extension() == Some(OsStr::new("script")) {
        return
    } else if path.extension() == Some(OsStr::new("zip")) || path.extension() == Some(OsStr::new("7z")) {
        let mut source = File::open(path).expect("Couldn't open zipfile");
        uncompress_archive(&mut source, &parent_path, Ownership::Ignore).expect("Couldn't unzip zipfile");
        remove_file(path).expect("Failed to clean up zip file");
    } else {
        println!("ERROR: no acceptable extension for filename detected, skipping...");
        remove_file(path).expect("Failed to remove temp file");
    }
}

async fn parse_filename (headers: &reqwest::header::HeaderMap) -> Option<String>{
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
