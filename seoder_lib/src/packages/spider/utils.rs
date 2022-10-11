use super::ResponseOutFileType;
use log::{info, log_enabled, Level};
use reqwest::Client;
use reqwest::StatusCode;

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
