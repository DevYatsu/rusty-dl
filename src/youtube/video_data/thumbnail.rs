use super::AccessibilityDataWrapper;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ThumbnailData {
    pub thumbnails: Vec<ThumbnailSizeData>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ThumbnailSizeData {
    pub height: u16,
    pub url: String,
    pub width: u16,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum ThumbnailOverlay {
    #[serde(rename = "thumbnailOverlayTimeStatusRenderer")]
    TimeStatus(ThumbnailOverlayTimeStatusRenderer),
    #[serde(rename = "thumbnailOverlayNowPlayingRenderer")]
    NowPlaying(ThumbnailOverlayNowPlayingRenderer),
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ThumbnailOverlayTimeStatusRenderer {
    pub style: String,
    pub text: ThumbnailTimeText,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ThumbnailOverlayNowPlayingRenderer {
    pub text: ThumbnailNowPlayingText,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ThumbnailTimeText {
    pub accessibility: AccessibilityDataWrapper,
    #[serde(rename = "simpleText")]
    pub simple_text: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(untagged)]
pub enum ThumbnailText {
    RunText {
        runs: Vec<TextRun>,
    },

    // only present if style is ig
    StyleText {
        accessibility: AccessibilityDataWrapper,

        #[serde(rename = "simpleText")]
        simple_text: String,
    },
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ThumbnailNowPlayingText {
    pub runs: Vec<TextRun>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct TextRun {
    pub text: String,
}
