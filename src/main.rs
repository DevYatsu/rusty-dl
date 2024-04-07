use rusty_dl::prelude::*;

#[tokio::main]
async fn main() -> Result<(), DownloadError> {
    // let link = std::env::args()
    //     .nth(1)
    //     .expect("expected a link passed as argument");

    let start = tokio::time::Instant::now();

    rusty_dl::prelude::TwitterDownloader::new("https://x.com/hereliesdoa/status/1776341857834045888").unwrap().download().await?;

    println!("Downloading finished!");
    println!("it took {} seconds!", start.elapsed().as_secs_f64());

    Ok(())
}
