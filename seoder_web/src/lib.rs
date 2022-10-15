#![forbid(unsafe_code)]

#[cfg(all(not(windows), not(target_os = "android"), feature = "jemalloc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use crate::string_concat::string_concat;
use crate::string_concat::string_concat_impl;
use lazy_static::lazy_static;
use seoder_lib::tokio::io::AsyncBufReadExt;
use seoder_lib::tokio::sync::mpsc::unbounded_channel;
use seoder_lib::Website;
use sysinfo::{System, SystemExt};
use tungstenite::{Message, Result};

use crate::tokio::fs::File;
use futures_util::SinkExt;
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use seoder_lib::packages::spider::utils::{log, logd};
use seoder_lib::tokio::io::AsyncWriteExt;
pub use seoder_lib::{serde_json, string_concat, tokio};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::io::Error as IoError;
use std::path::Path;
use std::{
    collections::HashMap,
    convert::Infallible,
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::serde_json::json;
use crate::tokio::io::BufReader;
use crate::tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use tokio::fs::create_dir;
use tokio::fs::OpenOptions;
use tokio::net::{TcpListener, TcpStream};

extern crate lazy_static;
extern crate tera;

mod builder;
mod ft;
mod panel;
mod utils;

/// determine action
#[derive(PartialEq)]
enum Action {
    Stats,
    Config,
    ListCampaigns,
    ListEngines,
    ListFiles,
    ListFileCount,
    ListValidCampaigns,
    RunCampaign,
    RunAllCampaigns,
    CreateCampaign,
    CreateEngine,
    RemoveCampaign,
    RemoveEngine,
    RemoveFile,
    SetList,
    SetBuffer,
    SetProxy,
}

type Tx = futures_channel::mpsc::UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
type Controller = (Action, String);

/// new engine
#[derive(Serialize, Deserialize, Debug, Default)]
struct Eng {
    name: String,
    paths: String,
    patterns: String,
}

lazy_static! {
    static ref ENTRY_PROGRAM: &'static str = {
        let mut path_base = "./";
        let name = "SEODER_PROGRAM";

        match env::var(name) {
            Ok(v) => {
                println!("{}: {}", name, v);

                if v == "app" {
                    path_base = "../"
                }
            }
            Err(e) => {
                println!("${} is not set ({})", name, e);
            }
        }

        path_base
    };
}

/// read file to target
async fn get_file_value(path: &str, value: &str) -> String {
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

/// clean target _db string
fn clean_target_cmp_str(dpt: &str) -> String {
    if dpt.starts_with("../_db/campaigns/") {
        dpt.replacen("../_db/campaigns/", "", 1)
    } else {
        dpt.replacen("_db/campaigns/", "", 1)
    }
}

/// handle async connections to socket
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

    let (sender, mut receiver): (UnboundedSender<Controller>, UnboundedReceiver<Controller>) =
        unbounded_channel();

    let handle = tokio::spawn(async move {
        use sysinfo::CpuExt;
        use sysinfo::NetworkExt;
        let mut s = System::new_all();
        // let mut interval = tokio::time::interval(Duration::from_millis(1000));

        while let Some(m) = receiver.recv().await {
            let (st, input) = m;

            if st == Action::Stats {
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

            // list all campaigns
            if st == Action::ListCampaigns {
                let pt = if Path::new("./_db/campaigns").exists() {
                    "./_db/campaigns"
                } else {
                    "../_db/campaigns"
                };

                let mut dir = tokio::fs::read_dir(pt).await.unwrap();

                while let Some(child) = dir.next_entry().await.unwrap_or_default() {
                    if child.metadata().await.unwrap().is_dir() {
                        let dpt = child.path().to_str().unwrap().to_owned();

                        if !dpt.ends_with("/valid") {
                            // todo: set engine in memory
                            let mut engine =
                                get_file_value(&string_concat!(dpt, "/config.txt"), "engine").await;

                            if engine.is_empty() {
                                engine = String::from("default");
                            }

                            let mut d = dpt.replacen(pt, "", 1);

                            d.remove(0);

                            let v = json!({ "path": d, "pengine": engine });

                            outgoing
                                .send(Message::Text(v.to_string()))
                                .await
                                .unwrap_or_default();
                        }
                    }
                }
            }

            // run all campaigns
            if st == Action::RunAllCampaigns {
                let mut dir = tokio::fs::read_dir("_db/campaigns").await.unwrap();

                while let Some(child) = dir.next_entry().await.unwrap_or_default() {
                    if child.metadata().await.unwrap().is_dir() {
                        // path
                        let dpt = child.path().to_str().unwrap().to_owned();
                        if !dpt.ends_with("/valid") {
                            let dptt = dpt.clone();
                            let (pt, pat, target) = builder::engine_builder(dptt).await;

                            let mut website: Website = Website::new(&target);

                            website.engine.campaign.name = dpt;
                            website.engine.campaign.paths = pt;
                            website.engine.campaign.patterns = pat;

                            tokio::spawn(async move {
                                website.crawl().await;
                                log("crawl finished - ", &website.engine.campaign.name)
                            });
                        }
                    }
                }
            }

            // determine valid count across files
            if st == Action::ListValidCampaigns {
                let c = if Path::new("./_db/campaigns").exists() {
                    "./_db/campaigns"
                } else {
                    "../_db/campaigns"
                };

                let mut dir = tokio::fs::read_dir(c).await.unwrap();

                while let Some(child) = dir.next_entry().await.unwrap_or_default() {
                    if child.metadata().await.unwrap().is_dir() {
                        let dpt = child.path().to_str().unwrap().to_owned();

                        if !dpt.ends_with("/valid") {
                            let file = OpenOptions::new()
                                .read(true)
                                .open(string_concat!(dpt, "/valid/links.txt"))
                                .await;

                            let mut lns = 0;

                            let mut d = dpt.replacen(c, "", 1);

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
            }

            // list all files
            if st == Action::ListFiles {
                let f = string_concat!(&ENTRY_PROGRAM, "_db/files");
                let mut dir = tokio::fs::read_dir(&f).await.unwrap();

                while let Some(child) = dir.next_entry().await.unwrap_or_default() {
                    if child.metadata().await.unwrap().is_file() {
                        let dpt = child.path().to_str().unwrap().to_owned();
                        let dpt = dpt.replacen(&f, "", 1);
                        if dpt != "README.md" {
                            let v = json!({ "fpath": dpt });
                            outgoing
                                .send(Message::Text(v.to_string()))
                                .await
                                .unwrap_or_default();
                        }
                    }
                }
            }

            // remoe file todo remove from ui
            if st == Action::RemoveFile {
                let f = string_concat!(&ENTRY_PROGRAM, "_db/files");

                tokio::fs::remove_file(string_concat!(f, &input))
                    .await
                    .unwrap();

                let v = json!({ "dfpath": input });
                outgoing
                    .send(Message::Text(v.to_string()))
                    .await
                    .unwrap_or_default();
            }

            // set enable proxies
            if st == Action::SetProxy {
                utils::write_config("proxy", &input).await;
            }

            // set selected buffer timeout
            if st == Action::SetBuffer {
                utils::write_config("buffer", &input).await;
            }

            // set selected list item
            if st == Action::SetList {
                let f = string_concat!(&ENTRY_PROGRAM, "_db/files");

                utils::write_config("target", &string_concat!(f, input.to_string())).await;
            }

            // list all engines
            if st == Action::ListEngines {
                // todo: get the static root paths on app start
                let eg = string_concat!(&ENTRY_PROGRAM, "_db/engines");
                let mut dir = tokio::fs::read_dir(&eg).await.unwrap();

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

                        let mut d = dpt.replacen(&eg, "", 1);

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
            }

            let crun_input = input.clone();

            // run campaign
            if st == Action::RunCampaign {
                let cp = input.clone();
                let (pt, pat, target) = builder::engine_builder(crun_input).await;

                let mut website: Website = Website::new(&target);

                website.engine.campaign.name = cp;
                website.engine.campaign.paths = pt;
                website.engine.campaign.patterns = pat;

                tokio::spawn(async move {
                    let performance = crate::tokio::time::Instant::now();

                    website.crawl().await;

                    let b = string_concat!(
                        performance.elapsed().as_secs().to_string(),
                        "s - ",
                        website.engine.campaign.name
                    );

                    log("crawl finished - time elasped: ", &b);
                });
            }

            let d_input = input.clone();

            if st == Action::RemoveCampaign {
                let eg = string_concat!(&ENTRY_PROGRAM, "_db/campaigns/",&d_input);

                tokio::fs::remove_dir_all(eg)
                    .await
                    .unwrap();

                let v = json!({ "dcpath": input });
                outgoing
                    .send(Message::Text(v.to_string()))
                    .await
                    .unwrap_or_default();
            }

            // delete engine - this does not delete configs attached!
            if st == Action::RemoveEngine {
                let eg = string_concat!(&ENTRY_PROGRAM, "_db/engines/",&d_input);

                tokio::fs::remove_dir_all(eg)
                    .await
                    .unwrap();

                let v = json!({ "depath": input });
                outgoing
                    .send(Message::Text(v.to_string()))
                    .await
                    .unwrap_or_default();
            }

            if st == Action::Config {
                let file = OpenOptions::new().read(true).open("config.txt").await;

                let mut timeout = 50;
                let mut buffer = 50;
                let mut proxy = false;
                let mut target = String::from("./_db/files/urls-input.txt"); // todo: fix target

                match file {
                    Ok(ff) => {
                        let reader = BufReader::new(ff);
                        let mut lines = reader.lines();

                        while let Some(line) = lines.next_line().await.unwrap() {
                            let hh = line.split(" ").collect::<Vec<&str>>();

                            if hh.len() >= 2 {
                                let h0 = hh[0];
                                let h1 = hh[1].to_string();

                                if h0 == "timeout" {
                                    timeout = h1.parse::<u16>().unwrap_or(15);
                                }
                                if h0 == "buffer" {
                                    buffer = h1.parse::<u16>().unwrap_or(50);
                                }
                                if h0 == "proxy" {
                                    proxy = h1.parse::<bool>().unwrap_or(false);
                                }
                                if h0 == "target" {
                                    if h1.starts_with("./_db/files/") {
                                        target = h1.replacen("./_db/files/", "", 1);
                                    } else {
                                        target = h1.clone();
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
                   "target": target
                });

                outgoing
                    .send(Message::Text(sl.to_string()))
                    .await
                    .unwrap_or_default();
            }

            if st == Action::ListFileCount {
                let pt = if Path::new("./_db/campaigns").exists() {
                    "./_db/campaigns"
                } else {
                    "../_db/campaigns"
                };
                let mut dir = tokio::fs::read_dir(pt).await.unwrap();

                while let Some(child) = dir.next_entry().await.unwrap_or_default() {
                    if child.metadata().await.unwrap().is_dir() {
                        let dpt = child.path().to_str().unwrap().to_owned();

                        if !dpt.ends_with("/valid") {
                            let mut target = get_file_value("./config.txt", "target").await;
                            let mut engine =
                                get_file_value(&string_concat!(dpt, "/config.txt"), "engine").await;

                            if target.is_empty() {
                                target = String::from("urls-input.txt");
                            }

                            if engine.is_empty() {
                                engine = String::from("default");
                            }

                            let mut nml = 0;

                            // target file length
                            match OpenOptions::new().read(true).open(target).await {
                                Ok(file) => {
                                    let reader = BufReader::new(file);
                                    let mut lines = reader.lines();

                                    while let Some(_) = lines.next_line().await.unwrap() {
                                        nml += 1;
                                    }
                                }
                                _ => {}
                            };

                            let dpt = clean_target_cmp_str(&dpt);

                            let v = json!({ "apath": dpt.replacen("/", "", 1), "pengine": engine, "ploc": nml });

                            outgoing
                                .send(Message::Text(v.to_string()))
                                .await
                                .unwrap_or_default();
                        }
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
            if let Err(_) = sender.send((Action::Stats, "".to_string())) {
                logd("the receiver dropped");
            }
        }
        // create new campaign to store crawl results
        else if c == "create-campaign" {
            let cf = cc.to_owned();
            tokio::spawn(async move {
                let v: Value = serde_json::from_str(&cf).unwrap_or_default();

                let campaign_dir = string_concat!("_db/campaigns/", v["name"].as_str().unwrap());

                let dir = &string_concat!(campaign_dir, "/valid");

                create_dir(&campaign_dir).await.unwrap();
                create_dir(&dir).await.unwrap();

                let mut file = File::create(string_concat!(campaign_dir, "/config.txt"))
                    .await
                    .unwrap();

                let e = string_concat!("engine ", v["engine"].as_str().unwrap_or("default"));

                file.write_all(&e.as_bytes()).await.unwrap();

                if let Err(_) = sender.send((Action::CreateCampaign, "".to_string())) {
                    logd("the receiver dropped");
                }
            });
        } else if c == "config" {
            let campain_name = cc.to_owned();

            if let Err(_) = sender.send((Action::Config, campain_name)) {
                logd("receiver dropped");
            }
        } else if c == "run-campaign" {
            let campain_name = cc.to_owned();

            if let Err(_) = sender.send((Action::RunCampaign, campain_name)) {
                logd("receiver dropped");
            }
        } else if c == "set-list" {
            let list_name = cc.to_owned();

            if let Err(_) = sender.send((Action::SetList, list_name)) {
                logd("receiver dropped");
            }
        } else if c == "set-buffer" {
            let list_name = cc.to_owned();

            if let Err(_) = sender.send((Action::SetBuffer, list_name)) {
                logd("receiver dropped");
            }
        } else if c == "set-proxy" {
            let list_name = cc.to_owned();

            if let Err(_) = sender.send((Action::SetProxy, list_name)) {
                logd("receiver dropped");
            }
        } else if c == "delete-campaign" {
            let campain_name = cc.to_owned();

            if let Err(_) = sender.send((Action::RemoveCampaign, campain_name)) {
                logd("receiver dropped");
            }
        } else if c == "delete-file" {
            let file_name = cc.to_owned();

            if let Err(_) = sender.send((Action::RemoveFile, file_name)) {
                logd("receiver dropped");
            }
        } else if c == "list-campaigns" {
            if let Err(_) = sender.send((Action::ListCampaigns, "".to_string())) {
                logd("the receiver dropped");
            }
        } else if ms == "run-all-campaigns" {
            if let Err(_) = sender.send((Action::RunAllCampaigns, "".to_string())) {
                logd("the receiver dropped");
            }
        } else if c == "list-campaign-stats" {
            tokio::task::spawn(async move {
                if let Err(_) = sender.send((Action::ListValidCampaigns, "".to_string())) {
                    logd("the receiver dropped");
                }
            });
        } else if c == "list-engines" {
            tokio::task::spawn(async move {
                if let Err(_) = sender.send((Action::ListEngines, "".to_string())) {
                    logd("the receiver dropped");
                }
            });
        } else if c == "list-files" {
            tokio::task::spawn(async move {
                if let Err(_) = sender.send((Action::ListFiles, "".to_string())) {
                    logd("the receiver dropped");
                }
            });
        } else if c == "create-engine" {
            let cc = cc.clone();
            let v: Eng = serde_json::from_str(&cc).unwrap_or_default();

            let n = v.name;

            if n.is_empty() == false {
                let db_dir = string_concat!("_db/engines/", n);
                let pt = v.paths;
                let pat = v.patterns;

                tokio::task::spawn(async move {
                    let ptt = pt.split(',');
                    let ott = pat.split(',');

                    if !std::path::Path::new(&db_dir).exists() {
                        tokio::fs::create_dir(&db_dir).await.unwrap();
                    }

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
                if let Err(_) = sender.send((Action::CreateEngine, "".to_string())) {
                    logd("the receiver dropped");
                }
            }
        } else if c == "delete-engine" {
            let e_name = cc.to_owned();

            if let Err(_) = sender.send((Action::RemoveEngine, e_name)) {
                logd("receiver dropped");
            }
        } else if c == "list-totals" {
            let e_name = cc.to_owned();

            if let Err(_) = sender.send((Action::ListFileCount, e_name)) {
                logd("receiver dropped");
            }
        }

        future::ok(())
    });

    // inputs received to the request possible to broadcast all
    pin_mut!(broadcast_incoming);

    broadcast_incoming.await.unwrap_or_default();

    logd(string_concat::string_concat!(
        &addr.to_string(),
        " disconnected"
    ));

    handle.await.unwrap();
}

pub async fn start() -> Result<(), IoError> {
    env_logger::init();
    utils::init_config().await;

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let client_addr = env::args().nth(2).unwrap_or_else(|| "0.0.0.0".to_string());
    let client_port = env::args().nth(3).unwrap_or_else(|| "3000".to_string());

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    let address = client_addr.split(".");

    let cad = address
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    // http server
    tokio::spawn(async move {
        let port = client_port.parse::<u16>().unwrap_or(3000);
        // todo allow custom http address
        let addr = SocketAddr::from(([cad[0], cad[1], cad[2], cad[3]], port));

        let make_svc = make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(panel::panel_html::panel_handle))
        });

        if let Err(e) = Server::bind(&addr).serve(make_svc).await {
            eprintln!("server error: {}. Port not found {}", e, port);
        }
    });

    println!("Listening on: {} and 0.0.0.0:3000", &addr);

    tokio::spawn(async move { ft::file_server().await });

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }

    Ok(())
}
