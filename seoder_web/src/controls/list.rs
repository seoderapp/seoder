use crate::BufReader;
use crate::tokio;
use crate::ENTRY_PROGRAM;
use crate::string_concat::string_concat;
use crate::string_concat::string_concat_impl;
use crate::OpenOptions;
use seoder_lib::tokio::io::AsyncBufReadExt;
use crate::json;
use tungstenite::{Message};
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use tokio_tungstenite::WebSocketStream;
use crate::TcpStream;

type OutGoing = SplitSink<WebSocketStream<TcpStream>, Message>;

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
