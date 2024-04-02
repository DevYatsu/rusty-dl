use std::{path::Path, time::Instant};

use downloading::prelude::*;

#[tokio::main]
async fn main() -> Result<(), DownloadError> {
    let link = std::env::args()
        .nth(1)
        .expect("expected a link passed as argument");

    let downloader = YoutubeDownloader::new(&link)?;

    let start = Instant::now();

    downloader.download_to(&Path::new("./src")).await?;

    println!("it took {} seconds!", start.elapsed().as_secs_f64());

    Ok(())
}
