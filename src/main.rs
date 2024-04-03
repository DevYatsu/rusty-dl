use downloading::prelude::*;
use std::{path::Path, time::Instant};

#[tokio::main]
async fn main() -> Result<(), DownloadError> {
    // let link = std::env::args()
    //     .nth(1)
    //     .expect("expected a link passed as argument");

    let start = Instant::now();

    let content = tokio::fs::read_to_string("test.txt").await?;

    let lines = content.lines();

    let results = futures::future::join_all(lines.into_iter().map(|line| async move {
        let downloader = TwitterDownloader::new(line.trim())?;
        downloader
            .download_as_tweets_folder_to(Path::new("./tweets/"))
            .await
    }))
    .await;

    for result in results {
        result?
    }

    println!("it took {} seconds!", start.elapsed().as_secs_f64());

    Ok(())
}
