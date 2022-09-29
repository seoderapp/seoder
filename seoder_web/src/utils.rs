use crate::tokio::io::BufReader;
use seoder_lib::packages::spider::utils::log;
use seoder_lib::tokio;
use seoder_lib::tokio::io::AsyncBufReadExt;
use tokio::fs::File;
use tokio::fs::OpenOptions;

/// read a file line by line to a vector
pub async fn lines_to_vec(pt: String) -> Vec<String> {
    let mut builder: Vec<String> = Vec::new();
    match File::open(&pt).await {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut lines = reader.lines();

            while let Some(line) = lines.next_line().await.unwrap() {
                builder.push(line);
            }
        }
        Err(_) => {
            log("{} file does not exist {}", &pt);
        }
    };
    builder
}

/// write to config file
pub async fn write_config(config: &str, input: &String) {
    use seoder_lib::tokio::io::AsyncWriteExt;
    let file = OpenOptions::new().read(true).open("config.txt").await;

    let mut sl: Vec<String> = vec![];

    match file {
        Ok(ff) => {
            let reader = BufReader::new(ff);
            let mut lines = reader.lines();

            while let Some(line) = lines.next_line().await.unwrap() {
                let hh = line.split(" ").collect::<Vec<&str>>();

                let mut slots: [String; 2] = ["".to_string(), "".to_string()];

                if hh.len() >= 2 {
                    slots[0] = hh[0].to_string();
                    if hh[0] == config {
                        slots[1] = input.to_string();
                    } else {
                        slots[1] = hh[1].to_string();
                    }
                    sl.push(slots.join(" "));
                }
            }
        }
        _ => {}
    };

    let mut filec = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("config.txt")
        .await
        .unwrap();

    filec.write_all(&sl.join("\n").as_bytes()).await.unwrap();
    filec.flush().await.unwrap();
}
