//! This module provides the functionality for reading data from matroska input sources.

pub mod ebml;
pub mod meta_seek;
pub mod segment;
pub mod track;

use std::io::Read;

/// Represents the different kinds of informative data that can be in a matroska file.
/// Contrary to `Block` data, `Information` does not contain any media data, only metadata.
pub enum Information {
    Ebml(ebml::Info),
    MetaSeek(meta_seek::Info),
    Segment(segment::Info),
    Tracks(Vec<track::Info>),
}

/// High-level object that provides access to the different sections of the matroska file.
pub struct Reader<R: Read> {
    r: R,
}

impl<R: Read> Reader<R> {
}

impl<R: Read> ::std::convert::From<R> for Reader<R> {
    fn from(r: R) -> Reader<R> {
        Reader {
            r: r,
        }
    }
}
