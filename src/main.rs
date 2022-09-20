#![forbid(unsafe_code)]

pub mod packages;

extern crate jsonl;
extern crate log;
extern crate reqwest;
extern crate scraper;
extern crate tokio;
extern crate ua_generator;
extern crate url;

#[macro_use]
extern crate lazy_static;

pub use packages::spider::website::Website;
use std::env;
use std::time::Instant;

/// web json crawler.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // list of file paths to run against
    let args: Vec<String> = env::args().collect();
    // measure time for entire crawl
    let performance = Instant::now();

    if args.len() >= 2 {
        let mut iter = args.iter();
        iter.next(); // skip the cargo entry

        while let Some(input) = iter.next() {
            let input = input.clone();

            tokio::join!(async move {
                let mut website: Website = Website::new(&input);

                website.crawl().await;
            });
        }
    } else {
        let mut website: Website = Website::new("urls-input.txt");

        website.crawl().await;
    }

    println!("Time elasped: {:?}", performance.elapsed()); //always stdoout time

    Ok(())
}
