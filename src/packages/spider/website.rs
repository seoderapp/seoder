use super::configuration::Configuration;
use super::utils::log;
use super::utils::fetch_page_html;

use reqwest::header::CONNECTION;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::time::Duration;
use tokio;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::task;
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::fs::File;
use jsonl::write;
use serde_json::Value;

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
    pub path: Option<String>,
    /// Path to jsonl output.
    pub jsonl_output_path: String,
}

type Message = (String, String);

impl Website {
    /// Initialize Website object with a start link to crawl.
    pub fn new(domain: &str) -> Self {
        Self {
            configuration: Configuration::new(),
            path: Some(domain.to_string()),
            jsonl_output_path: "output.jsonl".to_string()
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
        let mut o = File::create(&self.jsonl_output_path).await.unwrap();

        let reader = BufReader::new(f);
        let mut lines = reader.lines();
        let (tx, mut rx): (Sender<Message>, Receiver<Message>) = channel(25);

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
            let (link, json) = i;
            
            if json == "" {
                // write connection_error.txt
                continue;
            }

            // parse json todo: validate start json chars to prevent non json (perf)
            let j: Value = serde_json::from_str(&json).unwrap_or_default();

            // if valid json write to file
            if !j.is_null() {
                match write(&mut o, &j).await {
                    Ok(_) => {
                        log("wrote jsonl = {}", link);
                        // write into ok-valid_json.txt
                    },
                    _ => {
                        log("failed to write jsonl = {}", link);
                        // write all-others.txt
                    },
                }
            } else {
                log("The file is not valid json = {}", &link);
                // write ok-not_valid_json.txt 
            }
        }
    }

}

#[tokio::test]
async fn crawl() {
    let mut website: Website = Website::new("urls-input.txt");
    website.crawl().await;
    let f = File::open(website.path.as_ref().unwrap()).await.unwrap();

    assert_eq!(
        f.metadata().await.unwrap().len() > 1,
        true
    );
}