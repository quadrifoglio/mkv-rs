//! Read information contained in the EBML header of the matroska file.

use ebml;
use ebml::common::ElementArray;

use error::{self, Result};

pub struct Info {
    /// A String that represents the type of the matroska document ("matroska, webm...").
    pub doc_type: String,
}

/// Read the EBML header information. Expected input: children of the `EBML` master element.
pub fn read(mut elems: ElementArray) -> Result<Info> {
    let doc_type = elems.find(ebml::header::DOC_TYPE)
        .ok_or(error::not_found(ebml::header::DOC_TYPE))?
        .content().into_utf8()?;

    Ok(Info {
        doc_type: doc_type,
    })
}
