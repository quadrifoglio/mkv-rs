/// This module contains the functionality to allow reading/writing of MKV video.

use std::io::Read;

use ebml::io::ReadEbml;

use elements;
use error::Result;

/// Represents an MKV/EBML header.
/// Documentation for each field signification can be found on the official Matroska website.
pub struct Header {
    pub ebml_version: u64,
    pub ebml_read_version: u64,
    pub ebml_max_id_length: u64,
    pub ebml_max_size_length: u64,
    pub doc_type: String,
    pub doc_type_version: u64,
    pub doc_type_read_version: u64,
}

/// Read and parse an EBML header from an input source.
pub fn parse_header<R: Read + Sized>(r: &mut R) -> Result<Header> {
    let mut count = 0 as usize;
    let (elem, _) = r.read_ebml_element_info()?;

    let mut ebml_version = 0;
    let mut ebml_read_version = 0;
    let mut ebml_max_id_length = 0;
    let mut ebml_max_size_length = 0;
    let mut doc_type = String::new();
    let mut doc_type_version = 0;
    let mut doc_type_read_version = 0;

    while count < elem.size() {
        let (elem, r) = r.read_ebml_element()?;
        count += r;

        match elem.info().id() {
            elements::EBML_VERSION => ebml_version = elem.data_u64(),
            elements::EBML_READ_VERSION => ebml_read_version = elem.data_u64(),
            elements::EBML_MAX_ID_LENGTH => ebml_max_id_length = elem.data_u64(),
            elements::EBML_MAX_SIZE_LENGTH => ebml_max_size_length = elem.data_u64(),
            elements::DOC_TYPE => doc_type = elem.data_utf8()?,
            elements::DOC_TYPE_VERSION => doc_type_version = elem.data_u64(),
            elements::DOC_TYPE_READ_VERSION => doc_type_read_version = elem.data_u64(),

            _ => {}
        };
    }

    Ok(Header {
        ebml_version: ebml_version,
        ebml_read_version: ebml_read_version,
        ebml_max_id_length: ebml_max_id_length,
        ebml_max_size_length: ebml_max_size_length,
        doc_type: doc_type,
        doc_type_version: doc_type_version,
        doc_type_read_version: doc_type_read_version,
    })
}
