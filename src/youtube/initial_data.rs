use crate::prelude::DownloadError;

/// Represents the initial data retrieved from a
/// **Playlist**
/// Youtube Page.
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct InitialData {
    /// The response context.
    #[serde(rename = "responseContext")]
    pub response_context: ResponseContext,

    /// The contents of the page.
    pub contents: Content,

    /// The header section of the page.
    pub header: SkippedFields,

    /// The metadata of the playlist.
    pub metadata: PlaylistMetadata,

    /// The tracking parameters.
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,

    /// The top bar section of the page.
    pub topbar: SkippedFields,

    /// The microformat section of the page.
    pub microformat: SkippedFields,

    /// The sidebar section of the page.
    pub sidebar: SkippedFields,

    /// Optional framework updates.
    #[serde(rename = "frameworkUpdates")]
    pub framework_updates: Option<SkippedFields>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct PlaylistMetadata {
    #[serde(rename = "playlistMetadataRenderer")]
    playlist_metadata_render: PlaylistMetadataRenderer,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct PlaylistMetadataRenderer {
    pub title: String,
    #[serde(rename = "androidAppindexingLink")]
    pub android_appindexing_link: String,
    #[serde(rename = "iosAppindexingLink")]
    pub ios_appindexing_link: String,
}

impl InitialData {
    /// Retrieve the data on videos in the playlist
    pub fn videos_data(self) -> Vec<VideoData> {
        self.contents.two_column_browse_results_renderer.tabs[0]
            .to_owned()
            .tab_renderer
            .content
            .section_list_renderer
            .contents
            .0
            .item_section_renderer
            .contents[0]
            .to_owned()
            .playlist_video_list_renderer
            .get_videos()
    }

    pub fn get_playlist_name(&self) -> String {
        self.metadata.playlist_metadata_render.title.to_owned()
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Content {
    #[serde(rename = "twoColumnBrowseResultsRenderer")]
    pub two_column_browse_results_renderer: TwoColumnBrowseResultsRenderer,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ResponseContext {
    #[serde(rename = "serviceTrackingParams")]
    pub service_tracking_params: Vec<SkippedFields>,
    #[serde(rename = "mainAppWebResponseContext")]
    pub main_app_web_response_context: SkippedFields,
    #[serde(rename = "webResponseContextExtensionData")]
    pub web_response_context_extension_data: SkippedFields,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct SkippedFields {}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct TwoColumnBrowseResultsRenderer {
    pub tabs: [TabsObject; 1],
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct TabsObject {
    #[serde(rename = "tabRenderer")]
    pub tab_renderer: TabRendererObject,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct TabRendererObject {
    pub selected: bool,
    pub content: TabContent,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct TabContent {
    #[serde(rename = "sectionListRenderer")]
    pub section_list_renderer: SectionListRenderer,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct SectionListRenderer {
    pub contents: (Content1, Content2),
    #[serde(rename = "targetId")]
    pub target_id: String,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Content1 {
    #[serde(rename = "itemSectionRenderer")]
    pub item_section_renderer: ItemSectionRenderer,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Content2 {
    #[serde(rename = "continuationItemRenderer")]
    pub continuation_item_renderer: SkippedFields,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ItemSectionRenderer {
    pub contents: [PlaylistVideoListRendererContainer; 1],
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct PlaylistVideoListRendererContainer {
    #[serde(rename = "playlistVideoListRenderer")]
    pub playlist_video_list_renderer: PlaylistVideoListRenderer,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct PlaylistVideoListRenderer {
    pub contents: Vec<PlaylistVideoRenderer>,
    #[serde(rename = "trackingParams")]
    pub tracking_params: Option<String>,
    #[serde(rename = "targetId")]
    pub target_id: String,
    #[serde(rename = "canReorder")]
    pub can_reorder: bool,
    #[serde(rename = "isEditable")]
    pub is_editable: bool,
    #[serde(rename = "playlistId")]
    pub playlist_id: String,
}

impl PlaylistVideoListRenderer {
    pub fn get_videos(self) -> Vec<VideoData> {
        self.contents
            .into_iter()
            .filter_map(PlaylistVideoRenderer::filter_videos_data)
            .collect()
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum PlaylistVideoRenderer {
    #[serde(rename = "playlistVideoRenderer")]
    PlaylistVideoRenderer(VideoData),
    #[serde(rename = "continuationItemRenderer")]
    ContinuationItemRenderer(SkippedFields),
}

impl PlaylistVideoRenderer {
    pub fn filter_videos_data(self) -> Option<VideoData> {
        match self {
            PlaylistVideoRenderer::PlaylistVideoRenderer(v) => Some(v),
            PlaylistVideoRenderer::ContinuationItemRenderer(_) => None,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct VideoData {
    #[serde(rename = "videoId")]
    pub video_id: String,
    pub thumbnail: SkippedFields,
    pub title: VideoTitle,
    pub index: SkippedFields,
    #[serde(rename = "shortBylineText")]
    pub short_byline_text: SkippedFields,
    #[serde(rename = "lengthText")]
    pub length_text: SkippedFields,
    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: SkippedFields,
    #[serde(rename = "lengthSeconds")]
    pub length_seconds: String,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
    #[serde(rename = "isPlayable")]
    pub is_playable: bool,
    pub menu: SkippedFields,
    #[serde(rename = "thumbnailOverlays")]
    pub thumbnail_overlays: Vec<SkippedFields>,
    #[serde(rename = "videoInfo")]
    pub video_info: SkippedFields,
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

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct VideoTitle {
    pub runs: Vec<TitleText>,
    pub accessibility: SkippedFields,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct TitleText {
    text: String,
}
