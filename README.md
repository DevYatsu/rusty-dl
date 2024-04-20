# rusty_dl

[![crates.io Version](https://img.shields.io/crates/v/rusty_dl.svg)](https://crates.io/crates/rusty_dl)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.65.0+-lightgray.svg)](#rust-version-requirements)

Rusty_dl Library is a versatile crate designed for effortlessly fetching a wide range of content types, including YouTube videos, tweet media (such as videos, images, and gifs), and eventually various other media directly from the web.

## Features

- Download YouTube videos
- Download tweet media (videos, images, gifs)
- Download other media from the web (through url fetching)

## Todo

### In the Library

- [x] Add features to download content to specific directories
- [x] Implement functions to retrieve metadata on downloaded media
- [x] Allow users to specify whether spaces in file names should be replaced by underscores
- [x] Add support for downloading images-only or videos-only from tweets
- [x] Add support for downloading playlists on youtube (only the 100 videos coming first)
- [x] Add examples

### Other

- [ ] Create a CLI for easy downloading from the terminal
- [ ] Develop a desktop application (using Tauri ?)
- [ ] Build a web application for downloading media from the browser

## Get started

- Add the crate to your project

```bash
cargo add rusty_dl
```

- Use the crate in a synchronous environment

```rust
use rusty_dl::prelude::{Downloader, YoutubeDownloader, DownloadError};

const URL: &str = "my_yt_video_link";

fn main() -> Result<(), DownloadError>  {
    let downloader = YoutubeDownloader::new(URL);

    downloader.blocking_download()
}
```

- Use the crate in a tokio environment

```rust
use rusty_dl::prelude::{Downloader, YoutubeDownloader, DownloadError};

const URL: &str = "my_yt_video_link";

#[tokio::main]
async fn main() -> Result<(), DownloadError> {
    let downloader = YoutubeDownloader::new(URL);

    downloader.download().await
}
```

This crate also exports a TwitterDownloader and a ResourceDownloader which work the same way as presented up there.

See more in [examples directory](/examples/)

## Rust Version Requirements

As of now, this crate requires **Rustc version 1.65** or later. However, there are plans to lower this requirement in upcoming releases.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve this library.

## License

This project is licensed under the [MIT License](./LICENSE).
