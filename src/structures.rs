/// This module contains all the structures that can be contained withing an MKV file.

/// Represents an MKV/EBML header.
/// Documentation for each field signification can be found on the official Matroska website.
#[derive(Default)]
pub struct Header {
    pub ebml_version: u64,
    pub ebml_read_version: u64,
    pub ebml_max_id_length: u64,
    pub ebml_max_size_length: u64,
    pub doc_type: String,
    pub doc_type_version: u64,
    pub doc_type_read_version: u64,
}

/// Represents MKV seek information data.
pub type SeekInfo = Vec<SeekEntry>;

/// Represents an MKV seek entry to an element.
pub struct SeekEntry {
    pub seek_id: Vec<u8>,
    pub seek_position: u64,
}

/// Contains information about an MKV segment.
#[derive(Default)]
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
    pub writing_app: String,
}

/// List of tracks contained in an MKV file.
pub type TrackInfo = Vec<Track>;

/// Informations about a media track.
#[derive(Default)]
pub struct Track {
    pub number: u64,
    pub uid: Vec<u8>,
    pub track_type: u64,
    pub flag_enabled: u64,
    pub flag_default: u64,
    pub flag_forced: u64,
    pub flag_lacing: u64,
    pub min_cache: u64,
    pub max_cache: u64,
    pub default_duration: u64,
    pub default_decoded_field_duration: u64,
    pub track_timecode_scale: f32,
    pub track_offset: i64,
    pub max_block_addition_id: u64,
    pub name: String,
    pub language: String,
    pub codec_id: String,
    pub codec_private: Vec<u8>,
    pub codec_name: String,
    pub attachment_link: u64,
    pub codec_settings: String,
    pub codec_info_url: String,
    pub codec_download_url: String,
    pub codec_decode_all: u64,
    pub track_overlay: u64,
    pub codec_delay: u64,
    pub seek_pre_roll: u64,
}

/// Informations about a video track.
#[derive(Default)]
pub struct VideoTrack {
    pub flag_interlaced: u64,
    pub field_order: u64,
    pub stereo_mode: u64,
    pub alpha_mode: u64,
    pub old_stereo_mode: u64,
    pub pixel_width: u64,
    pub pixel_height: u64,
    pub pixel_crop_bottom: u64,
    pub pixel_crop_top: u64,
    pub pixel_crop_left: u64,
    pub pixel_crop_right: u64,
    pub display_width: u64,
    pub display_height: u64,
    pub display_unit: u64,
    pub aspect_ratio_type: u64,
    pub colour_space: Vec<u8>,
    pub gamma_value: f32,
    pub frame_rate: f32,
}

/// Informations about colors in a video track.
#[derive(Default)]
pub struct VideoColor {
    pub matrix_coefficients: u64,
    pub bits_per_channel: u64,
    pub chroma_subsampling_horz: u64,
    pub chroma_subsampling_vert: u64,
    pub cb_subsampling_horz: u64,
    pub cb_subsampling_vert: u64,
    pub chroma_siting_horz: u64,
    pub chroma_siting_vert: u64,
    pub range: u64,
    pub transfer_characteristics: u64,
    pub primaries: u64,
    pub max_cll: u64,
    pub max_fall: u64,
}

/// Video colors mastering metadata.
#[derive(Default)]
pub struct VideoColorMasteringData {
    pub primary_rchromaticity_x: f64,
    pub primary_rchromaticity_y: f64,
    pub primary_gchromaticity_x: f64,
    pub primary_gchromaticity_y: f64,
    pub primary_bchromaticity_x: f64,
    pub primary_bchromaticity_y: f64,
    pub white_point_chromaticity_x: f64,
    pub white_point_chromaticity_y: f64,
    pub luminance_max: f64,
    pub luminance_min: f64,
}

/// Informations about an audio track.
#[derive(Default)]
pub struct AudioTrack {
    pub sampling_frequency: f32,
    pub output_sampling_frequency: f32,
    pub channels: u64,
    pub channel_positions: Vec<u8>,
    pub bit_depth: u64,
}
