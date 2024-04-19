#[derive(Debug, serde::Deserialize, Clone, serde::Serialize)]
pub struct BrowseEndpoint {
    #[serde(rename = "browseId")]
    pub browse_id: String,
    #[serde(rename = "canonicalBaseUrl")]
    pub canonical_base_url: String,
}

#[derive(Debug, serde::Deserialize, Clone, serde::Serialize)]
pub struct WebCommandMetadata {
    #[serde(rename = "apiUrl")]
    pub api_url: String,
    #[serde(rename = "rootVe")]
    pub root_ve: i32,
    #[serde(rename = "url")]
    pub url: String,       
    #[serde(rename = "webPageType")]
    pub web_page_type: String,    
}

#[derive(Debug, serde::Deserialize, Clone, serde::Serialize)]
pub struct CommandMetadata {
    #[serde(rename = "webCommandMetadata")]
    pub web_command_metadata: WebCommandMetadata
}


#[derive(Debug, serde::Deserialize, Clone, serde::Serialize)]
pub struct NavigationEndpoint {
    #[serde(rename = "browseEndpoint")]
    pub browse_endpoint: BrowseEndpoint,
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    #[serde(rename = "commandMetadata")]
    pub command_metadata: CommandMetadata,
}

#[derive(Debug, serde::Deserialize, Clone, serde::Serialize)]
pub struct Run {
    #[serde(rename = "navigationEndpoint")]
    pub navigation_endpoint: NavigationEndpoint,
    pub text: String,
}

#[derive(Debug, serde::Deserialize, Clone, serde::Serialize)]
pub struct ShortBylineText {
    pub runs: Vec<Run>,
}
