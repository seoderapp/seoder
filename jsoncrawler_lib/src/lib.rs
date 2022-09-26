#![forbid(unsafe_code)]

pub mod packages;

extern crate jsonl;
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

/// crawl executed with args vec list
pub async fn crawl(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // list of file paths to run against
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
