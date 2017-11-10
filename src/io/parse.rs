/// This module contains all the parsing functionality.

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
pub fn header<R: Read + Sized>(r: &mut R) -> Result<Header> {
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
pub fn seek_info<R: Read + Sized>(r: &mut R) -> Result<SeekInfo> {
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

/// Contains information about an MKV segment.
pub struct SegmentInfo {
    pub uid: Vec<u8>,
    pub filename: String,
    pub prev_uid: Vec<u8>,
    pub prev_filename: String,
    pub next_uid: Vec<u8>,
    pub next_filename: String,
    pub family: Vec<u8>,
    pub timecode_scale: u64,
    pub duration: f32,
    pub date_utc: i64,
    pub title: String,
    pub muxing_app: String,
    pub writing_app: String
}

/// Read and parse MKV segment information.
pub fn segment_info<R: Read + Sized>(r: &mut R) -> Result<SegmentInfo> {
    let mut count = 0 as usize;

    let mut uid = Vec::new();
    let mut filename = String::new();
    let mut prev_uid = Vec::new();
    let mut prev_filename = String::new();
    let mut next_uid = Vec::new();
    let mut next_filename = String::new();
    let mut family = Vec::new();
    let mut timecode_scale = 0 as u64;
    let mut duration = 0.0 as f32;
    let mut date_utc = 0 as i64;
    let mut title = String::new();
    let mut muxing_app = String::new();
    let mut writing_app = String::new();

    let (segment_info, _) = r.read_ebml_element_info()?;

    while count < segment_info.size() {
        let (elem, c) = r.read_ebml_element()?;
        count += c;

        match elem.info().id() {
            elements::SEGMENT_UID => uid = elem.data_binary(),
            elements::SEGMENT_FILENAME => filename = elem.data_utf8()?,
            elements::PREV_UID => prev_uid = elem.data_binary(),
            elements::PREV_FILENAME => prev_filename = elem.data_utf8()?,
            elements::NEXT_UID => next_uid = elem.data_binary(),
            elements::NEXT_FILENAME => next_filename = elem.data_utf8()?,
            elements::SEGMENT_FAMILY => family = elem.data_binary(),
            elements::TIMECODE_SCALE => timecode_scale = elem.data_u64(),
            elements::DURATION => duration = elem.data_f32(),
            elements::DATE_UTC => date_utc = elem.data_i64(),
            elements::TITLE => title = elem.data_utf8()?,
            elements::MUXING_APP => muxing_app = elem.data_utf8()?,
            elements::WRITING_APP => writing_app = elem.data_utf8()?,

            _ => {},
        };
    }

    Ok(SegmentInfo {
        uid: uid,
        filename: filename,
        prev_uid: prev_uid,
        prev_filename: prev_filename,
        next_uid: next_uid,
        next_filename: next_filename,
        family: family,
        timecode_scale: timecode_scale,
        duration: duration,
        date_utc: date_utc,
        title: title,
        muxing_app: muxing_app,
        writing_app: writing_app
    })
}
