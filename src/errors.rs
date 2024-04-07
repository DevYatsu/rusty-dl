use std::{error, fmt};

#[derive(Debug)]
pub enum DownloadError {
    Reqwest(reqwest::Error),
    IoError(std::io::Error),
    ParseError(url::ParseError),
    InvalidUrl(String),
    VideoNotFound(String),
    FailedToBuildBlockingRuntime(String),
    Downloader(String),

    #[cfg(feature = "twitter")]
    TwitterError(String),

    #[cfg(feature = "youtube")]
    Video(rusty_ytdl::VideoError),
    #[cfg(feature = "youtube")]
    YoutubeError(String),
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DownloadError::Reqwest(err) => write!(f, "{}", err),
            DownloadError::IoError(err) => write!(f, "{}", err),
            DownloadError::ParseError(err) => write!(f, "{}", err),
            DownloadError::InvalidUrl(err) => write!(f, "{}", err),
            DownloadError::VideoNotFound(err) => write!(f, "{}", err),
            DownloadError::FailedToBuildBlockingRuntime(err) => write!(f, "{}", err),
            DownloadError::Downloader(err) => write!(f, "{}", err),
            #[cfg(feature = "youtube")]
            DownloadError::Video(err) => write!(f, "{}", err),
            #[cfg(feature = "twitter")]
            DownloadError::TwitterError(err) => write!(f, "{}", err),
            #[cfg(feature = "youtube")]
            DownloadError::YoutubeError(err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for DownloadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            DownloadError::Reqwest(err) => Some(err),
            DownloadError::IoError(err) => Some(err),
            DownloadError::ParseError(err) => Some(err),
            DownloadError::InvalidUrl(_) => None,
            DownloadError::VideoNotFound(_) => None,
            DownloadError::FailedToBuildBlockingRuntime(_) => None,
            DownloadError::Downloader(_) => None,
            #[cfg(feature = "youtube")]
            DownloadError::Video(err) => Some(err),
            #[cfg(feature = "twitter")]
            DownloadError::TwitterError(_) => None,
            #[cfg(feature = "youtube")]
            DownloadError::YoutubeError(_) => None,
        }
    }
}

impl From<reqwest::Error> for DownloadError {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

#[cfg(feature = "youtube")]
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
