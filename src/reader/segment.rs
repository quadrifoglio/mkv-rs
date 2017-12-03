//! MKV Segment reading (Meta Seek Information & Segment Information).

use std::io::Read;
use std::collections::HashMap;

use ebml;
use ebml::common::types::*;

use error::{self, Result};
use elements as el;

/// A type alias representing a map of Element IDs to their position in the MKV file.
pub type SeekEntries = HashMap<ElementId, UnsignedInt>;

/// Contains parsed information about an MKV segment.
#[derive(Default)]
pub struct SegmentInfo {
    pub uid: Option<Binary>,
    pub filename: Option<Utf8>,
    pub timecode_scale: UnsignedInt
}

/// Read seeking information. Returns a map of Elements to their position in the file.
pub fn read_seek_information<R: Read>(r: &mut R) -> Result<(SeekEntries, usize)> {
    let mut entries = HashMap::new();
    let (elem, count) = ebml::reader::read_element(r)?;

    for entry in elem.content().children()?.vec() {
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

    Ok((entries, count))
}

/// Read segment information.
pub fn read_information<R: Read>(r: &mut R) -> Result<(SegmentInfo, usize)> {
    let (elem, count) = ebml::reader::read_element(r)?;

    let mut data = elem.content().children()?;

    let uid = data.find(el::SEGMENT_UID)
        .map(|elem| elem.content().into_binary());

    let filename = data.find(el::SEGMENT_FILENAME)
        .map_or_else(|| Ok(None), |elem| elem.content().into_utf8().map(|s| Some(s)))?;

    let timecode_scale = data.find(el::TIMECODE_SCALE)
        .map_or(1000000, |elem| elem.content().into_uint());

    let segment_info = SegmentInfo {
        uid: uid,
        filename: filename,
        timecode_scale: timecode_scale
    };

    Ok((segment_info, count))
}
