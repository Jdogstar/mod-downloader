// def main():
//     """Main driver for threads to download mods."""
//     mod_list = []
//     # grab config file in same folder as main.py
//     with open('config.csv') as csvfile:
//         mods = csv.reader(csvfile)
//         # for row in the csv file
//         for mod in mods:
//             # if it has all three components, tuple them and add to list
//             if len(mod) > 2:
//                 mod_list.append((mod[0].strip(), mod[1].strip(), mod[2].strip()))
//             # else if it has two components, tuple them with the filename as none
//             elif len(mod) == 2:
//                 mod_list.append((mod[0].strip(), mod[1].strip(), None))
//             # else complain about the format for that particular row, but continue regardless
//             else:
//                 print("ERROR: missing information for mod entry, please " +
//                       "ensure it's in the following form: ")
//                 print("https://fakeurl.somedownloadlink, " +
//                       "C://fullpathtoplacedownload, optional_filename")
//     with ThreadPoolExecutor() as process:
//         # map each row to a thread from the threadpool
//         process.map(get_mod, mod_list)
mod lib;
extern crate csv;
#[macro_use]
extern crate serde_derive;
use lib::*;
use threadpool::ThreadPool;
use std::{vec::Vec, error::Error, path::Path};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

/// Simple program to download sims mod
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the config file
    #[clap(short, long)]
    path: String,

    /// Number of threads to run
    #[clap(short, long, default_value_t = 4)]
    count: usize,
}

#[derive(Debug,Deserialize)]
struct Record {
    path: String,
    url: String,
    file_name:Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut mods: Vec<Record> = Vec::new();
    let args = Args::parse();
    let config_file = Path::new(&args.path);
    if config_file.exists() {
        let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_path(&config_file)?;
        for result in rdr.deserialize() {
            let record: Record = result?;
            mods.push(record);
        }
        let pool = ThreadPool::new(args.count);
        let bar = ProgressBar::new(mods.len().try_into().unwrap());
        bar.set_style(ProgressStyle::default_bar().template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}"));
        for rec in mods{
            bar.inc(1);
            bar.set_message(format!("Downloading {}", &rec.url));
            pool.execute(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(get_mod((&rec.path, &rec.url, &rec.file_name)));
            });
        }
        pool.join();
        bar.finish();
    } else {
        println!("Config file does not exist.")
    }
    Ok(())
}
