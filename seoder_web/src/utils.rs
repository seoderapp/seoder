use std::path::Path;

use crate::tokio::io::BufReader;

use crate::string_concat_impl;
use seoder_lib::packages::spider::utils::log;
use seoder_lib::string_concat::string_concat;
use seoder_lib::tokio;
use seoder_lib::tokio::io::AsyncBufReadExt;
use seoder_lib::tokio::io::AsyncWriteExt;
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
    let file = OpenOptions::new().read(true).open("config.txt").await;

    let mut sl: Vec<String> = vec![];

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
                    } else {
                        slots[1] = hh[1].to_string();
                    }
                    sl.push(slots.join(" "));
                }
            }
        }
        _ => {}
    };

    // set license handling
    if config == "license" && !input.is_empty() && !sl.contains(&String::from("license")) {
        sl.push(string_concat!("license ", input));
    }

    let mut filec = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("config.txt")
        .await
        .unwrap();

    filec.write_all(&sl.join("\n").as_bytes()).await.unwrap();
    filec.flush().await.unwrap();
}

/// make sure conf is ready
pub async fn init_config() {
    let loc = "config.txt";
    let conf = Path::new(&loc).is_file();

    // setup one time config
    if !conf {
        let mut file = File::create(&loc).await.unwrap();
        file.write_all(
            b"timeout 15
buffer 30
proxy false
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

    let endpoint = if cfg!(debug_assertions) {
        "http://127.0.0.1/api/keygen-validate"
    } else {
        "https://seoder.io/api/keygen-validate"
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(endpoint)
        .header("content-type", "application/json")
        .body(Body::from(string_concat!(r#"{"key":"#, key, r#""}"#)))
        .unwrap_or_default();

    let client = Client::new();

    let resp = client.request(req).await.unwrap_or_default();

    resp.status() == 200
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
