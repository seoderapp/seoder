use log::{info, log_enabled, Level};
use reqwest::Client;
use reqwest::StatusCode;

/// Perform a network request to a resource extracting all content as text.
pub async fn fetch_page_html(url: &str, client: &Client) -> String {
    // a) ok-valid_json.txt | URLs that loaded OK + contain valid JSON on any of the requests before max retries exhausted
    // b) ok-not_valid_json.txt | site loaded OK, but not valid JSON payload (only save if after all retries are exhausted on the last retry)
    // c) connection_error.txt | URLs that timed out, did not resolve, timeout, or other connection error
    // d) all-others.txt | all other URLs (e.g. returned some unknown status code at the end of all retries) 
    // todo: pass status code along with output
   
    match client.get(format!("http://{}/wp-json/wp/v2/users?per_page=100", url)).send().await {
        Ok(res) if res.status() == StatusCode::OK => match res.text().await {
            Ok(text) => text,
            Err(_) => {
                log("- error fetching {}", &url);

                String::new()
            }
        },
        Ok(_) => String::new(),
        Err(_) => {
            log("- error parsing json {}", &url);
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
