#![forbid(unsafe_code)]

pub mod packages;

extern crate jsonl;
extern crate log;
extern crate reqwest;
extern crate scraper;
pub extern crate tokio;
extern crate ua_generator;
extern crate url;

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

    Ok(())
}
