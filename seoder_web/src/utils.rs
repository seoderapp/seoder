use std::path::Path;

use crate::tokio::io::BufReader;

use crate::string_concat_impl;
use hyper::{Body, Client, Method, Request};
use lazy_static::lazy_static;
use seoder_lib::packages::spider::utils::log;
use seoder_lib::serde_json::json;
use seoder_lib::string_concat::string_concat;
use seoder_lib::tokio;
use seoder_lib::tokio::io::AsyncBufReadExt;
use seoder_lib::tokio::io::AsyncWriteExt;
use seoder_lib::ENTRY_PROGRAM;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use warp::reply::Response;

/// read a file line by line to a vector
pub async fn lines_to_vec(pt: String) -> Vec<String> {
    let mut builder: Vec<String> = Vec::new();

    match File::open(&pt).await {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut lines = reader.lines();

            while let Some(line) = lines.next_line().await.unwrap() {
                builder.push(line);
            }
        }
        Err(_) => {
            log("file does not exist", &pt);
        }
    };
    builder
}

/// write to config file
pub async fn write_config(config: &str, input: &str) {
    // exit if empty input
    if input.is_empty() {
        return;
    }

    let mut sl: Vec<String> = vec![];
    let mut found_match = false;

    match OpenOptions::new().read(true).open(&ENTRY_PROGRAM.2).await {
        Ok(ff) => {
            let reader = BufReader::new(ff);
            let mut lines = reader.lines();

            while let Some(line) = lines.next_line().await.unwrap() {
                let hh = line.split(" ").collect::<Vec<&str>>();

                let mut slots: [String; 2] = ["".to_string(), "".to_string()];

                if hh.len() >= 2 {
                    slots[0] = hh[0].to_string();
                    if hh[0] == config {
                        slots[1] = input.to_string();
                        found_match = true;
                    } else {
                        slots[1] = build_query(&hh);
                    }
                    sl.push(slots.join(" "));
                }
            }
        }
        _ => {}
    };

    if !found_match && !config.is_empty() && !input.is_empty() {
        sl.push(string_concat!(&config, " ", &input));
    }

    let mut filec = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&ENTRY_PROGRAM.2)
        .await
        .unwrap();

    filec.write_all(&sl.join("\n").as_bytes()).await.unwrap();
    filec.flush().await.unwrap();
}

/// make sure conf is ready
pub async fn init_config() {
    let loc = &ENTRY_PROGRAM.2;
    let conf = Path::new(&loc).is_file();

    // setup one time config
    if !conf {
        let mut file = File::create(&loc).await.unwrap();

        file.write(
            b"timeout 15
buffer false
proxy false
tor false
license false\n",
        )
        .await
        .unwrap();

        let target = string_concat!("target ", ENTRY_PROGRAM.1, "urls-input.txt");

        file.write(&target.as_bytes()).await.unwrap();
    }
}

/// validate program license key external
pub async fn validate_program(key: &str) -> bool {
    lazy_static! {
        /// url endpoint
        pub static ref ENDPOINT: &'static str =  match std::env::var("DEV") {
            Ok(_) => {
                "http://127.0.0.1:3000/api/keygen-validate"
            }
            Err(_) => {
                "https://seoder.com/api/keygen-validate"
            }
        };
    }

    // hardware uid
    let id: String = machine_uid::get().unwrap();

    let json = json!({
        "key": key,
        "fingerprint": id.to_string(),
        "platform": std::env::consts::OS
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri(*ENDPOINT)
        .header("content-type", "application/json")
        .body(Body::from(json.to_string()))
        .unwrap_or_default();

    let resp = if *ENDPOINT == "http://127.0.0.1:3000/api/keygen-validate" {
        let client = Client::new();

        client.request(req).await.unwrap_or_default()
    } else {
        use hyper_tls::HttpsConnector;
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        client.request(req).await.unwrap_or_default()
    };

    resp.status().is_success()
}

/// download latest free public proxies
pub async fn download_proxies() -> bool {
    use hyper::body::HttpBody;
    let endpoint = "https://api.proxyscrape.com/v2/?request=displayproxies&protocol=http&timeout=10000&country=all&ssl=all&anonymity=all";
    let req = Request::builder()
        .method(Method::GET)
        .uri(endpoint)
        .header("content-type", "application/text")
        .body(Body::from(""))
        .unwrap_or_default();

    use hyper_tls::HttpsConnector;
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut resp = client.request(req).await.unwrap_or_default();

    let file_path = string_concat!(&ENTRY_PROGRAM.3, "proxies.txt");

    let mut file = File::create(&file_path).await.unwrap();

    while let Some(next) = resp.data().await {
        let chunk = next.unwrap_or_default();
        file.write(&chunk).await.unwrap();
    }

    true
}

/// read file to target
pub async fn get_file_value(path: &str, value: &str) -> String {
    let mut v = String::from("");

    match OpenOptions::new().read(true).open(&path).await {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut lines = reader.lines();

            while let Some(line) = lines.next_line().await.unwrap() {
                let hh = line.split(" ").collect::<Vec<&str>>();

                if hh.len() >= 2 {
                    if hh[0] == value {
                        v = build_query(&hh);
                    }
                }
            }
        }
        _ => {}
    };

    v
}

/// build ranged query
pub fn build_query(hh: &Vec<&str>) -> String {
    let mut h1 = hh[1].to_string();

    // todo: push all into array after first index
    if hh.len() >= 3 {
        for (r, i) in hh.iter().enumerate() {
            if r < 2 {
                continue;
            }
            h1.push_str(" ");
            h1.push_str(&i);
        }
    }

    h1
}

/// validate program license key external
pub async fn get_prospects(key: &str, title: &str) -> Response {
    lazy_static! {
        /// url endpoint
        pub static ref ENDPOINT: &'static str =  match std::env::var("DEV") {
            Ok(_) => {
                "http://127.0.0.1:3000/api/prospect"
            }
            Err(_) => {
                "https://seoder.com/api/prospect"
            }
        };
    }

    // hardware uid
    let id: String = machine_uid::get().unwrap();

    let json = json!({
        "key": key,
        "fingerprint": id.to_string(),
        "platform": std::env::consts::OS,
        "title": title.to_string()
    });

    let req = Request::builder()
        .method(Method::POST)
        .uri(*ENDPOINT)
        .header("content-type", "application/json")
        .body(Body::from(json.to_string()))
        .unwrap_or_default();

    let resp = if *ENDPOINT == "http://127.0.0.1:3000/api/prospect" {
        let client = Client::new();

        client.request(req).await.unwrap_or_default()
    } else {
        use hyper_tls::HttpsConnector;
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        client.request(req).await.unwrap_or_default()
    };

    resp
}

lazy_static! {
    /// units of memory
    static ref UNITS: Vec<&'static str> = vec!["bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
}

/// convert to bytes
pub fn convert_to_bytes(bytes: u64) -> String {
    let mut l = 0;
    let mut n = bytes;

    // // each iteration capture the bytes
    while n >= 1024 {
        n = n / 1024;
        l += 1;
    }

    string_concat!(n.to_string(), " ", UNITS[l])
}
