extern crate csv;
#[macro_use]
extern crate serde_derive;
use threadpool::ThreadPool;
use std::{vec::Vec, error::Error, path::Path};
use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use mt_logger::*;
use mod_downloader::mods::mod_details::ModDetails;

/// A simple program to download mods
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the csv config file
    #[clap(short, long)]
    path: String,

    /// Number of threads to give the pool
    #[clap(short, long, default_value_t = 4)]
    count: usize,
}

#[derive(Debug, Deserialize)]
struct Record {
    path: String,
    url: String,
    #[serde(default)]
    file_name:Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    mt_new!(None, Level::Info, OutputStream::File);
    let mut mods: Vec<Record> = Vec::new();
    let args = Args::parse();
    let config_file = Path::new(&args.path);
    if config_file.exists() {
        let mut rdr = csv::ReaderBuilder::new().has_headers(false).flexible(true).from_path(&config_file)?;
        for result in rdr.deserialize() {
            let record: Record = result?;
            mods.push(record);
        }
        let pool = ThreadPool::new(args.count);
        let bar = MultiProgress::new();
        for rec in mods{
            let pb = bar.add(ProgressBar::new_spinner());
            pb.set_style(ProgressStyle::default_spinner().template("{prefix:.bold.dim} {spinner} {wide_msg}"));
            pb.enable_steady_tick(300);
            pool.execute(move || {
                pb.set_message(format!("Downloading and saving: {}", &rec.url));
                let mod_details = ModDetails::new(rec.file_name, rec.url, rec.path);
                let res = mod_downloader::get_mods(&mod_details);
                match res {
                    Ok(_) => pb.finish_and_clear(),
                    Err(err) => {
                        pb.finish_and_clear();
                        println!("{}", err);
                    }
                }
            });
        }
        bar.join().unwrap();
        pool.join();
    } else {
        mt_log!(Level::Fatal, "Config file {:?} was not found", config_file)
    }
    mt_log!(Level::Info, "Finished program");
    mt_flush!().unwrap();
    Ok(())
}
