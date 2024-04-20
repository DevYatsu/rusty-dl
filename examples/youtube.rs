use rusty_dl::{
    errors::DownloadError,
    youtube::{VideoInfo, YoutubeDownloader},
    Downloader,
};

// can be the LINK to a video or a playlist: the downloader detects it and take care of it
// playlist downloading is limited to 100 videos for the moment
const LINK: &str = "https://youtube.com/video_id";

fn main() -> Result<(), DownloadError> {
    // returns an error if LINK in invalid
    let mut downloader = YoutubeDownloader::new(LINK)?;

    // downloads the audio and the video
    // *DEFAULT BEHAVIOUR*
    downloader.video_and_audio();

    // downloads only the audio of the video
    downloader.only_audio();

    // downloads only the video and not the audio
    downloader.only_video();

    // keeps track of the download status in the console
    downloader.print_dl_status();

    // renames the video with underscores locally
    downloader.rename_with_underscores();

    // sets a custom name to the downloaded video
    // initially the local name will be the name attached to the video in the youtube api
    downloader.set_name("My fav video".to_owned());

    // blocks the main thread to download the video(s)
    downloader.blocking_download()?;

    Ok(())
}

#[allow(dead_code)]
async fn video_data(downloader: &YoutubeDownloader) -> Result<VideoInfo, DownloadError> {
    let video = downloader.get_video()?;
    let infos = video.get_basic_info().await?;

    Ok(infos)
}

#[allow(dead_code)]
async fn async_example() -> Result<(), DownloadError> {
    // returns an error if LINK in invalid
    let downloader = YoutubeDownloader::new(LINK)?;

    // downloads the video(s) in current working dir
    downloader.download().await?;

    Ok(())
}

#[allow(dead_code)]
async fn dl_to<P>(path: P) -> Result<(), DownloadError>
where
    P: AsRef<std::path::Path> + std::marker::Send,
{
    // returns an error if LINK in invalid
    let downloader = YoutubeDownloader::new(LINK)?;

    // downloads the medias in current working dir
    // a blocking version of this method also exists
    downloader.download_to(path).await?;

    Ok(())
}

#[allow(dead_code)]
async fn dl_playlist_to<P>(path: P) -> Result<(), DownloadError>
where
    P: AsRef<std::path::Path> + std::marker::Send,
{
    // we assume LINK is the LINK to a playlist
    let mut downloader = YoutubeDownloader::new(LINK)?;

    downloader.set_playlist_video_filter(|video| {
        video.is_playable && video.title.accessibility.accessibility_data.label == "My fav video"
    });

    // downloads the medias in current working dir
    // a blocking version of this method also exists
    downloader.download_to(path).await?;

    Ok(())
}
