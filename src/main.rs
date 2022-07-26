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
extern crate csv;
#[macro_use]
extern crate serde_derive;
use threadpool::ThreadPool;
use std::{vec::Vec, error::Error, path::Path};
use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use mt_logger::*;

/// A simple program to download sims mod
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

#[derive(Debug,Deserialize)]
struct Record {
    path: String,
    url: String,
    file_name:Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    mt_new!(None, Level::Info, OutputStream::File);
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
        let bar = MultiProgress::new();
        for rec in mods{
            let pb = bar.add(ProgressBar::new_spinner());
            pb.set_style(ProgressStyle::default_spinner().template("{prefix:.bold.dim} {spinner} {wide_msg}"));
            pb.enable_steady_tick(300);
            pool.execute(move || {
                pb.set_message(format!("Downloading and saving: {}", &rec.url));
                let res = mod_downloader::get_mod((&rec.path, &rec.url, &rec.file_name));
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
    mt_flush!().unwrap();
    Ok(())
}
