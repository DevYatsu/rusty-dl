use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WebCommandMetadata {
    #[serde(rename = "webCommandMetadata")]
    pub web_command_metadata: InnerWebCommandMetadata,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InnerWebCommandMetadata {
    #[serde(rename = "rootVe")]
    pub root_ve: i32,
    pub url: String,
    #[serde(rename = "webPageType")]
    pub web_page_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VssLoggingContext {
    #[serde(rename = "serializedContextData")]
    pub serialized_context_data: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggingContext {
    #[serde(rename = "vssLoggingContext")]
    pub vss_logging_context: VssLoggingContext,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommonConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Html5PlaybackOnesieConfig {
    #[serde(rename = "commonConfig")]
    pub common_config: CommonConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WatchEndpointSupportedOnesieConfig {
    #[serde(rename = "html5PlaybackOnesieConfig")]
    pub html5_playback_onesie_config: Html5PlaybackOnesieConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WatchEndpoint {
    pub index: i32,
    #[serde(rename = "loggingContext")]
    pub logging_context: LoggingContext,
    pub params: String,
    #[serde(rename = "playerParams")]
    pub player_params: String,
    #[serde(rename = "playlistId")]
    pub playlist_id: String,
    #[serde(rename = "videoId")]
    pub video_id: String,
    #[serde(rename = "watchEndpointSupportedOnesieConfig")]
    pub watch_endpoint_supported_onesie_config: WatchEndpointSupportedOnesieConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NavigationEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    #[serde(rename = "commandMetadata")]
    pub command_metadata: WebCommandMetadata,
    #[serde(rename = "watchEndpoint")]
    pub watch_endpoint: WatchEndpoint,
}
