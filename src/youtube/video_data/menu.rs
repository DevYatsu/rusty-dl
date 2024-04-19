use serde::{Deserialize, Serialize};

use super::{AccessibilityDataWrapper, SkippedFields};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Menu {
    #[serde(rename = "menuRenderer")]
    pub menu_renderer: MenuRenderer,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MenuRenderer {
    pub accessibility: AccessibilityDataWrapper,
    pub items: Vec<MenuItem>,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MenuItem {
    #[serde(rename = "menuServiceItemRenderer")]
    pub menu_service_item_renderer: MenuServiceItemRenderer,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MenuServiceItemRenderer {
    pub icon: Icon,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: ServiceEndpoint,
    pub text: Text,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
    #[serde(rename = "hasSeparator")]
    pub has_separator: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Icon {
    #[serde(rename = "iconType")]
    pub icon_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServiceEndpoint {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    #[serde(rename = "commandMetadata")]
    pub command_metadata: CommandMetadata,
    #[serde(rename = "signalServiceEndpoint")]
    pub signal_service_endpoint: Option<SignalServiceEndpoint>,
    #[serde(rename = "shareEntityServiceEndpoint")]
    pub share_entity_service_endpoint: Option<ShareEntityServiceEndpoint>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommandMetadata {
    #[serde(rename = "webCommandMetadata")]
    pub web_command_metadata: WebCommandMetadata,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WebCommandMetadata {
    #[serde(rename = "sendPost")]
    pub send_post: bool,
    #[serde(rename = "apiUrl")]
    pub api_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SignalServiceEndpoint {
    pub actions: Vec<Action>,
    pub signal: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Action {
    #[serde(rename = "addToPlaylistCommand")]
    pub add_to_playlist_command: AddToPlaylistCommand,
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AddToPlaylistCommand {
    #[serde(rename = "listType")]
    pub list_type: String,
    #[serde(rename = "onCreateListCommand")]
    pub on_create_list_command: OnCreateListCommand,
    #[serde(rename = "openMiniplayer")]
    pub open_miniplayer: bool,
    #[serde(rename = "videoId")]
    pub video_id: String,
    #[serde(rename = "videoIds")]
    pub video_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OnCreateListCommand {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    #[serde(rename = "commandMetadata")]
    pub command_metadata: CommandMetadata,
    #[serde(rename = "createPlaylistServiceEndpoint")]
    pub create_playlist_service_endpoint: CreatePlaylistServiceEndpoint,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreatePlaylistServiceEndpoint {
    pub params: String,
    #[serde(rename = "videoIds")]
    pub video_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShareEntityServiceEndpoint {
    pub commands: Vec<ShareEntityCommand>,
    pub serialized_share_entity: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShareEntityCommand {
    #[serde(rename = "clickTrackingParams")]
    pub click_tracking_params: String,
    pub open_popup_action: Option<OpenPopupAction>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenPopupAction {
    #[serde(rename = "beReused")]
    pub be_reused: bool,
    pub popup: Popup,
    #[serde(rename = "popupType")]
    pub popup_type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Popup {
    #[serde(rename = "popupType")]
    pub unified_share_panel_renderer: UnifiedSharePanelRenderer,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnifiedSharePanelRenderer {
    #[serde(rename = "showLoadingSpinner")]
    pub show_loading_spinner: bool,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Text {
    pub runs: Vec<Run>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Run {
    pub text: String,
}
