use std::path::Path;

use crate::tokio::io::BufReader;

use crate::string_concat_impl;
use seoder_lib::packages::spider::utils::log;
use seoder_lib::string_concat::string_concat;
use seoder_lib::tokio;
use seoder_lib::tokio::io::AsyncBufReadExt;
use seoder_lib::tokio::io::AsyncWriteExt;
use seoder_lib::ENTRY_PROGRAM;
use tokio::fs::File;
use tokio::fs::OpenOptions;

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
pub async fn write_config(config: &str, input: &String) {
    let file = OpenOptions::new().read(true).open(&ENTRY_PROGRAM.2).await;

    let mut sl: Vec<String> = vec![];

    let mut found_match = false;

    match file {
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
                        slots[1] = hh[1].to_string();
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

        file.write_all(
            b"timeout 15
buffer 30
proxy false
tor false
license false
target ./urls-input.txt",
        )
        .await
        .unwrap();
    }
}

/// validate program license key external
pub async fn validate_program(key: &str) -> bool {
    use hyper::{Body, Client, Method, Request};

    let dev = cfg!(debug_assertions);

    let endpoint = if dev {
        "http://127.0.0.1:3000/api/keygen-validate"
    } else {
        "https://seoder.io/api/keygen-validate"
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(endpoint)
        .header("content-type", "application/json")
        .body(Body::from(string_concat!(r#"{"key": ""#, key, r#""}"#)))
        .unwrap_or_default();

    let resp = if dev {
        let client = Client::new();

        client.request(req).await.unwrap_or_default()
    } else {
        use hyper_tls::HttpsConnector;
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        client.request(req).await.unwrap_or_default()
    };

    resp.status().is_success() && !resp.headers().is_empty()
}

/// read file to target
pub async fn get_file_value(path: &str, value: &str) -> String {
    let mut target = String::from("");

    match OpenOptions::new().read(true).open(&path).await {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut lines = reader.lines();

            while let Some(line) = lines.next_line().await.unwrap() {
                let hh = line.split(" ").collect::<Vec<&str>>();

                if hh.len() == 2 {
                    if hh[0] == value {
                        target = hh[1].to_string();
                    }
                }
            }
        }
        _ => {}
    };

    target
}
