#![forbid(unsafe_code)]

pub mod packages;

pub extern crate tokio;
extern crate jsonl;
extern crate log;
extern crate reqwest;
extern crate scraper;
extern crate ua_generator;
extern crate url;
extern crate num_cpus;

#[macro_use]
extern crate string_concat;

#[macro_use]
extern crate lazy_static;

pub use packages::spider::website::Website;

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
