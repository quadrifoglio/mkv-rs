//! This module provides the functionality for reading data from matroska input sources.

pub mod ebml;
pub mod meta_seek;
pub mod segment;
pub mod track;

use std::io::Read;

use ::ebml as libebml;

use elements as el;
use error::{self, Result};

/// Represents the different kinds of informative data that can be in a matroska file.
/// Contrary to `Block` data, `Info` does not contain any media data, only metadata.
pub enum Info {
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
    /// Read all the metadata from the matroska input source. This functions will return the list
    /// of parsed information structures.
    pub fn info(&mut self) -> Result<Vec<Info>> {
        let mut info = Vec::new();

        loop {
            let (id, size, _) = libebml::reader::read_element_info(&mut self.r)?;

            match id {
                libebml::header::EBML => {
                    let (content, _) = libebml::reader::read_element_data(&mut self.r, size)?;
                    info.push(Info::Ebml(ebml::read(content.children()?)?));
                },

                el::SEEK_HEAD => {
                    let (content, _) = libebml::reader::read_element_data(&mut self.r, size)?;
                    info.push(Info::MetaSeek(meta_seek::read(content.children()?)?));
                },

                el::INFO => {
                    let (content, _) = libebml::reader::read_element_data(&mut self.r, size)?;
                    info.push(Info::Segment(segment::read(content.children()?)?));
                },

                el::TRACKS => {
                    let (content, _) = libebml::reader::read_element_data(&mut self.r, size)?;
                    info.push(Info::Tracks(track::read(content.children()?)?));
                },

                // Segment Top-Level-Element: read its child elements.
                el::SEGMENT => continue,

                // TODO: Process Chapters, Cues, Attachements and Tags.
                el::CHAPTERS | el::CUES | el::ATTACHEMENTS | el::TAGS => {
                    libebml::reader::read_element_data(&mut self.r, size)?;
                }

                // Found the first cluster: information reading is done.
                el::CLUSTER => break,

                wtf => bail!(error::unexpected(libebml::header::EBML, wtf)),
            };
        }

        Ok(info)
    }
}

impl<R: Read> ::std::convert::From<R> for Reader<R> {
    fn from(r: R) -> Reader<R> {
        Reader {
            r: r,
        }
    }
}
