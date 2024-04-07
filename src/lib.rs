pub mod errors;
pub mod header;

// #[cfg(feature = "resource")]
pub mod resource;
// #[cfg(feature = "twitter")]
pub mod twitter;

// #[cfg(feature = "youtube")]
pub mod youtube;

use crate::errors::DownloadError;
use std::future::Future;
use url::Url;

/// A trait representing a downloader.
pub trait Downloader {
    /// Parses the provided URL string into a `Url` struct.
    ///
    /// ### Arguments
    ///
    /// * `link` - The URL string to parse.
    /// * `expected_url_format` - Optional expected URL format to use for parsing. Defaults to `"https://www.<domain>.<extension>/<parameters>"`.
    ///
    /// ### Returns
    ///
    /// Returns a `Result` containing the parsed `Url` on success, or a `DownloadError` if parsing fails.
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

    /// Downloads the file to the current working directory.
    ///
    /// ### Returns
    ///
    /// Returns a future representing the download operation, which resolves to a `Result` indicating success or failure.
    fn download(&self) -> impl std::future::Future<Output = Result<(), DownloadError>> + Send
    where
        Self: Sync,
    {
        self.download_to(&std::path::Path::new("./"))
    }

    /// Downloads the file to the specified folder path.
    ///
    /// ### Arguments
    ///
    /// * `path` - The path to the folder where the file will be downloaded.
    ///
    /// ### Returns
    ///
    /// Returns a future representing the download operation, which resolves to a `Result` indicating success or failure.
    fn download_to(
        &self,
        path: &std::path::Path,
    ) -> impl std::future::Future<Output = Result<(), DownloadError>> + Send
    where
        Self: Sync;

    /// Blocks the current thread until the download completes, using asynchronous execution.
    ///
    /// ### Returns
    ///
    /// Returns a `Result` indicating success or failure of the download operation.
    fn blocking_download(&self) -> Result<(), DownloadError>
    where
        Self: Sync,
    {
        Self::blocking(async { self.download().await })
    }

    fn blocking<F: Future>(async_block: F) -> Result<(), DownloadError> {
        // Create a multi-threaded Tokio runtime with the default number of worker threads
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|_| {
                DownloadError::FailedToBuildBlockingRuntime(
                    "Failed to build blocking runtime".to_owned(),
                )
            })?;

        // Block the current thread until the download completes
        rt.block_on(async_block);
        Ok(())
    }

    /// Blocks the current thread until the download completes, using asynchronous execution, and saves the file to the specified folder path.
    ///
    /// ### Arguments
    ///
    /// * `path` - The path to the folder where the file will be downloaded.
    ///
    /// ### Returns
    ///
    /// Returns a `Result` indicating success or failure of the download operation.
    fn blocking_download_to(&self, path: &std::path::Path) -> Result<(), DownloadError>
    where
        Self: Sync,
    {
        Self::blocking(async { self.download_to(path).await })
    }
}

pub mod prelude {
    pub use crate::errors::DownloadError;
    pub use crate::Downloader;

    // #[cfg(feature = "resource")]
    pub use crate::resource::ResourceDownloader;

    // #[cfg(feature = "twitter")]
    pub use crate::twitter::TwitterDownloader;

    // #[cfg(feature = "youtube")]
    pub use crate::youtube::YoutubeDownloader;
}

pub mod test {
    use std::{fs::read_dir, io::Error, path::Path};

    pub fn assert_folder_len(name: &str, len: usize) -> Result<(), Error> {
        assert_eq!(read_dir(Path::new(name))?.count(), len);

        Ok(())
    }
}
