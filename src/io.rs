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

/// Represents MKV seek information data.
pub type SeekInfo = Vec<SeekEntry>;

/// Represents an MKV seek entry to an element.
pub struct SeekEntry {
    pub seek_id: Vec<u8>,
    pub seek_position: u64,
}

/// Read and parse MKV seek information.
pub fn parse_seek_info<R: Read + Sized>(r: &mut R) -> Result<SeekInfo> {
    let mut entries = Vec::new();
    let mut count = 0 as usize;

    let (seek_head, _) = r.read_ebml_element_info()?;

    while count < seek_head.size() {
        // SeekHead element
        let (_, c) = r.read_ebml_element_info()?;
        count += c;

        let (seek_id, c) = r.read_ebml_element()?;
        count += c;

        let (seek_position, c) = r.read_ebml_element()?;
        count += c;

        entries.push(SeekEntry {
            seek_id: seek_id.data_binary(),
            seek_position: seek_position.data_u64(),
        });
    }

    Ok(entries)
}
