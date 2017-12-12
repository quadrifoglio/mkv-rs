//! Read Meta Seek information contained in the matroska file.

use std::collections::HashMap;

use ebml::types::*;
use ebml::ElementArray;

use elements as el;
use error::{self, Result};

/// Type alias for the Meta Seek Information. Correspondance between an EBML element ID and
/// its position in the matroska file.
pub type Info = HashMap<ElementId, u64>;

/// Read Meta Seek entries. Expected input: array of `Seek` elements (the children of the
/// `SeekHead` master element).
pub fn read(elems: ElementArray) -> Result<Info> {
    let mut entries = HashMap::new();

    for entry in elems.vec() {
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
