#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct VideoDataInfo {
    pub runs: Vec<VideoDataInfoData>,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct VideoDataInfoData {
    pub text: String,
}
