//! MKV Segment reading (Meta Seek Information & Segment Information).

use std::collections::HashMap;

use ebml::common::types::*;
use ebml::common::ElementArray as EbmlElementArray;

use error::{self, Result};
use elements as el;

/// A type alias representing a map of Element IDs to their position in the MKV file.
pub type SeekEntries = HashMap<ElementId, UnsignedInt>;

/// Contains parsed information about an MKV segment.
#[derive(Default)]
pub struct SegmentInfo {
    pub(crate) uid: Option<Binary>,
    pub(crate) filename: Option<Utf8>,
    pub(crate) timecode_scale: UnsignedInt
}

/// Read seeking information. Returns a map of Elements to their position in the file.
pub fn read_seek_information(seek_data: EbmlElementArray) -> Result<SeekEntries> {
    let mut entries = HashMap::new();

    for entry in seek_data.vec() {
        let mut data = entry.content().children()?;

        let id = data.find(el::SEEK_ID)
            .ok_or(error::not_found(el::SEEK_ID))?
            .content()
            .into_uint();

        let pos = data.find(el::SEEK_POSITION)
            .ok_or(error::not_found(el::SEEK_POSITION))?
            .content()
            .into_uint();

        entries.insert(id, pos);
    }

    Ok(entries)
}

/// Read segment information.
pub fn read_information(mut segment_info: EbmlElementArray) -> Result<SegmentInfo> {
    let uid = segment_info.find(el::SEGMENT_UID)
        .map(|elem| elem.content().into_binary());

    let filename = segment_info.find(el::SEGMENT_FILENAME)
        .map_or_else(|| Ok(None), |elem| elem.content().into_utf8().map(|s| Some(s)))?;

    let timecode_scale = segment_info.find(el::TIMECODE_SCALE)
        .map_or(1000000, |elem| elem.content().into_uint());

    let segment_info = SegmentInfo {
        uid: uid,
        filename: filename,
        timecode_scale: timecode_scale
    };

    Ok(segment_info)
}
