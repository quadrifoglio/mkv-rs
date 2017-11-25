//! MKV Data Structures.

use ebml::element::types::Binary as EbmlBinary;

/// Information about a segment contained in an MKV file.
pub struct Segment {
    uid: EbmlBinary,
}

impl Segment {
    /// Create a new segment.
    pub fn new(uid: EbmlBinary) -> Segment {
        Segment { uid: uid }
    }

    /// Returns the UID of the Segment.
    pub fn uid<'a>(&'a self) -> &'a EbmlBinary {
        &self.uid
    }
}
