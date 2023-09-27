use crate::ENTRY_PROGRAM;

use super::{utils::log, website::Engine};

/// Structure to configure `Website` crawler
/// ```rust
/// use seoder_lib::packages::spider::website::Website;
/// let mut website: Website = Website::new("https://choosealicense.com");
/// website.configuration.user_agent = "Android".to_string();
/// website.crawl();
/// ```
#[derive(Debug, Default)]
pub struct Configuration {
    /// User-Agent
    pub user_agent: String,
}

/// configure application program api path, timeout, channel buffer, (proxy, tor), and engine
pub fn setup(eg: bool) -> (String, std::time::Duration, bool, (bool, bool), Engine) {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let eg_enabled = std::env::var("ENGINE_FD").is_ok() || eg;

    let mut timeout: u64 = 20;
    let mut buffer = false;
    let mut proxy = false;
    let mut tor = false;

    // read through config file cpu bound quickly
    match File::open(&ENTRY_PROGRAM.2) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let lines = reader.lines();

            for line in lines {
                let line = line.unwrap_or_default();
                if !line.is_empty() {
                    let hh = line.split(" ").collect::<Vec<&str>>();

                    if hh.len() == 2 {
                        let cf = hh[0];
                        let v = hh[1];

                        if cf == "timeout" && !v.is_empty() {
                            timeout = v.parse::<u64>().unwrap_or(20);
                        }
                        if cf == "buffer" && !v.is_empty() {
                            buffer = v.parse::<bool>().unwrap_or(false);
                        }
                        if cf == "proxy" && !v.is_empty() {
                            proxy = v.parse::<bool>().unwrap_or(false);
                        }
                        if cf == "tor" && !v.is_empty() {
                            tor = v.parse::<bool>().unwrap_or(false);
                        }
                    }
                }
            }
        }
        Err(_) => {
            log("config.txt file does not exist", "");
        }
    };

    let mut engine: Engine = Engine::default();

    // get paths * patterns
    if eg_enabled {
        let f = string_concat!(ENTRY_PROGRAM.0, "paths.txt");

        // build file paths
        match File::open(f) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let lines = reader.lines();

                for line in lines {
                    let line = line.unwrap_or_default();
                    if !line.is_empty() {
                        engine.campaign.paths.push(line);
                    }
                }
            }
            Err(_) => {
                log("_db/engines paths.txt file does not exist", "");
            }
        };

        let f = string_concat!(ENTRY_PROGRAM.0, "patterns.txt");

        match File::open(f) {
            Ok(file) => {
                let reader = BufReader::new(file);
                let lines = reader.lines();

                for line in lines {
                    let line = line.unwrap_or_default();
                    if !line.is_empty() {
                        engine.campaign.patterns.push(line);
                    }
                }
            }
            Err(_) => {
                log("_db/engines patterns.txt file does not exist", "");
            }
        };
    }

    (
        "".to_string(),
        std::time::Duration::new(timeout, 0),
        buffer,
        (proxy, tor),
        engine,
    )
}

impl Configuration {
    /// Represents crawl configuration for a website.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
