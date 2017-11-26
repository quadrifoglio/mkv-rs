//! MKV Data Structures.

use ebml::element::types::*;

/// Seek entry to an EBML Element.
#[derive(Default)]
pub struct SeekEntry {
    pub(crate) id: Binary,
    pub(crate) position: UnsignedInt,
}

/// Contains miscellaneous general information and statistics on the file.
pub struct SegmentInfo {
    pub(crate) uid: Binary,
    pub(crate) segment_filename: Utf8,
    pub(crate) prev_uid: Binary,
    pub(crate) prev_filename: Utf8,
    pub(crate) next_uid: Binary,
    pub(crate) next_filename: Utf8,
    pub(crate) segment_familys: Vec<Binary>,
    pub(crate) chapter_translates: Vec<ChapterTranslate>,
    pub(crate) timecode_scale: UnsignedInt,
    pub(crate) duration: Float,
    pub(crate) date_utc: Binary,
    pub(crate) title: Utf8,
    pub(crate) muxing_app: Utf8,
    pub(crate) writing_app: Utf8,
}

impl ::std::default::Default for SegmentInfo {
    fn default() -> Self {
        SegmentInfo {
            uid: Binary::default(),
            segment_filename: Utf8::default(),
            prev_uid: Binary::default(),
            prev_filename: Utf8::default(),
            next_uid: Binary::default(),
            next_filename: Utf8::default(),
            segment_familys: Vec::new(),
            chapter_translates: Vec::new(),
            timecode_scale: 1000000 as UnsignedInt,
            duration: Float::default(),
            date_utc: Binary::default(),
            title: Utf8::default(),
            muxing_app: Utf8::default(),
            writing_app: Utf8::default(),
        }
    }
}

/// A tuple of corresponding ID used by chapter codecs to represent a segment.
#[derive(Default)]
pub struct ChapterTranslate {
    pub(crate) edition_uids: Vec<UnsignedInt>,
    pub(crate) codec: UnsignedInt,
    pub(crate) id: Binary,
}

/// Contains information about a track.
pub struct Track {
    pub(crate) number: UnsignedInt,
    pub(crate) uid: UnsignedInt,
    pub(crate) track_type: UnsignedInt,
    pub(crate) flag_enabled: UnsignedInt,
    pub(crate) flag_default: UnsignedInt,
    pub(crate) flag_forced: UnsignedInt,
    pub(crate) flag_lacing: UnsignedInt,
    pub(crate) min_cache: UnsignedInt,
    pub(crate) max_cache: UnsignedInt,
    pub(crate) default_duration: UnsignedInt,
    pub(crate) default_decoded_field_duration: UnsignedInt,
    pub(crate) track_timecode_scale: Float,
    pub(crate) track_offset: SignedInt,
    pub(crate) max_block_addition_id: UnsignedInt,
    pub(crate) name: Utf8,
    pub(crate) language: Utf8,
    pub(crate) codec_id: Utf8,
    pub(crate) codec_private: Binary,
    pub(crate) codec_name: Utf8,
    pub(crate) attachment_link: UnsignedInt,
    pub(crate) codec_settings: Utf8,
    pub(crate) codec_info_url: Utf8,
    pub(crate) codec_download_url: Utf8,
    pub(crate) codec_decode_all: UnsignedInt,
    pub(crate) track_overlay: UnsignedInt,
    pub(crate) codec_delay: UnsignedInt,
    pub(crate) seek_pre_roll: UnsignedInt,

    pub(crate) video: Option<TrackVideo>,
    pub(crate) audio: Option<TrackAudio>,
}

impl ::std::default::Default for Track {
    fn default() -> Self {
        Track {
            number: UnsignedInt::default(),
            uid: UnsignedInt::default(),
            track_type: UnsignedInt::default(),
            flag_enabled: 1,
            flag_default: 1,
            flag_forced: UnsignedInt::default(),
            flag_lacing: 1,
            min_cache: UnsignedInt::default(),
            max_cache: UnsignedInt::default(),
            default_duration: UnsignedInt::default(),
            default_decoded_field_duration: UnsignedInt::default(),
            track_timecode_scale: Float::default(),
            track_offset: SignedInt::default(),
            max_block_addition_id: UnsignedInt::default(),
            name: Utf8::default(),
            language: String::from("eng"),
            codec_id: Utf8::default(),
            codec_private: Binary::default(),
            codec_name: Utf8::default(),
            attachment_link: UnsignedInt::default(),
            codec_settings: Utf8::default(),
            codec_info_url: Utf8::default(),
            codec_download_url: Utf8::default(),
            codec_decode_all: 1,
            track_overlay: UnsignedInt::default(),
            codec_delay: UnsignedInt::default(),
            seek_pre_roll: UnsignedInt::default(),
            video: None,
            audio: None,
        }
    }
}

/// Video-specific track information.
pub struct TrackVideo {
    pub(crate) flag_interlaced: UnsignedInt,
    pub(crate) field_order: UnsignedInt,
    pub(crate) stereo_mode: UnsignedInt,
    pub(crate) alpha_mode: UnsignedInt,
    pub(crate) old_stereo_mode: UnsignedInt,
    pub(crate) pixel_width: UnsignedInt,
    pub(crate) pixel_height: UnsignedInt,
    pub(crate) pixel_crop_bottom: UnsignedInt,
    pub(crate) pixel_crop_top: UnsignedInt,
    pub(crate) pixel_crop_left: UnsignedInt,
    pub(crate) pixel_crop_right: UnsignedInt,
    pub(crate) display_width: UnsignedInt,
    pub(crate) display_height: UnsignedInt,
    pub(crate) display_unit: UnsignedInt,
    pub(crate) aspect_ratio_type: UnsignedInt,
    pub(crate) colour_space: Binary,
    pub(crate) gamma_value: Float,
    pub(crate) frame_rate: Float,

    pub(crate) color: Option<TrackVideoColor>,
}

impl ::std::default::Default for TrackVideo {
    fn default() -> Self {
        TrackVideo {
            flag_interlaced: UnsignedInt::default(),
            field_order: 2,
            stereo_mode: UnsignedInt::default(),
            alpha_mode: UnsignedInt::default(),
            old_stereo_mode: UnsignedInt::default(),
            pixel_width: UnsignedInt::default(),
            pixel_height: UnsignedInt::default(),
            pixel_crop_bottom: UnsignedInt::default(),
            pixel_crop_top: UnsignedInt::default(),
            pixel_crop_left: UnsignedInt::default(),
            pixel_crop_right: UnsignedInt::default(),
            display_width: UnsignedInt::default(),
            display_height: UnsignedInt::default(),
            display_unit: UnsignedInt::default(),
            aspect_ratio_type: UnsignedInt::default(),
            colour_space: Binary::default(),
            gamma_value: Float::default(),
            frame_rate: Float::default(),
            color: None,
        }
    }
}

/// Color information for a video track.
pub struct TrackVideoColor {
    pub(crate) matrix_coefficients: UnsignedInt,
    pub(crate) bits_per_channel: UnsignedInt,
    pub(crate) chroma_subsampling_horz: UnsignedInt,
    pub(crate) chroma_subsampling_vert: UnsignedInt,
    pub(crate) cb_subsampling_horz: UnsignedInt,
    pub(crate) cb_subsampling_vert: UnsignedInt,
    pub(crate) chroma_siting_horz: UnsignedInt,
    pub(crate) chroma_siting_vert: UnsignedInt,
    pub(crate) range: UnsignedInt,
    pub(crate) transfer_characteristics: UnsignedInt,
    pub(crate) primaries: UnsignedInt,
    pub(crate) max_cll: UnsignedInt,
    pub(crate) max_fall: UnsignedInt,
    pub(crate) mastering_data: Option<TrackVideoColorMasteringData>
}

impl ::std::default::Default for TrackVideoColor {
    fn default() -> Self {
        TrackVideoColor {
            matrix_coefficients: 2,
            bits_per_channel: UnsignedInt::default(),
            chroma_subsampling_horz: UnsignedInt::default(),
            chroma_subsampling_vert: UnsignedInt::default(),
            cb_subsampling_horz: UnsignedInt::default(),
            cb_subsampling_vert: UnsignedInt::default(),
            chroma_siting_horz: UnsignedInt::default(),
            chroma_siting_vert: UnsignedInt::default(),
            range: UnsignedInt::default(),
            transfer_characteristics: 2,
            primaries: UnsignedInt::default(),
            max_cll: UnsignedInt::default(),
            max_fall: UnsignedInt::default(),
            mastering_data: None,
        }
    }
}

/// Color mastering metadata for a video track.
#[derive(Default)]
pub struct TrackVideoColorMasteringData {
    pub(crate) primary_r_chromaticity_x: Float,
    pub(crate) primary_r_chromaticity_y: Float,
    pub(crate) primary_g_chromaticity_x: Float,
    pub(crate) primary_g_chromaticity_y: Float,
    pub(crate) primary_b_chromaticity_x: Float,
    pub(crate) primary_b_chromaticity_y: Float,
    pub(crate) white_point_chromaticity_x: Float,
    pub(crate) white_point_chromaticity_y: Float,
    pub(crate) luminance_max: Float,
    pub(crate) luminance_min: Float,
}

/// Audio-specific track information.
pub struct TrackAudio {
    pub(crate) sampling_frequency: Float,
    pub(crate) output_sampling_frequency: Float,
    pub(crate) Channels: UnsignedInt,
    pub(crate) channel_positions: Binary,
    pub(crate) bit_depth: UnsignedInt,
}

impl ::std::default::Default for TrackAudio {
    fn default() -> Self {
        TrackAudio {
            sampling_frequency: 8000.0,
            output_sampling_frequency: 8000.0,
            Channels: 1,
            channel_positions: Binary::default(),
            bit_depth: UnsignedInt::default(),
        }
    }
}
