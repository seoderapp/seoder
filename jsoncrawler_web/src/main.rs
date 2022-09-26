#![forbid(unsafe_code)]

#[cfg(all(not(windows), not(target_os = "android"), feature = "jemalloc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use jsoncrawler_lib::tokio::sync::mpsc::unbounded_channel;
use jsoncrawler_lib::Website;
use sysinfo::{System, SystemExt};
use tungstenite::{Message, Result};

use crate::string_concat::string_concat_impl;
use jsoncrawler_lib::packages::spider::utils::{log, logd};
use jsoncrawler_lib::{serde_json, string_concat, tokio};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::SinkExt;
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use std::io::{Error as IoError, Write};
use std::time::Duration;
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

mod panel_html;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
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

    let (tx, _) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);

    let (txx, mut rxx): (
        tokio::sync::mpsc::UnboundedSender<String>,
        tokio::sync::mpsc::UnboundedReceiver<String>,
    ) = unbounded_channel();

    let p = peer_map.clone();

    let handle = tokio::spawn(async move {
        use sysinfo::CpuExt;
        use sysinfo::NetworkExt;
        let mut interval = tokio::time::interval(Duration::from_millis(1000));
        let mut s = System::new_all();

        match receiver.recv().await {
            Some(1) => {
                loop {
                    interval.tick().await;

                    s.refresh_all();

                    let mut net_total_received = 0;
                    let mut net_total_transmited = 0;

                    let networks = s.networks();

                    for (_, data) in networks {
                        net_total_received += data.received();
                        net_total_transmited += data.transmitted();
                    }

                    let v = json!({
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
                    });
                    tokio::task::yield_now().await;
                    // println!("feed in progress {:?}", &v);

                    outgoing
                        .send(Message::Text(v.to_string().into()))
                        .await
                        .unwrap_or_default();

                    if !p.lock().unwrap().contains_key(&addr) {
                        break;
                    }
                }
            }
            Some(2) => {
                // stream iterate list campaigns to client
                outgoing
                    .send(Message::Text("campaigns list todo!".to_string().into()))
                    .await
                    .unwrap_or_default();
            }
            _ => println!("the sender dropped"),
        };
    });

    let broadcast_incoming = incoming.try_for_each(|msg| {
        let mut lock = std::io::stdout().lock();
        let m = msg.clone();
        let txt = m.to_text().unwrap();
        let sender = sender.clone();

        writeln!(lock, "Received a message from {}: {}", &addr, &txt).unwrap();

        // remove newline
        let ms = txt.trim();

        let hh = ms.split(" ").collect::<Vec<&str>>();

        // validate crud messages
        let (c, cc) = if hh.len() == 2 {
            (hh[0], hh[1])
        } else {
            ("", "")
        };

        // start the feed stats
        if ms == "feed" {
            tokio::spawn(async move {
                if let Err(_) = sender.send(1) {
                    logd("the receiver dropped");
                }
            });
        }
        // create new campaign to store crawl results
        else if c == "create-campaign" {
            let cf = cc.to_owned();
            tokio::spawn(async move {
                use crate::string_concat::string_concat;
                let campaign_dir = string_concat!("_engines_/campaigns/", cf);

                tokio::fs::create_dir(&campaign_dir).await.unwrap();
                tokio::fs::create_dir(&string_concat!(campaign_dir, "/valid"))
                    .await
                    .unwrap();
            });
        } else if c == "run-campaign" {
            let txx = txx.clone();
            let cc = cc.to_owned();
            if let Err(_) = txx.send(cc) {
                logd("receiver dropped");
            }
        } else if c == "list-campaigns" {
            tokio::spawn(async move {
                if let Err(_) = sender.send(2) {
                    logd("the receiver dropped");
                }
            });
        } else if c == "run-all-campaigns" {
            // tokio::spawn(async move {
            //     if let Err(_) = sender.send(3) {
            //         logd("the receiver dropped");
            //     }
            // });
        }

        future::ok(())
    });

    // inputs received to the request possible to broadcast all
    pin_mut!(broadcast_incoming);

    broadcast_incoming.await.unwrap_or_default();

    while let Some(input) = rxx.recv().await {
        let input = input.clone();
        let mut website: Website = Website::new(&input);

        tokio::spawn(async move {
            website.crawl().await;
            log("crawl finished - ", &input)
        });
    }

    logd(string_concat::string_concat!(
        &addr.to_string(),
        " disconnected"
    ));

    peer_map.lock().unwrap().remove(&addr);

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
            Ok::<_, Infallible>(service_fn(panel_html::panel_handle))
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
