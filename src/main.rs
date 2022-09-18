
#![forbid(unsafe_code)]

pub mod packages;

#[macro_use]
extern crate lazy_static;

extern crate tokio;
extern crate ua_generator;
extern crate log;
extern crate reqwest;
extern crate scraper;
extern crate url;
extern crate jsonl;

pub use packages::spider::website::Website;
use std::time::Instant;

/// web json crawler.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let performance = Instant::now();
    let mut website: Website = Website::new("urls-input.txt");
    website.crawl().await;
    packages::spider::utils::log("Time elasped: {}", format!("{:?}", performance.elapsed()));

    Ok(())
}
