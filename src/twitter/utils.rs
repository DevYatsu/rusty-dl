use crate::prelude::DownloadError;
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestDetails {
    pub features: Features,
    pub variables: Variables,
}

pub async fn retrieve_request_details() -> Result<RequestDetails, DownloadError> {
    let request_details_file = "RequestDetails.json";

    // Read the JSON file into a string
    let mut file = File::open(request_details_file)
        .await
        .expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .await
        .expect("Failed to read file");

    // Parse the JSON string into a serde_json::Value
    let request_details: RequestDetails =
        serde_json::from_str(&contents).expect("Failed to parse JSON");

    Ok(request_details)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Features {
    responsive_web_graphql_exclude_directive_enabled: bool,
    verified_phone_label_enabled: bool,
    responsive_web_graphql_timeline_navigation_enabled: bool,
    responsive_web_graphql_skip_user_profile_image_extensions_enabled: bool,
    tweetypie_unmention_optimization_enabled: bool,
    vibe_api_enabled: bool,
    responsive_web_edit_tweet_api_enabled: bool,
    graphql_is_translatable_rweb_tweet_is_translatable_enabled: bool,
    view_counts_everywhere_api_enabled: bool,
    longform_notetweets_consumption_enabled: bool,
    tweet_awards_web_tipping_enabled: bool,
    freedom_of_speech_not_reach_fetch_enabled: bool,
    standardized_nudges_misinfo: bool,
    tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled: bool,
    interactive_text_enabled: bool,
    responsive_web_twitter_blue_verified_badge_is_enabled: bool,
    responsive_web_text_conversations_enabled: bool,
    longform_notetweets_richtext_consumption_enabled: bool,
    responsive_web_enhance_cards_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variables {
    with_rux_injections: bool,
    includePromotedContent: bool,
    withCommunity: bool,
    withQuickPromoteEligibilityTweetFields: bool,
    withBirdwatchNotes: bool,
    withDownvotePerspective: bool,
    withReactionsMetadata: bool,
    withReactionsPerspective: bool,
    withVoice: bool,
    withV2Timeline: bool,
    tweetId: Option<String>,
}

impl Variables {
    pub fn set_tweet_id(&mut self, tweet_id: String) {
        self.tweetId = Some(tweet_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ErrorResponse {
    errors: Vec<Error>,
}

#[derive(Debug, serde::Deserialize)]
struct Error {
    message: String,
    extensions: Extension,
    code: u32,
    kind: String,
    name: String,
    source: String,
    tracing: Trace,
}

#[derive(Debug, serde::Deserialize)]
struct Extension {
    name: String,
    source: String,
    code: u32,
    kind: String,
    tracing: Trace,
}

#[derive(Debug, serde::Deserialize)]
struct Trace {
    trace_id: String,
}
