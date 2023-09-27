use super::ResponseOutFileType;
use log::{info, log_enabled, Level};
use reqwest::Client;
use reqwest::StatusCode;
use std::sync::Arc;
use tokio::sync::watch;
use tokio::sync::watch::Receiver;
use tokio::sync::watch::Sender;
use tokio::sync::Mutex;

// return a static string based on condition
fn static_truthy_string(v: bool) -> &'static str {
    if v {
        "true"
    } else {
        "false"
    }
}

// build the error string
fn build_error(url: &str, error: &reqwest::Error) -> String {
    string_concat!(
        "- error ",
        &url,
        " - connect_error: ",
        static_truthy_string(error.is_connect()),
        " - redirect_error ",
        static_truthy_string(error.is_redirect()),
        " - status_error: ",
        static_truthy_string(error.is_status()),
        " - timeout_error: ",
        static_truthy_string(error.is_timeout()),
        " - request_error: ",
        static_truthy_string(error.is_request())
    )
}

/// Perform a network request to a resource extracting all content as text.
pub async fn fetch_page_html(
    url: &str,
    client: &Client,
    path: &str,
) -> (String, ResponseOutFileType) {
    match client
        .get(string_concat!("http://", url, path))
        .send()
        .await
    {
        Ok(res) if res.status() == StatusCode::OK => match res.text().await {
            Ok(text) => (text, ResponseOutFileType::Valid),
            Err(error) => {
                let response = build_error(&url, &error);

                logd(&response);

                (response, ResponseOutFileType::Invalid)
            }
        },
        Ok(_) => (String::new(), ResponseOutFileType::Unknown),
        Err(error) => {
            let response = build_error(&url, &error);

            logd(&response);

            (response, ResponseOutFileType::Error)
        }
    }
}

/// log to console if configuration verbose.
pub fn log(message: &'static str, data: impl AsRef<str>) {
    if log_enabled!(Level::Info) {
        info!("{message} - {}", data.as_ref());
    }
}

/// log to console if configuration verbose direct value.
pub fn logd(data: impl AsRef<str>) {
    if log_enabled!(Level::Info) {
        info!("{}", data.as_ref());
    }
}

/// determine action
#[derive(PartialEq, Debug)]
pub enum Handler {
    /// crawl start state
    Start,
    /// crawl pause state
    Pause,
    /// crawl resume
    Resume,
    /// crawl shutdown
    Shutdown,
}

lazy_static! {
    /// control handle for crawls
    pub static ref CONTROLLER: Arc<Mutex<(Sender<(String, Handler)>, Receiver<(String, Handler)>)>> = Arc::new(Mutex::new(watch::channel(("handles".to_string(), Handler::Start))));
}

/// pause a target website running crawl
pub async fn pause(domain: &str) {
    let s = CONTROLLER.clone();

    s.lock()
        .await
        .0
        .send((domain.to_string(), Handler::Pause))
        .unwrap();
}

/// resume a target website crawl
pub async fn resume(domain: &str) {
    let s = CONTROLLER.clone();

    s.lock()
        .await
        .0
        .send((domain.to_string(), Handler::Resume))
        .unwrap();
}

/// shutdown a target website crawl
pub async fn shutdown(domain: &str) {
    let s = CONTROLLER.clone();

    s.lock()
        .await
        .0
        .send((domain.to_string(), Handler::Shutdown))
        .unwrap();
}
