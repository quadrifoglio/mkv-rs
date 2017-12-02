//! MKV Cues reading.

use std::io::Read;

use ebml::reader::Reader;

use elements as el;
use error::{ErrorKind, Result};

/// Represents a read cue point for quick seeking.
pub struct CuePoint;

/// Initialize the specified EBML reader to make it ready to read MKV cue information.
pub fn init<R: Read>(ebml: &mut Reader<R>) {
    ebml.register_container(el::CUES);
    ebml.register_container(el::CUE_POINT);
    ebml.register_container(el::CUE_TRACK_POSITIONS);
    ebml.register_container(el::CUE_REFERENCE);
}

/// Read all the cue points of an MKV input.
pub fn read_cue_point<R: Read>(ebml: &mut Reader<R>) -> Result<Vec<CuePoint>> {
    let cue_points = Vec::new();

    let (elem, _) = ebml.read_element(true)?;

    if elem.id() != el::CUES {
        bail!(ErrorKind::UnexpectedElement(el::CUES, elem.id()));
    }

    // TODO: Actually read cue points.

    Ok(cue_points)
}
