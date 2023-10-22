use reqwest::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestIpinfoError {
    #[error("Request error: {0}")]
    Request(reqwest::Error),
    #[error("The response JSON is not valid")]
    JsonDecode(reqwest::Error),
    #[error("Something unexpected happened: the server responded with {0}")]
    Http(StatusCode),
}
