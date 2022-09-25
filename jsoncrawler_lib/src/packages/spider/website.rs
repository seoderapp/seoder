use super::configuration::{setup, Configuration};
use super::fs::{store_fs_io, store_fs_io_matching};
use super::utils::fetch_page_html;
use super::utils::log;
use super::JsonOutFileType;

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
    /// custom engine to run
    pub engine: Engine,
}

/// link, (res, code), spawned
pub type Message = (String, (String, JsonOutFileType), bool);

lazy_static! {
    /// application global configurations
    pub static ref CONFIG: (String, Duration, usize, bool, Engine) = setup();
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
}

#[derive(Debug, Default, Clone)]
/// custom application engine
pub struct Engine {
    /// campaign engine conf
    pub campaign: Campaign,
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
            .tcp_keepalive(None)
            .pool_max_idle_per_host(0)
            .brotli(true)
            .gzip(true)
            .use_native_tls()
            .tcp_nodelay(false)
            .connect_timeout(CONFIG.1.div_f32(1.8))
            .timeout(CONFIG.1);

        // if proxy enabled build proxies
        if CONFIG.3 {
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
        }

        client.build().unwrap_or_default()
    }

    /// setup config for crawl
    pub async fn setup(&mut self) -> Client {
        self.configure_http_client().await
    }

    /// Start to crawl website with async conccurency
    pub async fn crawl(&mut self) {
        let client = self.setup().await;

        self.crawl_concurrent(client).await;
    }

    /// Start to crawl website concurrently using gRPC callback
    async fn crawl_concurrent(&mut self, client: Client) {
        let spawn_limit = CONFIG.2 * num_cpus::get();
        // full
        let (tx, rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) = unbounded_channel();
        // soft
        let (txx, mut rxx): (UnboundedSender<Message>, UnboundedReceiver<Message>) =
            unbounded_channel();

        let txxx = tx.clone();

        // hard main spawn limit
        let global_thread_count = Arc::new(Mutex::new(0));
        // global counter clone
        let thread_count = global_thread_count.clone();

        let client_sem = client.clone();

        let p = self.path.to_owned();

        task::spawn(async move {
            // todo: check if file exist in multi paths
            let f = File::open(&p).await.unwrap();

            let reader = BufReader::new(f);
            let mut lines = reader.lines();

            while let Some(link) = lines.next_line().await.unwrap() {
                if *thread_count.lock().unwrap() < spawn_limit {
                    *thread_count.lock().unwrap() += 1;

                    let tx = tx.clone();
                    let client = client.clone();

                    task::spawn(async move {
                        let json = fetch_page_html(&link, &client).await;
                        if let Err(_) = tx.send((link, json, true)) {
                            log("receiver dropped", "");
                        }
                    });
                } else {
                    if let Err(_) = txx.send((link, ("".into(), JsonOutFileType::Unknown), false)) {
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

                let permit = semaphore.clone().acquire_owned().await.unwrap();

                join_handles.push(tokio::spawn(async move {
                    let json = fetch_page_html(&link, &client).await;
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

        if std::env::var("ENGINE_FD").is_ok() {
            store_fs_io_matching(
                &self.engine.campaign.name.to_string(),
                self.engine.campaign.patterns.to_owned(),
                rx,
                global_thread_count,
            )
            .await;
        } else {
            store_fs_io(
                (
                    &self.jsonl_output_path,
                    &self.ok_txt_output_path,
                    &self.okv_txt_output_path,
                    &self.cr_txt_output_path,
                    &self.al_txt_output_path,
                ),
                rx,
                global_thread_count,
            )
            .await;
        }

        soft_spawn.await.unwrap();
    }
}

#[tokio::test]
async fn crawl() {
    let mut website: Website = Website::new("urls-input.txt");
    website.crawl().await;
    let f = File::open(website.path).await.unwrap();

    assert_eq!(f.metadata().await.unwrap().len() > 1, true);
}
