use crate::json;
use crate::string_concat::string_concat;
use crate::string_concat::string_concat_impl;
use crate::tokio;
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

            if !dpt.ends_with("/valid") {
                let file = OpenOptions::new()
                    .read(true)
                    .open(string_concat!(dpt, "/valid/links.txt"))
                    .await;

                let mut lns = 0;

                let mut d = dpt.replacen(&ENTRY_PROGRAM.0, "", 1);

                if d.starts_with("/") {
                    d.remove(0);
                }

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

                let v = json!({ "count": lns, "path": d });

                outgoing
                    .send(Message::Text(v.to_string()))
                    .await
                    .unwrap_or_default();
            }
        }
    }

    outgoing
}

/// list all valid files accross sys
pub async fn list_files(mut outgoing: OutGoing) -> OutGoing {
    let mut dir = tokio::fs::read_dir(&ENTRY_PROGRAM.1).await.unwrap();

    while let Some(child) = dir.next_entry().await.unwrap_or_default() {
        if child.metadata().await.unwrap().is_file() {
            let dpt = child.path().to_str().unwrap().to_owned();
            let dpt = dpt.replacen(&ENTRY_PROGRAM.1, "", 1);

            if dpt.ends_with(".txt") {
                // file path for ws conversion
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
