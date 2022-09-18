#![forbid(unsafe_code)]

pub mod packages;

extern crate jsonl;
extern crate log;
extern crate reqwest;
extern crate scraper;
extern crate tokio;
extern crate ua_generator;
extern crate url;
extern crate num_cpus;

pub use packages::spider::website::Website;
use std::time::Instant;

/// web json crawler.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let performance = Instant::now();
    let mut website: Website = Website::new("domains.txt");
    website.crawl().await;
    println!("Time elasped: {:?}", performance.elapsed()); //always stdoout time

    Ok(())
}
