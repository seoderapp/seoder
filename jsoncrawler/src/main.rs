#![forbid(unsafe_code)]

#[cfg(all(not(windows), not(target_os = "android"), feature = "jemalloc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use jsoncrawler_lib::tokio;
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

    jsoncrawler_lib::crawl(args).await.unwrap();

    println!("Time elasped: {:?}", performance.elapsed()); //always stdoout time

    Ok(())
}
