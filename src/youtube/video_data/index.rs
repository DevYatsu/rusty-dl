#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct VideoIndex {
    #[serde(rename = "simpleText")]
    pub simple_text: String,
}
