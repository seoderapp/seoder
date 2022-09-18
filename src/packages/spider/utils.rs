use log::{info, log_enabled, Level};
use reqwest::Client;
use reqwest::StatusCode;

/// Perform a network request to a resource extracting all content as text.
pub async fn fetch_page_html(url: &str, client: &Client) -> String {
    // todo: pass status code
    match client.get(format!("http://{}/wp-json/wp/v2/users?per_page=100", url)).send().await {
        Ok(res) if res.status() == StatusCode::OK => match res.text().await {
            Ok(text) => text,
            Err(_) => {
                // failed to fetch data from page
                log("- error fetching {}", &url);

                String::new()
            }
        },
        Ok(_) => {
            // connection status not ok
            String::new()
        },
        Err(_) => {
            // connection error website offline
            log("- error parsing text {}", &url);
            String::new()
        }
    }
}

/// log to console if configuration verbose.
pub fn log(message: &'static str, data: impl AsRef<str>) {
    if log_enabled!(Level::Info) {
        info!("{message} - {}", data.as_ref());
    }
}
