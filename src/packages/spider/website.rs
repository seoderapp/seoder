use super::configuration::Configuration;
use super::utils::log;
use super::utils::fetch_page_html;

// use hashbrown::HashSet;
// use rayon::prelude::*;
use reqwest::header::CONNECTION;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::time::Duration;
use tokio;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task;
use tokio::io::{self, BufReader, AsyncBufReadExt};
use tokio::fs::File;

/// Represents a website to crawl and gather all pages.
/// ```rust
/// use website_crawler::spider::website::Website;
/// let mut localhost = Website::new("http://example.com");
/// localhost.crawl();
/// ```
#[derive(Debug)]
pub struct Website {
    /// configuration properties for website.
    pub configuration: Configuration,
    /// Path to list of files.
    path: Option<String>,
}

type Message = (String, String);

impl Website {
    /// Initialize Website object with a start link to crawl.
    pub fn new(domain: &str) -> Self {
        Self {
            configuration: Configuration::new(),
            path: Some(domain.to_string())
        }
    }

    /// configure http client
    fn configure_http_client(&mut self) -> Client {
        lazy_static! {
            static ref HEADERS: HeaderMap<HeaderValue> = {
                let mut headers = HeaderMap::new();
                headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));

                headers
            };
        }
        Client::builder()
            .default_headers(HEADERS.clone())
            .user_agent(&self.configuration.user_agent)
            .brotli(true)
            .timeout(Duration::new(15, 0))
            .build()
            .unwrap_or_default()
    }

    /// setup config for crawl
    pub async fn setup(&mut self) -> Client {
        let client = self.configure_http_client();

        client
    }

    /// Start to crawl website with async parallelization
    pub async fn crawl(&mut self) {
        let client = self.setup().await;

        self.crawl_concurrent(&client).await;
    }

    /// Start to crawl website concurrently using gRPC callback
    async fn crawl_concurrent(&mut self, client: &Client) {
        // file to run for page
        let f = File::open(self.path.as_ref().unwrap()).await.unwrap();
        let reader = BufReader::new(f);
        let mut lines = reader.lines();
        let (tx, mut rx): (Sender<Message>, Receiver<Message>) = channel(50);

        // stream the files to next line and spawn read efficiently
        while let Some(link) = lines.next_line().await.unwrap() {
            log("gathering json {}", &link);
            let tx = tx.clone();
            let client = client.clone();
            let l = link.to_owned();

            task::spawn(async move {
                let json = fetch_page_html(&l, &client).await;  
                                      
                if let Err(_) = tx.send((l, json)).await {
                    log("receiver dropped", "");
                }
            });
        }
        
        drop(tx);

        while let Some(i) = rx.recv().await {
            let (link, _) = i;
            log("got = {}", link);
        }
    }

}

#[tokio::test]
async fn crawl() {
    let mut website: Website = Website::new("https://choosealicense.com");
    website.crawl().await;
}

#[tokio::test]
async fn not_crawl_blacklist() {
    let mut website: Website = Website::new("https://choosealicense.com");
    website
        .configuration
        .blacklist_url
        .push("https://choosealicense.com/licenses/".to_string());
    website.crawl().await;
}

#[tokio::test]
async fn test_respect_robots_txt() {
    let mut website: Website = Website::new("https://stackoverflow.com");
    website.configuration.user_agent = "*".into();

    let client = website.setup().await;

    assert_eq!(website.configuration.delay, 250);

    assert!(!website.is_allowed(&"https://stackoverflow.com/posts/".to_string()));

    // test match for bing bot
    let mut website_second: Website = Website::new("https://www.mongodb.com");
    website_second.configuration.user_agent = "bingbot".into();

    let client_second = website_second.setup().await;

    assert_eq!(website_second.configuration.delay, 60000); // should equal one minute in ms

    // test crawl delay with wildcard agent [DOES not work when using set agent]
    let mut website_third: Website = Website::new("https://www.mongodb.com");
    let client_third = website_third.setup().await;

    assert_eq!(website_third.configuration.delay, 10000); // should equal 10 seconds in ms
}

#[tokio::test]
async fn test_link_duplicates() {
    fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + std::hash::Hash,
    {
        let mut uniq = HashSet::new();
        iter.into_iter().all(move |x| uniq.insert(x))
    }

    let mut website: Website = Website::new("http://0.0.0.0:8000");
    website.crawl().await;
}
