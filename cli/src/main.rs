use std::path::{Path, PathBuf};

use clap::{arg, command, value_parser, ArgAction, Command, Error};
use rusty_dl::prelude::*;
use url::Url;

fn main() -> Result<(), DownloadError> {
    let matches = command!()
        .arg(
            arg!([link] "The link to download the resource from")
                .required(true)
                .value_parser(is_valid_download_url),
        )
        .arg(
            arg!([path] "The local path to download the resource to, may be relative or absolute")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let link = matches.get_one::<Url>("link");
    let path = matches.get_one::<PathBuf>("path");

    let url = link.unwrap(/* safe as we set it as required beforehand */);

    let rt = tokio::runtime::Runtime::new().expect("tokio runtime can be initialized");

    let downloader: DownloaderWrapper = match url {
        link if TwitterDownloader::is_valid_url(url) => {
            TwitterDownloader::new(link.as_str()).unwrap().into()
        }
        link if YoutubeDownloader::is_valid_url(url) => {
            YoutubeDownloader::new(link.as_str()).unwrap().into()
        }
        link => ResourceDownloader::new(link.as_str()).unwrap().into(),
    };

    match path {
        Some(path) => rt.block_on(downloader.download_to(path))?,
        None => rt.block_on(downloader.download())?,
    };

    Ok(())
}

fn is_valid_download_url(input: &str) -> Result<Url, DownloadError> {
    let url = Url::parse(input)?;

    Ok(url)
}

#[derive(Clone)]
pub enum DownloaderWrapper {
    Rsrc(ResourceDownloader),
    Yt(YoutubeDownloader),
    Twi(TwitterDownloader),
}

impl DownloaderWrapper {
    pub async fn download(&self) -> Result<(), DownloadError> {
        match self {
            DownloaderWrapper::Rsrc(d) => d.download().await,
            DownloaderWrapper::Yt(d) => d.download().await,
            DownloaderWrapper::Twi(d) => d.download().await,
        }
    }

    pub async fn download_to<P: AsRef<Path> + Send>(&self, path: P) -> Result<(), DownloadError> {
        match self {
            DownloaderWrapper::Rsrc(d) => d.download_to(path).await,
            DownloaderWrapper::Yt(d) => d.download_to(path).await,
            DownloaderWrapper::Twi(d) => d.download_to(path).await,
        }
    }
}

impl From<TwitterDownloader> for DownloaderWrapper {
    fn from(value: TwitterDownloader) -> Self {
        DownloaderWrapper::Twi(value)
    }
}
impl From<YoutubeDownloader> for DownloaderWrapper {
    fn from(value: YoutubeDownloader) -> Self {
        DownloaderWrapper::Yt(value)
    }
}
impl From<ResourceDownloader> for DownloaderWrapper {
    fn from(value: ResourceDownloader) -> Self {
        DownloaderWrapper::Rsrc(value)
    }
}
