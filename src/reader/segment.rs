//! Read matroska segment information.

use ebml::common::types::*;
use ebml::common::ElementArray;

use elements as el;
use error::Result;

pub struct Info {
    /// Optional. Unique identifier of the segment.
    pub uid: Option<Binary>,

    /// Optional. The file name corresponding to the segment.
    pub filename: Option<Utf8>,

    /// Timestamp scale of the segment in nanoseconds.
    pub timecode_scale: UnsignedInt,
}

/// Read matroska segment information. Expected input: children of the `Info` master element.
pub fn read(mut elems: ElementArray) -> Result<Info> {
    let uid = elems.find(el::SEGMENT_UID)
        .map(|elem| elem.content().into_binary());

    let filename = elems.find(el::SEGMENT_FILENAME)
        .map_or_else(|| Ok(None), |elem| elem.content().into_utf8().map(|s| Some(s)))?;

    let timecode_scale = elems.find(el::TIMECODE_SCALE)
        .map_or(1000000, |elem| elem.content().into_uint());

    Ok(Info {
        uid: uid,
        filename: filename,
        timecode_scale: timecode_scale
    })
}
