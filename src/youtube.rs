use crate::prelude::{DownloadError, Downloader};
use reqwest::Url;
use rusty_ytdl::FFmpegArgs;
use rusty_ytdl::{VideoOptions, VideoQuality, VideoSearchOptions};
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Clone)]
/// Implementation of a YouTube downloader.
pub struct YoutubeDownloader {
    url: Url,
    filter: VideoSearchOptions,
    to_mp3: bool,
}

impl YoutubeDownloader {
    /// Creates a new instance of the `YoutubeDownloader` with the provided YouTube video link.
    ///
    /// ### Arguments
    ///
    /// * `link` - The YouTube video link to download.
    ///
    /// ### Returns
    ///
    /// Returns a `Result` containing the `YoutubeDownloader` instance on success, or a `DownloadError` if parsing the URL fails or if the URL is invalid.
    pub fn new(link: &str) -> Result<Self, DownloadError> {
        let url = Self::parse_url(
            link,
            Some("https://www.youtube.com/v=<VIDEO_ID> or https://www.youtu.be/<VIDEO_ID>/"),
        )?;

        if url.domain() != Some("www.youtube.com")
            && url.domain() != Some("youtube.com")
            && url.domain() != Some("www.youtu.be")
            && url.domain() != Some("youtu.be")
        {
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
    /// Sets the filter to download only the audio of the video.
    pub fn only_audio(&mut self) -> &mut Self {
        self.filter = VideoSearchOptions::Audio;

        self
    }
    /// Sets the filter to download both the video and audio of the video.
    pub fn video_and_audio(&mut self) -> &mut Self {
        self.filter = VideoSearchOptions::VideoAudio;

        self
    }
    /// Sets the filter to download only the video.
    pub fn only_video(&mut self) -> &mut Self {
        self.filter = VideoSearchOptions::Video;

        self
    }

    /// Enables conversion of downloaded video to MP3 format.
    pub fn to_mp3(&mut self) -> &mut Self {
        self.to_mp3 = true;

        self
    }
}

impl Downloader for YoutubeDownloader {
    async fn download_to(&self, path: &Path) -> Result<(), DownloadError> {
        if path.is_file() {
            return Err(DownloadError::Downloader(format!(
                "Path must point to a directory. That is not the case for `{}`",
                path.display()
            )));
        }

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

        let base_path: PathBuf = path.into();

        let new_path = base_path.join(video_info.video_details.title.replace(" ", "_"));
        let title = new_path.display();

        if let Some(parent) = new_path.parent() {
            tokio::fs::create_dir_all(parent).await?
        }

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
        Self::blocking(async { self.download().await })
    }
}
