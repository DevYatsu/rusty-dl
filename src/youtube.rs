use reqwest::Url;
use rusty_ytdl::{blocking, FFmpegArgs};
use rusty_ytdl::{VideoOptions, VideoQuality, VideoSearchOptions};

use crate::prelude::{DownloadError, Downloader};

#[derive(Debug, PartialEq, Clone)]
pub struct YoutubeDownloader {
    url: Url,
    filter: VideoSearchOptions,
    to_mp3: bool,
}

impl YoutubeDownloader {
    pub fn new(link: &str) -> Result<Self, DownloadError> {
        let url = Self::parse_url(link, Some("https://www.youtube.com/v=<VIDEO_ID>"))?;

        if url.domain() != Some("www.youtube.com") && url.domain() != Some("youtube.com") {
            return Err(DownloadError::InvalidUrl(
                "Invalid URL! The domain must be 'youtube.com'.".to_owned(),
            ));
        }

        Ok(Self {
            url,
            filter: VideoSearchOptions::VideoAudio,
            to_mp3: false,
        })
    }
    pub fn only_audio(&mut self) -> &mut Self {
        self.filter = VideoSearchOptions::Audio;

        self
    }
    pub fn video_and_audio(&mut self) -> &mut Self {
        self.filter = VideoSearchOptions::VideoAudio;

        self
    }
    pub fn only_video(&mut self) -> &mut Self {
        self.filter = VideoSearchOptions::Video;

        self
    }

    pub fn to_mp3(&mut self) -> &mut Self {
        self.to_mp3 = true;

        self
    }
}

impl Downloader for YoutubeDownloader {
    async fn download(&self) -> Result<(), DownloadError> {
        let filter = if self.to_mp3 {
            VideoSearchOptions::Audio
        } else {
            self.filter.to_owned()
        };
        let video_options = VideoOptions {
            quality: VideoQuality::Highest,
            filter: filter.to_owned(),
            ..Default::default()
        };

        let video = rusty_ytdl::Video::new_with_options(self.url.clone(), video_options)
            .map_err(|_| DownloadError::VideoNotFound("Video Not Found".to_owned()))?;
        let video_info = video.get_basic_info().await?;

        let title = video_info.video_details.title.replace(" ", "_");

        match &filter {
            VideoSearchOptions::VideoAudio => video.download(format!("{title}.mp4")).await?,
            VideoSearchOptions::Video => video.download(format!("{title}.mp4")).await?,
            VideoSearchOptions::Audio => {
                //todo! for now only working if `ffmpeg` is installed on the computer
                // try to implement a way using `symphonia` crate maybe
                if self.to_mp3 {
                    video
                        .download_with_ffmpeg(
                            format!("{title}.mp3"),
                            Some(FFmpegArgs {
                                format: Some("mp3".to_string()),
                                audio_filter: None,
                                video_filter: None,
                            }),
                        )
                        .await?
                } else {
                    video.download(format!("{title}.webm")).await?
                }
            }
            VideoSearchOptions::Custom(_) => video.download(format!("{title}")).await?,
        }

        Ok(())
    }

    fn blocking_download(&self) -> Result<(), DownloadError> {
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
