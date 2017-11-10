/*!
Basic implementation of the MKV (Matroska) video format for the Rust Programming Language.
*/

#[macro_use]
extern crate error_chain;

extern crate ebml;

mod error;

#[allow(dead_code)]
mod elements;

pub mod io;
pub mod structures;

use std::io::Read;

use error::Result;

/// Represents an MKV Video source (file or otherwise).
pub struct Video {}

impl Video {
    /// Open a Matroska video from some kind of a reader, and retreive basic information about the
    /// media.
    pub fn open<R: Read + Sized>(r: &mut R) -> Result<Video> {
        Ok(Video {})
    }
}

#[cfg(test)]
mod tests;
