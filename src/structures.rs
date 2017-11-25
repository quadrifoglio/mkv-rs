//! MKV Data Structures.

use ebml::element::types::*;

/// Contains miscellaneous general information and statistics on the file.
pub struct Segment {
    pub(crate) uid: Binary,
    pub(crate) segment_filename: Utf8,
    pub(crate) prev_uid: Binary,
    pub(crate) prev_filename: Utf8,
    pub(crate) next_uid: Binary,
    pub(crate) next_filename: Utf8,
    pub(crate) segment_familys: Vec<Binary>,
    pub(crate) chapter_translates: Vec<ChapterTranslate>,
    pub(crate) timecode_scale: UnsignedInt,
    pub(crate) duration: Float,
    pub(crate) date_utc: Binary,
    pub(crate) title: Utf8,
    pub(crate) muxing_app: Utf8,
    pub(crate) writing_app: Utf8,
}

impl ::std::default::Default for Segment {
    fn default() -> Segment {
        Segment {
            uid: Binary::default(),
            segment_filename: Utf8::default(),
            prev_uid: Binary::default(),
            prev_filename: Utf8::default(),
            next_uid: Binary::default(),
            next_filename: Utf8::default(),
            segment_familys: Vec::new(),
            chapter_translates: Vec::new(),
            timecode_scale: 1000000 as UnsignedInt,
            duration: Float::default(),
            date_utc: Binary::default(),
            title: Utf8::default(),
            muxing_app: Utf8::default(),
            writing_app: Utf8::default(),
        }
    }
}

/// A tuple of corresponding ID used by chapter codecs to represent this Segment.
#[derive(Default)]
pub struct ChapterTranslate {
    pub(crate) edition_uids: Vec<UnsignedInt>,
    pub(crate) codec: UnsignedInt,
    pub(crate) id: Binary,
}

impl Segment {
    /// Returns the UID of the Segment.
    pub fn uid<'a>(&'a self) -> &'a Binary {
        &self.uid
    }
}
