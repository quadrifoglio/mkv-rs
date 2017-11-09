/// This module contains the functionality to allow reading/writing of MKV video.

use std::io::Read;

use ebml::io::ReadEbml;

use error::Result;

/// Represents an MKV/EBML header.
pub struct Header {
    ebml_version: u64,
    ebml_read_version: u64,
    ebml_max_id_length: u64,
    ebml_max_size_length: u64,
    doc_type: String,
    doc_type_version: u64,
    doc_type_read_version: u64,
}

/// Read an EBML header from an input source.
pub fn read_header<R: Read + Sized>(r: &mut R) -> Result<()> {
    let mut count = 0 as usize;
    let (elem, _) = r.read_ebml_element_info()?;

    while count < elem.size() {
        let (elem, r) = r.read_ebml_element()?;
        count += r;

        println!("{:X} - {} bytes", elem.info().id(), elem.info().size());
    }

    Ok(())
}
