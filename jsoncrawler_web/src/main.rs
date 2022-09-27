#![forbid(unsafe_code)]

#[cfg(all(not(windows), not(target_os = "android"), feature = "jemalloc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use crate::string_concat::string_concat;
use jsoncrawler_lib::packages::spider::configuration::setup;
use jsoncrawler_lib::tokio::sync::mpsc::unbounded_channel;
use jsoncrawler_lib::Website;

use sysinfo::{System, SystemExt};
use tokio::fs::OpenOptions;
use tungstenite::{Message, Result};

use serde::{Deserialize, Serialize};
use serde_json::{Value};
use crate::string_concat::string_concat_impl;
use crate::tokio::fs::File;
use futures_util::SinkExt;
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use jsoncrawler_lib::packages::spider::utils::{log, logd};
use jsoncrawler_lib::tokio::io::AsyncWriteExt;
use jsoncrawler_lib::{serde_json, string_concat, tokio};
use std::io::Error as IoError;
use std::{
    collections::HashMap,
    convert::Infallible,
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use tokio::net::{TcpListener, TcpStream};

use crate::serde_json::json;

mod panel;

type Tx = futures_channel::mpsc::UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

/// new engine
#[derive(Serialize, Deserialize, Debug, Default)]
struct Eng {
    name: String,
    paths: String,
    patterns: String,
}

async fn handle_connection(_peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    logd(string_concat::string_concat!(
        "TCP connection from: ",
        &addr.to_string()
    ));

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");

    logd(string_concat::string_concat!(
        "WebSocket connection established: ",
        &addr.to_string()
    ));

    let (mut outgoing, incoming) = ws_stream.split();

    let (sender, mut receiver) = unbounded_channel();

    let (txx, mut rxx): (
        tokio::sync::mpsc::UnboundedSender<String>,
        tokio::sync::mpsc::UnboundedReceiver<String>,
    ) = unbounded_channel();

    let handle = tokio::spawn(async move {
        use sysinfo::CpuExt;
        use sysinfo::NetworkExt;
        let mut s = System::new_all();
        // let mut interval = tokio::time::interval(Duration::from_millis(1000));

        while let Some(st) = receiver.recv().await {
            if st == 1 {
                s.refresh_all();

                let mut net_total_received = 0;
                let mut net_total_transmited = 0;

                let networks = s.networks();

                for (_, data) in networks {
                    net_total_received += data.received();
                    net_total_transmited += data.transmitted();
                }

                let v = json!({
                    "stats": {
                        // network
                        "network_received": net_total_received,
                        "network_transmited": net_total_transmited,
                        "network_total_transmitted": net_total_received + net_total_transmited,
                        // cpu
                        "load_avg_min": s.load_average().one,
                        "cpu_usage": s.global_cpu_info().cpu_usage(),
                        // memory
                        "memory_total": s.total_memory(),
                        "memory_used": s.used_memory(),
                        "memory_available": s.available_memory(),
                        "memory_free": s.free_memory()
                    }
                });

                tokio::task::yield_now().await;

                outgoing
                    .send(Message::Text(v.to_string()))
                    .await
                    .unwrap_or_default();
            }

            // list all websites
            if st == 2 {
                let mut dir = tokio::fs::read_dir("_db/campaigns").await.unwrap();

                while let Some(child) = dir.next_entry().await.unwrap_or_default() {
                    if child.metadata().await.unwrap().is_dir() {
                        let dpt = child.path().to_str().unwrap().to_owned();
                        if !dpt.ends_with("/valid") {
                            let v = json!({ "path": dpt.replacen("_db/campaigns/", "", 1) });
                            outgoing
                                .send(Message::Text(v.to_string()))
                                .await
                                .unwrap_or_default();
                        }
                    }
                }
            }

            // run campaigns
            if st == 3 {
                let mut dir = tokio::fs::read_dir("_db/campaigns").await.unwrap();

                while let Some(child) = dir.next_entry().await.unwrap_or_default() {
                    if child.metadata().await.unwrap().is_dir() {
                        let dpt = child.path().to_str().unwrap().to_owned();
                        if !dpt.ends_with("/valid") {
                            let (_, __, ___, ____, engine) = setup(true);
                            tokio::task::yield_now().await;
                            let mut website: Website = Website::new(&"urls-input.txt");
                            website.engine.campaign.name = dpt;
                            website.engine.campaign.paths = engine.campaign.paths;
                            website.engine.campaign.patterns = engine.campaign.patterns;

                            tokio::spawn(async move {
                                website.crawl().await;
                                log("crawl finished - ", &website.engine.campaign.name)
                            });
                            // let v = json!({ "path": dpt });
                            // outgoing
                            //     .send(Message::Text(v.to_string()))
                            //     .await
                            //     .unwrap_or_default();
                        }
                    }
                }
            }

            // determine valid count across files
            if st == 4 {
                let mut dir = tokio::fs::read_dir("_db/campaigns").await.unwrap();

                while let Some(child) = dir.next_entry().await.unwrap_or_default() {
                    if child.metadata().await.unwrap().is_dir() {
                        let dpt = child.path().to_str().unwrap().to_owned();

                        if !dpt.ends_with("/valid") {
                            use crate::tokio::io::BufReader;
                            use jsoncrawler_lib::tokio::io::AsyncBufReadExt;
                            let file = OpenOptions::new()
                                .read(true)
                                .open(string_concat!(dpt, "/valid/links.txt"))
                                .await;

                            let mut lns = 0;

                            match file {
                                Ok(file) => {
                                    let reader = BufReader::new(file);
                                    let mut lines = reader.lines();

                                    while let Some(_) = lines.next_line().await.unwrap() {
                                        lns += 1;
                                    }
                                }
                                _ => {}
                            };

                            let v = json!({ "count": lns, "path": dpt.replacen("_db/campaigns/", "", 1) });

                            outgoing
                                .send(Message::Text(v.to_string()))
                                .await
                                .unwrap_or_default();
                        }
                    }
                }
            }

            // list all engines
            if st == 5 {
                let mut dir = tokio::fs::read_dir("_engines_/").await.unwrap();

                while let Some(child) = dir.next_entry().await.unwrap_or_default() {
                    if child.metadata().await.unwrap().is_dir() {
                        let dpt = child.path().to_str().unwrap().to_owned();
                        let v = json!({ "epath": dpt.replacen("_engines_/", "", 1) });
                        outgoing
                            .send(Message::Text(v.to_string()))
                            .await
                            .unwrap_or_default();
                    }
                }
            }
        }
    });

    tokio::task::yield_now().await;

    let broadcast_incoming = incoming.try_for_each(|msg| {
        let m = msg.clone();
        let txt = m.to_text().unwrap();
        let sender = sender.clone();

        // let mut lock = std::io::stdout().lock();
        // writeln!(lock, "Received a message from {}: {}", &addr, &txt).unwrap();

        // remove newline
        let ms = txt.trim();

        let mut p1 = false;

        let mut s = "".to_string();
        let mut ss = "".to_string();

        for c in ms.chars().into_iter() {
            let cc = c.to_string();

            // split at first white space
            if p1 == false && c == ' ' {
                p1 = true;
            } else if !p1 {
                s.push_str(&cc);
            } else {
                ss.push_str(&cc);
            }
        }

        let c = s;
        let cc = ss;
        
        // start the feed stats
        if c == "feed" {
            if let Err(_) = sender.send(1) {
                logd("the receiver dropped");
            }
        }
        // create new campaign to store crawl results
        else if c == "create-campaign" {
            let cf = cc.to_owned();
            tokio::spawn(async move {
                let v: Value = serde_json::from_str(&cf).unwrap_or_default();

                let campaign_dir = string_concat!("_db/campaigns/", v["name"].as_str().unwrap());

                tokio::fs::create_dir(&campaign_dir).await.unwrap();
                tokio::fs::create_dir(&string_concat!(campaign_dir, "/valid"))
                    .await
                    .unwrap();

                let mut file = File::create(string_concat!(campaign_dir, "/config.txt"))
                    .await
                    .unwrap();

                let e = string_concat!("engine ", v["engine"].as_str().unwrap_or("default"));
                
                file.write_all(&e.as_bytes()).await.unwrap();

                if let Err(_) = sender.send(2) {
                    logd("the receiver dropped");
                }
            });
        } else if c == "run-campaign" {
            let txx = txx.clone();
            let cc = cc.to_owned();
            if let Err(_) = txx.send(cc) {
                logd("receiver dropped");
            }
        } else if c == "list-campaigns" {
            if let Err(_) = sender.send(2) {
                logd("the receiver dropped");
            }
        } else if ms == "run-all-campaigns" {
            if let Err(_) = sender.send(3) {
                logd("the receiver dropped");
            }
        } else if c == "list-campaign-stats" {
            tokio::task::spawn(async move {
                if let Err(_) = sender.send(4) {
                    logd("the receiver dropped");
                }
            });
        } else if c == "list-engines" {
            tokio::task::spawn(async move {
                if let Err(_) = sender.send(5) {
                    logd("the receiver dropped");
                }
            });
        } else if c == "create-engine" {
            let cc = cc.clone();
            let v: Eng = serde_json::from_str(&cc).unwrap_or_default();

            let n = v.name;

            if n.is_empty() == false {
                let db_dir = string_concat!("_engines_/", n);
                let pt = v.paths;
                let pat = v.patterns;

                tokio::task::spawn(async move {
                    let ptt = pt.split(',');
                    let ott = pat.split(',');
                    tokio::fs::create_dir(&db_dir).await.unwrap();

                    let mut file = File::create(string_concat!(db_dir, "/paths.txt"))
                        .await
                        .unwrap();

                    for x in ptt {
                        let x = string_concat!(x, "\n");
                        file.write_all(&x.as_bytes()).await.unwrap();
                    }

                    let mut file = File::create(string_concat!(db_dir, "/patterns.txt"))
                        .await
                        .unwrap();

                    for x in ott {
                        let x = string_concat!(x, "\n");
                        file.write_all(&x.as_bytes()).await.unwrap();
                    }
                });
                if let Err(_) = sender.send(2) {
                    logd("the receiver dropped");
                }
            }
        }

        future::ok(())
    });

    // inputs received to the request possible to broadcast all
    pin_mut!(broadcast_incoming);

    broadcast_incoming.await.unwrap_or_default();

    // run direct campaign
    while let Some(input) = rxx.recv().await {
        let (_, __, ___, ____, engine) = setup(true);
        tokio::task::yield_now().await;
        let input = input.clone();
        let mut website: Website = Website::new(&"urls-input.txt");
        website.engine.campaign.name = input;
        website.engine.campaign.paths = engine.campaign.paths;
        website.engine.campaign.patterns = engine.campaign.patterns;

        tokio::spawn(async move {
            website.crawl().await;
            log("crawl finished - ", &website.engine.campaign.name)
        });
    }

    logd(string_concat::string_concat!(
        &addr.to_string(),
        " disconnected"
    ));

    handle.await.unwrap();
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    env_logger::init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let client_port = env::args().nth(2).unwrap_or_else(|| "3000".to_string());

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    // http server
    tokio::spawn(async move {
        let port = client_port.parse::<u16>().unwrap_or(3000);
        // todo allow custom http address
        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        let make_svc = make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(panel::panel_html::panel_handle))
        });

        if let Err(e) = Server::bind(&addr).serve(make_svc).await {
            eprintln!("server error: {}. Port not found {}", e, port);
        }
    });

    println!("Listening on: {} and 127.0.0.1:3000", &addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }

    Ok(())
}
