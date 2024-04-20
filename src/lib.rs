//! # rusty_dl
//!
//! Rusty_dl Library is a versatile crate designed for effortlessly fetching a wide range of
//! content types, including YouTube videos, tweet media (such as videos, images, and gifs),
//! and eventually various other media directly from the web.
//!
//! ## Examples
//!
//! ### In a synchronous environment
//!
//! Two blocking methods are provided to easily download data in a single thread:
//! - blocking_download
//! - blocking_download_to
//!
//! ```rust
//! use rusty_dl::prelude::{Downloader, YoutubeDownloader, DownloadError};
//! const URL: &str = "my_yt_video_link";
//!
//! fn main() -> Result<(), DownloadError>  {
//!     let downloader = YoutubeDownloader::new(URL);
//!     downloader.blocking_download()
//! }
//! ```
//!
//! ### In a tokio environment
//!
//! Two methods are provided to easily download data asynchronously:
//! - download
//! - download_to
//!
//! ```rust
//! use rusty_dl::prelude::{Downloader, YoutubeDownloader, DownloadError};
//! const URL: &str = "my_yt_video_link";
//!
//! #[tokio::main]
//! async fn main() -> Result<(), DownloadError> {
//!     let downloader = YoutubeDownloader::new(URL);
//!
//!     downloader.download().await
//! }
//! ```
//!
//! ## More
//!
//! These methods can be used with any type that implements the `Downloader` trait.
//! May that be `TwitterDownloader`, `YoutubeDownloader` or `ResourceDownloader`.
//!
//! See more in [github's examples directory](https://github.com/DevYatsu/rusty-dl/examples/)
pub mod errors;
pub mod header;

// #[cfg(feature = "resource")]
pub mod resource;
// #[cfg(feature = "twitter")]
pub mod twitter;

// #[cfg(feature = "youtube")]
pub mod youtube;

use crate::errors::DownloadError;
use std::{future::Future, path::Path};
use url::Url;

/// A trait representing a downloader.
#[async_trait::async_trait]
pub trait Downloader {
    /// Parses the provided URL string into a [`Url`] struct.
    ///
    /// ## Arguments
    ///
    /// * `link` - The URL string to parse.
    /// * `expected_url_format` - Optional expected URL format to use for parsing. It's only used as advice displayed in error messages.
    /// Defaults to `"https://www.<domain>.<extension>/<parameters>"`.
    ///
    /// ## Returns
    ///
    /// Returns a [`Result`] containing the parsed [`Url`] on success, or a [`DownloadError`] if parsing fails.
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

    /// Checks if the given URL is a valid Download URL.
    fn is_valid_url(url: &Url) -> bool;

    /// Prints the download status.
    ///
    /// This function sets the field print_download_status to `true` and thus allows displaying the current progress of the download to the console.
    fn print_dl_status(&mut self) -> &mut Self {
        let status = self.get_dl_status();
        *status = true;

        self
    }

    /// Gets a mutable reference to the download status.
    ///
    /// This function returns a mutable reference to the download status, allowing it to be modified.
    fn get_dl_status(&mut self) -> &mut bool;

    /// Sanitizes the file name
    fn sanitize_file_name(s: &str) -> String {
        s.replace("\\", "|").replace("/", "|")
    }

    /// Downloads and saves the file to the current working directory.
    ///
    /// ## Returns
    ///
    /// Returns a future representing the download operation, which resolves to a [`Result`] indicating success or failure.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rusty_dl::prelude::{DownloadError, Downloader, ResourceDownloader}; // ResourceDownloader is used but it could be YoutubeDownloader or any other struct implementing Downloader trait
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DownloadError> {
    ///     let downloader = ResourceDownloader::new("https://github.com/manifest.json").unwrap();
    ///     let result = downloader.download().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn download(&self) -> Result<(), DownloadError> {
        self.download_to("./").await
    }

    /// Downloads and saves the file at the specified path.
    ///
    /// ## Arguments
    ///
    /// * `path` - The path to the file where the resource will be downloaded.
    ///
    /// ## Returns
    ///
    /// Returns a future representing the download operation, which resolves to a [`Result`] indicating success or failure.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rusty_dl::prelude::{DownloadError, Downloader, ResourceDownloader}; // ResourceDownloader is used but it could be YoutubeDownloader or any other struct implementing Downloader trait
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DownloadError> {
    ///     let downloader = ResourceDownloader::new("https://www.youtube.com/manifest.webmanifest").unwrap();
    ///     let result = downloader.download_to("./downloads/manifest.webmanifest").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn download_to<P: AsRef<Path> + std::marker::Send>(
        &self,
        path: P,
    ) -> Result<(), DownloadError>;

    /// Blocks the current thread until the download completes, using asynchronous execution.
    ///
    /// The file is saved to the default location (`./`).
    ///
    /// ## Returns
    ///
    /// Returns a [`Result`] indicating success or failure of the download operation.
    ///
    /// ## Examples
    ///
    /// ```
    /// use rusty_dl::prelude::{DownloadError, Downloader, ResourceDownloader}; // ResourceDownloader is used but it could be YoutubeDownloader or any other struct implementing Downloader trait
    ///
    /// fn main() -> Result<(), DownloadError> {
    ///     let downloader = ResourceDownloader::new("https://crates.io/manifest.webmanifest").unwrap();
    ///     downloader.blocking_download()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn blocking_download(&self) -> Result<(), DownloadError>
    where
        Self: Sync,
    {
        Self::blocking(async { self.download().await })
    }

    /// Blocks the current thread until the download completes, using asynchronous execution, and saves the file at the specified path.
    ///
    /// ## Arguments
    ///
    /// * `path` - The path to the file where the resource will be downloaded.
    ///
    /// ## Returns
    ///
    /// Returns a `Result` indicating success or failure of the download operation.
    ///
    /// ## Example
    ///
    /// ```
    /// use rusty_dl::prelude::{DownloadError, Downloader, ResourceDownloader}; // ResourceDownloader is used but it could be YoutubeDownloader or any other struct implementing Downloader trait
    ///
    /// fn main() -> Result<(), DownloadError> {
    ///     let downloader = ResourceDownloader::new("https://twitter.com/manifest.json").unwrap();
    ///     downloader.blocking_download_to("./downloads/manifest.json")?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn blocking_download_to<P: AsRef<Path> + std::marker::Send>(
        &self,
        path: P,
    ) -> Result<(), DownloadError>
    where
        Self: Sync,
    {
        Self::blocking(async { self.download_to(path).await })
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
