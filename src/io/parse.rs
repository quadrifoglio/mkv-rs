/// This module contains all the parsing functionality.

use std::io::Read;

use ebml::io::ReadEbml;

use elements;
use error::Result;
use structures::{Header, SeekInfo, SeekEntry, SegmentInfo, TrackInfo, Track};

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

/// Read and parse track information.
pub fn track_info<R: Read + Sized>(r: &mut R) -> Result<TrackInfo> {
    let mut count = 0 as usize;
    let mut tracks = Vec::new();

    let (seek_info, _) = r.read_ebml_element_info()?;

    while count < seek_info.size() {
        let mut track = Track::default();

        let (elem, c) = r.read_ebml_element()?;
        count += c;

        match elem.info().id() {
            elements::TRACK_NUMBER => track.number = elem.data_u64(),
            elements::TRACK_UID => track.uid = elem.data_binary(),
            elements::TRACK_TYPE => track.track_type = elem.data_u64(),
            elements::FLAG_ENABLED => track.flag_enabled = elem.data_u64(),
            elements::FLAG_DEFAULT => track.flag_default = elem.data_u64(),
            elements::FLAG_FORCED => track.flag_forced = elem.data_u64(),
            elements::FLAG_LACING => track.flag_lacing = elem.data_u64(),
            elements::MIN_CACHE => track.min_cache = elem.data_u64(),
            elements::MAX_CACHE => track.max_cache = elem.data_u64(),
            elements::DEFAULT_DURATION => track.default_duration = elem.data_u64(),
            elements::DEFAULT_DECODED_FIELD_DURATION => track.default_decoded_field_duration = elem.data_u64(),
            elements::TRACK_OFFSET => track.track_offset = elem.data_i64(),
            elements::MAX_BLOCK_ADDITION_ID => track.max_block_addition_id = elem.data_u64(),
            elements::NAME => track.name = elem.data_utf8()?,
            elements::LANGUAGE => track.language = elem.data_utf8()?,
            elements::CODEC_ID => track.codec_id = elem.data_utf8()?,
            elements::CODEC_PRIVATE => track.codec_private = elem.data_binary(),
            elements::CODEC_NAME => track.codec_name = elem.data_utf8()?,
            elements::ATTACHMENT_LINK => track.attachment_link = elem.data_u64(),
            elements::CODEC_SETTINGS => track.codec_settings = elem.data_utf8()?,
            elements::CODEC_INFO_URL => track.codec_info_url = elem.data_utf8()?,
            elements::CODEC_DOWNLOAD_URL => track.codec_download_url = elem.data_utf8()?,
            elements::CODEC_DECODE_ALL => track.codec_decode_all = elem.data_u64(),
            elements::TRACK_OVERLAY => track.track_overlay = elem.data_u64(),
            elements::CODEC_DELAY => track.codec_delay = elem.data_u64(),
            elements::SEEK_PRE_ROLL => track.seek_pre_roll = elem.data_u64(),

            _ => {},
        }

        tracks.push(track);
    }

    Ok(tracks)
}
