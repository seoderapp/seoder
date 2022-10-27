#![forbid(unsafe_code)]

pub mod packages;

pub extern crate dirs;
extern crate log;
extern crate num_cpus;
extern crate reqwest;
extern crate scraper;
pub extern crate serde_json;
pub extern crate tokio;
extern crate ua_generator;
extern crate url;

#[macro_use]
pub extern crate string_concat;

#[macro_use]
extern crate lazy_static;
pub use packages::spider::website::Website;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::create_dir_all;
use tokio::sync::Mutex;
// use tokio::sync::{Receiver, Sender};
use tokio::sync::watch;
use tokio::sync::watch::Receiver;
use tokio::sync::watch::Sender;

lazy_static! {
    /// engines, files, config.txt, and the data directory
    pub static ref ENTRY_PROGRAM: (String, String, String, String) = {
        let data_dir = dirs::data_dir().unwrap().into_os_string().into_string().unwrap();

        (
            string_concat!(data_dir, "/seoder/engines/"),
            string_concat!(data_dir, "/seoder/files/"),
            string_concat!(data_dir, "/seoder/config.txt"),
            string_concat!(data_dir, "/seoder/"),
        )
    };
}

/// determine action
#[derive(PartialEq, Debug)]
pub enum Handler {
    Start,
    Pause,
    Shutdown,
    Resume,
}

lazy_static! {
    /// mutable sender across realms
    pub static ref SEND: Arc<Mutex<(Sender<(String, Handler)>, Receiver<(String, Handler)>)>> = Arc::new(Mutex::new(watch::channel(("handles".to_string(), Handler::Start))));
}

/// init entry dirs for prog
pub async fn init() {
    if !Path::new(&ENTRY_PROGRAM.0).is_dir() {
        create_dir_all(&ENTRY_PROGRAM.0).await.unwrap();
    }

    if !Path::new(&ENTRY_PROGRAM.1).is_dir() {
        create_dir_all(&ENTRY_PROGRAM.1).await.unwrap();
    }
    // copy files from build step TODO:
    if cfg!(debug_assertions) {
        let bs_url_input = string_concat!(ENTRY_PROGRAM.1, "urls-input.txt");

        if !Path::new(&bs_url_input).is_file() {
            if Path::new("../urls-input.txt").is_file() {
                std::fs::copy(&"../urls-input.txt", bs_url_input).unwrap();
            }
        }
    }
}

/// crawl executed with args vec list
pub async fn crawl(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // list of file paths to run against make sure it is ready each run
    init().await;

    if args.len() >= 2 {
        let mut iter = args.iter();
        iter.next(); // skip the cargo entry

        // todo: prevent file deleting if multiple files and append to list
        while let Some(input) = iter.next() {
            let mut website: Website = Website::new(&input);

            website.crawl().await;
        }
    } else {
        let mut website: Website = Website::new("urls-input.txt");

        website.crawl().await;
    }

    Ok(())
}
