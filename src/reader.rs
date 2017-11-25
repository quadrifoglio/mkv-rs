//! Matroska stream reading functionality.

use std::io::Read;

use ebml;
use ebml::element::Element;

use elements;
use error::{Error, ErrorKind, Result};
use structures::Segment;

/// The object that allows to retereive information from an MKV input source.
pub struct Reader<R: Read> {
    ebml: ebml::reader::Reader<R>,
}

impl<R: Read> Reader<R> {
    fn new(r: R) -> Reader<R> {
        Reader {
            ebml: ebml::reader::Reader::from(r),
        }
    }

    /// Read segment informations from the MKV input source.
    pub fn read_segment_info(&mut self) -> Result<Segment> {
        let (segment_info, _) = self.ebml.read_element(true)?;

        if segment_info.id() != elements::Info::id() {
            bail!(ErrorKind::UnexpectedElement(
                elements::Info::id(),
                segment_info.id()
            ));
        }

        let uid = segment_info
            .find::<elements::SegmentUid>()
            .ok_or(Error::from(ErrorKind::ElementNotFound))?
            .data()
            .clone()
            .take()?;

        Ok(Segment::new(uid))
    }
}

impl<R: Read> ::std::convert::From<R> for Reader<R> {
    fn from(r: R) -> Reader<R> {
        Reader::new(r)
    }
}
