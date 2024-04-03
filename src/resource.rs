use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client, Response,
};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{prelude::DownloadError, Downloader};

#[derive(Debug, Clone)]
/// The `ResourceDownloader` is designed for downloading resources directly from the internet, such as files hosted on a website.
pub struct ResourceDownloader {
    url: url::Url,
    client: Client,
    headers: HeaderMap,
}

impl ResourceDownloader {
    pub fn new(url: &str) -> Result<Self, DownloadError> {
        let url = Self::parse_url(url, None)?;

        let client = Self::init_client();
        let headers = Self::init_headers();

        Ok(Self {
            url,
            client,
            headers,
        })
    }

    pub fn with_header(mut self, key: &'static str, value: &'static str) -> Self {
        self.headers.insert(key, HeaderValue::from_static(value));
        self
    }
    pub fn with_header_value(mut self, key: &'static str, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn with_custom_client(mut self, client: Client) -> Self {
        self.client = client;
        self
    }

    fn init_client() -> Client {
        Client::new()
    }

    fn init_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:84.0) Gecko/20100101 Firefox/84.0",
            ),
        );
        headers
    }

    async fn send_request(&self) -> Result<Response, DownloadError> {
        let response = self
            .client
            .get(self.url.clone())
            .headers(self.headers.clone())
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
    async fn download_to(&self, path: &std::path::Path) -> Result<(), DownloadError> {
        if path.is_dir() {
            return Err(DownloadError::Downloader(format!(
                "Path must point to a file. That is not the case for `{}`",
                path.display()
            )));
        }

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = File::create(path).await?;

        let response = self.send_request().await?;
        let content = response.bytes().await?;
        file.write_all(&content).await?;

        Ok(())
    }
}
