use std::time::Instant;

use downloading::prelude::*;

#[tokio::main]
async fn main() -> Result<(), DownloadError> {
    let start = Instant::now();
    let link = std::env::args()
        .nth(1)
        .expect("expected a link passed as argument");

    let downloader = YoutubeDownloader::new(&link)?;
    downloader.download().await?;

    println!("it took {} seconds!", start.elapsed().as_secs_f64());

    Ok(())
}
