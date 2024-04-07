use std::path::Path;

use crate::header::HeaderMapBuilder;
use crate::prelude::{DownloadError, Downloader};
use reqwest::{Client, Response};
use tokio::{fs::File, io::AsyncWriteExt};
use url::Url;

#[derive(Debug, Clone)]
/// The `ResourceDownloader` is designed for downloading resources directly from the internet, such as files hosted on a website.
pub struct ResourceDownloader {
    url: Url,
}

impl ResourceDownloader {
    pub fn new(url: &str) -> Result<Self, DownloadError> {
        let url = Self::parse_url(url, None)?;

        Ok(Self { url })
    }

    async fn send_request(&self) -> Result<Response, DownloadError> {
        let client = Client::new();

        let headers_builder = HeaderMapBuilder::new().with_user_agent();

        let response = client
            .get(self.url.clone())
            .headers(headers_builder.build())
            .send()
            .await?;
        Ok(response)
    }
}

impl Downloader for ResourceDownloader {
    /// Downloads the file to the specified file path.
    ///
    /// ### Arguments
    ///
    /// * `path` - The file path to download the file to.
    ///
    /// ### Returns
    ///
    /// A future representing the download operation, which resolves to a `Result` indicating success or failure.
    ///
    /// ### Errors
    ///
    /// Returns a `DownloadError` if there are any issues during the download process.
    async fn download_to(&self, path: &Path) -> Result<(), DownloadError> {
        if path.is_dir() {
            return Err(DownloadError::Downloader(format!(
                "Path must point to a file. That is not the case for `{}`",
                path.display()
            )));
        }

        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let mut file = File::create(path).await?;

        let response = self.send_request().await?;
        let content = response.bytes().await?;
        file.write_all(&content).await?;

        Ok(())
    }
}
