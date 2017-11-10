/// This module contains all the parsing functionality.

use std::io::Read;

use ebml::io::ReadEbml;

use elements;
use error::Result;
use structures::{Header, SeekInfo, SeekEntry, SegmentInfo};

/// Read and parse an EBML header from an input source.
pub fn header<R: Read + Sized>(r: &mut R) -> Result<Header> {
    let mut count = 0 as usize;
    let (elem, _) = r.read_ebml_element_info()?;

    let mut header = Header::default();

    while count < elem.size() {
        let (elem, r) = r.read_ebml_element()?;
        count += r;

        match elem.info().id() {
            elements::EBML_VERSION => header.ebml_version = elem.data_u64(),
            elements::EBML_READ_VERSION => header.ebml_read_version = elem.data_u64(),
            elements::EBML_MAX_ID_LENGTH => header.ebml_max_id_length = elem.data_u64(),
            elements::EBML_MAX_SIZE_LENGTH => header.ebml_max_size_length = elem.data_u64(),
            elements::DOC_TYPE => header.doc_type = elem.data_utf8()?,
            elements::DOC_TYPE_VERSION => header.doc_type_version = elem.data_u64(),
            elements::DOC_TYPE_READ_VERSION => header.doc_type_read_version = elem.data_u64(),

            _ => {}
        };
    }

    Ok(header)
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

/// Read and parse MKV segment information.
pub fn segment_info<R: Read + Sized>(r: &mut R) -> Result<SegmentInfo> {
    let mut count = 0 as usize;
    let mut seg_info = SegmentInfo::default();

    let (segment_info, _) = r.read_ebml_element_info()?;

    while count < segment_info.size() {
        let (elem, c) = r.read_ebml_element()?;
        count += c;

        match elem.info().id() {
            elements::SEGMENT_UID => seg_info.uid = elem.data_binary(),
            elements::SEGMENT_FILENAME => seg_info.filename = elem.data_utf8()?,
            elements::PREV_UID => seg_info.prev_uid = elem.data_binary(),
            elements::PREV_FILENAME => seg_info.prev_filename = elem.data_utf8()?,
            elements::NEXT_UID => seg_info.next_uid = elem.data_binary(),
            elements::NEXT_FILENAME => seg_info.next_filename = elem.data_utf8()?,
            elements::SEGMENT_FAMILY => seg_info.family = elem.data_binary(),
            elements::TIMECODE_SCALE => seg_info.timecode_scale = elem.data_u64(),
            elements::DURATION => seg_info.duration = elem.data_f32(),
            elements::DATE_UTC => seg_info.date_utc = elem.data_i64(),
            elements::TITLE => seg_info.title = elem.data_utf8()?,
            elements::MUXING_APP => seg_info.muxing_app = elem.data_utf8()?,
            elements::WRITING_APP => seg_info.writing_app = elem.data_utf8()?,

            _ => {},
        };
    }

    Ok(seg_info)
}
