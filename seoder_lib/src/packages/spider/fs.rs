use super::website::Campaign;
use super::website::Message;
use crate::packages::spider::utils::logd;
use crate::packages::spider::website::CONFIG;

use crate::ENTRY_PROGRAM;
use regex::RegexSet;
use scraper::Html;
use scraper::Selector;
use std::sync::atomic::{AtomicI8, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tokio::fs::read_dir;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task;

const MEDIA_IGNORE_SELECTOR: &str = r#":not([href$=".ico"]):not([href$=".png"]):not([href$=".jpg"]):not([href$=".jpeg"]):not([href$=".svg"]):not([href$=".webp"]):not([href$=".gif"]):not([href$=".pdf"]):not([href$=".tiff"]):not([href$=".wav"]):not([href$=".mp3"]):not([href$=".mp4"]):not([href$=".ogg"]):not([href$=".webm"]):not([href$=".sql"]):not([href$=".zip"]):not([href$=".docx"]):not([href$=".git"]):not([href$=".json"]):not([href$=".xml"]):not([href$=".css"]):not([href$=".md"]):not([href$=".txt"]):not([href$=".js"]):not([href$=".jsx"]):not([href$=".csv"])"#;

/// create a new file at path
pub async fn create_file(path: &str) -> File {
    File::create(&path).await.unwrap()
}

/// build file target directorys
pub async fn create_run_files(e: &str) -> (File, File, File, File) {
    match tokio::fs::metadata(&e).await {
        Ok(_) => (),
        _ => {
            tokio::fs::create_dir(&e).await.unwrap_or_default();
            ()
        }
    }

    let cmp_base = string_concat!(&e, "/valid");
    let cmp_invalid = string_concat!(&e, "/invalid");
    let cmp_errors = string_concat!(&e, "/errors");

    tokio::fs::create_dir(&&cmp_base).await.unwrap_or_default();
    tokio::fs::create_dir(&&cmp_invalid)
        .await
        .unwrap_or_default();
    tokio::fs::create_dir(&&cmp_errors)
        .await
        .unwrap_or_default();

    let o = create_file(&string_concat!(&cmp_base, "/links.txt")).await;
    let oo = create_file(&string_concat!(&cmp_invalid, "/links.txt")).await;
    let oe = create_file(&string_concat!(&cmp_errors, "/links.txt")).await;
    let oc = create_file(&string_concat!(&e, "/contacts.txt")).await;

    (o, oo, oe, oc)
}

/// get contact information for followups
pub async fn store_fs_io_contacts(html: &String) -> Vec<String> {
    // todo: conditional lazy static
    lazy_static! {
        static ref SELECTOR: Selector =
            Selector::parse(&string_concat!("a[href^=mailto]", MEDIA_IGNORE_SELECTOR)).unwrap();
    }

    let res = html.clone();

    let handles = task::spawn(async move {
        let doc = Html::parse_document(&res);
        let items = doc.select(&SELECTOR);
        let mut senders: Vec<String> = Vec::with_capacity(items.size_hint().0);

        for element in items {
            senders.push(element.value().attr("href").unwrap_or_default().to_string());
        }

        senders
    });

    handles.await.unwrap()
}

/// store the content to file system
pub async fn store_fs_io_matching(
    campaign: &Campaign,
    mut rx: UnboundedReceiver<Message>,
    global_thread_count: Arc<Mutex<usize>>,
    chandle: Arc<AtomicI8>,
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

    let mut interval = tokio::time::interval(Duration::from_millis(10));

    // if campaign is empty loop through all folders and spawn custom threads
    if path.is_empty() {
        let mut entries = match read_dir(&ENTRY_PROGRAM.0).await {
            Ok(dir) => dir,
            Err(_) => {
                logd("No Campaigns found!");
                return;
            }
        };

        while let Some(entry) = entries.next_entry().await.unwrap() {
            if entry.metadata().await.unwrap().is_dir() {
                let e = entry.path().to_str().unwrap().to_owned();

                let (mut o, mut oo, mut oe, mut oc) = create_run_files(&e).await;

                while let Some(i) = rx.recv().await {
                    while chandle.load(Ordering::Relaxed) == 1 {
                        interval.tick().await;
                    }
                    if chandle.load(Ordering::Relaxed) == 2 {
                        break;
                    }

                    let (original_link, jor, spawned) = i;
                    let (response, _) = jor;

                    if spawned && *global_thread_count.lock().unwrap() > 0 {
                        *global_thread_count.lock().unwrap() -= 1;
                    }

                    let error = response.starts_with("- error ");
                    let link = string_concat!(&original_link, "\n");
                    task::yield_now().await;

                    // errors
                    if response == "" || error {
                        oe.write(&link.as_bytes()).await.unwrap();
                        continue;
                    }

                    let rgx = rgx.clone();

                    if source_match {
                        let result = rgx.is_match(&response);

                        if result {
                            o.write(&link.as_bytes()).await.unwrap();
                            let h = store_fs_io_contacts(&response).await;
                            for l in h {
                                let l = string_concat!(&original_link, " ", &l, "\n");
                                oc.write(&l.as_bytes()).await.unwrap();
                            }
                        } else {
                            oo.write(&link.as_bytes()).await.unwrap();
                        }
                    } else {
                        // match only html rendered
                        let (tx, rxx) = tokio::sync::oneshot::channel();

                        let res = response.clone();

                        task::spawn(async move {
                            let doc = Html::parse_document(&res);
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
                                if result {
                                    o.write(&link.as_bytes()).await.unwrap();
                                    let h = store_fs_io_contacts(&response).await;
                                    for l in h {
                                        let l = string_concat!(&original_link, " ", &l, "\n");
                                        oc.write(&l.as_bytes()).await.unwrap();
                                    }
                                } else {
                                    oo.write(&link.as_bytes()).await.unwrap();
                                }
                            }
                            Err(_) => logd("the sender dropped"),
                        }
                    }
                }
            }
        }
    } else {
        // pass in the entry program path
        let cmp = string_concat!(ENTRY_PROGRAM.0, &path);
        let (mut o, mut oo, mut oe, mut oc) = create_run_files(&cmp).await;

        while let Some(i) = rx.recv().await {
            while chandle.load(Ordering::Relaxed) == 1 {
                interval.tick().await;
            }
            if chandle.load(Ordering::Relaxed) == 2 {
                break;
            }

            let (original_link, jor, spawned) = i;
            let (response, _) = jor;

            if spawned && *global_thread_count.lock().unwrap() > 0 {
                *global_thread_count.lock().unwrap() -= 1;
            }

            let error = response.starts_with("- error ");

            let link = string_concat!(&original_link, "\n");

            task::yield_now().await;

            if response == "" || error {
                oe.write(&link.as_bytes()).await.unwrap();
                continue;
            }

            if source_match {
                let result = rgx.is_match(&response);
                if result {
                    o.write(&link.as_bytes()).await.unwrap();
                    let h = store_fs_io_contacts(&response).await;
                    for l in h {
                        let l = string_concat!(&original_link, " ", &l, "\n");
                        oc.write(&l.as_bytes()).await.unwrap();
                    }
                } else {
                    oo.write(&link.as_bytes()).await.unwrap();
                }
            } else {
                let (tx, rxx) = tokio::sync::oneshot::channel();

                let res = response.clone();

                task::spawn(async move {
                    let doc = Html::parse_document(&res);
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
                        if result {
                            o.write(&link.as_bytes()).await.unwrap();
                            let h = store_fs_io_contacts(&response).await;
                            for l in h {
                                let l = string_concat!(&original_link, " ", &l, "\n");
                                oc.write(&l.as_bytes()).await.unwrap();
                            }
                        } else {
                            oo.write(&link.as_bytes()).await.unwrap();
                        }
                    }
                    Err(_) => logd("the sender dropped"),
                }
            }
        }
    }
}
