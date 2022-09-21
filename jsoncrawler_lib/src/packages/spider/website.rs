use super::configuration::Configuration;
use super::utils::fetch_page_html;
use super::utils::log;
use super::JsonOutFileType;

use jsonl::write;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use tokio;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::task;

/// Represents a a web crawler for gathering links.
/// ```rust
/// use jsoncrawler_lib::packages::spider::website::Website;
/// let mut localhost = Website::new("urls-input.txt");
/// localhost.crawl();
/// ```
#[derive(Debug)]
pub struct Website {
    /// configuration properties for website.
    pub configuration: Configuration,
    /// Path to list of files.
    pub path: String,
    /// Path to jsonl output.
    pub jsonl_output_path: String,
    /// Path to ok txt output.
    pub ok_txt_output_path: String,
    /// Path to ok txt invalid output.
    pub okv_txt_output_path: String,
    /// Path to connection error txt output.
    pub cr_txt_output_path: String,
    /// Path to all other outputs.
    pub al_txt_output_path: String,
}

type Message = (String, (String, JsonOutFileType));

lazy_static! {
    /// application global configurations
    pub static ref CONFIG: (&'static str, Duration, usize) = setup();
}

/// configure application program api path, timeout, and channel buffer
fn setup() -> (&'static str, Duration, usize) {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let mut query = 1;
    let mut timeout: u64 = 15;
    let mut buffer: usize = 100;

    // read through config file cpu bound quickly to avoid atomics and extra memory from clones
    match File::open("config.txt") {
        Ok(file) => {
            let reader = BufReader::new(file);
            let lines = reader.lines();

            for line in lines {
                let line = line.unwrap_or_default();
                if !line.is_empty() {
                    let hh = line.split(" ").collect::<Vec<&str>>();

                    if hh.len() == 2 {
                        let cf = hh[0];
                        let v = hh[1];
                        // query config
                        if cf == "query" && !v.is_empty() {
                            // validate acceptable queries
                            match v {
                                "posts" => query = 1,
                                "pages" => query = 2,
                                "users" => query = 3,
                                "comments" => query = 4,
                                "search" => query = 5,
                                _ => {
                                    log("not valid config file {}", "");
                                }
                            };
                        }

                        if cf == "timeout" && !v.is_empty() {
                            timeout = v.parse::<u64>().unwrap_or(15);
                        }

                        if cf == "buffer" && !v.is_empty() {
                            buffer = v.parse::<usize>().unwrap_or(100);
                        }
                    }
                }
            }
        }
        Err(_) => {
            log("config.txt file does not exist {}", "");
        }
    };

    // reverse query dip
    let query = match query {
        1 => "/wp-json/wp/v2/posts?per_page=100",
        2 => "/wp-json/wp/v2/pages?per_page=100",
        3 => "/wp-json/wp/v2/users?per_page=100",
        4 => "/wp-json/wp/v2/comments?per_page=100",
        5 => "/wp-json/wp/v2/search?per_page=100",
        _ => "/wp-json/wp/v2/posts?per_page=100",
    };

    (query, Duration::new(timeout, 0), buffer)
}

impl Website {
    /// Initialize Website object with a start link to crawl.
    pub fn new(domain: &str) -> Self {
        Self {
            configuration: Configuration::new(),
            path: domain.to_string(),
            jsonl_output_path: "output.jsonl".to_string(),
            ok_txt_output_path: "ok-valid_json.txt".to_string(),
            okv_txt_output_path: "ok-not_valid_json.txt".to_string(),
            cr_txt_output_path: "connection_error.txt".to_string(),
            al_txt_output_path: "all-others.txt".to_string(),
        }
    }

    /// configure http client
    async fn configure_http_client(&mut self) -> Client {
        let mut headers = HeaderMap::new();

        match File::open("headers.txt").await {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut lines = reader.lines();

                while let Some(header) = lines.next_line().await.unwrap() {
                    if !header.is_empty() {
                        let hh = header.split(" ").collect::<Vec<&str>>();

                        if hh.len() == 2 {
                            let key =
                                reqwest::header::HeaderName::from_bytes(hh[0].as_bytes()).unwrap();
                            let val =
                                reqwest::header::HeaderValue::from_bytes(hh[1].as_bytes()).unwrap();

                            headers.insert(key, val);
                        }
                    }
                }
            }
            Err(_) => {
                log("headers.txt file does not exist {}", "");
            }
        };

        let mut client = Client::builder()
            .default_headers(headers)
            .user_agent(if !&self.configuration.user_agent.is_empty() {
                &self.configuration.user_agent
            } else {
                ua_generator::ua::spoof_ua()
            })
            .brotli(true)
            .gzip(true)
            .timeout(CONFIG.1);

        match File::open("proxies.txt").await {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut lines = reader.lines();

                while let Some(proxy) = lines.next_line().await.unwrap() {
                    if !proxy.is_empty() {
                        client = client.proxy(reqwest::Proxy::http::<&str>(&proxy).unwrap());
                    }
                }
            }
            Err(_) => {
                log("proxies.txt file does not exist {}", "");
            }
        };

        client.build().unwrap_or_default()
    }

    /// setup config for crawl
    pub async fn setup(&mut self) -> Client {
        let client = self.configure_http_client().await;

        client
    }

    /// create a new file at path
    async fn create_file(&self, path: &String) -> File {
        File::create(&path).await.unwrap()
    }

    /// Start to crawl website with async conccurency
    pub async fn crawl(&mut self) {
        let client = self.setup().await;

        self.crawl_concurrent(client).await;
    }

    /// Start to crawl website concurrently using gRPC callback
    async fn crawl_concurrent(&mut self, client: Client) {
        let (tx, mut rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) =
            unbounded_channel();

        // json output file
        let mut o = self.create_file(&self.jsonl_output_path).await;

        let fpath = self.path.to_owned();

        task::spawn(async move {
            // file to get crawl list [todo] validate error
            let f = File::open(&fpath).await.unwrap();
            let reader = BufReader::new(f);
            let mut lines = reader.lines();
            
            let tx = tx.clone();

            // todo: use channel buffer to break loop and set while
            // // stream the files to next line and spawn read efficiently
            while let Some(link) = lines.next_line().await.unwrap() {
                log("gathering json {}", &link);
                let tx = tx.clone();
                let client = client.clone();

                task::spawn(async move {
                    let json = fetch_page_html(&link, &client).await;

                    if let Err(_) = tx.send((link, json)) {
                        log("receiver dropped", "");
                    }
                });
            }

            drop(tx);
        });

        // output txt files
        let (mut ok_t, mut okv_t, mut ce_t, mut al_t) = tokio::join!(
            self.create_file(&self.ok_txt_output_path),
            self.create_file(&self.okv_txt_output_path),
            self.create_file(&self.cr_txt_output_path),
            self.create_file(&self.al_txt_output_path)
        );

        while let Some(i) = rx.recv().await {
            let (link, jor) = i;
            let (json, oo) = jor;

            let link = string_concat!(link, "\n");

            if oo == JsonOutFileType::Error {
                ce_t.write(&link.as_bytes()).await.unwrap();
            }

            if oo == JsonOutFileType::Unknown {
                al_t.write(&link.as_bytes()).await.unwrap();
            }

            if json == "" {
                continue;
            }

            let j: Value = serde_json::from_str(&json).unwrap_or_default();

            if !j.is_null() {
                match write(&mut o, &j).await {
                    Ok(_) => {
                        log("wrote jsonl = {}", &link);
                        ok_t.write(&link.as_bytes()).await.unwrap();
                    }
                    _ => {
                        log("failed to write jsonl = {}", &link);
                        okv_t.write(&link.as_bytes()).await.unwrap();
                    }
                }
            } else {
                log("The file is not valid json = {}", &link);
                okv_t.write(&link.as_bytes()).await.unwrap();
            }
        }
    }
}

#[tokio::test]
async fn crawl() {
    let mut website: Website = Website::new("urls-input.txt");
    website.crawl().await;
    let f = File::open(website.path).await.unwrap();

    assert_eq!(f.metadata().await.unwrap().len() > 1, true);
}