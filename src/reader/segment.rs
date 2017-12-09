//! Read matroska segment information.

use ebml::common::types::*;
use ebml::common::ElementArray;

use elements as el;
use error::Result;

pub struct Info {
    timecode_scale: UnsignedInt,
    uid: Option<Binary>,
    filename: Option<Utf8>,
}

impl Info {
    /// Timestamp scale of the segment in nanoseconds.
    pub fn timecode_scale(&self) -> u64 {
        self.timecode_scale
    }

    /// Optional. Unique identifier of the segment.
    pub fn uid(&self) -> Option<&[u8]> {
        if let Some(ref uid) = self.uid {
            Some(uid.as_slice())
        } else {
            None
        }
    }

    /// Optional. The file name corresponding to the segment.
    pub fn filename(&self) -> Option<&str> {
        if let Some(ref filename) = self.filename {
            Some(filename.as_str())
        } else {
            None
        }
    }
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
