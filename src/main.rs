use std::{fs::File, io::Write, path::Path};

use rusty_dl::prelude::*;

#[tokio::main]
async fn main() -> Result<(), DownloadError> {
    // let link = std::env::args()
    //     .nth(1)
    //     .expect("expected a link passed as argument");

    let start = tokio::time::Instant::now();
    println!("Downloading...");

    let content = tokio::fs::read_to_string("list.txt").await?;

    let mut lines = content.lines();

    // let results = futures::future::join_all(lines.into_iter().map(|line| async move {
    //     let mut downloader = YoutubeDownloader::new(line.trim())?;
    //     downloader
    //         .only_audio()
    //         .download_to(Path::new("/Users/Yanis/Downloads/musics/"))
    //         .await
    // }))
    // .await;

    // for result in results {
    //     result?
    // }

    
        let mut downloader = YoutubeDownloader::new(lines.next().unwrap().trim())?;
        downloader
            
            .download()
            .await?;

    println!("Downloading finished!");
    println!("it took {} seconds!", start.elapsed().as_secs_f64());

    Ok(())
}

// use symphonia::core::{
//     audio::{AudioBuffer, SampleBuffer, SignalSpec}, codecs::{DecoderOptions, CODEC_TYPE_NULL}, conv::IntoSample, errors::Error, formats::FormatOptions, io::MediaSourceStream, meta::MetadataOptions, probe::Hint, sample::SampleFormat
// };

// fn decode() {
//     let file = File::open(
//         "/Users/Yanis/Documents/Programming/rusty_dl/YAMÊ reprend Charles Aznavour 'La Bohème'.mp4",
//     )
//     .unwrap();

//     // Create the media source stream.
//     let mss = MediaSourceStream::new(Box::new(file), Default::default());

//     let mut hint = Hint::new();
//     hint.with_extension("mp4");

//     let meta_opts: MetadataOptions = Default::default();
//     let fmt_opts: FormatOptions = Default::default();

//     // Probe the media source.
//     let probed = symphonia::default::get_probe()
//         .format(&hint, mss, &fmt_opts, &meta_opts)
//         .expect("unsupported format");

//     // Get the instantiated format reader.
//     let mut format = probed.format;

//     // Find the first audio track with a known (decodeable) codec.
//     let track = format
//         .tracks()
//         .iter()
//         .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
//         .expect("no supported audio tracks");

//     // Use the default options for the decoder.
//     let dec_opts: DecoderOptions = Default::default();

//     // Create a decoder for the track.
//     let mut decoder = symphonia::default::get_codecs()
//         .make(&track.codec_params, &dec_opts)
//         .expect("unsupported codec");

//     // Store the track identifier, it will be used to filter packets.
//     let track_id = track.id;

//     let mut output_file = File::create("output.mp3").unwrap();

//     // The decode loop.
//     loop {
//         // Get the next packet from the media format.
//         let packet = match format.next_packet() {
//             Ok(packet) => packet,
//             Err(Error::ResetRequired) => {
//                 // The track list has been changed. Re-examine it and create a new set of decoders,
//                 // then restart the decode loop. This is an advanced feature and it is not
//                 // unreasonable to consider this "the end." As of v0.5.0, the only usage of this is
//                 // for chained OGG physical streams.
//                 unimplemented!();
//             }
//             Err(err) => match err {
//                 Error::DecodeError(_) => todo!(),
//                 Error::IoError(e) => match e.kind() {
//                     std::io::ErrorKind::UnexpectedEof => break,
//                     _ => todo!(),
//                 },
//                 Error::SeekError(_) => todo!(),
//                 Error::Unsupported(_) => todo!(),
//                 Error::LimitError(_) => todo!(),
//                 Error::ResetRequired => todo!("fcc"),
//             },
//         };

//         // Consume any new metadata that has been read since the last packet.
//         while !format.metadata().is_latest() {
//             // Pop the old head of the metadata queue.
//             format.metadata().pop();

//             // Consume the new metadata at the head of the metadata queue.
//         }

//         // If the packet does not belong to the selected track, skip over it.
//         if packet.track_id() != track_id {
//             continue;
//         }

//         // Decode the packet into audio samples.
//         match decoder.decode(&packet) {
//             Ok(mut _decoded) => {
//                 let sample_rate = _decoded.spec().rate;
//                 let channels = _decoded.spec().channels;
//                 let bitrate = 128_000; // Adjust bitrate as needed (in bits per second)

//                 output_file.write_all(_decoded.into_sample());;
//             }
//             Err(Error::IoError(_)) => {
//                 println!("err");
//                 continue;
//             }
//             Err(Error::DecodeError(_)) => {
//                 println!("err");
//                 continue;
//             }
//             Err(err) => {
//                 println!("err");
//                 panic!("{}", err);
//             }
//         }
//     }
// }
