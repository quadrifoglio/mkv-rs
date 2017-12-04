//! Simple example showing how the mkv-rs library can be used to retreive and display informations
//! about an MKV video file.
//!
//! Usage: mkvinfo <file>

extern crate mkv;

use std::env;
use std::fs::File;

use mkv::reader::VideoReader;
use mkv::reader::tracks::TrackKind;

fn main() {
    let path = env::args().nth(1).expect("Please specify a filename");
    let file = File::open(path).expect("Failed to open file");

    let mut reader = VideoReader::from(file);
    let info = reader.info().unwrap();

    println!("Document Type: {}", info.doc_type());
    println!("Timecode Scale: {}", info.timecode_scale());

    for track in info.tracks() {
        match track.kind() {
            &TrackKind::Video(ref video) => println!("Found video track: {}x{}", video.width(), video.height()),
            &TrackKind::Audio(ref audio) => println!("Found audio track: {} Hz", audio.sampling_frequency()),

            _ => {},
        }
    }
}
