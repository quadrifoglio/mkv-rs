# mkv-rs

Basic implementation of the MKV (Matroska) video format for the Rust Programming Language.

## Example

```rust
extern crate mkv;

use std::fs::File;
use mkv::reader::{Reader, Info};

fn main() {
    let file = File::open("video.mkv").unwrap();
    let mut video = Reader::new(file).unwrap();

    for info in video.info().unwrap() {
        match info {
            Info::Tracks(ref tracks) => {
                for track in tracks {
                    println!("Track n°{} - Codec: {}", track.index(), track.codec());
                }
            },

            _ => {},
        };
    }
}
```
