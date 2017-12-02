//! MKV Segment reading (Meta Seek Information & Segment Information).

use std::io::Read;
use std::collections::HashMap;

use ebml::element::types::*;
use ebml::element::Element;
use ebml::element::Id as EbmlId;
use ebml::reader::Reader;

use error::Result;
use elements as el;

/// A type alias representing a map of Element IDs to their position in the MKV file.
pub type SeekEntries = HashMap<EbmlId, UnsignedInt>;

/// Contains parsed information about an MKV segment.
#[derive(Default)]
pub struct SegmentInfo {
    pub uid: Option<Binary>,
    pub filename: Option<Utf8>,
    pub timecode_scale: UnsignedInt
}

/// Initialize the specified EBML reader to make it ready to read MKV segment information.
pub fn init<R: Read>(ebml: &mut Reader<R>) {
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
}

/// Read seeking information. Returns a map of Elements to their position in the file.
pub fn read_seek_information<R: Read>(ebml: &mut Reader<R>) -> Result<SeekEntries> {
    let mut entries = HashMap::new();
    let (elem, _) = ebml.read_element(true)?;

    for entry in elem.children() {
        entries.insert(find_child_uint!(entry, el::SeekID), find_child_uint!(entry, el::SeekPosition));
    }

    Ok(entries)
}

/// Read segment information.
pub fn read_information<R: Read>(ebml: &mut Reader<R>) -> Result<SegmentInfo> {
    let mut segment_info = SegmentInfo::default();
    let (elem, _) = ebml.read_element(true)?;

    if let Some(uid) = elem.find::<el::SegmentUID>() {
        segment_info.uid = Some(uid.data().clone().take()?);
    }

    if let Some(filename) = elem.find::<el::SegmentFilename>() {
        segment_info.filename = Some(filename.data().to_utf8()?);
    }

    segment_info.timecode_scale = find_child_uint_or!(elem, el::TimecodeScale, 1000000);

    Ok(segment_info)
}
