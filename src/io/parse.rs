/// This module contains all the parsing functionality.

use std::io::Read;

use ebml::io::ReadEbml;

use elements as el;
use error::Result;
use structures::*;

/// Read and parse an EBML header from an input source.
/// Returns the parsed object and the number of bytes that were read.
pub fn header<R: Read + Sized>(r: &mut R) -> Result<(Header, usize)> {
    let mut count = 0 as usize;
    let (elem, c) = r.read_ebml_element_info()?;

    let mut header = Header::default();

    while count < elem.size() {
        let (elem, c) = r.read_ebml_element()?;
        count += c;

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

    count += c;
    Ok((header, count))
}

/// Read and parse MKV seek information.
/// Returns the parsed object and the number of bytes that were read.
pub fn seek_info<R: Read + Sized>(r: &mut R) -> Result<(SeekInfo, usize)> {
    let mut entries = Vec::new();
    let mut count = 0 as usize;

    let (seek_head, c) = r.read_ebml_element_info()?;

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

    count += c;
    Ok((entries, count))
}

/// Read and parse MKV segment information.
/// Returns the parsed object and the number of bytes that were read.
pub fn segment_info<R: Read + Sized>(r: &mut R) -> Result<(SegmentInfo, usize)> {
    let mut count = 0 as usize;
    let mut seg_info = SegmentInfo::default();

    let (segment_info, c) = r.read_ebml_element_info()?;

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

    count += c;
    Ok((seg_info, count))
}

/// Read and parse track information.
/// Returns the parsed object and the number of bytes that were read.
pub fn track_info<R: Read + Sized>(r: &mut R) -> Result<(TrackInfo, usize)> {
    let mut count = 0 as usize;
    let mut tracks = Vec::new();

    let (track_info, c) = r.read_ebml_element_info()?;

    while count < track_info.size() {
        let (track, c) = track(r)?;
        count += c;

        tracks.push(track);
    }

    count += c;
    Ok((tracks, count))
}

/// Read and parse a single track entry.
/// Returns the parsed object and the number of bytes that were read.
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

/// Read and parse track video information.
/// Returns the parsed object and the number of bytes that were read.
pub fn track_video<R: Read + Sized>(r: &mut R) -> Result<(VideoTrack, usize)> {
    let mut count = 0 as usize;
    let mut video = VideoTrack::default();

    let (video_master, c) = r.read_ebml_element_info()?;

    while count < video_master.size() {
        let (elem, c) = r.read_ebml_element()?;
        count += c;

        match elem.info().id() {
            el::FLAG_INTERLACED => video.flag_interlaced = elem.data_u64(),
            el::FIELD_ORDER => video.field_order = elem.data_u64(),
            el::STEREO_MODE => video.stereo_mode = elem.data_u64(),
            el::ALPHA_MODE => video.alpha_mode = elem.data_u64(),
            el::OLD_STEREO_MODE => video.old_stereo_mode = elem.data_u64(),
            el::PIXEL_WIDTH => video.pixel_width = elem.data_u64(),
            el::PIXEL_HEIGHT => video.pixel_height = elem.data_u64(),
            el::PIXEL_CROP_BOTTOM => video.pixel_crop_bottom = elem.data_u64(),
            el::PIXEL_CROP_TOP => video.pixel_crop_top = elem.data_u64(),
            el::PIXEL_CROP_LEFT => video.pixel_crop_left = elem.data_u64(),
            el::PIXEL_CROP_RIGHT => video.pixel_crop_right = elem.data_u64(),
            el::DISPLAY_WIDTH => video.display_width = elem.data_u64(),
            el::DISPLAY_HEIGHT => video.display_height = elem.data_u64(),
            el::DISPLAY_UNIT => video.display_unit = elem.data_u64(),
            el::ASPECT_RATIO_TYPE => video.aspect_ratio_type = elem.data_u64(),
            el::COLOUR_SPACE => video.colour_space = elem.data_binary(),
            el::GAMMA_VALUE => video.gamma_value = elem.data_f32(),
            el::FRAME_RATE => video.frame_rate = elem.data_f32(),

            _ => {}
        };
    }

    count += c;
    Ok((video, count))
}

/// Read and parse video track color information.
/// Returns the parsed object and the number of bytes that were read.
pub fn track_video_color<R: Read + Sized>(r: &mut R) -> Result<(VideoColor, usize)> {
    let mut count = 0 as usize;
    let mut color = VideoColor::default();

    let (color_master, c) = r.read_ebml_element_info()?;

    while count < color_master.size() {
        let (elem, c) = r.read_ebml_element()?;
        count += c;

        match elem.info().id() {
            el::MATRIX_COEFFICIENTS => color.matrix_coefficients = elem.data_u64(),
            el::BITS_PER_CHANNEL => color.bits_per_channel = elem.data_u64(),
            el::CHROMA_SUBSAMPLING_HORZ => color.chroma_subsampling_horz = elem.data_u64(),
            el::CHROMA_SUBSAMPLING_VERT => color.chroma_subsampling_vert = elem.data_u64(),
            el::CB_SUBSAMPLING_HORZ => color.cb_subsampling_horz = elem.data_u64(),
            el::CB_SUBSAMPLING_VERT => color.cb_subsampling_vert = elem.data_u64(),
            el::CHROMA_SITING_HORZ => color.chroma_siting_horz = elem.data_u64(),
            el::CHROMA_SITING_VERT => color.chroma_siting_vert = elem.data_u64(),
            el::RANGE => color.range = elem.data_u64(),
            el::TRANSFER_CHARACTERISTICS => color.transfer_characteristics = elem.data_u64(),
            el::PRIMARIES => color.primaries = elem.data_u64(),
            el::MAX_CLL => color.max_cll = elem.data_u64(),
            el::MAX_FALL => color.max_fall = elem.data_u64(),

            _ => {}
        };
    }

    count += c;
    Ok((color, count))
}

/// Read and parse video track color mastering metadata.
/// Returns the parsed object and the number of bytes that were read.
pub fn track_video_color_mastering_data<R: Read + Sized>(
    r: &mut R,
) -> Result<(VideoColorMasteringData, usize)> {
    let mut count = 0 as usize;
    let mut mastering_data = VideoColorMasteringData::default();

    let (mastering_data_master, c) = r.read_ebml_element_info()?;

    while count < mastering_data_master.size() {
        let (elem, c) = r.read_ebml_element()?;
        count += c;

        match elem.info().id() {
            el::PRIMARY_RCHROMATICITY_X => mastering_data.primary_rchromaticity_x = elem.data_f64(),
            el::PRIMARY_RCHROMATICITY_Y => mastering_data.primary_rchromaticity_y = elem.data_f64(),
            el::PRIMARY_GCHROMATICITY_X => mastering_data.primary_gchromaticity_x = elem.data_f64(),
            el::PRIMARY_GCHROMATICITY_Y => mastering_data.primary_gchromaticity_y = elem.data_f64(),
            el::PRIMARY_BCHROMATICITY_X => mastering_data.primary_bchromaticity_x = elem.data_f64(),
            el::PRIMARY_BCHROMATICITY_Y => mastering_data.primary_bchromaticity_y = elem.data_f64(),
            el::WHITE_POINT_CHROMATICITY_X => {
                mastering_data.white_point_chromaticity_x = elem.data_f64()
            }
            el::WHITE_POINT_CHROMATICITY_Y => {
                mastering_data.white_point_chromaticity_y = elem.data_f64()
            }
            el::LUMINANCE_MAX => mastering_data.luminance_max = elem.data_f64(),
            el::LUMINANCE_MIN => mastering_data.luminance_min = elem.data_f64(),

            _ => {}
        };
    }

    count += c;
    Ok((mastering_data, count))
}

/// Read and parse audio track information.
/// Returns the parsed object and the number of bytes that were read.
pub fn track_audio<R: Read + Sized>(r: &mut R) -> Result<(AudioTrack, usize)> {
    let mut count = 0 as usize;
    let mut audio = AudioTrack::default();

    let (video_master, c) = r.read_ebml_element_info()?;

    while count < video_master.size() {
        let (elem, c) = r.read_ebml_element()?;
        count += c;

        match elem.info().id() {
            el::SAMPLING_FREQUENCY => audio.sampling_frequency = elem.data_f32(),
            el::OUTPUT_SAMPLING_FREQUENCY => audio.output_sampling_frequency = elem.data_f32(),
            el::CHANNELS => audio.channels = elem.data_u64(),
            el::CHANNEL_POSITIONS => audio.channel_positions = elem.data_binary(),
            el::BIT_DEPTH => audio.bit_depth = elem.data_u64(),

            _ => {}
        };
    }

    count += c;
    Ok((audio, count))
}
