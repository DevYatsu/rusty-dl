use crate::prelude::DownloadError;
use serde::{Deserialize, Serialize};

/// Represents the details of a Twitter request when fetching a tweet.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RequestDetails {
    /// Features of the request.
    pub features: Features,

    /// Variables of the request.
    pub variables: Variables,
}

/// Retrieves the request details for a Twitter request.
///
/// ### Errors
///
/// Returns a [`DownloadError`] if there is an issue retrieving the request details.
pub async fn retrieve_request_details() -> Result<RequestDetails, DownloadError> {
    // in the past i was using a json to kee the request details but it's not convenient enough to be used in a library.

    // let request_details_file = "RequestDetails.json";

    // // Read the JSON file into a string
    // let mut file = File::open(request_details_file)
    //     .await
    //     .expect("Failed to open file");
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)
    //     .await
    //     .expect("Failed to read file");

    // // Parse the JSON string into a serde_json::Value
    // let request_details: RequestDetails =
    //     serde_json::from_str(&contents).expect("Failed to parse JSON");

    Ok(RequestDetails::default())
}

/// Features of a Twitter request.
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

/// Variables of a Twitter request.
#[derive(Debug, Serialize, Deserialize)]
pub struct Variables {
    with_rux_injections: bool,
    #[serde(rename = "includePromotedContent")]
    include_promoted_content: bool,

    #[serde(rename = "withCommunity")]
    with_community: bool,
    #[serde(rename = "withQuickPromoteEligibilityTweetFields")]
    with_quick_promote_eligibility_tweet_fields: bool,
    #[serde(rename = "withBirdwatchNotes")]
    with_birdwatch_notes: bool,
    #[serde(rename = "withDownvotePerspective")]
    with_downvote_perspective: bool,
    #[serde(rename = "withReactionsMetadata")]
    with_reactions_metadata: bool,
    #[serde(rename = "withReactionsPerspective")]
    with_reactions_perspective: bool,
    #[serde(rename = "withVoice")]
    with_voice: bool,
    #[serde(rename = "withV2Timeline")]
    with_v2_timeline: bool,
    #[serde(rename = "tweetId")]
    tweet_id: Option<String>,
}

impl Variables {
    pub fn set_tweet_id(&mut self, tweet_id: String) {
        self.tweet_id = Some(tweet_id)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct ErrorResponse {
    pub errors: Vec<Error>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Error {
    pub message: String,
    pub extensions: Extension,
    pub code: u32,
    pub kind: String,
    pub name: String,
    pub source: String,
    pub tracing: Trace,
}

#[derive(Debug, serde::Deserialize)]
struct Extension {
    pub name: String,
    pub source: String,
    pub code: u32,
    pub kind: String,
    pub tracing: Trace,
}

#[derive(Debug, serde::Deserialize)]
struct Trace {
    pub trace_id: String,
}

impl Default for Variables {
    fn default() -> Self {
        Self {
            with_rux_injections: false,
            include_promoted_content: true,
            with_community: true,
            with_quick_promote_eligibility_tweet_fields: true,
            with_birdwatch_notes: true,
            with_downvote_perspective: false,
            with_reactions_metadata: false,
            with_reactions_perspective: false,
            with_voice: true,
            with_v2_timeline: true,
            tweet_id: None,
        }
    }
}

impl Default for Features {
    fn default() -> Self {
        Self {
            responsive_web_graphql_exclude_directive_enabled: true,
            verified_phone_label_enabled: false,
            responsive_web_graphql_timeline_navigation_enabled: true,
            responsive_web_graphql_skip_user_profile_image_extensions_enabled: false,
            tweetypie_unmention_optimization_enabled: true,
            vibe_api_enabled: false,
            responsive_web_edit_tweet_api_enabled: false,
            graphql_is_translatable_rweb_tweet_is_translatable_enabled: false,
            view_counts_everywhere_api_enabled: true,
            longform_notetweets_consumption_enabled: true,
            tweet_awards_web_tipping_enabled: false,
            freedom_of_speech_not_reach_fetch_enabled: false,
            standardized_nudges_misinfo: false,
            tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled: false,
            interactive_text_enabled: false,
            responsive_web_twitter_blue_verified_badge_is_enabled: true,
            responsive_web_text_conversations_enabled: false,
            longform_notetweets_richtext_consumption_enabled: false,
            responsive_web_enhance_cards_enabled: false,
        }
    }
}
