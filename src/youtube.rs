use crate::prelude::{DownloadError, Downloader};
use reqwest::Url;
use rusty_ytdl::{FFmpegArgs, Video};
use rusty_ytdl::{VideoOptions, VideoQuality, VideoSearchOptions};
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq, Clone)]
/// Implementation of a YouTube downloader.
pub struct YoutubeDownloader {
    url: Url,
    filter: VideoSearchOptions,
    add_underscores_in_name: bool,
    video_name: Option<String>,
    video: Option<Video>,
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

        if !Self::is_valid_url(&url) {
            return Err(DownloadError::InvalidUrl(
                "Invalid URL! The domain must be 'youtube.com'.".to_owned(),
            ));
        }

        Ok(Self {
            url,
            filter: VideoSearchOptions::VideoAudio,
            add_underscores_in_name: false,
            video_name: None,
            video: None,
        })
    }

    /// Retrieves information about the video.
    ///
    /// This method returns a `Result` containing a `Video` instance, which represents the video and allows accessing its
    /// **metadata** and downloading it.
    pub fn get_video(&self) -> Result<Video, DownloadError> {
        let filter = self.filter.to_owned();

        let video_options = VideoOptions {
            quality: VideoQuality::Highest,
            filter,
            ..Default::default()
        };

        let video = rusty_ytdl::Video::new_with_options(self.url.clone(), video_options)
            .map_err(|_| DownloadError::VideoNotFound("Video Not Found".to_owned()))?;

        Ok(video)
    }

    /// Enables renaming the downloaded video with underscores.
    pub fn rename_with_underscores(&mut self) -> &mut Self {
        self.add_underscores_in_name = true;
        self
    }

    /// Sets a custom name for the downloaded video.
    ///
    /// ### Arguments
    ///
    /// * `new_name` - The new name for the downloaded video.
    pub fn set_name(&mut self, new_name: String) -> &mut Self {
        self.video_name = Some(new_name);

        self
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
}

impl Downloader for YoutubeDownloader {
    async fn download_to(&self, path: &Path) -> Result<(), DownloadError> {
        if path.is_file() {
            return Err(DownloadError::Downloader(format!(
                "Path must point to a directory. That is not the case for `{}`",
                path.display()
            )));
        }

        let video = self.get_video()?;
        let video_info = video.get_basic_info().await?;

        let base_path: PathBuf = path.into();

        let mut video_name = self
            .video_name
            .to_owned()
            .unwrap_or(video_info.video_details.title);

        if self.add_underscores_in_name {
            video_name = video_name.replace(" ", "_");
        }

        let mut video_path = base_path.join(video_name);

        if let Some(parent) = video_path.parent() {
            tokio::fs::create_dir_all(parent).await?
        }

        match &self.filter {
            VideoSearchOptions::VideoAudio => {
                video_path.set_extension("mp4");
                video.download(video_path).await?
            }
            VideoSearchOptions::Video => {
                video_path.set_extension("mp4");
                video.download(video_path).await?
            }
            VideoSearchOptions::Audio => {
                video_path.set_extension("mp3");
                println!("{}", video_path.display());

                // `ffmpeg` must be installed on the computer to download a mp3 file

                match video
                    .download_with_ffmpeg(
                        video_path.to_owned(),
                        Some(FFmpegArgs {
                            format: Some("mp3".to_string()),
                            audio_filter: None,
                            video_filter: None,
                        }),
                    )
                    .await
                {
                    Ok(v) => v,
                    Err(_) => {
                        // If download with ffmpeg fails, try downloading without ffmpeg
                        video_path.set_extension("webm");
                        video.download(video_path).await?
                    }
                }
            }
            VideoSearchOptions::Custom(_) => video.download(video_path).await?,
        }

        Ok(())
    }

    fn is_valid_url(url: &Url) -> bool {
        url.domain() == Some("youtube.com")
            || url.domain() == Some("youtu.be")
            || url.domain() == Some("www.youtube.com")
            || url.domain() == Some("www.youtu.be")
    }
}
