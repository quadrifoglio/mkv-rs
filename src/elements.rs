//! This module defines all the MKV/EBML elements that are used in Matroska files.

use ebml::element::types::*;

// Segment Information.

ebml_element_container!(Info => 0x1549a966);

ebml_element_mandatory!(SegmentUid => 0x73A4, Binary);
ebml_element_mandatory!(SegmentFilename => 0x7384, Utf8);
ebml_element_mandatory!(PrevUid => 0x3CB923, Binary);
ebml_element_mandatory!(PrevFilename => 0x3C83AB, Utf8);
ebml_element_mandatory!(NextUid => 0x3EB923, Binary);
ebml_element_mandatory!(NextFilename => 0x3E83BB, Utf8);
ebml_element_mandatory!(SegmentFamily => 0x4444, Binary);

ebml_element_container!(ChapterTranslate => 0x6924);
ebml_element_mandatory!(ChapterTranslateEditionUid => 0x69FC, UnsignedInt);
ebml_element_mandatory!(ChapterTranslateCodec => 0x69BF, UnsignedInt);
ebml_element_mandatory!(ChapterTranslateId => 0x69A5, Binary);

ebml_element_mandatory!(TimecodeScale => 0x2AD7B1, UnsignedInt);
ebml_element_mandatory!(Duration => 0x4489, Float);
ebml_element_mandatory!(DateUtc => 0x4461, Binary);
ebml_element_mandatory!(Title => 0x7BA9, Utf8);
ebml_element_mandatory!(MuxingApp => 0x4D80, Utf8);
ebml_element_mandatory!(WritingApp => 0x5741, Utf8);
