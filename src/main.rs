use rusty_dl::prelude::*;

#[tokio::main]
// #[cfg(feature = "all")]
async fn main() -> Result<(), DownloadError> {
    // let link = std::env::args()
    //     .nth(1)
    //     .expect("expected a link passed as argument");

    let start = tokio::time::Instant::now();

    println!("Downloading finished!");
    println!("it took {} seconds!", start.elapsed().as_secs_f64());

    Ok(())
}

// #[cfg(not(feature = "dev"))]
// fn main() {

// }
