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
    pub writing_app: String
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
