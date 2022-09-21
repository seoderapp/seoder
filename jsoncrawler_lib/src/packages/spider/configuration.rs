use super::utils::log;

/// Structure to configure `Website` crawler
/// ```rust
/// use jsoncrawler_lib::packages::spider::website::Website;
/// let mut website: Website = Website::new("https://choosealicense.com");
/// website.configuration.user_agent = "Android".to_string();
/// website.crawl();
/// ```
#[derive(Debug, Default)]
pub struct Configuration {
    /// User-Agent
    pub user_agent: String,
}

/// configure application program api path, timeout, and channel buffer
pub fn setup() -> (&'static str, std::time::Duration, usize) {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let mut query = 1;
    let mut timeout: u64 = 15;
    let mut buffer: usize = 100;

    // read through config file cpu bound quickly to avoid atomics and extra memory from clones
    match File::open("config.txt") {
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
                        // query config
                        if cf == "query" && !v.is_empty() {
                            // validate acceptable queries
                            match v {
                                "posts" => query = 1,
                                "pages" => query = 2,
                                "users" => query = 3,
                                "comments" => query = 4,
                                "search" => query = 5,
                                _ => {
                                    log("not valid config file {}", "");
                                }
                            };
                        }

                        if cf == "timeout" && !v.is_empty() {
                            timeout = v.parse::<u64>().unwrap_or(15);
                        }

                        if cf == "buffer" && !v.is_empty() {
                            buffer = v.parse::<usize>().unwrap_or(100);
                        }
                    }
                }
            }
        }
        Err(_) => {
            log("config.txt file does not exist {}", "");
        }
    };

    // reverse query dip
    let query = match query {
        1 => "/wp-json/wp/v2/posts?per_page=100",
        2 => "/wp-json/wp/v2/pages?per_page=100",
        3 => "/wp-json/wp/v2/users?per_page=100",
        4 => "/wp-json/wp/v2/comments?per_page=100",
        5 => "/wp-json/wp/v2/search?per_page=100",
        _ => "/wp-json/wp/v2/posts?per_page=100",
    };

    (query, std::time::Duration::new(timeout, 0), buffer)
}

impl Configuration {
    /// Represents crawl configuration for a website.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
