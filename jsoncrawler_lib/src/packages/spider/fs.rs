use super::utils::log;
use super::website::Message;
use super::JsonOutFileType;
use jsonl::write;
use serde_json::Value;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::task;

use std::sync::Arc;
use std::sync::Mutex;

/// create a new file at path
pub async fn create_file(path: &String) -> File {
    File::create(&path).await.unwrap()
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
    use regex::RegexSet;

    let rgx = RegexSet::new(patterns).unwrap();

    // outdirectory
    // output txt files
    let cmp_base = string_concat!("_engines_/campaign/", path);
    tokio::fs::create_dir(&cmp_base).await.unwrap_or_default();
    let cmp_base = string_concat!(&cmp_base, "/valid");
    tokio::fs::create_dir(&cmp_base).await.unwrap_or_default();

    let mut o = create_file(&string_concat!(&cmp_base, "/links.txt")).await;

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
