//! This module provides the functionality for reading data from matroska input sources.

pub mod meta_seek;
pub mod segment;
pub mod track;
pub mod cluster;

use std::io::Read;

use ::ebml as libebml;
use self::libebml::types::*;

use elements as el;
use error::{self, Result};

use self::cluster::Cluster;

/// Represents the different kinds of informative data that can be in a matroska file.
/// Contrary to `Block` data, `Info` does not contain any media data, only metadata.
pub enum Info {
    MetaSeek(meta_seek::Info),
    Segment(segment::Info),
    Tracks(Vec<track::Info>),
}

/// High-level object that provides access to the different sections of the matroska file.
pub struct Reader<R: Read> {
    r: R,

    // EBML header of this matroska file.
    header: libebml::header::Header,

    // Current position in the segment and its size in bytes.
    segment_position: usize,
    segment_size: usize,

    // This is set to the next element that should be read when a reading process is nominally
    // stopped at some point. For example, metadata reading stops when a cluster is encountered.
    // This variable is then set to the cluster element information for eventual later processing.
    queued_element: Option<(ElementId, ElementSize)>,
}

impl<R: Read> Reader<R> {
    /// Initialize a new matroska reader. This function also parses the EBML header of the matroska
    /// file.
    pub fn new(mut r: R) -> Result<Reader<R>> {
        let (header, _) = libebml::reader::read_header(&mut r)?;

        let (id, size, _) = libebml::reader::read_element_info(&mut r)?;
        if id != el::SEGMENT {
            bail!(error::unexpected(el::SEGMENT, id));
        }

        Ok(Reader {
            r: r,
            header: header,
            segment_position: 0,
            segment_size: size,
            queued_element: None
        })
    }

    /// Read all the metadata from the matroska input source. This functions will return the list
    /// of parsed information structures.
    pub fn info(&mut self) -> Result<Vec<Info>> {
        let mut info = Vec::new();

        while self.segment_position < self.segment_size {
            let (id, size) = match self.queued_element.take() {
                Some((id, size)) => {
                    match id {
                        // If a cluster element is queued, then we ignore it as this method is
                        // only supposed to retreive metadata.

                        el::CLUSTER => {
                            let (_, c) = libebml::reader::read_element_data(&mut self.r, size)?;
                            self.segment_position += c;

                            return self.info();
                        },

                        _ => (id, size),
                    }
                },

                None => {
                    let (id, size, c) = libebml::reader::read_element_info(&mut self.r)?;
                    self.segment_position += c;

                    (id, size)
                },
            };

            match id {
                el::SEEK_HEAD => {
                    let (content, c) = libebml::reader::read_element_data(&mut self.r, size)?;
                    self.segment_position += c;

                    info.push(Info::MetaSeek(meta_seek::read(content.children()?)?));
                },

                el::INFO => {
                    let (content, c) = libebml::reader::read_element_data(&mut self.r, size)?;
                    self.segment_position += c;

                    info.push(Info::Segment(segment::read(content.children()?)?));
                },

                el::TRACKS => {
                    let (content, c) = libebml::reader::read_element_data(&mut self.r, size)?;
                    self.segment_position += c;

                    info.push(Info::Tracks(track::read(content.children()?)?));
                },

                // Segment Top-Level-Element: read its child elements.
                el::SEGMENT => continue,

                // TODO: Process Chapters, Cues, Attachements and Tags.
                el::CHAPTERS | el::CUES | el::ATTACHEMENTS | el::TAGS => {
                    let (_, c) = libebml::reader::read_element_data(&mut self.r, size)?;
                    self.segment_position += c;
                }

                // Found the first cluster: information reading is done.
                el::CLUSTER => {
                    self.queued_element = Some((id, size));
                    break;
                },

                wtf => bail!(error::unexpected(libebml::header::EBML, wtf)),
            };
        }

        Ok(info)
    }

    /// Read the next matroska cluster. Returns `None` if there is no more to read. Borrows `self`.
    pub fn next_cluster<'a>(&'a mut self) -> Result<Option<Cluster<'a, R>>> {
        if self.segment_position >= self.segment_size {
            return Ok(None);
        }

        // If a cluster size has already been read & stored, use it. Otherwise, find the next
        // cluster's size by reading the next EBML element.

        let size = match self.queued_element {
            Some(_) => {
                let (id, size) = self.queued_element.take().unwrap();

                match id {
                    el::CLUSTER => size,
                    _ => return self.next_cluster(),
                }
            },

            None => {
                // Read the next EBML element. If it is a cluster, we can go on reading cluster data.
                // If not, then we return `None` to stop the iteration process.

                let (id, s, c) = libebml::reader::read_element_info(&mut self.r)?;
                self.segment_position += c;

                match id {
                    // Found another cluster. Get its size in bytes.
                    el::CLUSTER => s,

                    // Found some other element, queue it for eventual later processing by a user
                    // call to the `info` method.
                    _ => {
                        self.queued_element = Some((id, s));
                        return Ok(None);
                    },
                }
            },
        };

        Ok(Some(Cluster::new(self, size)))
    }

    /// Returns a reference to the EBML header of this matroska file.
    pub fn header(&self) -> &libebml::header::Header {
        &self.header
    }
}
