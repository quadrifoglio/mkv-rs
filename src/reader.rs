//! Matroska stream reading functionality.

use std::io::Read;

use ebml;
use ebml::element::Element;

use elements as el;
use structures::*;
use error::{ErrorKind, Result};

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
        let mut seg = Segment::default();
        let (segment_info, _) = self.ebml.read_element(true)?;

        if segment_info.id() != el::Info::id() {
            bail!(ErrorKind::UnexpectedElement(
                el::Info::id(),
                segment_info.id()
            ));
        }

        for child in segment_info.children() {
            match child.id() {
                el::SEGMENT_UID => seg.uid = child.data().clone().take()?,

                el::SEGMENT_FILENAME => seg.segment_filename = child.data().to_utf8()?,
                el::PREV_UID => seg.prev_uid = child.data().clone().take()?,
                el::PREV_FILENAME => seg.prev_filename = child.data().to_utf8()?,
                el::NEXT_UID => seg.next_uid = child.data().clone().take()?,
                el::NEXT_FILENAME => seg.next_filename = child.data().to_utf8()?,
                el::SEGMENT_FAMILY => seg.segment_familys.push(child.data().clone().take()?),
                el::CHAPTER_TRANSLATE => {
                    seg.chapter_translates.push(self.read_chapter_translate()?)
                }
                el::TIMECODE_SCALE => seg.timecode_scale = child.data().to_unsigned_int()?,
                el::DURATION => seg.duration = child.data().to_float()?,
                el::DATE_UTC => seg.date_utc = child.data().clone().take()?,
                el::TITLE => seg.title = child.data().to_utf8()?,
                el::MUXING_APP => seg.muxing_app = child.data().to_utf8()?,
                el::WRITING_APP => seg.writing_app = child.data().to_utf8()?,

                _ => {}
            };
        }

        Ok(seg)
    }

    /// Read ChapterTranslate information.
    fn read_chapter_translate(&mut self) -> Result<ChapterTranslate> {
        let mut ct = ChapterTranslate::default();

        let (parent, _) = self.ebml.read_element(true)?;

        if parent.id() != el::ChapterTranslate::id() {
            bail!(ErrorKind::UnexpectedElement(
                el::ChapterTranslate::id(),
                parent.id()
            ));
        }

        for child in parent.children() {
            match child.id() {
                el::CHAPTER_TRANSLATE_EDITION_UID => {
                    ct.edition_uids.push(child.data().to_unsigned_int()?)
                }
                el::CHAPTER_TRANSLATE_CODEC => ct.codec = child.data().to_unsigned_int()?,
                el::CHAPTER_TRANSLATE_ID => ct.id = child.data().clone().take()?,

                _ => {}
            };
        }

        Ok(ct)
    }
}

impl<R: Read> ::std::convert::From<R> for Reader<R> {
    fn from(r: R) -> Reader<R> {
        Reader::new(r)
    }
}
