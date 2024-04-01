use reqwest::Url;
use rusty_ytdl::blocking;
use rusty_ytdl::{VideoOptions, VideoQuality, VideoSearchOptions};

use crate::prelude::{DownloadError, Downloader};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct YoutubeDownloader {
    url: Url,
}

impl YoutubeDownloader {
    pub fn new(link: &str) -> Result<Self, DownloadError> {
        let url = Url::parse(link).map_err(|_| {
            DownloadError::InvalidUrl(
                "Invalid Url! Expected Url with format: `https://www.youtube.com/watch?v=VIDEO_ID`"
                    .to_owned(),
            )
        })?;

        Ok(Self { url })
    }
}

impl Downloader for YoutubeDownloader {
    async fn download(&self) -> Result<(), DownloadError> {
        let video_options = VideoOptions {
            quality: VideoQuality::Highest,
            filter: VideoSearchOptions::VideoAudio,
            ..Default::default()
        };

        let video = rusty_ytdl::Video::new_with_options(self.url.clone(), video_options)
            .map_err(|_| DownloadError::VideoNotFound("Video Not Found".to_owned()))?;
        let video_info = video.get_basic_info().await?;

        let title = video_info.video_details.title.replace(" ", "_");

        video.download(format!("{title}.mp4")).await?;

        Ok(())
    }

    fn blocking_download(&self) -> Result<(), DownloadError>
    where
        Self: Sync,
    {
        let video_options = VideoOptions {
            quality: VideoQuality::Highest,
            filter: VideoSearchOptions::VideoAudio,
            ..Default::default()
        };

        let video = blocking::Video::new_with_options(self.url.clone(), video_options)
            .map_err(|_| DownloadError::VideoNotFound("Video Not Found".to_owned()))?;
        let video_info = video.get_basic_info()?;

        let title = video_info.video_details.title.replace(" ", "_");

        video.download(format!("{title}.mp4"))?;

        Ok(())
    }
}
