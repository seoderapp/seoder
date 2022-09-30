use crate::string_concat::string_concat;
use crate::string_concat::string_concat_impl;
use crate::tokio::io::BufReader;
use seoder_lib::packages::spider::configuration::setup;
use seoder_lib::packages::spider::utils::log;
use seoder_lib::tokio;
use seoder_lib::tokio::io::AsyncBufReadExt;
use tokio::fs::File;

/// build a custom engine config from path
pub async fn engine_builder(dptt: String) -> (Vec<String>, Vec<String>) {
    let selected_engine = tokio::spawn(async move {
        let mut engine = "".to_string();

        match File::open(string_concat!("./_db/campaigns/", dptt, "/config.txt")).await {
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

    if selected_engine.is_empty() == false {
        let eselected = selected_engine.clone();

        tokio::spawn(async move {
            let paths =
                crate::utils::lines_to_vec(string_concat!("_db/engines/", eselected, "/paths.txt"))
                    .await;

            let patterns = crate::utils::lines_to_vec(string_concat!(
                "_db/engines/",
                eselected,
                "/patterns.txt"
            ))
            .await;

            (paths, patterns)
        })
        .await
        .unwrap()
    } else {
        let (_, __, ___, ____, engine) = setup(true);

        (engine.campaign.paths, engine.campaign.patterns)
    }
}
