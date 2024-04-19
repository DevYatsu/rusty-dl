#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ThumbnailData {
    pub thumbnails: Vec<VideoThumbnail>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct VideoThumbnail {
    pub height: u16,
    pub url: String,
    pub width: u16,
}
