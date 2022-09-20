#![warn(missing_docs)]

//! Website crawling library that rapidly crawls all pages to
//! gather links in concurrently
//!
//! JsonCrawler is multi-threaded crawler that can be configured
//! to handle a large list and write to disk. It has the ability to gather
//! tens of thousands of pages within seconds.
//!
//! # How to use Spider
//!
//! Example crawling with JsonCrawler:
//!
//! - **Concurrent** is the fastest way to start crawling a web page and
//!   typically the most efficient.
//!   - [`crawl`] is used to crawl concurrently :blocking.
//!
//! [`crawl`]: website/struct.Website.html#method.crawl
//!
//! # Basic usage
//!
//! First, you will need to add `jsoncrawler` to your `Cargo.toml`.
//!
//! Next, simply add the website url in the struct of website and crawl,
//! you can also crawl sequentially.

/// Configuration structure for `Website`.
pub mod configuration;
/// Application utils.
pub mod utils;
/// A website to crawl.
pub mod website;

/// file output determination
#[derive(PartialEq)]
pub enum JsonOutFileType {
    /// response is ok and valid - ok-valid_json.txt
    Valid,
    /// response is ok but not valid - ok-not_valid_json.txt
    Invalid,
    /// response connection error and not valid - connection_error.txt
    Error,
    /// response returned unknown status code - all-others.txt
    Unknown,
}
