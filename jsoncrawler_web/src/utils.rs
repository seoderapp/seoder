use crate::tokio::io::BufReader;
use jsoncrawler_lib::packages::spider::utils::log;
use jsoncrawler_lib::tokio;
use jsoncrawler_lib::tokio::io::AsyncBufReadExt;
use tokio::fs::File;

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
