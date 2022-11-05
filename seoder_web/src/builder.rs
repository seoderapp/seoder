use crate::string_concat::string_concat;
use crate::string_concat::string_concat_impl;
use crate::tokio::io::BufReader;
use crate::utils::build_query;
use seoder_lib::packages::spider::configuration::setup;
use seoder_lib::packages::spider::utils::log;
use seoder_lib::tokio;
use seoder_lib::tokio::io::AsyncBufReadExt;
use seoder_lib::ENTRY_PROGRAM;
use tokio::fs::File;

/// build a custom engine config from path and target file
pub async fn engine_builder(selected_engine: &str) -> (Vec<String>, Vec<String>, String, bool) {
    let e = selected_engine.to_string();

    let (f, ff, cf) = (
        string_concat!(ENTRY_PROGRAM.0, &e, "/paths.txt"),
        string_concat!(ENTRY_PROGRAM.0, &e, "/patterns.txt"),
        string_concat!(ENTRY_PROGRAM.0, &e, "/config.txt"),
    );

    // todo: allow param passing from special configs and optional if config.txt already set
    let (source_match, selected_file) = tokio::spawn(async move {
        let mut target = "urls-input.txt".to_string();
        let mut source_match = true;

        // configuration file
        match File::open(&cf).await {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut lines = reader.lines();

                while let Some(line) = lines.next_line().await.unwrap() {
                    let hh = line.split(" ").collect::<Vec<&str>>();
                    let h0 = hh[0];
                    let h1 = build_query(&hh);

                    if hh.len() >= 2 {
                        if h0 == "source" {
                            source_match = h1.parse::<bool>().unwrap_or_default();
                        }
                    }
                }
            }
            Err(_) => {
                log("file does not exist - ", &cf);
            }
        };

        match File::open(&ENTRY_PROGRAM.2).await {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut lines = reader.lines();

                while let Some(line) = lines.next_line().await.unwrap() {
                    let hh = line.split(" ").collect::<Vec<&str>>();
                    let h0 = hh[0];
                    let h1 = build_query(&hh);

                    if hh.len() >= 2 {
                        if h0 == "target" {
                            let path = std::path::Path::new(&h1);
                            let filename = path.file_name().unwrap();
                            let f = filename.to_str().unwrap_or_default().to_string();

                            target = f.to_string();
                        }
                    }
                }
            }
            Err(_) => {
                log("config.txt file does not exist", "");
            }
        };

        (source_match, target)
    })
    .await
    .unwrap();

    if !selected_engine.is_empty() {
        tokio::spawn(async move {
            let paths = crate::utils::lines_to_vec(f).await;
            let patterns = crate::utils::lines_to_vec(ff).await;

            (paths, patterns, selected_file, source_match)
        })
        .await
        .unwrap()
    } else {
        let (_, __, ___, ____, engine) = setup(true);

        (
            engine.campaign.paths,
            engine.campaign.patterns,
            selected_file,
            source_match
        )
    }
}
