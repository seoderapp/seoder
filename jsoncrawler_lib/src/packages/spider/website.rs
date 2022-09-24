use super::configuration::{setup, Configuration};
use super::utils::fetch_page_html;
use super::utils::log;
use super::JsonOutFileType;

use jsonl::write;
use reqwest::header::HeaderMap;
use reqwest::Client;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
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
}

// link, (res, code), spawned
type Message = (String, (String, JsonOutFileType), bool);

lazy_static! {
    /// application global configurations
    pub static ref CONFIG: (String, Duration, usize, bool) = setup();
}

/// create a new file at path
async fn create_file(path: &String) -> File {
    File::create(&path).await.unwrap()
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
        let (tx, mut rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) =
            unbounded_channel();
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

        // output txt files
        let (mut o, mut ok_t, mut okv_t, mut ce_t, mut al_t) = tokio::join!(
            create_file(&self.jsonl_output_path),
            create_file(&self.ok_txt_output_path),
            create_file(&self.okv_txt_output_path),
            create_file(&self.cr_txt_output_path),
            create_file(&self.al_txt_output_path)
        );

        task::yield_now().await;

        // todo: config setup builder
        let engine_find = std::env::var("ENGINE_FD").is_ok();
        let engine_find_patterns = match std::env::var("ENGINE_FD_PATH") {
            Ok(pat) => pat,
            _ => "".to_string(),
        };
        // todo: find campaign running and get file
        let mut en_c: Option<File> = if engine_find {
            // todo: validate directory or auto generate new
            Some(create_file(&"_engines_/campaign/_c2/valid/list.txt".to_string()).await)
        } else {
            None
        };
        while let Some(i) = rx.recv().await {
            let (link, jor, spawned) = i;
            let (response, oo) = jor;

            if spawned && *global_thread_count.lock().unwrap() > 0 {
                *global_thread_count.lock().unwrap() -= 1;
            }

            let error = response.starts_with("- error ");
            // detailed json message
            let link = if error {
                string_concat!(response.replacen("- error ", "", 1), "\n")
            } else {
                string_concat!(link, "\n")
            };

            if oo == JsonOutFileType::Error {
                ce_t.write(&link.as_bytes()).await.unwrap();
            }

            if oo == JsonOutFileType::Unknown {
                al_t.write(&link.as_bytes()).await.unwrap();
            }

            if response == "" || error {
                continue;
            }

            // parse and find
            if engine_find {
                let f = &response.clone();

                if f.contains(&engine_find_patterns) {
                    en_c.as_mut()
                        .unwrap()
                        .write(&link.as_bytes())
                        .await
                        .unwrap();
                }

                continue;
            }

            // json program continue

            let j: Value = serde_json::from_str(&response).unwrap_or_default();
            task::yield_now().await;

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
