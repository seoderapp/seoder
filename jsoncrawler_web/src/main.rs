#![forbid(unsafe_code)]

#[cfg(all(not(windows), not(target_os = "android"), feature = "jemalloc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use jsoncrawler_lib::tokio::sync::mpsc::unbounded_channel;
use sysinfo::{System, SystemExt};

use tungstenite::{Message, Result};

use crate::string_concat::string_concat_impl;
use jsoncrawler_lib::packages::spider::utils::logd;
use jsoncrawler_lib::{serde_json, string_concat, tokio};

use std::io::{Error as IoError, Write};
use std::time::Duration;
use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::SinkExt;
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};

use crate::serde_json::json;

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

    let p = peer_map.clone();

    let handle = tokio::spawn(async move {
        use sysinfo::CpuExt;
        use sysinfo::NetworkExt;
        let mut interval = tokio::time::interval(Duration::from_millis(1000));
        let mut s = System::new_all();

        match receiver.recv().await {
            Some(_) => {
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
            _ => println!("the sender dropped"),
        };
    });

    let broadcast_incoming = incoming.try_for_each(|msg| {
        let mut lock = std::io::stdout().lock();
        let m = msg.clone();
        let txt = m.to_text().unwrap();

        let sender = sender.clone();

        writeln!(lock, "Received a message from {}: {}", &addr, &txt).unwrap();

        // start the feed
        if txt.trim() == "feed".to_string() {
            tokio::spawn(async move {
                if let Err(_) = sender.send(1) {
                    println!("the receiver dropped");
                }
            });
        }

        future::ok(())
    });

    // inputs received to the reqwest
    pin_mut!(broadcast_incoming);
    broadcast_incoming.await.unwrap_or_default();

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

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    println!("Listening on: {}", &addr);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }

    Ok(())
}