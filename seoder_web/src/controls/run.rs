use crate::builder;
use crate::log;
use crate::string_concat::string_concat;
use crate::string_concat::string_concat_impl;
use crate::tokio;
use crate::OutGoing;
use crate::Website;
use crate::ENTRY_PROGRAM;

/// run all programs
pub async fn run_all(outgoing: OutGoing) -> OutGoing {
    let mut dir = tokio::fs::read_dir(&ENTRY_PROGRAM.0).await.unwrap();

    while let Some(child) = dir.next_entry().await.unwrap_or_default() {
        if child.metadata().await.unwrap().is_dir() {
            // path
            let dpt = child.path().to_str().unwrap().to_owned();

            if !dpt.ends_with("/valid") {
                let (pt, pat, target) = builder::engine_builder(&dpt).await;

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

    outgoing
}

/// run single program
pub async fn run(outgoing: OutGoing, input: &str) -> OutGoing {
    let (pt, pat, target) = builder::engine_builder(&input).await;

    let mut website: Website = Website::new(&target);

    website.engine.campaign.name = input.into();
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

    outgoing
}
