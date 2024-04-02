pub mod errors;
pub mod twitter;
pub mod youtube;

use crate::errors::DownloadError;
use tokio::runtime::Builder;
use url::Url;

pub trait Downloader {
    fn parse_url(link: &str, expected_url_format: Option<&str>) -> Result<Url, DownloadError> {
        let expected_format =
            expected_url_format.unwrap_or("https://www.<domain>.<extension>/<parameters>");
        let url = Url::parse(link).map_err(|_| {
            DownloadError::InvalidUrl(
                format!("Invalid URL format! Please provide a URL in the format: {expected_format}. Ensure the URL includes a valid domain, extension, and any necessary parameters.")
                    ,
            )
        })?;

        Ok(url)
    }

    /// A function to download the file inside of the folder in which the program is called.
    fn download(&self) -> impl std::future::Future<Output = Result<(), DownloadError>> + Send
    where
        Self: Sync,
    {
        self.download_to(&std::path::Path::new("./"))
    }

    /// A function to download the file to a given folder, where path is a folder.
    fn download_to(
        &self,
        path: &std::path::Path,
    ) -> impl std::future::Future<Output = Result<(), DownloadError>> + Send
    where
        Self: Sync;

    fn blocking_download(&self) -> Result<(), DownloadError>
    where
        Self: Sync,
    {
        // Create a multi-threaded Tokio runtime with the default number of worker threads
        let rt = Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|_| {
                DownloadError::FailedToBuildBlockingRuntime(
                    "Failed to build blocking runtime".to_owned(),
                )
            })?;

        // Block the current thread until the download completes
        rt.block_on(async { self.download().await })
    }

    fn blocking_download_to(&self, path: &std::path::Path) -> Result<(), DownloadError>
    where
        Self: Sync,
    {
        // Create a multi-threaded Tokio runtime with the default number of worker threads
        let rt = Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|_| {
                DownloadError::FailedToBuildBlockingRuntime(
                    "Failed to build blocking runtime".to_owned(),
                )
            })?;

        // Block the current thread until the download completes
        rt.block_on(async { self.download_to(path).await })
    }
}

pub mod prelude {
    pub use crate::errors::DownloadError;
    pub use crate::twitter::TwitterDownloader;
    pub use crate::youtube::YoutubeDownloader;
    pub use crate::Downloader;
}
