use crate::builder;
use crate::json;
use crate::log;
use crate::string_concat::string_concat;
use crate::string_concat::string_concat_impl;
use crate::tokio;
use crate::OutGoing;
use crate::Website;
use crate::ENTRY_PROGRAM;
use futures_util::SinkExt;
use seoder_lib::packages::spider::utils::logd;
use seoder_lib::tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tungstenite::Message;

/// run all programs
pub async fn run_all(mut outgoing: OutGoing) -> OutGoing {
    let mut dir = tokio::fs::read_dir(&ENTRY_PROGRAM.0).await.unwrap();

    let (sender, mut receiver): (
        UnboundedSender<(String, String)>,
        UnboundedReceiver<(String, String)>,
    ) = unbounded_channel();

    while let Some(child) = dir.next_entry().await.unwrap_or_default() {
        if child.metadata().await.unwrap().is_dir() {
            // path
            let dpt = child.path().to_str().unwrap().to_string();
            let engine_name = dpt.replacen(&ENTRY_PROGRAM.0, "", 1);

            // includes the base path
            let (pt, pat, target) = builder::engine_builder(&engine_name, false).await;

            let mut website: Website = Website::new(&target);

            website.engine.campaign.name = engine_name;
            website.engine.campaign.paths = pt;
            website.engine.campaign.patterns = pat;

            let sender = sender.clone();

            tokio::spawn(async move {
                let performance = crate::tokio::time::Instant::now();
                website.crawl().await;
                let b = string_concat!(
                    performance.elapsed().as_secs().to_string(),
                    "s - ",
                    &website.engine.campaign.name
                );

                log("crawl finished - time elasped: ", &b);

                if let Err(_) = sender.send((website.engine.campaign.name, b)) {
                    logd("the receiver dropped");
                }
            });
        }
    }

    drop(sender);

    while let Some(m) = receiver.recv().await {
        let (dpt, b) = m;
        let v = json!({ "finished": dpt, "time": b });

        outgoing
            .send(Message::Text(v.to_string()))
            .await
            .unwrap_or_default();
    }

    outgoing
}

/// run single program
pub async fn run(mut outgoing: OutGoing, input: &str) -> OutGoing {
    let (pt, pat, target) = builder::engine_builder(&input, false).await;

    let mut website: Website = Website::new(&target);

    website.engine.campaign.name = input.into();
    website.engine.campaign.paths = pt;
    website.engine.campaign.patterns = pat;

    let performance = crate::tokio::time::Instant::now();

    let handle = tokio::spawn(async move { website.crawl().await });

    handle.await.unwrap();

    let b = string_concat!(performance.elapsed().as_secs().to_string(), "s - ", &input);

    log("crawl finished - time elasped: ", &b);

    let v = json!({ "finished": input, "time": b });

    outgoing
        .send(Message::Text(v.to_string()))
        .await
        .unwrap_or_default();

    outgoing
}
