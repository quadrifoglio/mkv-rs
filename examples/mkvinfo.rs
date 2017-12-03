//! Simple example showing how the mkv-rs library can be used to retreive and display informations
//! about an MKV video file.
//!
//! Usage: mkvinfo <file>

extern crate mkv;

use std::env;
use std::fs::File;

use mkv::reader::VideoReader;

fn main() {
    let path = env::args().nth(1).expect("Please specify a filename");
    let mut file = File::open(path).expect("Failed to open file");

    let video = VideoReader::from(file).begin();
}
