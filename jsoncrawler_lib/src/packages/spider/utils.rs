use super::website::CONFIG;
use super::JsonOutFileType;
use log::{info, log_enabled, Level};
use reqwest::Client;
use reqwest::StatusCode;

/// Perform a network request to a resource extracting all content as text.
pub async fn fetch_page_html(url: &str, client: &Client) -> (String, JsonOutFileType) {
    match client
        .get(string_concat!("http://", url, CONFIG.0))
        .send()
        .await
    {
        Ok(res) if res.status() == StatusCode::OK => match res.text().await {
            Ok(text) => (text, JsonOutFileType::Valid),
            Err(error) => {
                log(
                    "- error fetching {}",
                    &format!(
                        "{} - connect_error: {} - status_error: {}",
                        &url,
                        error.is_connect(),
                        error.is_status()
                    ),
                );

                (String::new(), JsonOutFileType::Invalid)
            }
        },
        Ok(_) => (String::new(), JsonOutFileType::Unknown),
        Err(error) => {
            log(
                "- error parsing {}",
                &format!(
                    "{} - timeout_error: {} - request_error: {}",
                    &url,
                    error.is_timeout(),
                    error.is_request()
                ),
            );
            (String::new(), JsonOutFileType::Error)
        }
    }
}

/// log to console if configuration verbose.
pub fn log(message: &'static str, data: impl AsRef<str>) {
    if log_enabled!(Level::Info) {
        info!("{message} - {}", data.as_ref());
    }
}
