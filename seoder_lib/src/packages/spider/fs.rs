use crate::packages::spider::utils::logd;
use crate::packages::spider::website::CONFIG;

use super::website::Campaign;
use super::website::Message;

use crate::ENTRY_PROGRAM;
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

/// store the content to file system
pub async fn store_fs_io_matching(
    campaign: &Campaign,
    mut rx: UnboundedReceiver<Message>,
    global_thread_count: Arc<Mutex<usize>>,
) {
    // todo: conditional lazy static
    lazy_static! {
        static ref SELECTOR: Selector =
            Selector::parse("body > *:not(script):not(noscript):not(css):not(style):not(link)")
                .unwrap();
    }

    let path = &campaign.name;
    let patterns = &campaign.patterns;
    let source_match = campaign.source_match;

    let rgx = RegexSet::new(if !&patterns.is_empty() {
        patterns
    } else {
        &CONFIG.4.campaign.patterns
    })
    .unwrap();

    // if campaign is empty loop through all folders and spawn custom threads
    if path.is_empty() {
        let mut entries = match read_dir(&ENTRY_PROGRAM.0).await {
            Ok(dir) => dir,
            Err(_) => {
                logd("No Campaigns found!");
                // todo: print error
                return;
            }
        };

        while let Some(entry) = entries.next_entry().await.unwrap() {
            let e = entry.path().to_str().unwrap().to_owned();
            let cmp_base = string_concat!(&e, "/valid");
            let cmp_invalid = string_concat!(&e, "/invalid");
            // todo: use error folder split
            // let cmp_errors = string_concat!(&e, "/errors");

            // only iterate through directory contents
            if !match tokio::fs::metadata(&cmp_base).await {
                Ok(dir) => dir.is_dir(),
                _ => false,
            } {
                continue;
            }

            let mut o = create_file(&string_concat!(&cmp_base, "/links.txt")).await;
            let mut oo = create_file(&string_concat!(&cmp_invalid, "/links.txt")).await;

            while let Some(i) = rx.recv().await {
                let (link, jor, spawned) = i;
                let (response, _) = jor;

                if spawned && *global_thread_count.lock().unwrap() > 0 {
                    *global_thread_count.lock().unwrap() -= 1;
                }

                let error = response.starts_with("- error ");
                let link = string_concat!(link, "\n");

                // errors 
                if response == "" || error {
                    oo.write(&link.as_bytes()).await.unwrap();
                    continue;
                }

                let response = response.clone();
                let rgx = rgx.clone();

                if source_match {
                    let result = rgx.is_match(&response);
                    if result {
                        o.write(&link.as_bytes()).await.unwrap();
                    } else {
                        // right empty matches as Errors - TODO Split file
                        oo.write(&link.as_bytes()).await.unwrap();
                        task::yield_now().await;
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

                match rxx.await {
                    Ok(v) => {
                        let result = rgx.is_match(&v.join(""));
                        task::yield_now().await;
                        if result {
                            o.write(&link.as_bytes()).await.unwrap();
                        } else {
                            // todo: split file
                            oo.write(&link.as_bytes()).await.unwrap();
                        }
                    }
                    Err(_) => logd("the sender dropped"),
                }
            }
        }
    } else {
        let cmp = string_concat!(ENTRY_PROGRAM.0, path);

        match tokio::fs::metadata(&cmp).await {
            Ok(_) => (),
            _ => {
                tokio::fs::create_dir(&cmp).await.unwrap_or_default();
                ()
            }
        }

        let cmp_base = string_concat!(&cmp, "/valid");
        let cmp_invalid = string_concat!(&cmp, "/invalid");

        tokio::fs::create_dir(&&cmp_base).await.unwrap_or_default();
        tokio::fs::create_dir(&&cmp_invalid)
            .await
            .unwrap_or_default();

        let mut o = create_file(&&string_concat!(&cmp_base, "/links.txt")).await;
        let mut oo = create_file(&&string_concat!(&cmp_invalid, "/links.txt")).await;

        while let Some(i) = rx.recv().await {
            let (link, jor, spawned) = i;
            let (response, _) = jor;

            if spawned && *global_thread_count.lock().unwrap() > 0 {
                *global_thread_count.lock().unwrap() -= 1;
            }

            let error = response.starts_with("- error ");
            let link = string_concat!(&link, "\n");

            if response == "" || error {
                oo.write(&link.as_bytes()).await.unwrap();
                continue;
            }

            if source_match {
                let result = rgx.is_match(&response);
                task::yield_now().await;
                if result {
                    o.write(&link.as_bytes()).await.unwrap();
                } else {
                    // todo: split file
                    oo.write(&link.as_bytes()).await.unwrap();
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

                match rxx.await {
                    Ok(v) => {
                        let result = rgx.is_match(&v.join(""));
                        task::yield_now().await;
                        if result {
                            o.write(&link.as_bytes()).await.unwrap();
                        } else {
                            // todo: split file
                            oo.write(&link.as_bytes()).await.unwrap();
                        }
                    }
                    Err(_) => logd("the sender dropped"),
                }
            }
        }
    }
}
