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
            .download_to(Path::new(&format!(
                "./tests-run/tweets/{}/",
                downloader.tweet_id()
            )))
            .await
    }))
    .await;

    for result in results {
        result?
    }

    println!("it took {} seconds!", start.elapsed().as_secs_f64());

    assert_folder_len("./tests-run/tweets/", 10)?;

    Ok(())
}

fn assert_folder_len(name: &str, len: usize) -> Result<(), std::io::Error> {
    assert_eq!(std::fs::read_dir(std::path::Path::new(name))?.count(), len);

    Ok(())
}
