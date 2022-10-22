use crate::ENTRY_PROGRAM;

use super::configuration::{setup, Configuration};
use super::fs::store_fs_io_matching;
use super::utils::fetch_page_html;
use super::utils::log;
use super::ResponseOutFileType;

use reqwest::header::HeaderMap;
use reqwest::Client;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::Semaphore;
use tokio::task;
use tokio_stream::StreamExt;

/// Represents a a web crawler for gathering links.
/// ```rust
/// use seoder_lib::packages::spider::website::Website;
/// let mut localhost = Website::new("urls-input.txt");
/// localhost.crawl();
/// ```
#[derive(Debug)]
pub struct Website {
    /// configuration properties for website.
    pub configuration: Configuration,
    /// Path to list of files.
    pub path: String,
    /// custom engine to run
    pub engine: Engine,
}

/// link, (res, code), spawned
pub type Message = (String, (String, ResponseOutFileType), bool);

lazy_static! {
    /// application global configurations
    pub static ref CONFIG: (String, Duration, bool, (bool, bool), Engine) = setup(false);
}

#[derive(Debug, Default, Clone)]
/// a boxed metric run, enabled if name found  is set
pub struct Campaign {
    /// campaign name
    pub name: String,
    /// custom target paths
    pub paths: Vec<String>,
    /// custom target patterns
    pub patterns: Vec<String>,
    /// look in source code to find matches
    pub source_match: bool,
}

#[derive(Debug, Default, Clone)]
/// custom application engine
pub struct Engine {
    /// campaign engine conf
    pub campaign: Campaign,
}

impl Website {
    /// Initialize Website object with a start link to crawl.
    pub fn new(path: &str) -> Self {
        Self {
            configuration: Configuration::new(),
            path: if !path.is_empty() {
                path.to_string()
            } else {
                "urls-input.txt".to_string()
            },
            engine: Engine::default(),
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
                log("headers.txt file does not exist", "");
            }
        };

        let mut client = Client::builder()
            .default_headers(headers)
            .user_agent(if !&self.configuration.user_agent.is_empty() {
                &self.configuration.user_agent
            } else {
                ua_generator::ua::spoof_ua()
            })
            .tcp_keepalive(None)
            .pool_max_idle_per_host(0)
            // .proxy(proxy)
            .brotli(true)
            .gzip(true)
            .use_native_tls()
            .tcp_nodelay(false)
            .connect_timeout(CONFIG.1.div_f32(1.8))
            .timeout(CONFIG.1);

        let (proxy, tor) = CONFIG.3;

        if tor {
            let proxy = reqwest::Proxy::all("socks5://127.0.0.1:9150").unwrap();
            client = client.proxy(proxy);
        }

        if proxy {
            match File::open(string_concat!(&ENTRY_PROGRAM.3, "proxies.txt")).await {
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
                    log("proxies.txt file does not exist", "");
                }
            };
        }

        client.build().unwrap_or_default()
    }

    /// setup config for crawl
    pub async fn setup(&mut self) -> Client {
        self.configure_http_client().await
    }

    /// Start to crawl with async conccurency
    pub async fn crawl(&mut self) {
        let client = self.setup().await;

        self.crawl_concurrent(client).await;
    }

    /// Start to crawl website concurrently using gRPC callback
    async fn crawl_concurrent(&mut self, client: Client) {
        let spawn_limit = if CONFIG.2 {
            num_cpus::get() / 2
        } else {
            // todo: use custom stats to determine number
            33 * num_cpus::get()
        };

        // hard main spawn limit
        let global_thread_count = Arc::new(Mutex::new(0));
        // global counter clone
        let thread_count = global_thread_count.clone();

        let p = self.path.to_owned();

        let path_names = if !self.engine.campaign.paths.is_empty() {
            self.engine.campaign.paths.to_owned()
        } else if !CONFIG.4.campaign.paths.is_empty() {
            CONFIG.4.campaign.paths.to_owned()
        } else {
            let mut vc = Vec::new();
            vc.push(String::from(""));

            vc
        };

        let mut st = tokio_stream::iter(path_names);

        // full
        let (tx, rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) = unbounded_channel();

        let txxx = tx.clone();

        let handle = tokio::spawn(async move {
            while let Some(path) = st.next().await {
                // soft
                let (txx, mut rxx): (UnboundedSender<Message>, UnboundedReceiver<Message>) =
                    unbounded_channel();

                let txxx = txxx.clone();
                let tx = tx.clone();
                let p = p.clone();
                let client = client.clone();
                let client_sem = client.clone();
                let thread_count = thread_count.clone();

                let path = path.clone();
                let path1 = path.clone();

                let f = File::open(string_concat!(&ENTRY_PROGRAM.1, &p)).await.unwrap();

                task::spawn(async move {
                    let reader = BufReader::new(f);
                    let mut lines = reader.lines();

                    while let Some(link) = lines.next_line().await.unwrap() {
                        if *thread_count.lock().unwrap() < spawn_limit {
                            *thread_count.lock().unwrap() += 1;

                            let client = client.clone();
                            let tx = tx.clone();
                            let link = link.clone();
                            let path = path.clone();

                            task::spawn(async move {
                                let json = fetch_page_html(&link, &client, &path).await;
                                if let Err(_) = tx.send((link, json, true)) {
                                    log("receiver dropped", "");
                                }
                            });
                        } else {
                            if let Err(_) =
                                txx.send((link, ("".into(), ResponseOutFileType::Unknown), false))
                            {
                                log("receiver dropped", "");
                            }
                        }
                    }

                    // end channels
                    drop(tx);
                    drop(txx);
                });

                let semaphore = Arc::new(Semaphore::new(spawn_limit / 4)); // 4x less than spawns
                let mut join_handles = Vec::new();

                let soft_spawn = task::spawn(async move {
                    while let Some(i) = rxx.recv().await {
                        let (link, _, __) = i;

                        let txxx = txxx.clone();
                        let client = client_sem.clone();
                        let link = link.clone();
                        let permit = semaphore.clone().acquire_owned().await.unwrap();
                        let path = path1.clone();

                        join_handles.push(tokio::spawn(async move {
                            let json = fetch_page_html(&link, &client, &path).await;
                            if let Err(_) = txxx.send((link, json, false)) {
                                log("receiver dropped", "");
                            }
                            drop(permit);
                        }));
                    }

                    for handle in join_handles {
                        handle.await.unwrap();
                    }
                });

                task::yield_now().await;

                soft_spawn.await.unwrap();
            }
            drop(tx);
        });

        store_fs_io_matching(&self.engine.campaign, rx, global_thread_count).await;

        handle.await.unwrap();
    }
}

#[tokio::test]
async fn crawl() {
    let mut website: Website = Website::new("urls-input.txt");
    website.crawl().await;
    let f = File::open(website.path).await.unwrap();

    assert_eq!(f.metadata().await.unwrap().len() > 1, true);
}
