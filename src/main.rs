use rusty_dl::{prelude::*, twitter::TwitterMedia};
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

            downloader
                .print_dl_status()
                .set_name_callback(names_callback)
                .download()
                .await?;
        }
        link if YoutubeDownloader::is_valid_url(&url) => {
            let mut downloader = YoutubeDownloader::new(link.as_str())?;
            downloader.print_dl_status().download().await?;
        }
        _ => {
            let mut downloader = ResourceDownloader::new(link.as_str())?;
            downloader.print_dl_status().download().await?;
        }
    };

    println!("Downloading finished!");
    println!("it took {} seconds!", start.elapsed().as_secs_f64());

    Ok(())
}

fn names_callback(_index: usize, media: TwitterMedia) -> String {
    let extension = media.extension();

    let filename = match media {
        TwitterMedia::Video { infos } => {
            let variant = infos
                .variants
                .iter()
                .max_by_key(|k| k.bitrate.unwrap_or(0))
                .unwrap();
            let url = variant.url.replace("/", "_");
            extension.map_or_else(
                || format!("{}", url),
                |ext| format!("{}.{}", url, ext.to_string_lossy()),
            )
        }
        TwitterMedia::Image { url } => extension.map_or_else(
            || format!("{}", url.replace("/", "_")),
            |ext| format!("{}.{}", url.replace("/", "_"), ext.to_string_lossy()),
        ),
    };

    filename
}
