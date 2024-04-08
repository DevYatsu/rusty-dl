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
        let video_name = downloader
            .get_video()?
            .get_basic_info()
            .await?
            .video_details
            .title;

        downloader
            .download_to(
                Path::new("./videos/")
                    .join(video_name)
                    .with_extension("mp4"),
            )
            .await
    }))
    .await;

    for result in results {
        result?
    }

    assert_folder_len("./videos/", 2)?;

    println!("Download finished!");
    println!("it took {} seconds!", start.elapsed().as_secs_f64());
    Ok(())
}

#[tokio::test]
async fn youtube_basic() -> Result<(), DownloadError> {
    let start = tokio::time::Instant::now();
    println!("Downloading...");

    let content = tokio::fs::read_to_string("test.youtube").await?;

    let mut lines = content.lines();

    let downloader = YoutubeDownloader::new(lines.next().unwrap()).unwrap();
    let result = downloader.download().await;

    assert!(result.is_ok());

    println!("Download finished!");
    println!("it took {} seconds!", start.elapsed().as_secs_f64());
    Ok(())
}

fn assert_folder_len(name: &str, len: usize) -> Result<(), std::io::Error> {
    assert_eq!(std::fs::read_dir(std::path::Path::new(name))?.count(), len);

    Ok(())
}
