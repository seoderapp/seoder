use crate::string_concat::string_concat;
use crate::string_concat::string_concat_impl;
use crate::tokio::io::BufReader;
use seoder_lib::ENTRY_PROGRAM;
use seoder_lib::packages::spider::configuration::setup;
use seoder_lib::packages::spider::utils::log;
use seoder_lib::tokio;
use seoder_lib::tokio::io::AsyncBufReadExt;
use tokio::fs::File;

/// build a custom engine config from path and target file
pub async fn engine_builder(dptt: String) -> (Vec<String>, Vec<String>, String) {
    let selected_engine = tokio::spawn(async move {
        let mut engine = "".to_string();

        let config = string_concat!(ENTRY_PROGRAM.0, dptt, "/config.txt");

        let f = if tokio::fs::metadata(&config).await.is_ok() {
            config
        } else {
            string_concat!(ENTRY_PROGRAM.0, dptt, "/config.txt")
        };

        match File::open(f).await {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut lines = reader.lines();

                while let Some(line) = lines.next_line().await.unwrap() {
                    let hh = line.split(" ").collect::<Vec<&str>>();

                    if hh.len() == 2 {
                        if hh[0] == "engine" {
                            engine = hh[1].to_string();
                        }
                    }
                }
            }
            Err(_) => {
                log("_db/engines config.txt file does not exist {}", "");
            }
        };

        engine
    })
    .await
    .unwrap();

    // todo: allow param passing from special configs
    let selected_file = tokio::spawn(async move {
        let mut target = "urls-input.txt".to_string();

        match File::open(string_concat!("./config.txt")).await {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut lines = reader.lines();

                while let Some(line) = lines.next_line().await.unwrap() {
                    let hh = line.split(" ").collect::<Vec<&str>>();

                    if hh.len() == 2 {
                        if hh[0] == "target" {
                            target = hh[1].to_string();
                        }
                    }
                }
            }
            Err(_) => {
                log("config.txt file does not exist", "");
            }
        };

        target
    })
    .await
    .unwrap();

    if !selected_engine.is_empty() {
        let eselected = selected_engine.clone();

        tokio::spawn(async move {
            let b = string_concat!("_db/engines/", eselected, "/paths.txt");
            let f = if tokio::fs::metadata(&b).await.is_ok() {
                b
            } else {
                string_concat!("../", b)
            };

            let paths = crate::utils::lines_to_vec(f).await;

            let b = string_concat!("_db/engines/", eselected, "/patterns.txt");
            let f = if tokio::fs::metadata(&b).await.is_ok() {
                b
            } else {
                string_concat!("../", b)
            };

            let patterns = crate::utils::lines_to_vec(f).await;

            (paths, patterns, selected_file)
        })
        .await
        .unwrap()
    } else {
        let (_, __, ___, ____, engine) = setup(true);

        (
            engine.campaign.paths,
            engine.campaign.patterns,
            selected_file,
        )
    }
}
