use std::io;

#[derive(Debug)]
pub struct Error {
    pub error_message: String,
    pub code: ErrorCode,
}
#[derive(Debug)]
pub enum ErrorCode {
    RetryableError,
}

pub type Result<T> = std::result::Result<T, DownloadError>;
#[derive(Debug)]
pub enum DownloadError {
    SystemError(Error),
    ParameterError(Error),
    ConnectionError(Error),
    RequestError(reqwest::Error),
    IOError(io::Error),
    JsonParseError(serde_json::error::Error),
}

impl DownloadError {
    pub fn system(message: &str) -> DownloadError {
        DownloadError::SystemError(Error {
            error_message: message.to_string(),
            code: ErrorCode::RetryableError,
        })
    }
    pub fn parameter(message: &str) -> DownloadError {
        DownloadError::ParameterError(Error {
            error_message: message.to_string(),
            code: ErrorCode::RetryableError,
        })
    }
}

impl From<reqwest::Error> for DownloadError {
    fn from(value: reqwest::Error) -> Self {
        DownloadError::RequestError(value)
    }
}
impl From<io::Error> for DownloadError {
    fn from(value: io::Error) -> Self {
        DownloadError::IOError(value)
    }
}

impl From<serde_json::error::Error> for DownloadError {
    fn from(value: serde_json::error::Error) -> Self {
        DownloadError::JsonParseError(value)
    }
}
