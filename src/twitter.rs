use crate::{
    prelude::{DownloadError, Downloader},
    twitter::utils::{retrieve_request_details, ErrorResponse},
};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response, Url,
};

mod details;
mod utils;
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
/// The `TwitterDownloader` is simply an implementation of [twitter_video-dl](https://github.com/inteoryx/twitter-video-dl.git) repository ported to rust!
pub struct TwitterDownloader {
    url: Url,
    tweet_id: String,
    status_id: String,
}

use self::{details::TweetDetails, utils::RequestDetails};
use serde::{de, Deserialize};

#[derive(Debug, Deserialize)]
struct GuestTokenResponse {
    guest_token: String,
}

impl TwitterDownloader {
    pub fn new(link: &str) -> Result<Self, DownloadError> {
        let url = Self::parse_url(
            link,
            Some("https://www.twitter.com/<USERNAME>/status/<TWEET_ID>"),
        )?;

        if url.host() != Some(url::Host::Domain("twitter.com"))
            && url.host() != Some(url::Host::Domain("x.com"))
        {
            return Err(DownloadError::InvalidUrl(
                "Invalid URL! The domain must be either 'www.twitter.com' or 'www.x.com'."
                    .to_owned(),
            ));
        }

        let pattern = r"https://(twitter|x)\.com/([^/]+)/status/(\d+)";
        let url_regex = Regex::new(pattern).unwrap();

        if let Some(captures) = url_regex.captures(url.as_str()) {
            if let (Some(status_id), Some(tweet_id)) = (captures.get(2), captures.get(3)) {
                let status_id = status_id.as_str().to_owned();
                let tweet_id = tweet_id.as_str().to_owned();
                return Ok(Self {
                    url,
                    status_id,
                    tweet_id,
                });
            }
        }

        Err(DownloadError::TwitterError(
            "Could not parse status_id and tweet_id from this tweet url.".to_owned(),
        ))
    }

    pub fn status_id(&self) -> &str {
        &self.status_id
    }
    pub fn tweet_id(&self) -> &str {
        &self.tweet_id
    }

    async fn get_mainjs_url(&self) -> Result<String, DownloadError> {
        let response = reqwest::get(self.url.as_str()).await?;

        if response.status() != 200 {
            return Err(DownloadError::TwitterError(
                "Failed to get tweet page! Check if the url is valid!".to_owned(),
            ));
        }
        let content = response.text().await?;

        let main_js_regex =
            Regex::new(r"https://abs.twimg.com/responsive-web/client-web-legacy/main\.[^.]+\.js")
                .unwrap();
        let mainjs_urls: Vec<&str> = main_js_regex
            .find_iter(&content)
            .map(|mat| mat.as_str())
            .collect();

        if mainjs_urls.is_empty() {
            return Err(DownloadError::TwitterError(
                "Failed to retrieve `main.js` file from the page.".to_owned(),
            ));
        }

        Ok(mainjs_urls[0].to_owned())
    }
    async fn get_bearer_token(&self, mainjs_url: &str) -> Result<String, DownloadError> {
        let main_js_response = reqwest::get(mainjs_url).await?;

        if main_js_response.status() != 200 {
            return Err(DownloadError::TwitterError(
                "Failed to retrieve `main.js` file from the page.".to_owned(),
            ));
        }
        let main_js_content = main_js_response.text().await?;

        let bearer_regex = Regex::new(r#"AAAAAAAAA[^\"']+"#).unwrap();
        let bearer_tokens: Vec<&str> = bearer_regex
            .find_iter(&main_js_content)
            .map(|mat| mat.as_str())
            .collect();

        if bearer_tokens.is_empty() {
            return Err(DownloadError::TwitterError(
                "Failed to find bearer token.".to_owned(),
            ));
        }

        let bearer_token = bearer_tokens[0];

        Ok(bearer_token.to_owned())
    }
    async fn get_guest_token(&self, bearer_token: &str) -> Result<String, DownloadError> {
        let client = Client::new();

        let mut headers = HeaderMap::new();

        headers.insert(
            "user-agent",
            HeaderValue::from_static(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:84.0) Gecko/20100101 Firefox/84.0",
            ),
        );
        headers.insert("accept", HeaderValue::from_static("*/*"));
        headers.insert(
            "accept-language",
            HeaderValue::from_static("fr,en-US;q=0.7,en;q=0.3"),
        );
        headers.insert("te", HeaderValue::from_static("trailers"));
        headers.insert(
            "authorization",
            HeaderValue::from_bytes(format!("Bearer {}", bearer_token).as_bytes())
                .expect("Failed to create HeaderValue"),
        );

        let response = client
            .post("https://api.twitter.com/1.1/guest/activate.json")
            .headers(headers)
            .send()
            .await?;

        if response.status() != 200 {
            return Err(DownloadError::TwitterError(
                "Failed to find guest token.".to_owned(),
            ));
        }

        let body = response.text().await?;

        let token_response: GuestTokenResponse = serde_json::from_str(&body)
            .map_err(|_| DownloadError::TwitterError("Failed to find guest token.".to_owned()))?;

        Ok(token_response.guest_token)
    }

    pub async fn get_tokens(&self) -> Result<(String, String), DownloadError> {
        let mainjs_url = self.get_mainjs_url().await?;
        let bearer_token = self.get_bearer_token(&mainjs_url).await?;
        let guest_token = self.get_guest_token(&bearer_token).await?;

        Ok((bearer_token, guest_token))
    }

    async fn get_details_url(&self) -> Result<String, DownloadError> {
        let RequestDetails {
            mut variables,
            features,
        } = retrieve_request_details().await?;

        variables.set_tweet_id(self.tweet_id().to_owned());

        // Features and Variables structs serialized
        let features_string = serde_json::to_string(&features).unwrap();
        let variables_string = serde_json::to_string(&variables).unwrap();

        // URL-encode the JSON string
        let features_encoded = utf8_percent_encode(&features_string, NON_ALPHANUMERIC).to_string();
        let variables_encoded =
            utf8_percent_encode(&variables_string, NON_ALPHANUMERIC).to_string();

        let url = format!("https://twitter.com/i/api/graphql/ncDeACNGIApPMaqGVuF_rw/TweetResultByRestId?variables={}&features={}", variables_encoded, features_encoded);

        Ok(url)
    }

    async fn details_req(
        &self,
        bearer_token: &str,
        guest_token: &str,
    ) -> Result<Response, DownloadError> {
        let url = self.get_details_url().await?;

        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            "authorization",
            HeaderValue::from_str(&format!("Bearer {}", bearer_token))
                .expect("Failed to create HeaderValue"),
        );
        headers.insert(
            "x-guest-token",
            HeaderValue::from_str(guest_token).expect("Failed to create HeaderValue"),
        );

        let details = client.get(url).headers(headers).send().await?;

        Ok(details)
    }

    async fn get_tweet_details(
        &self,
        bearer_token: &str,
        guest_token: &str,
    ) -> Result<TweetDetails, DownloadError> {
        let mut details = self.details_req(bearer_token, guest_token).await?;

        let mut try_count = 1;
        let max_tries = 11;

        // need to update the loop to automatically add new variables if needed when the variables change server side
        loop {
            if details.status() != 400 || try_count >= max_tries {
                break;
            }

            let details_status = details.status();
            let details_text = details.text().await?;
            println!("status {}, text: {}", details_status, details_text);

            let err_content: ErrorResponse = serde_json::from_str(&details_text).map_err(|_| {
                DownloadError::TwitterError(format!(
                    "Failed to parse json from details error, details text: `{details_text}`"
                ))
            })?;

            println!("{:?}", err_content);

            let needed_variable_pattern = Regex::new(r"Variable '([^']+)'").unwrap();
            let needed_features_pattern =
                Regex::new(r#"The following features cannot be null: ([^"]+)"#).unwrap();

            details = self.details_req(bearer_token, guest_token).await?;
            try_count += 1;
        }

        if details.status() != 200 {
            return Err(DownloadError::TwitterError(
                "Failed to get tweet details.".to_owned(),
            ));
        }

        let response_text = details.text().await?;
        let tweet_details = serde_json::from_str(&response_text).map_err(|e| {
            println!("{}", response_text);
            println!("{:?}", e);
            DownloadError::TwitterError("Failed to parse tweet details.".to_owned())
        })?;

        Ok(tweet_details)
    }
}

impl Downloader for TwitterDownloader {
    async fn download(&self) -> Result<(), DownloadError> {
        let (bearer_token, guest_token) = self.get_tokens().await?;

        let tweet_details = self.get_tweet_details(&bearer_token, &guest_token).await?;

        // medias contain all the informations regarding the tweet videos and images
        let medias = tweet_details.data.tweet_result.result.legacy.entities.media;

        Ok(())
    }
}
