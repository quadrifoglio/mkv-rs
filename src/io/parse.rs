/// This module contains all the parsing functionality.

use std::io::Read;

use ebml::io::ReadEbml;

use elements as el;
use error::Result;
use structures::{Header, SeekEntry, SeekInfo, SegmentInfo, Track, TrackInfo};

/// Read and parse an EBML header from an input source.
pub fn header<R: Read + Sized>(r: &mut R) -> Result<Header> {
    let mut count = 0 as usize;
    let (elem, _) = r.read_ebml_element_info()?;

    let mut header = Header::default();

    while count < elem.size() {
        let (elem, r) = r.read_ebml_element()?;
        count += r;

        match elem.info().id() {
            el::EBML_VERSION => header.ebml_version = elem.data_u64(),
            el::EBML_READ_VERSION => header.ebml_read_version = elem.data_u64(),
            el::EBML_MAX_ID_LENGTH => header.ebml_max_id_length = elem.data_u64(),
            el::EBML_MAX_SIZE_LENGTH => header.ebml_max_size_length = elem.data_u64(),
            el::DOC_TYPE => header.doc_type = elem.data_utf8()?,
            el::DOC_TYPE_VERSION => header.doc_type_version = elem.data_u64(),
            el::DOC_TYPE_READ_VERSION => header.doc_type_read_version = elem.data_u64(),

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
            el::SEGMENT_UID => seg_info.uid = elem.data_binary(),
            el::SEGMENT_FILENAME => seg_info.filename = elem.data_utf8()?,
            el::PREV_UID => seg_info.prev_uid = elem.data_binary(),
            el::PREV_FILENAME => seg_info.prev_filename = elem.data_utf8()?,
            el::NEXT_UID => seg_info.next_uid = elem.data_binary(),
            el::NEXT_FILENAME => seg_info.next_filename = elem.data_utf8()?,
            el::SEGMENT_FAMILY => seg_info.family = elem.data_binary(),
            el::TIMECODE_SCALE => seg_info.timecode_scale = elem.data_u64(),
            el::DURATION => seg_info.duration = elem.data_f32(),
            el::DATE_UTC => seg_info.date_utc = elem.data_i64(),
            el::TITLE => seg_info.title = elem.data_utf8()?,
            el::MUXING_APP => seg_info.muxing_app = elem.data_utf8()?,
            el::WRITING_APP => seg_info.writing_app = elem.data_utf8()?,

            _ => {}
        };
    }

    Ok(seg_info)
}

/// Read and parse track information.
pub fn track_info<R: Read + Sized>(r: &mut R) -> Result<TrackInfo> {
    let mut count = 0 as usize;
    let mut tracks = Vec::new();

    let (track_info, _) = r.read_ebml_element_info()?;

    while count < track_info.size() {
        let (track, c) = track(r)?;
        count += c;

        tracks.push(track);
    }

    Ok(tracks)
}

/// Read and parse a single track entry.
pub fn track<R: Read + Sized>(r: &mut R) -> Result<(Track, usize)> {
    let mut count = 0 as usize;
    let mut track = Track::default();

    let (track_master, c) = r.read_ebml_element_info()?;

    while count < track_master.size() {
        let (elem, c) = r.read_ebml_element()?;
        count += c;

        match elem.info().id() {
            el::TRACK_NUMBER => track.number = elem.data_u64(),
            el::TRACK_UID => track.uid = elem.data_binary(),
            el::TRACK_TYPE => track.track_type = elem.data_u64(),
            el::FLAG_ENABLED => track.flag_enabled = elem.data_u64(),
            el::FLAG_DEFAULT => track.flag_default = elem.data_u64(),
            el::FLAG_FORCED => track.flag_forced = elem.data_u64(),
            el::FLAG_LACING => track.flag_lacing = elem.data_u64(),
            el::MIN_CACHE => track.min_cache = elem.data_u64(),
            el::MAX_CACHE => track.max_cache = elem.data_u64(),
            el::DEFAULT_DURATION => track.default_duration = elem.data_u64(),
            el::DEFAULT_DECODED_FIELD_DURATION => {
                track.default_decoded_field_duration = elem.data_u64()
            }
            el::TRACK_TIMECODE_SCALE => track.track_timecode_scale = elem.data_f32(),
            el::TRACK_OFFSET => track.track_offset = elem.data_i64(),
            el::MAX_BLOCK_ADDITION_ID => track.max_block_addition_id = elem.data_u64(),
            el::NAME => track.name = elem.data_utf8()?,
            el::LANGUAGE => track.language = elem.data_utf8()?,
            el::CODEC_ID => track.codec_id = elem.data_utf8()?,
            el::CODEC_PRIVATE => track.codec_private = elem.data_binary(),
            el::CODEC_NAME => track.codec_name = elem.data_utf8()?,
            el::ATTACHMENT_LINK => track.attachment_link = elem.data_u64(),
            el::CODEC_SETTINGS => track.codec_settings = elem.data_utf8()?,
            el::CODEC_INFO_URL => track.codec_info_url = elem.data_utf8()?,
            el::CODEC_DOWNLOAD_URL => track.codec_download_url = elem.data_utf8()?,
            el::CODEC_DECODE_ALL => track.codec_decode_all = elem.data_u64(),
            el::TRACK_OVERLAY => track.track_overlay = elem.data_u64(),
            el::CODEC_DELAY => track.codec_delay = elem.data_u64(),
            el::SEEK_PRE_ROLL => track.seek_pre_roll = elem.data_u64(),

            _ => {}
        }
    }

    count += c;
    Ok((track, count))
}
