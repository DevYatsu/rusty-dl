use super::AccessibilityDataWrapper;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct VideoLengthText {
    pub accessibility: AccessibilityDataWrapper,
    #[serde(rename = "simpleText")]
    pub simple_text: String,
}
