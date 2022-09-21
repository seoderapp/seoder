#![forbid(unsafe_code)]

#[cfg(all(not(windows), not(target_os = "android"), feature = "jemalloc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use jsoncrawler_lib::{crawl, tokio};
use std::env;
use std::time::Instant;

/// web json crawler.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    // list of file paths to run against
    let args: Vec<String> = env::args().collect();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .build()
        .unwrap();

    // measure time for entire crawl
    let performance = Instant::now();

    rt.block_on(async {
        crawl(args).await.unwrap();
    });

    println!("Time elasped: {:?}", performance.elapsed()); //always stdoout time

    Ok(())
}
