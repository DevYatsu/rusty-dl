use std::path::Path;

use crate::header::HeaderMapBuilder;
use crate::prelude::{DownloadError, Downloader};
use reqwest::{Client, Response};
use tokio::{fs::File, io::AsyncWriteExt};
use url::Url;

#[derive(Debug, Clone)]
/// The [`ResourceDownloader`] is designed for downloading resources directly from the internet, such as files hosted on a website.
pub struct ResourceDownloader {
    url: Url,
}

impl ResourceDownloader {
    /// Creates a new instance of [`ResourceDownloader`] with the provided URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the resource to download.
    ///
    /// # Returns
    ///
    /// Returns a [`Result`] containing the [`ResourceDownloader`] instance on success, or a [`DownloadError`] if parsing the URL fails or if the URL is invalid.
    pub fn new(url: &str) -> Result<Self, DownloadError> {
        let url = Self::parse_url(url, None)?;

        if !Self::is_valid_url(&url) {
            return Err(DownloadError::InvalidUrl(
                "Invalid URL! The URL must start with 'http://' or 'https://' and include a host."
                    .to_owned(),
            ));
        }

        Ok(Self { url })
    }

    /// Sends a GET request to the URL of the resource and returns the response.
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
    async fn download_to<P: AsRef<Path> + std::marker::Send>(
        &self,
        file_path: P,
    ) -> Result<(), DownloadError> {
        let path = file_path.as_ref();
        if path.is_dir() {
            return Err(DownloadError::Downloader(format!(
                "Path must point to a file. That is not the case for `{}`",
                path.display()
            )));
        }

        tokio::fs::create_dir_all(path).await?;
        let mut file = File::create(path).await?;

        let response = self.send_request().await?;
        let content = response.bytes().await?;
        file.write_all(&content).await?;

        Ok(())
    }

    async fn download(&self) -> Result<(), DownloadError> {
        let name = self
            .url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or_else(|| self.url.as_str());
        self.download_to(Path::new("./").join(name)).await
    }

    fn is_valid_url(url: &Url) -> bool {
        url.has_host() && (url.scheme() == "https" || url.scheme() == "http")
    }
}
