use std::time::Instant;

use downloading::prelude::*;

#[tokio::main]
async fn main() -> Result<(), DownloadError> {
    let link = std::env::args()
        .nth(1)
        .expect("expected a link passed as argument");

    let downloader = TwitterDownloader::new(&link)?;

    let start = Instant::now();
    downloader.download().await?;

    println!("it took {} seconds!", start.elapsed().as_secs_f64());

    Ok(())
}
