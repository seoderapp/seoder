#![forbid(unsafe_code)]

#[cfg(all(not(windows), not(target_os = "android"), feature = "jemalloc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use crate::string_concat::string_concat;
use crate::string_concat::string_concat_impl;
use crate::tokio::fs::File;
use crate::tokio::time::Duration;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use seoder_lib::packages::spider::utils::{log, logd};
use seoder_lib::tokio::io::AsyncWriteExt;
use seoder_lib::tokio::sync::mpsc::unbounded_channel;
use seoder_lib::Website;
use seoder_lib::ENTRY_PROGRAM;
pub use seoder_lib::{serde_json, string_concat, tokio};
use serde::{Deserialize, Serialize};
use sysinfo::{System, SystemExt};
use tokio_tungstenite::WebSocketStream;
use tungstenite::{Message, Result};
use lazy_static::lazy_static;

use std::io::Error as IoError;
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
use utils::validate_program;
use std::sync::atomic::AtomicBool;

extern crate lazy_static;
extern crate tera;

mod builder;
mod controls;
mod ft;
mod panel;
mod utils;

/// determine action
#[derive(PartialEq)]
enum Action {
    Stats,
    Config,
    ListEngines,
    ListFiles,
    ListFileCount,
    ListValidCampaigns,
    RunCampaign,
    RunAllCampaigns,
    CreateEngine,
    RemoveEngine,
    RemoveFile,
    SetList,
    SetBuffer,
    SetProxy,
    SetLicense,
    Loop,
}

/// action handling
type Controller = (Action, String);

type Tx = futures_channel::mpsc::UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

pub type OutGoing = SplitSink<WebSocketStream<TcpStream>, Message>;

lazy_static! {
    /// is the license enabled
    static ref LICENSED: Mutex<bool> = Mutex::new(false);
}

/// new engine
#[derive(Serialize, Deserialize, Debug, Default)]
struct Eng {
    name: String,
    paths: String,
    patterns: String,
}

/// tick status refreshing
async fn ticker(mut outgoing: OutGoing, s: &System) -> OutGoing {
    outgoing
        .send(Message::Text(controls::stats::stats(&s).to_string()))
        .await
        .unwrap_or_default();

    outgoing
}

/// get ws stream
async fn build_ws_stream(addr: &SocketAddr, raw_stream: TcpStream) -> WebSocketStream<TcpStream> {
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

    ws_stream
}

/// handle async connections to socket and run command
async fn handle_connection(_peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    let ws_stream = build_ws_stream(&addr, raw_stream).await;

    let (mut outgoing, incoming) = ws_stream.split();

    let (sender, mut receiver): (UnboundedSender<Controller>, UnboundedReceiver<Controller>) =
        unbounded_channel();

    // validate license 
    let stored_license = controls::list::license().await;

    // set valid license in dev mode
    let mut valid_license = cfg!(debug_assertions) || *LICENSED.lock().unwrap();

    if !stored_license.is_empty() && !valid_license {
        valid_license = validate_program(&stored_license).await;
        *LICENSED.lock().unwrap() = valid_license;
    }
    
    let handle = tokio::spawn(async move {
        let mut s = System::new_all();
        // let mut interval = tokio::time::interval(Duration::from_millis(1000));

        while let Some(m) = receiver.recv().await {
            let (st, input) = m;

            if st == Action::Stats {
                s.refresh_all();
                let v = controls::stats::stats(&s);

                outgoing
                    .send(Message::Text(v.to_string()))
                    .await
                    .unwrap_or_default();
            }

            if st == Action::ListValidCampaigns {
                outgoing = controls::list::list_valid(outgoing).await;
            }

            if st == Action::ListEngines {
                outgoing = controls::list::list_engines(outgoing).await;
            }

            if st == Action::ListFileCount {
                outgoing = controls::list::list_file_count(outgoing).await;
            }

            if st == Action::ListFiles {
                outgoing = controls::list::list_files(outgoing).await;
            }

            if st == Action::RemoveFile {
                outgoing = controls::fs::remove_file(outgoing, &input).await;
            }

            if st == Action::RunAllCampaigns && valid_license {
                outgoing = controls::run::run_all(outgoing).await;
            }

            if st == Action::RunCampaign && valid_license {
                outgoing = controls::run::run(outgoing, &input).await;
            }

            if st == Action::RemoveEngine {
                outgoing = controls::fs::remove_engine(outgoing, &input).await;
            }

            if st == Action::Config {
                outgoing = controls::list::config(outgoing).await;
            }

            if st == Action::SetLicense {
                utils::write_config("license", &input).await;
                valid_license = validate_program(&stored_license).await;
            }

            if st == Action::SetProxy {
                utils::write_config("proxy", &input).await;
            }

            if st == Action::SetBuffer {
                utils::write_config("buffer", &input).await;
            }

            if st == Action::SetList {
                utils::write_config("target", &string_concat!(&ENTRY_PROGRAM.1, &input)).await;
            }
        }
    });

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
        } else if c == "config" {
            if let Err(_) = sender.send((Action::Config, cc)) {
                logd("receiver dropped");
            }
        } else if c == "run-campaign" {
            if let Err(_) = sender.send((Action::RunCampaign, cc)) {
                logd("receiver dropped");
            }
        } else if c == "set-list" {
            if let Err(_) = sender.send((Action::SetList, cc)) {
                logd("receiver dropped");
            }
        } else if c == "set-license" {
            if let Err(_) = sender.send((Action::SetLicense, cc)) {
                logd("receiver dropped");
            }
        } else if c == "set-buffer" {
            if let Err(_) = sender.send((Action::SetBuffer, cc)) {
                logd("receiver dropped");
            }
        } else if c == "set-proxy" {
            if let Err(_) = sender.send((Action::SetProxy, cc)) {
                logd("receiver dropped");
            }
        } else if c == "delete-file" {
            if let Err(_) = sender.send((Action::RemoveFile, cc)) {
                logd("receiver dropped");
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
            let v: Eng = serde_json::from_str(&cc).unwrap_or_default();

            let n = v.name;

            if n.is_empty() == false {
                let db_dir = string_concat!(ENTRY_PROGRAM.0, &n);
                let pt = v.paths;
                let pat = v.patterns;

                tokio::task::spawn(async move {
                    let ptt = pt.split(',');
                    let ott = pat.split(',');

                    create_dir(&db_dir).await.unwrap();
                    create_dir(&string_concat!(db_dir, "/valid")).await.unwrap();

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
            if let Err(_) = sender.send((Action::RemoveEngine, cc)) {
                logd("receiver dropped");
            }
        } else if c == "list-totals" {
            if let Err(_) = sender.send((Action::ListFileCount, cc)) {
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

/// handle async connections to socket loopback
async fn handle_connection_loop(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    let ws_stream = build_ws_stream(&addr, raw_stream).await;

    let (mut outgoing, incoming) = ws_stream.split();

    let (sender, mut receiver): (UnboundedSender<Controller>, UnboundedReceiver<Controller>) =
        unbounded_channel();

    let (tx, _rx) = futures_channel::mpsc::unbounded();

    peer_map.lock().unwrap().insert(addr, tx);

    let peer_m = peer_map.clone();

    let handle = tokio::spawn(async move {
        let mut s = System::new_all();
        let mut interval = tokio::time::interval(Duration::from_millis(1000));

        while let Some(_) = receiver.recv().await {
            let mut list_ticket = 0;
            let mut list_config = 0;

            // todo: handle fs tick count skip between
            loop {
                outgoing = ticker(outgoing, &s).await;

                if list_config == 0 {
                    outgoing = controls::list::config(outgoing).await;

                    list_config = list_config + 1;
                } else {
                    list_config = list_config + 1;
                    if list_ticket == 20 {
                        list_config = 0;
                    }
                }

                // list on this tick
                if list_ticket == 0 {
                    outgoing = controls::list::list_valid(outgoing).await;
                    outgoing = controls::list::list_engines(outgoing).await;
                    outgoing = controls::list::list_file_count(outgoing).await;
                    outgoing = controls::list::list_files(outgoing).await;
                    list_ticket = list_ticket + 1;
                } else {
                    list_ticket = list_ticket + 1;
                    if list_ticket == 12 {
                        list_ticket = 0;
                    }
                }

                s.refresh_all();
                interval.tick().await;

                if peer_m.lock().unwrap().get(&addr).is_none() {
                    break;
                }
            }
        }
    });

    let broadcast_incoming = incoming.try_for_each(|_| {
        let sender = sender.clone();

        if let Err(_) = sender.send((Action::Loop, "".to_string())) {
            logd("the receiver dropped");
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

    peer_map.lock().unwrap().remove(&addr);

    handle.await.unwrap();
}

pub async fn start() -> Result<(), IoError> {
    env_logger::init();
    utils::init_config().await;
    seoder_lib::init().await;
    // server peer state
    let state = PeerMap::new(Mutex::new(HashMap::new()));

    let mut prog = "";
    // todo: use custom port
    let mut addr = String::from("127.0.0.1:8080");

    match env::var("SEODER_PROGRAM") {
        Ok(v) => {
            if v == "app" {
                prog = "app"
            }
        }
        Err(_) => {}
    }

    match env::var("SERVER_ADDRESS") {
        Ok(v) => {
            addr = v;
        }
        Err(_) => {}
    }

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    // start the web server for the client and assets
    if prog != "app" {
        let client_addr = env::args().nth(2).unwrap_or_else(|| "0.0.0.0".to_string());
        let address = client_addr.split(".");

        let cad = address
            .map(|x| x.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        // http server
        tokio::spawn(async move {
            let client_port = env::args().nth(3).unwrap_or_else(|| "3000".to_string());
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
    }

    tokio::spawn(async move { ft::file_server().await });

    let mut addrl = addr.clone();
    addrl.pop();
    // loop ws
    addrl = string_concat!(addrl, "9");

    let s = state.clone();

    tokio::spawn(async move {
        let try_socketl = TcpListener::bind(&addrl).await;
        let listenerl = try_socketl.expect("Failed to bind");

        while let Ok((stream, addr)) = listenerl.accept().await {
            tokio::spawn(handle_connection_loop(s.clone(), stream, addr));
        }
    });

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }

    Ok(())
}
