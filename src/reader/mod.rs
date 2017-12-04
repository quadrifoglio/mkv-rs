//! This module contains the functionality to read and parse MKV elements.

use std::io::Read;

use ebml;

use elements as el;
use error::{self, Result};
use reader::segment::SegmentInfo;
use reader::tracks::Track;

/// Contains global information about an MKV media source.
#[derive(Default)]
pub struct Info {
    doc_type: String,
    segment: SegmentInfo,
    tracks: Vec<Track>,
}

impl Info {
    /// Return a string representing the DocType of this media file.
    pub fn doc_type<'a>(&'a self) -> &'a str{
        self.doc_type.as_str()
    }

    /// Return the UID of the MKV segment, if any.
    pub fn segment_uid<'a>(&'a self) -> Option<&'a Vec<u8>> {
        if let Some(ref uid) = self.segment.uid {
            return Some(uid);
        }

        None
    }

    /// Return the timestamp scale in nanoseconds (TimecodeScale element value).
    pub fn timecode_scale(&self) -> u64 {
        self.segment.timecode_scale
    }

    /// Returns the list of tracks contained within this MKV media source.
    pub fn tracks<'a>(&'a self) -> &'a Vec<Track> {
        &self.tracks
    }
}

/// An object used to read an MKV video.
pub struct VideoReader<R: Read> {
    reader: R
}

impl<R: Read> VideoReader<R> {
    /// Read information about the media. Parses the EBML header as well as segment and track
    /// informations, but does not read any block.
    pub fn info(&mut self) -> Result<Info> {
        let mut info = Info::default();

        // First root element: EBM Header.

        let (header, _) = ebml::reader::read_element(&mut self.reader)?;

        if header.id() != ebml::header::EBML {
            bail!(error::unexpected(ebml::header::EBML, header.id()));
        }

        let mut header_fields = header.content().children()?;

        info.doc_type = header_fields.find(ebml::header::DOC_TYPE)
            .ok_or(error::not_found(ebml::header::DOC_TYPE))?
            .content().into_utf8()?;

        // Second root element: MKV Segment.

        let (segment, _, _) = ebml::reader::read_element_info(&mut self.reader)?;

        if segment != el::SEGMENT {
            bail!(error::unexpected(el::SEGMENT, segment));
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
                        let (data, c) = ebml::reader::read_element_data(&mut self.reader, size)?;
                        count += c;

                        let _ = segment::read_seek_information(data.children()?)?;
                    },

                    el::INFO => {
                        let (data, c) = ebml::reader::read_element_data(&mut self.reader, size)?;
                        count += c;

                        let segment = segment::read_information(data.children()?)?;
                        info.segment = segment;
                    },

                    el::TRACKS => {
                        let (data, c) = ebml::reader::read_element_data(&mut self.reader, size)?;
                        count += c;

                        let tracks = tracks::read_track_information(data.children()?)?;
                        info.tracks = tracks;
                    },

                    el::CLUSTER => break 'main,

                    _ => {
                        let (_, c) = ebml::reader::read_element_data(&mut self.reader, size)?;
                        count += c;
                    },
                };
            }
        }

        Ok(info)
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
