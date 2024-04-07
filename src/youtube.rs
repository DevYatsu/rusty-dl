use crate::header::HeaderMapBuilder;
use crate::prelude::{DownloadError, Downloader};
use reqwest::{Client, Url};
use rusty_ytdl::{FFmpegArgs, Video};
use rusty_ytdl::{VideoOptions, VideoQuality, VideoSearchOptions};
use scraper::{Html, Selector};
use std::path::Path;

use self::initial_data::{InitialData, VideoData};

mod initial_data;

#[derive(Debug, Clone)]
/// Implementation of a YouTube downloader.
pub struct YoutubeDownloader {
    url: Url,
    filter: VideoSearchOptions,
    add_underscores_in_name: bool,
    video_name: Option<String>,

    // for playlist downloading
    is_playlist: bool,
}

impl YoutubeDownloader {
    /// Creates a new instance of the [`YoutubeDownloader`] with the provided YouTube video link.
    ///
    /// ### Arguments
    ///
    /// * `link` - The YouTube video/playlist link to download.
    ///
    /// ### Returns
    ///
    /// Returns a [`Result`] containing the [`YoutubeDownloader`] instance on success, or a [`DownloadError`] if parsing the URL fails or if the URL is invalid.
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

        let mut is_playlist = false;
        let path_segments = url
            .path_segments()
            .ok_or_else(|| DownloadError::InvalidUrl("Video Not Found".to_owned()))?;

        for segment in path_segments {
            if segment == "playlist" {
                is_playlist = true;
            }
        }

        Ok(Self {
            url,
            filter: VideoSearchOptions::VideoAudio,
            add_underscores_in_name: false,
            video_name: None,
            is_playlist,
        })
    }

    /// Retrieves information about the video.
    ///
    /// This method returns a [`Result`] containing a [`Video`] instance, which represents the video and allows accessing its
    /// **metadata** and downloading it.
    ///
    /// ### Returns
    ///
    /// Returns a [`Result`] containing a [`Video`] instance on success, or a [`DownloadError`] if the video is not found.
    ///
    /// ### Errors
    ///
    /// Returns a [`DownloadError`] if the video is not found.
    ///
    /// ### Examples
    ///
    /// ```
    /// use rusty_dl::prelude::{YoutubeDownloader, DownloadError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DownloadError> {
    ///     let downloader = YoutubeDownloader::new("https://www.youtube.com/watch?v=video_id").unwrap();
    ///     let video = downloader.get_video()?;
    ///     let title = video.get_basic_info().await?.video_details.title;
    ///     println!("Video Title: {}", title);
    ///     Ok(())
    /// }
    /// ```    
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

    /// Retrieves information about the video with a given URL or ID.
    ///
    /// This method returns a [`Result`] containing a [`Video`] instance, which represents the video and allows accessing its
    /// **metadata** and downloading it.
    ///
    /// ### Arguments
    ///
    /// * `url_or_id` - A string slice containing the URL or ID of the video.
    ///
    /// ### Returns
    ///
    /// Returns a [`Result`] containing a [`Video`] instance on success, or a [`DownloadError`] if the video is not found.
    ///
    /// ### Errors
    ///
    /// Returns a [`DownloadError`] if the video is not found.
    fn get_video_with_url_or_id(&self, url_or_id: &str) -> Result<Video, DownloadError> {
        let filter = self.filter.to_owned();

        let video_options = VideoOptions {
            quality: VideoQuality::Highest,
            filter,
            ..Default::default()
        };

        let video = rusty_ytdl::Video::new_with_options(url_or_id, video_options)
            .map_err(|_| DownloadError::VideoNotFound("Video Not Found".to_owned()))?;

        Ok(video)
    }

    /// Retrieves information about a YouTube playlist.
    ///
    /// This function asynchronously fetches information about a YouTube playlist from the provided URL.
    /// It sends an HTTP GET request to the YouTube URL using the [`reqwest`] crate and awaits the response.
    /// It extracts the name of the playlist and a list of video data by scraping the response HTML.
    /// Finally, it constructs and returns a [`Playlist`] instance containing the playlist name and video data.
    ///
    /// ### Errors
    ///
    /// Returns a [`DownloadError`] if any error occurs during the retrieval process, such as failure to send HTTP requests,
    /// receiving unexpected responses, or parsing HTML content.
    async fn get_playlist(&self) -> Result<Playlist, DownloadError> {
        let client = Client::new();

        let response = client
            .get(self.url.to_owned())
            .headers(HeaderMapBuilder::new().with_user_agent().build())
            .send()
            .await?
            .text()
            .await?;

        let (name, videos) = self.scrape_videos_data(response)?;
        Ok(Playlist { name, videos })
    }

    /// Scrapes video data from the HTML response.
    ///
    /// This function takes a string `response`, representing the HTML content of a YouTube playlist page,
    /// and parses it to extract relevant video data. It uses the `scraper` (and thus `html5ever`) crate to parse the HTML document.
    /// It then selects script elements using a CSS selector to find the JavaScript variable `ytInitialData`.
    /// The function extracts the JSON object from the JavaScript variable and deserializes it into an [`InitialData`] struct.
    /// Finally, it retrieves the playlist name and video data from the deserialized object and returns them as a tuple.
    ///
    /// ### Arguments
    ///
    /// * `response` - A string containing the HTML content of a YouTube playlist page.
    ///
    /// ### Returns
    ///
    /// Returns a tuple containing the name of the playlist and a vector of video data.
    ///
    /// ### Errors
    ///
    /// Returns a [`DownloadError`] if any error occurs during the process of parsing HTML content.
    fn scrape_videos_data(
        &self,
        response: String,
    ) -> Result<(String, Vec<VideoData>), DownloadError> {
        let document = Html::parse_document(&response);
        let scripts_selector = Selector::parse("script").unwrap();

        let string_object = document
            .select(&scripts_selector)
            .filter(|x| x.inner_html().contains("var ytInitialData ="))
            .map(|x| x.inner_html().replace("var ytInitialData =", ""))
            .next()
            .unwrap_or(String::from(""))
            .trim()
            .trim_end_matches(';')
            .to_string();

        let data: InitialData = serde_json::from_str(&string_object)
            .map_err(|_| DownloadError::YoutubeError(format!("Failed to scrape playlist data.")))?;
        let playlist_name = data.get_playlist_name();
        let videos_data = data.videos_data();

        Ok((playlist_name, videos_data))
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
    ///
    /// Youtube API returns `webm` format, we try to convert it to a `mp3` using ffmpeg, if it fails we return the initial response file.
    pub fn only_audio(&mut self) -> &mut Self {
        self.filter = VideoSearchOptions::Audio;

        self
    }
    /// Sets the filter to download both the video and audio of the video.
    ///
    /// That's the
    /// **DEFAULT** behavior!
    pub fn video_and_audio(&mut self) -> &mut Self {
        self.filter = VideoSearchOptions::VideoAudio;

        self
    }
    /// Sets the filter to download only the video.
    pub fn only_video(&mut self) -> &mut Self {
        self.filter = VideoSearchOptions::Video;

        self
    }

    /// Downloads a video to the specified path.
    ///
    /// This function is not meant to be used  directly by users. Instead it should be called through one of the other functions in this struct.
    ///
    /// This function asynchronously downloads a video to the provided folder path. It first fetches basic information about the video,
    /// such as its title, using the `get_basic_info` method of the [`Video`] struct. It then constructs the full path for the downloaded
    /// video file based on the provided path and optional video name set in the [`YoutubeDownloader`] instance. If the `add_underscores_in_name`
    /// flag is set to true, spaces in the video title are replaced with underscores.
    ///
    /// ### Arguments
    ///
    /// * `video` - The `[Video`] instance representing the video to be downloaded.
    /// * `path` - The path to the folder where the video will be downloaded.
    ///
    /// ### Errors
    ///
    /// Returns a [`DownloadError`] if any error occurs during the download process, such as failure to create directories,
    /// fetching video information, or downloading the video file.
    ///
    /// ### Examples
    ///
    /// ```
    /// use rusty_dl::prelude::{YoutubeDownloader, DownloadError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DownloadError> {
    ///     let downloader = YoutubeDownloader::new("https://www.youtube.com/watch?v=video_id").unwrap();
    ///     let video = downloader.get_video()?;
    ///     downloader.download_video_to_path(video, "/path/to/download").await?;
    ///     Ok(())
    /// }
    /// ```
    async fn download_video_to_path<P: AsRef<Path>>(
        &self,
        video: Video,
        path: P,
    ) -> Result<(), DownloadError> {
        let video_info = video.get_basic_info().await?;

        let base_path = path.as_ref();

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

    /// Downloads all videos from a playlist to the specified path.
    ///
    /// This function asynchronously downloads all videos from a YouTube playlist to the provided folder path.
    ///
    /// ### Arguments
    ///
    /// * `path` - The path to the folder where the videos will be downloaded.
    ///
    /// ### Errors
    ///
    /// Returns a `DownloadError` if any error occurs during the download process, such as failure to create directories,
    /// fetching playlist information, or downloading the videos.
    ///
    /// ### Examples
    ///
    /// ```
    /// use rusty_dl::prelude::{YoutubeDownloader, DownloadError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DownloadError> {
    ///     let downloader = YoutubeDownloader::new("https://www.youtube.com/playlist?list=playlist_id").unwrap();
    ///     downloader.download_playlist_to("/path/to/download").await?;
    ///     Ok(())
    /// }
    /// ```    
    pub async fn download_playlist_to(&self, path: &Path) -> Result<(), DownloadError> {
        let playlist = self.get_playlist().await?;
        let joined_path = &path.join(&playlist.name.replace("/", "_"));

        // create the directory of the playlist
        tokio::fs::create_dir_all(&joined_path).await?;

        let results =
            futures::future::join_all(playlist.videos.into_iter().map(|video_data| async move {
                let video = self.get_video_with_url_or_id(&video_data.video_id)?;
                let title = video.get_basic_info().await?.video_details.title;
                let download_result = self.download_video_to_path(video, &joined_path).await;

                if let Err(err) = &download_result {
                    eprintln!("Error downloading video named `{}`: {:?}", title, err);
                } else {
                    println!("Video downloaded successfully: {}", title);
                }

                download_result
            }))
            .await;

        for result in results {
            result?
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
/// Simplified representation of a youtube playlist.
pub struct Playlist {
    pub name: String,
    pub videos: Vec<VideoData>,
}

impl Downloader for YoutubeDownloader {
    async fn download_to<P: AsRef<Path> + std::marker::Send>(
        &self,
        folder_path: P,
    ) -> Result<(), DownloadError> {
        let path = folder_path.as_ref();
        if path.is_file() {
            return Err(DownloadError::Downloader(format!(
                "Path must point to a directory. That is not the case for `{}`",
                path.display()
            )));
        }

        if self.is_playlist {
            self.download_playlist_to(path).await?;

            return Ok(());
        }

        let video = self.get_video()?;
        self.download_video_to_path(video, path).await?;

        Ok(())
    }

    fn is_valid_url(url: &Url) -> bool {
        url.domain() == Some("youtube.com")
            || url.domain() == Some("youtu.be")
            || url.domain() == Some("www.youtube.com")
            || url.domain() == Some("www.youtu.be")
    }
}
