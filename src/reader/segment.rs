//! MKV Segment reading (Meta Seek Information & Segment Information).

use std::io::Read;
use std::collections::HashMap;

use ebml::element::types::*;
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
    ebml.register_container(el::SEEK_HEAD);
    ebml.register_container(el::SEEK);

    ebml.register_container(el::INFO);
    ebml.register_container(el::CHAPTER_TRANSLATE);
}

/// Read seeking information. Returns a map of Elements to their position in the file.
pub fn read_seek_information<R: Read>(ebml: &mut Reader<R>) -> Result<SeekEntries> {
    let mut entries = HashMap::new();
    let (elem, _) = ebml.read_element(true)?;

    for entry in elem.children() {
        entries.insert(find_child_uint!(entry, el::SEEK_ID), find_child_uint!(entry, el::SEEK_POSITION));
    }

    Ok(entries)
}

/// Read segment information.
pub fn read_information<R: Read>(ebml: &mut Reader<R>) -> Result<SegmentInfo> {
    let mut segment_info = SegmentInfo::default();
    let (elem, _) = ebml.read_element(true)?;

    if let Some(uid) = elem.find(el::SEGMENT_UID) {
        segment_info.uid = Some(uid.data().clone().take()?);
    }

    if let Some(filename) = elem.find(el::SEGMENT_FILENAME) {
        segment_info.filename = Some(filename.data().to_utf8()?);
    }

    segment_info.timecode_scale = find_child_uint_or!(elem, el::TIMECODE_SCALE, 1000000);

    Ok(segment_info)
}
