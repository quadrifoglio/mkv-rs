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
        let mut ebml = ebml::reader::Reader::from(r);

        // Meta Seek Information.

        ebml.register::<el::SeekHead>();
        ebml.register::<el::Seek>();
        ebml.register::<el::SeekID>();
        ebml.register::<el::SeekPosition>();

        // Segment Information.

        ebml.register::<el::Info>();
        ebml.register::<el::SegmentUID>();
        ebml.register::<el::SegmentFilename>();
        ebml.register::<el::PrevUID>();
        ebml.register::<el::PrevFilename>();
        ebml.register::<el::NextUID>();
        ebml.register::<el::NextFilename>();
        ebml.register::<el::SegmentFamily>();
        ebml.register::<el::ChapterTranslate>();
        ebml.register::<el::ChapterTranslateEditionUID>();
        ebml.register::<el::ChapterTranslateCodec>();
        ebml.register::<el::ChapterTranslateID>();
        ebml.register::<el::TimecodeScale>();
        ebml.register::<el::Duration>();
        ebml.register::<el::DateUTC>();
        ebml.register::<el::Title>();
        ebml.register::<el::MuxingApp>();
        ebml.register::<el::WritingApp>();

        Reader { ebml: ebml }
    }

    /// Read seek data from the MKV input source.
    pub fn read_seek_data(&mut self) -> Result<Vec<SeekEntry>> {
        let mut entries = Vec::new();
        let (parent, _) = self.ebml.read_element(true)?;

        if parent.id() != el::SEEK_HEAD {
            bail!(ErrorKind::UnexpectedElement(el::SEEK_HEAD, parent.id()));
        }

        for child in parent.children() {
            let mut entry = SeekEntry::default();

            if child.id() != el::SEEK {
                bail!(ErrorKind::UnexpectedElement(el::SEEK, child.id()));
            }

            for value in child.children() {
                match value.id() {
                    el::SEEK_ID => entry.id = value.data().clone().take()?,
                    el::SEEK_POSITION => entry.position = value.data().to_unsigned_int()?,

                    _ => {}
                };
            }

            entries.push(entry);
        }

        Ok(entries)
    }

    /// Read segment informations from the MKV input source.
    pub fn read_segment_info(&mut self) -> Result<SegmentInfo> {
        let mut info = SegmentInfo::default();
        let (parent, _) = self.ebml.read_element(true)?;

        if parent.id() != el::INFO {
            bail!(ErrorKind::UnexpectedElement(el::INFO, parent.id()));
        }

        for child in parent.children() {
            match child.id() {
                el::SEGMENT_UID => info.uid = child.data().clone().take()?,
                el::SEGMENT_FILENAME => info.segment_filename = child.data().to_utf8()?,
                el::PREV_UID => info.prev_uid = child.data().clone().take()?,
                el::PREV_FILENAME => info.prev_filename = child.data().to_utf8()?,
                el::NEXT_UID => info.next_uid = child.data().clone().take()?,
                el::NEXT_FILENAME => info.next_filename = child.data().to_utf8()?,
                el::SEGMENT_FAMILY => info.segment_familys.push(child.data().clone().take()?),
                el::TIMECODE_SCALE => info.timecode_scale = child.data().to_unsigned_int()?,
                el::DURATION => info.duration = child.data().to_float()?,
                el::DATE_UTC => info.date_utc = child.data().clone().take()?,
                el::TITLE => info.title = child.data().to_utf8()?,
                el::MUXING_APP => info.muxing_app = child.data().to_utf8()?,
                el::WRITING_APP => info.writing_app = child.data().to_utf8()?,
                el::CHAPTER_TRANSLATE => info.chapter_translates.push(self.read_segment_info_chapter_translate()?),

                _ => {}
            };
        }

        Ok(info)
    }

    /// Read ChapterTranslate information.
    fn read_segment_info_chapter_translate(&mut self) -> Result<ChapterTranslate> {
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
