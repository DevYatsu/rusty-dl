#[derive(Debug, serde::Deserialize)]
pub struct TweetDetails {
    pub data: Data,
}

#[derive(Debug, serde::Deserialize)]
pub struct Data {
    #[serde(rename = "tweetResult")]
    pub tweet_result: TweetResult,
}

#[derive(Debug, serde::Deserialize)]
pub struct TweetResult {
    pub result: TweetResultValue,
}

#[derive(Debug, serde::Deserialize)]
pub struct TweetResultValue {
    pub __typename: String,
    pub rest_id: String,
    pub has_birdwatch_notes: bool,
    pub core: Core,
    pub unmention_data: UnmentionData,
    pub views: Views,
    pub source: String,
    pub legacy: Legacy,
    pub quick_promote_eligibility: QuickPromoteEligibility,
}

#[derive(Debug, serde::Deserialize)]
pub struct Core {
    pub user_results: UserResult,
}

#[derive(Debug, serde::Deserialize)]
pub struct UserResult {
    pub result: UserResultValue,
}

#[derive(Debug, serde::Deserialize)]
pub struct UserResultValue {
    pub __typename: String,
    pub id: String,
    pub rest_id: String,
    pub affiliates_highlighted_label: AffiliatesHighlightedLabel,
    pub is_blue_verified: bool,
    pub legacy: LegacyUser,
    pub professional: Option<Professional>,
    pub business_account: BusinessAccount,
}

#[derive(Debug, serde::Deserialize)]
pub struct LegacyUser {
    pub created_at: String,
    pub default_profile: bool,
    pub default_profile_image: bool,
    pub description: String,
    pub entities: LegacyUserEntities,

    pub fast_followers_count: u64,
    pub favourites_count: u64,
    pub followers_count: u64,
    pub friends_count: u64,
    pub has_custom_timelines: bool,
    pub is_translator: bool,
    pub listed_count: u64,
    pub location: String,
    pub media_count: u64,
    pub name: String,
    pub normal_followers_count: u64,
    pub pinned_tweet_ids_str: Vec<String>,
    pub possibly_sensitive: bool,
    pub profile_banner_url: Option<String>,
    pub profile_image_url_https: String,
    pub profile_interstitial_type: String,
    pub screen_name: String,
    pub statuses_count: u64,
    pub translator_type: String,
    pub url: Option<String>,
    pub verified: bool,
    pub withheld_in_countries: Vec<String>, // vector type not sure
}

#[derive(Debug, serde::Deserialize)]
pub struct LegacyUserEntities {
    pub description: EntityDescription,
    pub url: Option<EntityUrl>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EntityDescription {
    pub urls: Vec<DescriptionUrl>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EntityUrl {
    pub urls: Vec<DescriptionUrl>, // vector type not sure
}

#[derive(Debug, serde::Deserialize)]
pub struct DescriptionUrl {
    pub display_url: String,
    pub expanded_url: String,
    pub url: String,
    pub indices: [u32; 2],
}

#[derive(Debug, serde::Deserialize)]
pub struct AffiliatesHighlightedLabel {
    // add fields
}

#[derive(Debug, serde::Deserialize)]
pub struct Professional {
    pub rest_id: String,
    pub professional_type: String,
    pub category: Vec<Category>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub icon_name: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct BusinessAccount {
    // add fields
}

#[derive(Debug, serde::Deserialize)]
pub struct UnmentionData {
    // add fields
}

#[derive(Debug, serde::Deserialize)]
pub struct Views {
    pub count: Option<String>,
    pub state: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Legacy {
    pub bookmarked: bool,
    pub created_at: String,
    pub conversation_id_str: String,
    pub display_text_range: [u32; 2],
    pub entities: LegacyEntity,
    pub extended_entities: Option<ExtendedLegacyEntity>,
    pub favorite_count: u64,
    pub favorited: bool,
    pub full_text: String,
    pub is_quote_status: bool,
    pub lang: String,

    pub possibly_sensitive: Option<bool>,
    pub possibly_sensitive_editable: Option<bool>,
    pub quote_count: u64,
    pub reply_count: u64,
    pub retweet_count: u64,
    pub retweeted: bool,
    pub user_id_str: String,
    pub id_str: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct LegacyEntity {
    pub media: Option<Vec<MediaEntity>>,
    pub user_mentions: Vec<UserMention>,
    pub urls: Vec<LegacyEntityUrl>,
    pub hashtags: Vec<Hashtag>,
    pub symbols: Vec<Symbol>,
}
#[derive(Debug, serde::Deserialize)]
pub struct LegacyEntityUrl {
    pub display_url: String,
    pub expanded_url: String,
    pub indices: [usize; 2],
}

#[derive(Debug, serde::Deserialize)]
pub struct Symbol {
    pub text: String,
    pub indices: [usize; 2],
}

#[derive(Debug, serde::Deserialize)]
pub struct UserMention {
    pub id_str: String,
    pub name: String,
    pub screen_name: String,
    pub indices: [usize; 2],
}
#[derive(Debug, serde::Deserialize)]
pub struct Hashtag {
    pub text: String,
    pub indices: [usize; 2],
}

#[derive(Debug, serde::Deserialize)]
pub struct ExtendedLegacyEntity {
    pub media: Vec<MediaEntity>,
}

#[derive(Debug, serde::Deserialize)]
pub struct MediaEntity {
    pub display_url: String,
    pub expanded_url: String,
    pub id_str: String,
    pub indices: [u32; 2],
    pub media_key: String,
    pub media_url_https: String,

    #[serde(rename = "type")]
    pub _type: MediaType,

    pub url: String,
    pub additional_media_info: Option<AdditionalMediaInfo>,

    pub ext_media_availability: ExtMediaAvailability,
    pub sizes: MediaSizes,

    pub features: Option<MediaFeatures>,

    pub original_info: OriginalInfo,
    pub video_info: Option<VideoInfo>,
}

#[derive(Debug, serde::Deserialize)]
pub enum MediaType {
    #[serde(rename = "photo")]
    Image,

    #[serde(rename = "video")]
    Video,

    #[serde(rename = "animated_gif")]
    Gif,
}

#[derive(Debug, serde::Deserialize)]
pub struct MediaFeatures {
    pub large: FeatureFace,
    pub medium: FeatureFace,
    pub small: FeatureFace,
    pub orig: FeatureFace,
}

#[derive(Debug, serde::Deserialize)]
pub struct FeatureFace {
    pub faces: Vec<FocusRect>,
}

#[derive(Debug, serde::Deserialize)]
pub struct AdditionalMediaInfo {
    pub monetizable: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct OriginalInfo {
    pub height: usize,
    pub width: usize,
    pub focus_rects: Vec<FocusRect>, //todo! not sure the type of the vec
}

#[derive(Debug, serde::Deserialize)]
pub struct FocusRect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

#[derive(Debug, serde::Deserialize)]
pub struct MediaSizes {
    pub large: MediaSize,
    pub medium: MediaSize,
    pub small: MediaSize,
    pub thumb: MediaSize,
}

#[derive(Debug, serde::Deserialize)]
pub struct MediaSize {
    pub h: usize,
    pub w: usize,
    pub resize: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ExtMediaAvailability {
    pub status: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct VideoInfo {
    pub aspect_ratio: Vec<f32>,
    pub duration_millis: Option<u32>,
    pub variants: Vec<VideoVariant>,
}

#[derive(Debug, serde::Deserialize)]
pub struct VideoVariant {
    pub content_type: String,
    pub url: String,
    pub bitrate: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct QuickPromoteEligibility {
    pub eligibility: String,
}
