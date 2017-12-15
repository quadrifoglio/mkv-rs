//! Usage example of the mkv-rs library. This program opens and parses the specified matroska video
//! file and displays the relevant information.
//! Usage: ./mkvinfo <file>

extern crate mkv;

use std::fs::File;
use mkv::reader::{Reader, Info};

fn main() {
    let path = ::std::env::args().nth(1).expect("Please specify a filename");
    let file = File::open(path).unwrap();

    let mut video = Reader::new(file).unwrap();

    println!("Document Type: {}", video.header().doc_type());
    println!("Document Type Version: {}", video.header().doc_type_version());
    println!("Document Type Read Version: {}", video.header().doc_type_read_version());

    // Read metadata: all the information that preceed the actual data blocks.
    print_info(video.info().unwrap());

    // Read all the data blocks in the file.
    while let Some(mut cluster) = video.next_cluster().unwrap() {
        for block in cluster.blocks() {
            let block = block.unwrap();
            println!("Block of size {}", block.size());

            for frame in block.frames().unwrap() {
                println!("Frame of size {}", frame.len());
            }
        }
    }

    // Read the information that is located after the clusters.
    print_info(video.info().unwrap());
}

/// Prints matroska metadata to stdout.
fn print_info(infos: Vec<Info>) {
    for info in infos {
        match info {
            Info::Segment(segment) => println!("Segment with a TimecodeScale of {}", segment.timecode_scale()),

            Info::MetaSeek(ref seek_entries) => {
                for (elem_id, elem_pos) in seek_entries {
                    println!("Meta Seek Entry: 0x{:X} is at {}", elem_id, elem_pos);
                }
            },

            Info::Tracks(ref tracks) => {
                for track in tracks {
                    println!("Track n°{} - Codec: {}", track.index(), track.codec());
                }
            },
        };
    }
}
