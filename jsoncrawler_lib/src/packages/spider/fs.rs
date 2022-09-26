use crate::packages::spider::utils::logd;
use crate::packages::spider::website::CONFIG;

use super::utils::log;
use super::website::Message;
use super::JsonOutFileType;
use jsonl::write;
use regex::RegexSet;
use scraper::Html;
use scraper::Selector;
use serde_json::Value;
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
    if a.starts_with("_engines_/campaigns/_engines_/campaigns/") {
        a.replace(
            "_engines_/campaigns/_engines_/campaigns/",
            "_engines_/campaigns/",
        )
        .to_string()
    } else {
        a.to_string()
    }
}

/// store the content to file system
pub async fn store_fs_io(
    paths: (
        &string_concat::String,
        &string_concat::String,
        &string_concat::String,
        &string_concat::String,
        &string_concat::String,
    ),
    mut rx: UnboundedReceiver<Message>,
    global_thread_count: Arc<Mutex<usize>>,
) {
    let (mut o, mut ok_t, mut okv_t, mut ce_t, mut al_t) = tokio::join!(
        create_file(&paths.0),
        create_file(&paths.1),
        create_file(&paths.2),
        create_file(&paths.3),
        create_file(&paths.4),
    );

    while let Some(i) = rx.recv().await {
        let (link, jor, spawned) = i;
        let (response, oo) = jor;

        if spawned && *global_thread_count.lock().unwrap() > 0 {
            *global_thread_count.lock().unwrap() -= 1;
        }

        let error = response.starts_with("- error ");
        // detailed json message
        let link = if error {
            string_concat!(response.replacen("- error ", "", 1), "\n")
        } else {
            string_concat!(link, "\n")
        };

        if oo == JsonOutFileType::Error {
            ce_t.write(&link.as_bytes()).await.unwrap();
        }

        if oo == JsonOutFileType::Unknown {
            al_t.write(&link.as_bytes()).await.unwrap();
        }

        if response == "" || error {
            continue;
        }

        let j: Value = serde_json::from_str(&response).unwrap_or_default();
        task::yield_now().await;

        if !j.is_null() {
            match write(&mut o, &j).await {
                Ok(_) => {
                    log("wrote jsonl = {}", &link);
                    ok_t.write(&link.as_bytes()).await.unwrap();
                }
                _ => {
                    log("failed to write jsonl = {}", &link);
                    okv_t.write(&link.as_bytes()).await.unwrap();
                }
            }
        } else {
            log("The file is not valid json = {}", &link);
            okv_t.write(&link.as_bytes()).await.unwrap();
        }
    }
}

/// store the content to file system
pub async fn store_fs_io_matching(
    path: &String,
    patterns: Vec<String>,
    mut rx: UnboundedReceiver<Message>,
    global_thread_count: Arc<Mutex<usize>>,
) {
    let rgx = RegexSet::new(if !&patterns.is_empty() {
        &patterns
    } else {
        &CONFIG.4.campaign.patterns
    })
    .unwrap();

    task::yield_now().await;

    let eg_c = "_engines_/campaigns/";

    if tokio::fs::metadata(eg_c).await.is_ok() == false {
        tokio::fs::create_dir(&repb(eg_c)).await.unwrap_or_default();
    }

    lazy_static! {
        static ref SELECTOR: Selector =
            Selector::parse("body > *:not(script):not(noscript):not(css):not(style):not(link)")
                .unwrap();
    }

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
                let response = response.clone();

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
        let cmp_base = string_concat!("_engines_/campaigns/", path);

        tokio::fs::create_dir(&repb(&cmp_base))
            .await
            .unwrap_or_default();
        let cmp_base = string_concat!(&cmp_base, "/valid");
        tokio::fs::create_dir(&repb(&cmp_base))
            .await
            .unwrap_or_default();

        let od = string_concat!(&cmp_base, "/links.txt");

        let mut o = create_file(&repb(&od)).await;

        while let Some(i) = rx.recv().await {
            let (link, jor, spawned) = i;
            let (response, _) = jor;

            if spawned && *global_thread_count.lock().unwrap() > 0 {
                *global_thread_count.lock().unwrap() -= 1;
            }

            let error = response.starts_with("- error ");
            // detailed json message
            let link = if error {
                string_concat!(response.replacen("- error ", "", 1), "\n")
            } else {
                string_concat!(link, "\n")
            };

            if response == "" || error {
                continue;
            }

            let fdr = rgx.is_match(&response);
            task::yield_now().await;

            if fdr {
                o.write(&link.as_bytes()).await.unwrap();
            }
        }
    }
}
