//! This module contains the functionality to read and parse MKV elements.

use std::io::Read;

use ebml;

use elements as el;
use error::{ErrorKind, Result};

/// An object used to read an MKV video.
pub struct VideoReader<R: Read> {
    reader: R
}

impl<R: Read> VideoReader<R> {
    /// Start the reading & parsing of the media.
    pub fn begin(&mut self) -> Result<()> {
        // First root element: EBM Header.

        let (header, _) = ebml::reader::read_element(&mut self.reader)?;

        if header.id() != ebml::header::EBML {
            bail!(ErrorKind::UnexpectedElement(ebml::header::EBML, header.id()));
        }

        // Second root element: MKV Segment.

        let (segment, _, _) = ebml::reader::read_element_info(&mut self.reader)?;

        if segment != el::SEGMENT {
            bail!(ErrorKind::UnexpectedElement(el::SEGMENT, segment));
        }

        // Parsing the child elements of the MKV Segment. They are called 'Top Level Elements'.
        // They can be SeekHead, SegmentInfo, Tracks, Cluster, Cues, Chapters, Tags or Attachements.
        // Once we get at a Cluster element, we stop and return control to the caller, because it
        // means that we gathered all the metadata. The user can then call `block()` to retreive
        // media data.

        'main: loop {
            let (tle, size, _) = ebml::reader::read_element_info(&mut self.reader)?;
            let mut count = 0 as usize;

            while count < size {
                match tle {
                    el::SEEK_HEAD => {
                        let (_, c) = segment::read_seek_information(&mut self.reader)?;
                        count += c;
                    },

                    el::INFO => {
                        let (_, c) = segment::read_information(&mut self.reader)?;
                        count += c;
                    },

                    el::TRACKS => {
                        let (_, c) = tracks::read_track_information(&mut self.reader)?;
                        count += c;
                    },

                    el::CLUSTER => break 'main,

                    _ => {
                        let _ = ebml::reader::read_element_data(&mut self.reader, size)?;
                    },
                };
            }
        }

        Ok(())
    }
}

impl<R: Read> ::std::convert::From<R> for VideoReader<R> {
    fn from(r: R) -> VideoReader<R> {
        VideoReader {
            reader: r
        }
    }
}

pub mod segment;
pub mod tracks;
pub mod cluster;
