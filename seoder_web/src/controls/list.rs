use crate::json;
use crate::string_concat::string_concat;
use crate::string_concat::string_concat_impl;
use crate::tokio;
use crate::utils::get_file_value;
use crate::BufReader;
use crate::OpenOptions;
use crate::OutGoing;
use crate::ENTRY_PROGRAM;

use futures_util::SinkExt;
use seoder_lib::tokio::io::AsyncBufReadExt;
use tungstenite::Message;

/// list valid count across sections
pub async fn list_valid(mut outgoing: OutGoing) -> OutGoing {
    let mut dir = tokio::fs::read_dir(&ENTRY_PROGRAM.0).await.unwrap();

    while let Some(child) = dir.next_entry().await.unwrap_or_default() {
        if child.metadata().await.unwrap().is_dir() {
            let dpt = child.path().to_str().unwrap().to_owned();

            let file = OpenOptions::new()
                .read(true)
                .open(string_concat!(dpt, "/valid/links.txt"))
                .await;

            let invalid_file = OpenOptions::new()
                .read(true)
                .open(string_concat!(dpt, "/invalid/links.txt"))
                .await;

            let mut d = dpt.replacen(&ENTRY_PROGRAM.0, "", 1);

            if d.starts_with("/") {
                d.remove(0);
            }

            let mut lns = 0;
            let mut ilns = 0;

            match file {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    let mut lines = reader.lines();

                    while let Some(url) = lines.next_line().await.unwrap() {
                        let v = json!({ "url": url, "path": d });

                        outgoing
                            .send(Message::Text(v.to_string()))
                            .await
                            .unwrap_or_default();

                        lns += 1;
                    }
                }
                _ => {}
            };

            match invalid_file {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    let mut lines = reader.lines();

                    while let Some(url) = lines.next_line().await.unwrap() {
                        let v = json!({ "invalid_url": url, "invalid_path": d });

                        outgoing
                            .send(Message::Text(v.to_string()))
                            .await
                            .unwrap_or_default();

                        ilns += 1;
                    }
                }
                _ => {}
            };

            let v = json!({ "count": lns, "ecount": ilns, "path": d });

            outgoing
                .send(Message::Text(v.to_string()))
                .await
                .unwrap_or_default();
        }
    }

    outgoing
}

/// list all valid files accross sys
pub async fn list_files(mut outgoing: OutGoing) -> OutGoing {
    let mut dir = tokio::fs::read_dir(&ENTRY_PROGRAM.1).await.unwrap();

    while let Some(child) = dir.next_entry().await.unwrap_or_default() {
        if child.metadata().await.unwrap().is_file() {
            let dpt = child
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap_or_default()
                .to_owned();

            if dpt.ends_with(".txt") {
                let v = json!({ "fpath": dpt });

                outgoing
                    .send(Message::Text(v.to_string()))
                    .await
                    .unwrap_or_default();
            }
        }
    }

    outgoing
}

/// list all valid engines
pub async fn list_engines(mut outgoing: OutGoing) -> OutGoing {
    let mut dir = tokio::fs::read_dir(&ENTRY_PROGRAM.0).await.unwrap();

    while let Some(child) = dir.next_entry().await.unwrap_or_default() {
        if child.metadata().await.unwrap().is_dir() {
            let dpt = child.path().to_str().unwrap().to_owned();

            // engine paths
            // engine patterns
            let file = OpenOptions::new()
                .read(true)
                .open(string_concat!(dpt, "/paths.txt"))
                .await;

            let mut paths: Vec<String> = vec![];
            let mut patterns: Vec<String> = vec![];

            match file {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    let mut lines = reader.lines();

                    while let Some(line) = lines.next_line().await.unwrap() {
                        paths.push(line)
                    }
                }
                _ => {}
            };

            let file = OpenOptions::new()
                .read(true)
                .open(string_concat!(dpt, "/patterns.txt"))
                .await;

            match file {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    let mut lines = reader.lines();

                    while let Some(line) = lines.next_line().await.unwrap() {
                        patterns.push(line)
                    }
                }
                _ => {}
            };

            let mut d = dpt.replacen(&ENTRY_PROGRAM.0, "", 1);

            if d.starts_with("/") {
                d.remove(0);
            }

            let v = json!({
                  "epath": d,
                  "paths": paths,
                  "patterns": patterns
            });

            outgoing
                .send(Message::Text(v.to_string()))
                .await
                .unwrap_or_default();
        }
    }
    outgoing
}

/// list file count for active file for program
pub async fn list_file_count(mut outgoing: OutGoing) -> OutGoing {
    let mut dir = tokio::fs::read_dir(&ENTRY_PROGRAM.0).await.unwrap();

    while let Some(child) = dir.next_entry().await.unwrap_or_default() {
        if child.metadata().await.unwrap().is_dir() {
            let dpt = child.path().to_str().unwrap().to_owned();

            if !dpt.ends_with("/valid") {
                let mut target = get_file_value(&ENTRY_PROGRAM.2, "target").await;

                let mut engine = dpt;

                if target.is_empty() {
                    target = String::from("urls-input.txt");
                }

                if engine.is_empty() {
                    engine = String::from("default");
                }

                let mut nml = 0;

                // target file length
                match OpenOptions::new()
                    .read(true)
                    .open(string_concat!(ENTRY_PROGRAM.1, target))
                    .await
                {
                    Ok(file) => {
                        let reader = BufReader::new(file);
                        let mut lines = reader.lines();

                        while let Some(_) = lines.next_line().await.unwrap() {
                            nml += 1;
                        }
                    }
                    _ => {}
                };

                let v = json!({ "pengine": engine.replacen(&ENTRY_PROGRAM.0, "", 1), "ploc": nml });

                outgoing
                    .send(Message::Text(v.to_string()))
                    .await
                    .unwrap_or_default();
            }
        }
    }

    outgoing
}

/// get config
pub async fn config(mut outgoing: OutGoing) -> OutGoing {
    let mut timeout = 50;
    let mut buffer = false;
    let mut proxy = false;
    let mut tor = false;
    let mut target = string_concat!(ENTRY_PROGRAM.1, "urls-input.txt"); // todo: fix target
    let mut license = String::from("");
    let file = OpenOptions::new().read(true).open(&ENTRY_PROGRAM.2).await;

    match file {
        Ok(ff) => {
            let reader = BufReader::new(ff);
            let mut lines = reader.lines();

            while let Some(line) = lines.next_line().await.unwrap() {
                let hh = line.split(" ").collect::<Vec<&str>>();

                if hh.len() >= 2 {
                    let h0 = hh[0];
                    let mut h1 = hh[1].to_string();

                    if hh.len() == 3 {
                        h1.push_str(hh[2]);
                    }

                    if h0 == "timeout" {
                        timeout = h1.parse::<u16>().unwrap_or(15);
                    }
                    if h0 == "buffer" {
                        buffer = h1.parse::<bool>().unwrap_or(false);
                    }

                    if h0 == "proxy" {
                        proxy = h1.parse::<bool>().unwrap_or(false);
                    }

                    if h0 == "tor" {
                        tor = h1.parse::<bool>().unwrap_or(false);
                    }

                    if h0 == "license" && h1 != "false" {
                        license = h1.clone();
                    }

                    if h0 == "target" {
                        let path = std::path::Path::new(&h1);
                        let filename = path.file_name().unwrap();

                        let f = filename.to_str().unwrap_or_default().to_string();

                        if !f.is_empty() {
                            target = f;
                        }
                    }
                }
            }
        }
        _ => {}
    };

    let sl = json!({
       "timeout": timeout,
       "buffer": buffer,
       "proxy": proxy,
       "tor": tor,
       "target": target,
       "license": license
    });

    outgoing
        .send(Message::Text(sl.to_string()))
        .await
        .unwrap_or_default();

    outgoing
}

/// get license
pub async fn license() -> String {
    let mut license = String::from("");
    let file = OpenOptions::new().read(true).open(&ENTRY_PROGRAM.2).await;

    match file {
        Ok(ff) => {
            let reader = BufReader::new(ff);
            let mut lines = reader.lines();

            while let Some(line) = lines.next_line().await.unwrap() {
                let hh = line.split(" ").collect::<Vec<&str>>();

                if hh.len() >= 2 {
                    let h0 = hh[0];
                    let mut h1 = hh[1].to_string();

                    if hh.len() == 3 {
                        h1.push_str(hh[2]);
                    }

                    if h0 == "license" && h1 != "false" {
                        license = h1.clone();
                    }
                }
            }
        }
        _ => {}
    };

    license
}
