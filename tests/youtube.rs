use rusty_dl::prelude::*;

#[tokio::test]
async fn youtube() -> Result<(), DownloadError> {
    use std::path::Path;
    let start = tokio::time::Instant::now();
    println!("Downloading...");

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

    println!("Downloading finished!");
    println!("it took {} seconds!", start.elapsed().as_secs_f64());

    rusty_dl::test::assert_folder_len("./videos/", 2)?;

    Ok(())
}
