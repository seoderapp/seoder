
#![forbid(unsafe_code)]

pub mod packages;

#[macro_use]
extern crate lazy_static;

extern crate tokio;
extern crate ua_generator;
extern crate hashbrown;
extern crate log;
extern crate rayon;
extern crate reqwest;
extern crate scraper;
extern crate url;

pub use packages::spider::website::Website;
use std::time::Instant;

/// web json crawler.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let performance = Instant::now();
    let mut website: Website = Website::new("crawl-list.txt");
    website.configuration.respect_robots_txt = false;
    website.configuration.delay = 0;
    website.crawl().await;
    println!("Time elasped: {:?}", performance.elapsed());
    // todo: store output or process amid crawl
    // if we got all the urls and wrote here 50,mill pages would be expensive

    Ok(())
}
