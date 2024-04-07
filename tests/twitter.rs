use rusty_dl::prelude::*;

#[tokio::test]
async fn twitter() -> Result<(), DownloadError> {
    use std::path::Path;
    let start = tokio::time::Instant::now();

    let content = tokio::fs::read_to_string("test.twitter").await?;

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

    rusty_dl::test::assert_folder_len("./tweets/", 10)?;

    Ok(())
}
