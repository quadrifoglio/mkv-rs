//! This module defines all the MKV/EBML elements that are used in Matroska files.

use ebml::element::types::*;

// Segment Information.

pub const INFO: UnsignedInt = 0x1549A966;
pub const SEGMENT_UID: UnsignedInt = 0x73A4;
pub const SEGMENT_FILENAME: UnsignedInt = 0x7384;
pub const PREV_UID: UnsignedInt = 0x3CB923;
pub const PREV_FILENAME: UnsignedInt = 0x3C83AB;
pub const NEXT_UID: UnsignedInt = 0x3EB923;
pub const NEXT_FILENAME: UnsignedInt = 0x3E83BB;
pub const SEGMENT_FAMILY: UnsignedInt = 0x4444;
pub const CHAPTER_TRANSLATE: UnsignedInt = 0x6924;
pub const CHAPTER_TRANSLATE_EDITION_UID: UnsignedInt = 0x69FC;
pub const CHAPTER_TRANSLATE_CODEC: UnsignedInt = 0x69BF;
pub const CHAPTER_TRANSLATE_ID: UnsignedInt = 0x69A5;
pub const TIMECODE_SCALE: UnsignedInt = 0x2AD7B1;
pub const DURATION: UnsignedInt = 0x4489;
pub const DATE_UTC: UnsignedInt = 0x4461;
pub const TITLE: UnsignedInt = 0x7BA9;
pub const MUXING_APP: UnsignedInt = 0x4D80;
pub const WRITING_APP: UnsignedInt = 0x5741;

ebml_element_container!(Info => INFO);

ebml_element_mandatory!(SegmentUID => SEGMENT_UID, Binary);
ebml_element_mandatory!(SegmentFilename => SEGMENT_FILENAME, Utf8);
ebml_element_mandatory!(PrevUID => PREV_UID, Binary);
ebml_element_mandatory!(PrevFilename => PREV_FILENAME, Utf8);
ebml_element_mandatory!(NextUID => NEXT_UID, Binary);
ebml_element_mandatory!(NextFilename => NEXT_FILENAME, Utf8);
ebml_element_mandatory!(SegmentFamily => SEGMENT_FAMILY, Binary);

ebml_element_container!(ChapterTranslate => CHAPTER_TRANSLATE);
ebml_element_mandatory!(ChapterTranslateEditionUID => CHAPTER_TRANSLATE_EDITION_UID, UnsignedInt);
ebml_element_mandatory!(ChapterTranslateCodec => CHAPTER_TRANSLATE_CODEC, UnsignedInt);
ebml_element_mandatory!(ChapterTranslateID => CHAPTER_TRANSLATE_ID, Binary);

ebml_element_mandatory!(TimecodeScale => TIMECODE_SCALE, UnsignedInt);
ebml_element_mandatory!(Duration => DURATION, Float);
ebml_element_mandatory!(DateUTC => DATE_UTC, Binary);
ebml_element_mandatory!(Title => TITLE, Utf8);
ebml_element_mandatory!(MuxingApp => MUXING_APP, Utf8);
ebml_element_mandatory!(WritingApp => WRITING_APP, Utf8);
