use crate::string_concat::string_concat;
use crate::string_concat::string_concat_impl;
use crate::tokio::io::BufReader;
use seoder_lib::packages::spider::configuration::setup;
use seoder_lib::packages::spider::utils::log;
use seoder_lib::tokio;
use seoder_lib::tokio::io::AsyncBufReadExt;
use seoder_lib::ENTRY_PROGRAM;
use tokio::fs::File;

/// build a custom engine config from path and target file
pub async fn engine_builder(
    selected_engine: &str,
    base_included: bool,
) -> (Vec<String>, Vec<String>, String) {
    let e = selected_engine.to_string();

    // todo: allow param passing from special configs
    let selected_file = tokio::spawn(async move {
        let mut target = "urls-input.txt".to_string();

        match File::open(&ENTRY_PROGRAM.2).await {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut lines = reader.lines();

                while let Some(line) = lines.next_line().await.unwrap() {
                    let hh = line.split(" ").collect::<Vec<&str>>();
                    let h0 = hh[0];
                    let mut h1 = hh[1].to_string();

                    // todo: push all into array after first index
                    if hh.len() == 3 {
                        h1.push_str(hh[2]);
                    }

                    if hh.len() == 2 {
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

        target
    })
    .await
    .unwrap();

    let (f, ff) = if base_included {
        (
            string_concat!(&e, "/paths.txt"),
            string_concat!(&e, "/patterns.txt"),
        )
    } else {
        (
            string_concat!(ENTRY_PROGRAM.0, &e, "/paths.txt"),
            string_concat!(ENTRY_PROGRAM.0, &e, "/patterns.txt"),
        )
    };

    if !selected_engine.is_empty() {
        tokio::spawn(async move {
            let paths = crate::utils::lines_to_vec(f).await;

            let patterns = crate::utils::lines_to_vec(ff).await;

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
