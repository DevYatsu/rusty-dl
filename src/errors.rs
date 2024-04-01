use std::{error, fmt};

#[derive(Debug)]
pub enum DownloadError {
    Reqwest(reqwest::Error),
    Video(rusty_ytdl::VideoError),
    IoError(std::io::Error),
    ParseError(url::ParseError),
    InvalidUrl(String),
    VideoNotFound(String),
    FailedToBuildBlockingRuntime(String),
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reqwest(err) => write!(f, "{}", err),
            Self::Video(err) => write!(f, "{}", err),
            Self::IoError(err) => write!(f, "{}", err),
            Self::ParseError(err) => write!(f, "{}", err),
            Self::InvalidUrl(err) => write!(f, "{}", err),
            Self::VideoNotFound(err) => write!(f, "{}", err),
            Self::FailedToBuildBlockingRuntime(err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for DownloadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Reqwest(err) => Some(err),
            Self::Video(err) => Some(err),
            Self::IoError(err) => Some(err),
            Self::ParseError(err) => Some(err),
            Self::InvalidUrl(_) => None,
            Self::VideoNotFound(_) => None,
            Self::FailedToBuildBlockingRuntime(_) => None,
        }
    }
}

impl From<reqwest::Error> for DownloadError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl From<rusty_ytdl::VideoError> for DownloadError {
    fn from(value: rusty_ytdl::VideoError) -> Self {
        Self::Video(value)
    }
}
impl From<std::io::Error> for DownloadError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}
impl From<url::ParseError> for DownloadError {
    fn from(value: url::ParseError) -> Self {
        Self::ParseError(value)
    }
}
