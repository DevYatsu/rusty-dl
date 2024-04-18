use rusty_dl::{errors::DownloadError, resource::ResourceDownloader, Downloader};

const LINK: &str =
    "https://www.mozilla.org/media/protocol/img/logos/mozilla/logo-word-hor.e20791bb4dd4.svg";

fn main() -> Result<(), DownloadError> {
    // returns an error if LINK in invalid
    let mut downloader = ResourceDownloader::new(LINK)?;

    // will download the file with the given name
    downloader.with_name("mozilla_icon.svg".to_owned());

    // blocks the main thread to download the given resource
    downloader.blocking_download()?;

    Ok(())
}

#[allow(dead_code)]
async fn async_example() -> Result<(), DownloadError> {
    // returns an error if link in invalid
    let downloader = ResourceDownloader::new(LINK)?;

    // downloads the medias in current working dir
    downloader.download().await?;

    Ok(())
}

#[allow(dead_code)]
async fn dl_to<P>(path: P) -> Result<(), DownloadError>
where
    P: AsRef<std::path::Path> + std::marker::Send,
{
    // returns an error if link in invalid
    let downloader = ResourceDownloader::new(LINK)?;

    // downloads the medias in current working dir
    // a blocking version of this method also exists
    downloader.download_to(path).await?;

    Ok(())
}
