use std::path::Path;

use crate::header::HeaderMapBuilder;
use crate::prelude::{DownloadError, Downloader};
use reqwest::{Client, Response};
use tokio::fs::create_dir_all;
use tokio::{fs::File, io::AsyncWriteExt};
use url::Url;

#[derive(Debug, Clone)]
/// The [`ResourceDownloader`] is designed for downloading resources directly from the internet, such as files hosted on a website.
pub struct ResourceDownloader {
    url: Url,
    name: Option<String>,
    print_download_status: bool,
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

        Ok(Self {
            url,
            name: None,
            print_download_status: false,
        })
    }

    /// Sets the output filename for downloaded resources. If not set, the filename will be derived from the last part of the link path.
    ///
    /// **ONLY WORKS WITH `download` method, not `download_to`**
    pub fn with_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Retrieves the file name and replaces `\` and `/` by `|`
    fn get_file_name(&self) -> String {
        let name = match &self.name {
            Some(name) => name,
            None => self
                .url
                .path_segments()
                .and_then(|segments| segments.last())
                .unwrap_or_else(|| self.url.as_str()),
        };

        ResourceDownloader::sanitize_file_name(&name)
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

#[async_trait::async_trait]
impl Downloader for ResourceDownloader {
    async fn download_to<P: AsRef<Path> + std::marker::Send>(
        &self,
        folder_path: P,
    ) -> Result<(), DownloadError> {
        let name = self.get_file_name();
        let path = folder_path
            .as_ref()
            .join(name);

        if self.print_download_status {
            println!("Downloading...");
        }

        create_dir_all(folder_path).await?;

        let mut file = File::create(path).await?;

        let response = self.send_request().await?;
        let content = response.bytes().await?;
        file.write_all(&content).await?;

        Ok(())
    }

    async fn download(&self) -> Result<(), DownloadError> {
        self.download_to(Path::new("./")).await
    }

    fn is_valid_url(url: &Url) -> bool {
        url.has_host() && (url.scheme() == "https" || url.scheme() == "http")
    }

    fn get_dl_status(&mut self) -> &mut bool {
        &mut self.print_download_status
    }
}
