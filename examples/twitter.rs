use rusty_dl::{errors::DownloadError, twitter::TwitterDownloader, Downloader};

const LINK: &str = "https://twitter.com/user_name/tweet_id";

fn main() -> Result<(), DownloadError> {
    // returns an error if LINK in invalid
    let mut downloader = TwitterDownloader::new(LINK)?;

    // to download only images
    downloader.only_images();

    // to download only videos
    downloader.only_videos();

    // set a callback to name the generated media files
    // here the files will have as name the index of the file
    downloader.set_name_callback(|index, _media| index.to_string());

    // blocks the main thread to download the medias in a tweet
    downloader.blocking_download()?;

    Ok(())
}

#[allow(dead_code)]
async fn async_example() -> Result<(), DownloadError> {
    // returns an error if link in invalid
    let downloader = TwitterDownloader::new(LINK)?;

    // downloads the medias in current working dir
    downloader.download().await?;

    Ok(())
}

#[allow(dead_code)]
async fn download_to_example() -> Result<(), DownloadError> {
    // returns an error if link in invalid
    let downloader = TwitterDownloader::new(LINK)?;

    // downloads the medias to the given path
    // a blocking version of this method also exists
    downloader.download_to("./tweet_medias/").await?;

    Ok(())
}

#[allow(dead_code)]
async fn tweet_folder_example() -> Result<(), DownloadError> {
    // returns an error if link in invalid
    let downloader = TwitterDownloader::new(LINK)?;

    // downloads the medias as a folder to the given path
    // Here a folder named "tweet_medias" will be created in current working dir and the medias will be downloaded into it
    // a blocking version of this method also exists
    downloader
        .download_as_tweets_folder_to("./tweet_medias/")
        .await?;

    Ok(())
}
