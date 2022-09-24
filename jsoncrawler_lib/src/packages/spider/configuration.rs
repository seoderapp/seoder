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

/// configure application program api path, timeout, channel buffer, and proxy
pub fn setup() -> (String, std::time::Duration, usize, bool) {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let mut query = 1;
    let mut timeout: u64 = 15;
    let mut buffer: usize = 100;
    let mut proxy = false;
    let mut page_limit = 100;

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

                        if cf == "proxy" && !v.is_empty() {
                            proxy = v.parse::<bool>().unwrap_or(false);
                        }

                        if cf == "page_limit" && !v.is_empty() {
                            page_limit = v.parse::<usize>().unwrap_or(100);
                        }
                    }
                }
            }
        }
        Err(_) => {
            log("config.txt file does not exist {}", "");
        }
    };

    let page_limit = string_concat!("?per_page=", page_limit.to_string());
    let query_base = "/wp-json/wp/v2/";

    // reverse query dip
    let query = match query {
        1 => string_concat!(query_base, "posts", page_limit),
        2 => string_concat!(query_base, "pages", page_limit),
        3 => string_concat!(query_base, "users", page_limit),
        4 => string_concat!(query_base, "comments", page_limit),
        5 => string_concat!(query_base, "search", page_limit),
        _ => string_concat!(query_base, "posts", page_limit),
    };

    (query, std::time::Duration::new(timeout, 0), buffer, proxy)
}

impl Configuration {
    /// Represents crawl configuration for a website.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
