use downloading::prelude::*;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), DownloadError> {
    // let link = std::env::args()
    //     .nth(1)
    //     .expect("expected a link passed as argument");

    let start = tokio::time::Instant::now();

    let content = tokio::fs::read_to_string("test.youtube").await?;

    let lines = content.lines();

    let results = futures::future::join_all(lines.into_iter().map(|line| async move {
        let downloader = YoutubeDownloader::new(line.trim())?;
        downloader.download_to(Path::new("./videos/")).await
    }))
    .await;

    for result in results {
        result?
    }

    println!("it took {} seconds!", start.elapsed().as_secs_f64());

    Ok(())
}
