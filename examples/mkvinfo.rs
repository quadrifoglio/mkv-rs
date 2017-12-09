//! Usage example of the mkv-rs library. This program opens and parses the specified matroska video
//! file and displays the relevant information.
//! Usage: ./mkvinfo <file>

extern crate mkv;

use std::fs::File;
use mkv::reader::{Reader, Info};

fn main() {
    let path = ::std::env::args().nth(1).expect("Please specify a filename");
    let file = File::open(path).unwrap();

    let mut video = Reader::from(file);

    for info in video.info().unwrap() {
        match info {
            Info::Ebml(header) => println!("Document Type: {}", header.doc_type),
            Info::Segment(segment) => println!("Segment with a TimecodeScale of {}", segment.timecode_scale),

            Info::MetaSeek(ref seek_entries) => {
                for (elem_id, elem_pos) in seek_entries {
                    println!("Meta Seek Entry: 0x{:X} is at {}", elem_id, elem_pos);
                }
            },

            Info::Tracks(ref tracks) => {
                for track in tracks {
                    println!("Track n°{} - Codec: {}", track.number, track.codec_id);
                }
            },
        };
    }

    loop {
        let mut cluster = video.cluster().unwrap();
        if cluster.is_none() {
            break;
        }

        loop {
            let block = cluster.as_mut().unwrap().block().unwrap();
            if block.is_none() {
                break;
            }

            println!("Found data block: {} bytes", block.unwrap().size());
        }
    }
}
