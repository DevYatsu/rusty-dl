use std::{ffi::OsStr, path::Path};

use self::{
    details::{MediaEntity, TweetDetails, VideoInfo},
    utils::RequestDetails,
};
use crate::{
    header::HeaderMapBuilder,
    prelude::{DownloadError, Downloader},
    resource::ResourceDownloader,
    twitter::{details::MediaType, utils::retrieve_request_details},
};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use regex::Regex;
use reqwest::{header::HeaderValue, Client, Response, Url};
use serde::Deserialize;

mod details;
pub mod utils;

/*
THIS MESSAGE IS COPY-PASTE FROM `https://github.com/inteoryx/twitter-video-dl.git` repository from which this [`TwitterDownloader`] is an implementation of.

Here's how this works:
1. To download a video you need a Bearer Token and a guest token.  The guest token definitely expires and the Bearer Token could, though in practice I don't think it does.
2. Use the video id get both of those as if you were an unauthenticated browser.
3. Call "TweetDetails" graphql endpoint with your tokens.
4. TweetDetails response includes a 'medias' key which is a list of video urls and details.  Find the one with the highest bitrate (bigger is better, right?) and then just download that.
5. Some videos are small.  They are contained in a single mp4 file.  Other videos are big.  They have an mp4 file that's a "container" and then a bunch of m4s files.  Once we know the name of the video file we are looking for we can look up what the m4s files are, download all of them, and then put them all together into one big file.  This currently all happens in memory.  I would guess that a very huge video might cause an out of memory error.  I don't know, I haven't tried it.
5. If it's broken, fix it yourself because I'm very slow.  Or, hey, let me know, but I might not reply for months.


Current state of work:

Currently the "TweetDetails" endpoint is https://twitter.com/i/api/graphql/ncDeACNGIApPMaqGVuF_rw/TweetResultByRestId?variables={}&features={}

Once we have both tokens, we generate the URL with all the variables and features, send a request to the endpoint
with headers containing our tokens, retrieve the "TweetDetails,":

now we need to extract the media download links, and finally download them!

IN THE FUTURE:
we should do the same as in the python version, that is whenever new variables and features are add, the program detects it and add them in the RequestDetails.json
or maybe not because we do not want the crate to depend on any exterior file, that implies we should get rid of the json
*/

/// The `TwitterDownloader` is a Rust implementation inspired by the functionality of the `twitter-video-dl` python repository.
#[derive(Clone)]
pub struct TwitterDownloader {
    /// The URL of the Twitter content.
    url: Url,
    /// The ID of the tweet.
    tweet_id: String,
    /// The status ID.
    status_id: String,
    /// Specifies the kind of media to download.
    only_media_kind: Option<MediaKind>,
    /// Callback function for naming downloaded media.
    names_callback: fn(usize, TwitterMedia) -> String,
    /// The name that should be given to all downloaded files
    name_all: Option<String>,
    /// The name that should be given to the downloaded file if there is only one
    name_if_only_one_file: Option<String>,

    print_download_status: bool,
}

/// Represents the kind of media to download from Twitter.
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub enum MediaKind {
    /// Indicates an image.
    Image,
    /// Indicates a video.
    Video,
}

#[derive(Debug, Deserialize)]
struct GuestTokenResponse {
    /// The guest token for accessing Twitter content.
    guest_token: String,
}

impl TwitterDownloader {
    /// Creates a new instance of [`TwitterDownloader`] with the provided Twitter tweet link.
    ///
    /// ## Arguments
    ///
    /// * `link` - The Twitter tweet link to download.
    ///
    /// ## Returns
    ///
    /// Returns a [`Result`] containing the [`TwitterDownloader`] instance on success, or a [`DownloadError`] if parsing the URL fails or if the URL is invalid.
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// use rusty_dl::prelude::TwitterDownloader;
    ///
    /// let link = "https://twitter.com/user/status/123456789";
    /// let downloader = TwitterDownloader::new(link);
    ///
    /// assert!(downloader.is_ok());
    /// ```    
    pub fn new(link: &str) -> Result<Self, DownloadError> {
        let url = Self::parse_url(
            link,
            Some("https://www.twitter.com/<USERNAME>/status/<TWEET_ID>"),
        )?;

        if !Self::is_valid_url(&url) {
            return Err(DownloadError::InvalidUrl(
                "Invalid URL! The domain must be either 'www.twitter.com' or 'www.x.com'."
                    .to_owned(),
            ));
        }

        let (status_id, tweet_id) = Self::extract_ids_from_url(&url)?;

        Ok(Self {
            url,
            status_id,
            tweet_id,
            only_media_kind: None,
            names_callback: |index: usize, media: TwitterMedia| {
                let extension = media.extension();

                let filename = extension.map_or_else(
                    || format!("{}", index + 1),
                    |ext| format!("{}.{}", index + 1, ext.to_string_lossy()),
                );

                filename
            },
            name_all: None,
            name_if_only_one_file: None,
            print_download_status: false,
        })
    }

    /// Define a callback function to generate file names from.
    ///
    /// ## Arguments
    ///
    /// * `callback` - A function that takes a [`TwitterMedia`] instance and returns a [`String`].
    ///
    /// ## Returns
    ///
    /// Returns a mutable reference to the modified [`TwitterDownloader`]
    ///
    /// ## Examples
    ///
    /// ```
    /// use rusty_dl::prelude::TwitterDownloader;
    ///
    /// let mut downloader = TwitterDownloader::new("https://twitter.com/user/status/123456789").unwrap();
    ///
    /// downloader.set_name_callback(|index, media| {
    ///     format!("tweet_{}_{}", index + 1, media.extension().unwrap_or_default().to_string_lossy())
    /// });
    /// ```
    pub fn set_name_callback(&mut self, callback: fn(usize, TwitterMedia) -> String) -> &mut Self {
        self.names_callback = callback;

        self
    }

    /// Set a given name for all the downloaded file.
    ///
    /// **THIS FUNCTION TAKES PRECEDENCE OVER `set_name_callback`.**
    pub fn name_all(&mut self, value: String) -> &mut Self {
        self.name_all = Some(value);

        self
    }

    /// Set a given name for a downloaded media if the tweet only contains one media.
    ///
    /// **THIS FUNCTION TAKES PRECEDENCE OVER `set_name_callback` and `name_all`.**
    pub fn name_if_only_file(&mut self, value: String) -> &mut Self {
        self.name_if_only_one_file = Some(value);

        self
    }

    /// Retrieves the media entities associated with the Twitter tweet.
    ///
    /// This method asynchronously fetches and returns the media entities (such as videos and images) associated with the Twitter tweet.
    ///
    /// ## Returns
    ///
    /// Returns a [`Result`] containing a vector of [`MediaEntity]` instances on success, or a [`DownloadError`] if the retrieval fails.
    async fn get_tweet_medias(&self) -> Result<Vec<MediaEntity>, DownloadError> {
        let (bearer_token, guest_token) = self.get_tokens().await?;

        let tweet_details = self.get_tweet_details(&bearer_token, &guest_token).await?;

        // medias contain all the informations regarding the tweet videos and images
        let opt_medias = tweet_details.data.tweet_result.result.legacy.entities.media;

        let medias = opt_medias.ok_or_else(|| {
            DownloadError::TwitterError(format!(
                "The tweet with ID `{}` does not contain any associated media.",
                self.tweet_id()
            ))
        })?;

        Ok(medias)
    }

    /// Extracts the status ID and tweet ID from the Twitter tweet URL.
    fn extract_ids_from_url(url: &Url) -> Result<(String, String), DownloadError> {
        let pattern = r"https://(twitter|x)\.com/([^/]+)/status/(\d+)";
        let url_regex = Regex::new(pattern).unwrap();

        if let Some(captures) = url_regex.captures(url.as_str()) {
            if let (Some(status_id), Some(tweet_id)) = (captures.get(2), captures.get(3)) {
                return Ok((status_id.as_str().to_owned(), tweet_id.as_str().to_owned()));
            }
        }

        Err(DownloadError::TwitterError(format!(
            "Failed to parse status_id and tweet_id from the tweet URL: `{}`",
            url
        )))
    }

    /// Sets the downloader to download only images from the Twitter tweet.
    pub fn only_images(&mut self) -> &mut Self {
        self.only_media_kind = Some(MediaKind::Image);
        self
    }

    /// Sets the downloader to download only videos from the Twitter tweet.
    pub fn only_videos(&mut self) -> &mut Self {
        self.only_media_kind = Some(MediaKind::Video);
        self
    }

    /// Returns the status ID of the Twitter tweet.
    pub fn status_id(&self) -> &str {
        &self.status_id
    }
    /// Returns the tweet ID of the Twitter tweet.
    pub fn tweet_id(&self) -> &str {
        &self.tweet_id
    }
    /// Returns the URL of the Twitter tweet.
    pub fn url_str(&self) -> &str {
        self.url.as_str()
    }

    /// Fetches the content of the Twitter tweet page asynchronously.
    async fn fetch_page_content(url: &str) -> Result<String, DownloadError> {
        let response = reqwest::get(url).await?;

        if !response.status().is_success() {
            return Err(DownloadError::TwitterError(format!(
                "Failed to fetch content from URL: {}",
                url
            )));
        }

        response.text().await.map_err(|_| {
            DownloadError::TwitterError(format!("Failed to read text from URL: {}", url))
        })
    }

    /// Asynchronously retrieves the URL of the main JavaScript file from the Twitter tweet page.
    async fn get_mainjs_url(&self) -> Result<String, DownloadError> {
        let content = Self::fetch_page_content(self.url_str()).await?;

        let main_js_regex =
            Regex::new(r"https://abs.twimg.com/responsive-web/client-web-legacy/main\.[^.]+\.js")
                .unwrap();
        let mainjs_urls: Vec<&str> = main_js_regex
            .find_iter(&content)
            .map(|mat| mat.as_str())
            .collect();

        if mainjs_urls.is_empty() {
            return Err(DownloadError::TwitterError(format!(
                "Failed to retrieve `main.js` file from `{}` page.",
                self.url
            )));
        }

        Ok(mainjs_urls[0].to_owned())
    }

    /// Asynchronously retrieves the bearer token from the main JavaScript file URL.
    async fn get_bearer_token(&self, mainjs_url: &str) -> Result<String, DownloadError> {
        let main_js_content = Self::fetch_page_content(mainjs_url).await?;

        let bearer_regex = Regex::new(r#"AAAAAAAAA[^\"']+"#).unwrap();
        let bearer_tokens: Vec<&str> = bearer_regex
            .find_iter(&main_js_content)
            .map(|mat| mat.as_str())
            .collect();

        if bearer_tokens.is_empty() {
            return Err(DownloadError::TwitterError(format!(
                "Failed to find bearer token from `{}` page",
                self.url
            )));
        }

        let bearer_token = bearer_tokens[0];

        Ok(bearer_token.to_owned())
    }

    /// Asynchronously retrieves the guest token using the provided bearer token.
    async fn get_guest_token(&self, bearer_token: &str) -> Result<String, DownloadError> {
        let client = Client::new();

        let headers = HeaderMapBuilder::new()
            .with_user_agent()
            .accept("*/*")
            .accept_language("fr,en-US;q=0.7,en;q=0.3")
            .te("trailers")
            .authorization(
                HeaderValue::from_bytes(format!("Bearer {}", bearer_token).as_bytes())
                    .expect("Failed to create HeaderValue"),
            )
            .build();

        let body = client
            .post("https://api.twitter.com/1.1/guest/activate.json")
            .headers(headers)
            .send()
            .await
            .map(|res| {
                if !res.status().is_success() {
                    return Err(DownloadError::TwitterError(format!(
                        "Failed to find guest token from `{}` page",
                        self.url
                    )));
                }

                Ok(res)
            })??
            .text()
            .await?;

        serde_json::from_str::<GuestTokenResponse>(&body)
            .map(|token_response| token_response.guest_token)
            .map_err(|_| {
                DownloadError::TwitterError(format!(
                    "Failed to find guest token from `{}` page",
                    self.url
                ))
            })
    }

    /// Asynchronously retrieves the bearer and guest tokens required for retrieving the tweet data next.
    async fn get_tokens(&self) -> Result<(String, String), DownloadError> {
        let mainjs_url = self.get_mainjs_url().await?;
        let bearer_token = self.get_bearer_token(&mainjs_url).await?;
        let guest_token = self.get_guest_token(&bearer_token).await?;

        Ok((bearer_token, guest_token))
    }

    /// Asynchronously constructs the URL for retrieving tweet details.
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

    /// Asynchronously sends a request to retrieve tweet details using bearer and guest tokens.
    async fn retrieve_details(
        &self,
        bearer_token: &str,
        guest_token: &str,
    ) -> Result<Response, DownloadError> {
        let url = self.get_details_url().await?;
        let client = Client::new();

        let headers = HeaderMapBuilder::new()
            .with_user_agent()
            .accept("*/*")
            .accept_language("fr,en-US;q=0.7,en;q=0.3")
            .te("trailers")
            .authorization(
                HeaderValue::from_bytes(format!("Bearer {}", bearer_token).as_bytes())
                    .expect("Failed to create HeaderValue"),
            )
            .field(
                "x-guest-token",
                HeaderValue::from_str(guest_token).expect("Failed to create HeaderValue"),
            )
            .build();

        let details = client.get(url).headers(headers).send().await?;
        Ok(details)
    }

    /// Asynchronously retrieves tweet details using bearer and guest tokens.
    async fn get_tweet_details(
        &self,
        bearer_token: &str,
        guest_token: &str,
    ) -> Result<TweetDetails, DownloadError> {
        let details = self.retrieve_details(bearer_token, guest_token).await?;

        // let mut try_count = 1;
        // let max_tries = 11;
        // should we update the loop to automatically add new variables if needed when the variables changes server side ??

        if !details.status().is_success() {
            return Err(DownloadError::TwitterError(format!(
                "Failed to get details of tweet with id `{}`",
                self.tweet_id()
            )));
        }

        let response_text = details.text().await?;
        let tweet_details = serde_json::from_str(&response_text).map_err(|_e| {
            DownloadError::TwitterError("Failed to parse tweet details.".to_owned())
        })?;

        Ok(tweet_details)
    }

    // actually these are useless, we can just use `download_to`!!
    // /// Downloads the Twitter tweet media and saves it to the specified folder with the tweet ID as the file name.
    // ///
    // /// ## Arguments
    // ///
    // /// * `path` - The path to the folder where the media will be downloaded.
    // ///
    // /// ## Returns
    // ///
    // /// Returns a [`Result`] indicating success or failure of the download operation.
    // pub async fn download_as_tweets_folder_to<P: AsRef<std::path::Path> + Send>(
    //     &self,
    //     path: P,
    // ) -> Result<(), DownloadError> {
    //     let folder_path = path.as_ref();
    //     let path_buf = folder_path.join(self.tweet_id());

    //     self.download_to(path_buf).await
    // }

    // /// Blocks the current thread until the Twitter tweet media is downloaded and saved to the specified folder with the tweet ID as the file name.
    // ///
    // /// ## Arguments
    // ///
    // /// * `path` - The path to the folder where the media will be downloaded.
    // ///
    // /// ## Returns
    // ///
    // /// Returns a [`Result`] indicating success or failure of the download operation.
    // pub fn blocking_download_as_tweets_folder_to(
    //     &self,
    //     path: &std::path::Path,
    // ) -> Result<(), DownloadError>
    // where
    //     Self: Sync,
    // {
    //     Self::blocking(async { self.download_as_tweets_folder_to(path).await })
    // }
}

#[async_trait::async_trait]
impl Downloader for TwitterDownloader {
    fn is_valid_url(url: &Url) -> bool {
        url.domain() == Some("twitter.com")
            || url.domain() == Some("x.com")
            || url.domain() == Some("www.twitter.com")
            || url.domain() == Some("www.x.com")
    }

    fn get_dl_status(&mut self) -> &mut bool {
        &mut self.print_download_status
    }

    async fn download_to<P: AsRef<Path> + std::marker::Send>(
        &self,
        folder_path: P,
    ) -> Result<(), DownloadError> {
        let path = folder_path.as_ref();
        let medias = self.get_tweet_medias().await?;

        let media_infos = medias
            .iter()
            .map(|media_entity| {
                media_entity.try_into().map_err(|e| {
                    DownloadError::TwitterError(format!(
                        "{} in `{}` tweet details.",
                        e,
                        self.tweet_id()
                    ))
                })
            })
            .collect::<Result<Vec<TwitterMedia>, DownloadError>>()?;

        let download_links: Vec<TwitterMedia> = media_infos
            .into_iter()
            .filter(|x| TwitterMedia::filter_media_kind(x, self.only_media_kind.as_ref()))
            .collect();

        if self.print_download_status {
            println!("Downloading...");
        }

        tokio::fs::create_dir_all(path).await?;

        let number_of_files = download_links.len();

        let results = futures::future::join_all(download_links.into_iter().enumerate().map(
            |(index, media)| async move {
                let url = media.url();

                let mut rsrc_downloader = ResourceDownloader::new(url).map_err(|_| {
                    DownloadError::TwitterError(format!("Invalid Media File path: `{}`", url))
                })?;

                let filename = if self.name_if_only_one_file.is_some() && number_of_files == 1 {
                    self.name_if_only_one_file.as_ref().unwrap().to_owned()
                } else if let Some(name) = self.name_all.as_ref() {
                    let mut s = name.to_owned();

                    if index != 0 {
                        s.push_str(&format!(" ({})", index.to_string()));
                    }

                    s
                } else {
                    (self.names_callback)(index, media)
                };

                rsrc_downloader.with_name(filename);

                let download_result = rsrc_downloader.download_to(&path).await;

                if self.print_download_status {
                    if let Err(err) = &download_result {
                        eprintln!("Error downloading with url `{}`: {:?}", url, err);
                    } else {
                        println!("Media downloaded successfully: {}", url);
                    }
                }

                download_result
            },
        ))
        .await;

        for result in results {
            result?
        }

        Ok(())
    }

    /// Downloads and saves the twitter file(s) to the current working directory.
    ///
    /// ## Returns
    ///
    /// Returns a future representing the download operation, which resolves to a [`Result`] indicating success or failure.
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// use rusty_dl::prelude::{DownloadError, Downloader, TwitterDownloader};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DownloadError> {
    ///     let downloader = TwitterDownloader::new("https://x.com/elonmusk/status/1776736700468990168").unwrap();
    ///     let result = downloader.download().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn download(&self) -> Result<(), DownloadError> {
        self.download_to("./").await
    }

    /// Blocks the current thread until the download completes, using asynchronous execution.
    ///
    /// The tweeter files are saved to the default location (`./`).
    ///
    /// ## Returns
    ///
    /// Returns a [`Result`] indicating success or failure of the download operation.
    ///
    /// ## Examples
    ///
    /// ```no_run
    /// use rusty_dl::prelude::{DownloadError, Downloader, TwitterDownloader};
    ///
    /// fn main() -> Result<(), DownloadError> {
    ///     let downloader = TwitterDownloader::new("https://x.com/SpaceX/status/1776412789768425751").unwrap();
    ///     downloader.blocking_download()?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn blocking_download(&self) -> Result<(), DownloadError>
    where
        Self: Sync,
    {
        Self::blocking(async { self.download().await })
    }

    /// Blocks the current thread until the download completes, using asynchronous execution, and saves the file at the specified path.
    ///
    /// ## Arguments
    ///
    /// * `path` - The path to the folder where the twitter files will be downloaded.
    ///
    /// ## Returns
    ///
    /// Returns a `Result` indicating success or failure of the download operation.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// use rusty_dl::prelude::{DownloadError, Downloader, TwitterDownloader};
    ///
    /// fn main() -> Result<(), DownloadError> {
    ///     let downloader = TwitterDownloader::new("https://x.com/SpaceX/status/1776412650836251053").unwrap();
    ///     downloader.blocking_download_to("./twitter_downloads/")?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn blocking_download_to<P: AsRef<Path> + std::marker::Send>(
        &self,
        path: P,
    ) -> Result<(), DownloadError>
    where
        Self: Sync,
    {
        Self::blocking(async { self.download_to(path).await })
    }
}

/// Represents a media file from Twitter, such as an image or video.
#[derive(Debug, Clone, Copy)]
pub enum TwitterMedia<'a> {
    /// Represents an image media file with its URL.
    Image { url: &'a str },

    /// Represents a video media file with its video information.
    Video { infos: &'a VideoInfo },
}

impl<'a> TwitterMedia<'a> {
    pub fn url(&self) -> &'a str {
        match self {
            TwitterMedia::Image { url } => *url,
            TwitterMedia::Video { infos, .. } => {
                let VideoInfo { variants, .. } = infos;

                // For videos, take the one with the highest bitrate, i.e., highest quality.
                let opt_variant = variants
                    .iter()
                    .max_by_key(|variant| variant.bitrate.unwrap_or(0));

                let variant = opt_variant.unwrap();
                &variant.url
            }
        }
    }

    pub fn extension(&self) -> Option<&OsStr> {
        match self {
            TwitterMedia::Image { url } => Path::new(url).extension(),
            TwitterMedia::Video { infos, .. } => {
                let VideoInfo { variants, .. } = infos;

                // For videos, take the one with the highest bitrate, i.e., highest quality.
                let opt_variant = variants
                    .iter()
                    .max_by_key(|variant| variant.bitrate.unwrap_or(0));

                let variant = opt_variant.unwrap();
                let extension = variant.content_type.split('/').nth(1);

                extension.map(|s| OsStr::new(s))
            }
        }
    }

    fn filter_media_kind(&self, media_kind: Option<&MediaKind>) -> bool {
        match media_kind {
            None => true,
            Some(_) if media_kind == Some(&MediaKind::Image) => match self {
                TwitterMedia::Image { .. } => true,
                _ => false,
            },
            Some(_) if media_kind == Some(&MediaKind::Video) => match self {
                TwitterMedia::Video { .. } => true,
                _ => false,
            },
            Some(_) => false,
        }
    }
}

impl<'a> TryFrom<&'a MediaEntity> for TwitterMedia<'a> {
    type Error = String;

    fn try_from(media_entity: &'a MediaEntity) -> Result<Self, Self::Error> {
        match media_entity._type {
            MediaType::Image => Ok(TwitterMedia::Image {
                url: &media_entity.media_url_https,
            }),
            MediaType::Video | MediaType::Gif => Ok(TwitterMedia::Video {
                infos: media_entity
                    .video_info
                    .as_ref()
                    .ok_or("Media with type video but with no video info found".to_owned())?,
            }),
        }
    }
}
