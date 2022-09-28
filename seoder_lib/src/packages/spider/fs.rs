use crate::packages::spider::utils::logd;
use crate::packages::spider::website::CONFIG;

use super::website::Campaign;
use super::website::Message;

use regex::RegexSet;
use scraper::Html;
use scraper::Selector;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::fs::read_dir;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task;

/// create a new file at path
pub async fn create_file(path: &str) -> File {
    File::create(&path).await.unwrap()
}

/// replace extra base adding
fn repb(a: &str) -> String {
    if a.starts_with("_db/campaigns/_db/campaigns/") {
        a.replace("_db/campaigns/_db/campaigns/", "_db/campaigns/")
            .to_string()
    } else {
        a.to_string()
    }
}

/// store the content to file system
pub async fn store_fs_io_matching(
    campaign: &Campaign,
    mut rx: UnboundedReceiver<Message>,
    global_thread_count: Arc<Mutex<usize>>,
) {
    let path = &campaign.name;
    let patterns = &campaign.patterns;

    let rgx = RegexSet::new(if !&patterns.is_empty() {
        patterns
    } else {
        &CONFIG.4.campaign.patterns
    })
    .unwrap();

    task::yield_now().await;

    let eg_c = "_db/campaigns/";

    if tokio::fs::metadata(eg_c).await.is_ok() == false {
        tokio::fs::create_dir(&repb(eg_c)).await.unwrap_or_default();
    }

    lazy_static! {
        static ref SELECTOR: Selector =
            Selector::parse("body > *:not(script):not(noscript):not(css):not(style):not(link)")
                .unwrap();
    }

    let source_match = campaign.source_match;

    // if campaign is empty loop through all folders and spawn custom threads
    if path.is_empty() {
        let mut entries = read_dir(&eg_c).await.unwrap();

        while let Some(entry) = entries.next_entry().await.unwrap() {
            let e = entry.path().to_str().unwrap().to_owned();
            let cmp_base = string_concat!(&e, "/valid");

            // only iterate through directory contents
            if !match tokio::fs::metadata(&cmp_base).await {
                Ok(dir) => dir.is_dir(),
                _ => false,
            } {
                continue;
            }

            tokio::fs::create_dir(&repb(&cmp_base))
                .await
                .unwrap_or_default();

            let mut o = create_file(&string_concat!(&cmp_base, "/links.txt")).await;

            while let Some(i) = rx.recv().await {
                let (link, jor, spawned) = i;
                let (response, _) = jor;

                if spawned && *global_thread_count.lock().unwrap() > 0 {
                    *global_thread_count.lock().unwrap() -= 1;
                }

                let error = response.starts_with("- error ");
                let link = if error {
                    string_concat!(response.replacen("- error ", "", 1), "\n")
                } else {
                    string_concat!(link, "\n")
                };

                if response == "" || error {
                    continue;
                }

                let response = response.clone();

                let rgx = rgx.clone();

                if source_match {
                    let result = rgx.is_match(&response);
                    task::yield_now().await;
                    if result {
                        o.write(&link.as_bytes()).await.unwrap();
                    }
                    continue;
                }

                let (tx, rxx) = tokio::sync::oneshot::channel();

                task::spawn(async move {
                    task::yield_now().await;
                    let doc = Html::parse_document(&response);
                    let items = doc.select(&SELECTOR);
                    let mut senders: Vec<String> = Vec::with_capacity(items.size_hint().0);

                    for element in items {
                        senders.push(element.text().map(|s| s.chars()).flatten().collect());
                    }

                    if let Err(_) = tx.send(senders) {
                        logd("the receiver dropped");
                    }
                });

                task::yield_now().await;

                match rxx.await {
                    Ok(v) => {
                        let result = rgx.is_match(&v.join(""));
                        task::yield_now().await;
                        if result {
                            o.write(&link.as_bytes()).await.unwrap();
                        }
                    }
                    Err(_) => logd("the sender dropped"),
                }
            }
        }
    } else {
        let cmp_base = string_concat!("_db/campaigns/", path);

        tokio::fs::create_dir(&repb(&cmp_base))
            .await
            .unwrap_or_default();

        let cmp_base = string_concat!(&cmp_base, "/valid");

        tokio::fs::create_dir(&repb(&cmp_base))
            .await
            .unwrap_or_default();

        let mut o = create_file(&repb(&string_concat!(&cmp_base, "/links.txt"))).await;

        while let Some(i) = rx.recv().await {
            let (link, jor, spawned) = i;
            let (response, _) = jor;

            if spawned && *global_thread_count.lock().unwrap() > 0 {
                *global_thread_count.lock().unwrap() -= 1;
            }

            let error = response.starts_with("- error ");

            if response == "" || error {
                continue;
            }

            let link = string_concat!(link, "\n");

            if source_match {
                let result = rgx.is_match(&response);
                task::yield_now().await;
                if result {
                    o.write(&link.as_bytes()).await.unwrap();
                }
            } else {
                let (tx, rxx) = tokio::sync::oneshot::channel();

                let ssource = response.clone();

                task::spawn(async move {
                    task::yield_now().await;
                    let doc = Html::parse_document(&ssource);
                    let items = doc.select(&SELECTOR);
                    let mut senders: Vec<String> = Vec::with_capacity(items.size_hint().0);

                    for element in items {
                        senders.push(element.text().map(|s| s.chars()).flatten().collect());
                    }

                    if let Err(_) = tx.send(senders) {
                        logd("the receiver dropped");
                    }
                });

                task::yield_now().await;

                match rxx.await {
                    Ok(v) => {
                        let result = rgx.is_match(&v.join(""));
                        task::yield_now().await;
                        if result {
                            o.write(&link.as_bytes()).await.unwrap();
                        }
                    }
                    Err(_) => logd("the sender dropped"),
                }
            }
        }
    }
}
