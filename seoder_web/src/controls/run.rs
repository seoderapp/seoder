use crate::builder;
// use crate::json;
use crate::log;
use crate::string_concat::string_concat;
use crate::string_concat::string_concat_impl;
use crate::tokio;
use crate::Website;
use crate::ENTRY_PROGRAM;
use seoder_lib::packages::spider::utils::logd;
use seoder_lib::tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

/// run all programs
pub async fn run_all() {
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
            let (pt, pat, target) = builder::engine_builder(&engine_name).await;

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

    while let Some(_) = receiver.recv().await {
        // let (dpt, b) = m;
        // TODO: set global finished handler
        // let v = json!({ "finished": dpt, "time": b });
        // todo set global finished via loop
    }
}

/// run single program
pub async fn run(input: &str) {
    let (pt, pat, target) = builder::engine_builder(&input).await;

    let mut website: Website = Website::new(&target);

    let engine_name = input.replacen(&ENTRY_PROGRAM.0, "", 1);
    
    website.engine.campaign.name = engine_name;
    website.engine.campaign.paths = pt;
    website.engine.campaign.patterns = pat;

    let performance = crate::tokio::time::Instant::now();

    website.crawl().await;

    let b = string_concat!(performance.elapsed().as_secs().to_string(), "s - ", &input);

    log("crawl finished - time elasped: ", &b);

    // let v = json!({ "finished": input, "time": b });

}
