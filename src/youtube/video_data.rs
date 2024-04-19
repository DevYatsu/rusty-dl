use crate::prelude::DownloadError;

use self::{
    index::VideoIndex,
    info::VideoDataInfo,
    length_text::VideoLengthText,
    menu::Menu,
    nav_endpoint::NavigationEndpoint,
    short_by_line_text::ShortBylineText,
    thumbnail::{ThumbnailData, VideoThumbnail},
    title::VideoTitle,
};

mod index;
mod info;
mod length_text;
mod menu;
mod nav_endpoint;
mod short_by_line_text;
mod thumbnail;
mod title;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct SkippedFields {}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct VideoData {
    #[serde(rename = "videoId")]
    pub video_id: String,
    pub thumbnail: ThumbnailData,
    pub title: VideoTitle,
    pub index: VideoIndex,

    #[serde(rename = "shortBylineText")]
    pub short_byline_text: ShortBylineText,

    #[serde(rename = "lengthText")]
    pub length_text: VideoLengthText,

    /// This one is not implemented yet, i need to put my mind into it...
    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: NavigationEndpoint,
    #[serde(rename = "lengthSeconds")]
    pub length_seconds: String,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
    #[serde(rename = "isPlayable")]
    pub is_playable: bool,

    pub menu: Menu,
    #[serde(rename = "thumbnailOverlays")]
    pub thumbnail_overlays: Vec<VideoThumbnail>,
    #[serde(rename = "videoInfo")]
    pub video_info: VideoDataInfo,
}

impl VideoData {
    // TO RETRIEVE THE VIDEO TITLE:
    // video.get_basic_info().await?.video_details.title
    // is the most surefire way but the issue is it's sending a request and it takes too much time

    pub fn get_title(&self) -> Result<String, DownloadError> {
        Ok(self
            .title
            .runs
            .get(0)
            .ok_or_else(|| {
                DownloadError::YoutubeError(format!(
                    "Could not retrieve title of video with id `{}`",
                    self.video_id
                ))
            })?
            .text
            .to_owned())
    }
}

// It's shared between several structs, namely: VideoTitle and VideoLengthText
// so pay attention whenever modifying it
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct AccessibilityDataWrapper {
    #[serde(rename = "accessibilityData")]
    pub accessibility_data: AccessibilityData,
}
// It's shared between several structs, namely: VideoTitle and VideoLengthText
// so pay attention whenever modifying it
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct AccessibilityData {
    pub label: String,
}
