pub mod errors;
pub mod youtube;

use crate::errors::DownloadError;
use tokio::runtime::Builder;

pub trait Downloader {
    fn download(&self) -> impl std::future::Future<Output = Result<(), DownloadError>> + Send
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
}

pub mod prelude {
    pub use crate::errors::DownloadError;
    pub use crate::youtube::YoutubeDownloader;
    pub use crate::Downloader;
}