//! This module contains the functionality to read and parse MKV elements.

use std::io::Read;

use ebml;

use elements as el;
use error::{ErrorKind, Result};

/// An object used to read an MKV video.
pub struct VideoReader<R: Read> {
    ebml: ebml::reader::Reader<R>
}

impl<R: Read> VideoReader<R> {
    /// Start the reading & parsing of the media.
    pub fn begin(&mut self) -> Result<()> {
        // First root element: EBM Header.

        let (header, _) = self.ebml.read_element(true)?;

        if header.id() != ebml::header::EBML {
            bail!(ErrorKind::UnexpectedElement(ebml::header::EBML, header.id()));
        }

        // Second root element: MKV Segment.

        let (segment, _) = self.ebml.read_element(false)?;

        if segment.id() != el::SEGMENT {
            bail!(ErrorKind::UnexpectedElement(el::SEGMENT, segment.id()));
        }

        // Parsing the child elements of the MKV Segment. They are called 'Top Level Elements'.
        // They can be SeekHead, SegmentInfo, Tracks, Cluster, Cues, Chapters, Tags or Attachements.
        // Once we get at a Cluster element, we stop and return control to the caller, because it
        // means that we gathered all the metadata. The user can then call `block()` to retreive
        // media data.

        loop {
            let (top_level_elem, _) = self.ebml.read_element(false)?;

            match top_level_elem.id() {
                el::SEEK_HEAD => {},
                el::INFO => {},
                el::TRACKS => {},
                el::CLUSTER => break,
                el::CUES => {},

                _ => {},
            };
        }

        Ok(())
    }
}

impl<R: Read> ::std::convert::From<R> for VideoReader<R> {
    fn from(r: R) -> VideoReader<R> {
        VideoReader {
            ebml: ebml::reader::Reader::from(r),
        }
    }
}

macro_rules! find_child {
    ($parent:ident, $child:expr) => {
        if let Some(res) = $parent.find($child) {
            res
        } else {
            bail!($crate::error::ErrorKind::ElementNotFound($child));
        }
    }
}

macro_rules! find_child_uint {
    ($parent:ident, $child:expr) => {
        find_child!($parent, $child).data().to_unsigned_int()?
    }
}

macro_rules! find_child_utf8 {
    ($parent:ident, $child:expr) => {
        find_child!($parent, $child).data().to_utf8()?
    }
}

macro_rules! find_child_uint_or {
    ($parent:ident, $child:expr, $default:expr) => {
        if let Some(value) = $parent.find($child) {
            value.data().to_unsigned_int()?
        } else {
            $default
        }
    }
}

macro_rules! find_child_float_or {
    ($parent:ident, $child:expr, $default:expr) => {
        if let Some(value) = $parent.find($child) {
            value.data().to_float()?
        } else {
            $default
        }
    }
}

pub mod segment;
pub mod tracks;
pub mod cluster;
pub mod cues;
