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

    TwitterError(String),
    YoutubeError(String),

    Downloader(String),
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DownloadError::Reqwest(err) => write!(f, "{}", err),
            DownloadError::Video(err) => write!(f, "{}", err),
            DownloadError::IoError(err) => write!(f, "{}", err),
            DownloadError::ParseError(err) => write!(f, "{}", err),
            DownloadError::InvalidUrl(err) => write!(f, "{}", err),
            DownloadError::VideoNotFound(err) => write!(f, "{}", err),
            DownloadError::FailedToBuildBlockingRuntime(err) => write!(f, "{}", err),
            DownloadError::TwitterError(err) => write!(f, "{}", err),
            DownloadError::YoutubeError(err) => write!(f, "{}", err),
            DownloadError::Downloader(err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for DownloadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            DownloadError::Reqwest(err) => Some(err),
            DownloadError::Video(err) => Some(err),
            DownloadError::IoError(err) => Some(err),
            DownloadError::ParseError(err) => Some(err),
            DownloadError::InvalidUrl(_) => None,
            DownloadError::VideoNotFound(_) => None,
            DownloadError::FailedToBuildBlockingRuntime(_) => None,
            DownloadError::TwitterError(_) => None,
            DownloadError::YoutubeError(_) => None,
            DownloadError::Downloader(_) => None,
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
