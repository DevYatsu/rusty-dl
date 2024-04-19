use super::AccessibilityDataWrapper;

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct VideoTitle {
    pub runs: Vec<TitleText>,
    pub accessibility: AccessibilityDataWrapper,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct TitleText {
    pub text: String,
}
