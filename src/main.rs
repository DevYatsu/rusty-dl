use std::{fs::File, io::Write, path::Path};

use rusty_dl::prelude::*;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), DownloadError> {
    let link = std::env::args()
        .nth(1)
        .expect("expected a link passed as argument");

    let start = tokio::time::Instant::now();
    println!("Downloading...");

    let url = Url::parse(&link)?;

    match url {
        link if TwitterDownloader::is_valid_url(&url) => {
            let mut downloader = TwitterDownloader::new(link.as_str())?;
            downloader.only_images().download().await?;
        }
        link if YoutubeDownloader::is_valid_url(&url) => {
            let downloader = YoutubeDownloader::new(link.as_str())?;
            downloader.download().await?;
        }
        _ => {
            let downloader = ResourceDownloader::new(link.as_str())?;
            downloader.download().await?;
        }
    };

    println!("Downloading finished!");
    println!("it took {} seconds!", start.elapsed().as_secs_f64());

    Ok(())
}
