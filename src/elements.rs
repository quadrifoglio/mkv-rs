//! Contains all the EBML element definitions for MKV.

use ebml::common::types::UnsignedInt;

// Root element.

pub const SEGMENT: UnsignedInt = 0x18538067;

// Meta Seek Information.

pub const SEEK_HEAD: UnsignedInt = 0x114D9B74;
pub const SEEK: UnsignedInt = 0x4DBB;
pub const SEEK_ID: UnsignedInt = 0x53AB;
pub const SEEK_POSITION: UnsignedInt = 0x53AC;

// Segment Information.

pub const INFO: UnsignedInt = 0x1549A966;
pub const SEGMENT_UID: UnsignedInt = 0x73A4;
pub const SEGMENT_FILENAME: UnsignedInt = 0x7384;
pub const PREV_UID: UnsignedInt = 0x3CB923;
pub const PREV_FILENAME: UnsignedInt = 0x3C83AB;
pub const NEXT_UID: UnsignedInt = 0x3EB923;
pub const NEXT_FILENAME: UnsignedInt = 0x3E83BB;
pub const SEGMENT_FAMILY: UnsignedInt = 0x4444;
pub const CHAPTER_TRANSLATE: UnsignedInt = 0x6924;
pub const CHAPTER_TRANSLATE_EDITION_UID: UnsignedInt = 0x69FC;
pub const CHAPTER_TRANSLATE_CODEC: UnsignedInt = 0x69BF;
pub const CHAPTER_TRANSLATE_ID: UnsignedInt = 0x69A5;
pub const TIMECODE_SCALE: UnsignedInt = 0x2AD7B1;
pub const DURATION: UnsignedInt = 0x4489;
pub const DATE_UTC: UnsignedInt = 0x4461;
pub const TITLE: UnsignedInt = 0x7BA9;
pub const MUXING_APP: UnsignedInt = 0x4D80;
pub const WRITING_APP: UnsignedInt = 0x5741;

// Track information.

pub const TRACKS: UnsignedInt = 0x1654AE6B;
pub const TRACK_ENTRY: UnsignedInt = 0xAE;

pub const TRACK_NUMBER: UnsignedInt = 0xD7;
pub const TRACK_UID: UnsignedInt = 0x73C5;
pub const TRACK_TYPE: UnsignedInt = 0x83;
pub const FLAG_ENABLED: UnsignedInt = 0xB9;
pub const FLAG_DEFAULT: UnsignedInt = 0x88;
pub const FLAG_FORCED: UnsignedInt = 0x55AA;
pub const FLAG_LACING: UnsignedInt = 0x9C;
pub const MIN_CACHE: UnsignedInt = 0x6DE7;
pub const MAX_CACHE: UnsignedInt = 0x6DF8;
pub const DEFAULT_DURATION: UnsignedInt = 0x23E383;
pub const DEFAULT_DECODED_FIELD_DURATION: UnsignedInt = 0x234E7A;
pub const TRACK_TIMECODE_SCALE: UnsignedInt = 0x23314F;
pub const TRACK_OFFSET: UnsignedInt = 0x537F;
pub const MAX_BLOCK_ADDITION_ID: UnsignedInt = 0x55EE;
pub const NAME: UnsignedInt = 0x536E;
pub const LANGUAGE: UnsignedInt = 0x22B59C;
pub const CODEC_ID: UnsignedInt = 0x86;
pub const CODEC_PRIVATE: UnsignedInt = 0x63A2;
pub const CODEC_NAME: UnsignedInt = 0x258688;
pub const ATTACHMENT_LINK: UnsignedInt = 0x7446;
pub const CODEC_SETTINGS: UnsignedInt = 0x3A9697;
pub const CODEC_INFO_URL: UnsignedInt = 0x3B4040;
pub const CODEC_DOWNLOAD_URL: UnsignedInt = 0x26B240;
pub const CODEC_DECODE_ALL: UnsignedInt = 0xAA;
pub const TRACK_OVERLAY: UnsignedInt = 0x6FAB;
pub const CODEC_DELAY: UnsignedInt = 0x56AA;
pub const SEEK_PRE_ROLL: UnsignedInt = 0x56BB;

pub const TRACK_TRANSLATE: UnsignedInt = 0x6624;
pub const TRACK_TRANSLATE_EDITION_UID: UnsignedInt = 0x66FC;
pub const TRACK_TRANSLATE_CODEC: UnsignedInt = 0x66BF;
pub const TRACK_TRANSLATE_TRACK_ID: UnsignedInt = 0x66A5;

pub const VIDEO: UnsignedInt = 0xE0;
pub const FLAG_INTERLACED: UnsignedInt = 0x9A;
pub const FIELD_ORDER: UnsignedInt = 0x9D;
pub const STEREO_MODE: UnsignedInt = 0x53B8;
pub const ALPHA_MODE: UnsignedInt = 0x53C0;
pub const OLD_STEREO_MODE: UnsignedInt = 0x53B9;
pub const PIXEL_WIDTH: UnsignedInt = 0xB0;
pub const PIXEL_HEIGHT: UnsignedInt = 0xBA;
pub const PIXEL_CROP_BOTTOM: UnsignedInt = 0x54AA;
pub const PIXEL_CROP_TOP: UnsignedInt = 0x54BB;
pub const PIXEL_CROP_LEFT: UnsignedInt = 0x54CC;
pub const PIXEL_CROP_RIGHT: UnsignedInt = 0x54DD;
pub const DISPLAY_WIDTH: UnsignedInt = 0x54B0;
pub const DISPLAY_HEIGHT: UnsignedInt = 0x54BA;
pub const DISPLAY_UNIT: UnsignedInt = 0x54B2;
pub const ASPECT_RATIO_TYPE: UnsignedInt = 0x54B3;
pub const COLOUR_SPACE: UnsignedInt = 0x2EB524;
pub const GAMMA_VALUE: UnsignedInt = 0x2FB523;
pub const FRAME_RATE: UnsignedInt = 0x2383E3;

pub const COLOUR: UnsignedInt = 0x55B0;
pub const MATRIX_COEFFICIENTS: UnsignedInt = 0x55B1;
pub const BITS_PER_CHANNEL: UnsignedInt = 0x55B2;
pub const CHROMA_SUBSAMPLING_HORZ: UnsignedInt = 0x55B3;
pub const CHROMA_SUBSAMPLING_VERT: UnsignedInt = 0x55B4;
pub const CB_SUBSAMPLING_HORZ: UnsignedInt = 0x55B5;
pub const CB_SUBSAMPLING_VERT: UnsignedInt = 0x55B6;
pub const CHROMA_SITING_HORZ: UnsignedInt = 0x55B7;
pub const CHROMA_SITING_VERT: UnsignedInt = 0x55B8;
pub const RANGE: UnsignedInt = 0x55B9;
pub const TRANSFER_CHARACTERISTICS: UnsignedInt = 0x55BA;
pub const PRIMARIES: UnsignedInt = 0x55BB;
pub const MAX_CLL: UnsignedInt = 0x55BC;
pub const MAX_FALL: UnsignedInt = 0x55BD;

pub const MASTERING_METADATA: UnsignedInt = 0x55D0;
pub const PRIMARY_RCHROMATICITY_X: UnsignedInt = 0x55D1;
pub const PRIMARY_RCHROMATICITY_Y: UnsignedInt = 0x55D2;
pub const PRIMARY_GCHROMATICITY_X: UnsignedInt = 0x55D3;
pub const PRIMARY_GCHROMATICITY_Y: UnsignedInt = 0x55D4;
pub const PRIMARY_BCHROMATICITY_X: UnsignedInt = 0x55D5;
pub const PRIMARY_BCHROMATICITY_Y: UnsignedInt = 0x55D6;
pub const WHITE_POINT_CHROMATICITY_X: UnsignedInt = 0x55D7;
pub const WHITE_POINT_CHROMATICITY_Y: UnsignedInt = 0x55D8;
pub const LUMINANCE_MAX: UnsignedInt = 0x55D9;
pub const LUMINANCE_MIN: UnsignedInt = 0x55DA;

pub const AUDIO: UnsignedInt = 0xe1;
pub const SAMPLING_FREQUENCY: UnsignedInt = 0xb5;
pub const OUTPUT_SAMPLING_FREQUENCY: UnsignedInt = 0x78B5;
pub const CHANNELS: UnsignedInt = 0x9F;
pub const CHANNEL_POSITIONS: UnsignedInt = 0x7D7B;
pub const BIT_DEPTH: UnsignedInt = 0x6264;

pub const TRACK_OPERATION: UnsignedInt = 0xe2;

pub const TRACK_COMBINE_PLANES: UnsignedInt = 0xe3;
pub const TRACK_PLANE: UnsignedInt = 0xe4;
pub const TRACK_PLANE_UID: UnsignedInt = 0xe5;
pub const TRACK_PLANE_TYPE: UnsignedInt = 0xe6;

pub const TRACK_JOIN_BLOCKS: UnsignedInt = 0xe9;
pub const TRACK_JOIN_UID: UnsignedInt = 0xeD;
pub const TRICK_TRACK_UID: UnsignedInt = 0xc0;
pub const TRICK_TRACK_SEGMENT_UID: UnsignedInt = 0xc1;
pub const TRICK_TRACK_FLAG: UnsignedInt = 0xc6;
pub const TRICK_MASTER_TRACK_UID: UnsignedInt = 0xc7;
pub const TRICK_MASTER_TRACK_SEGMENT_UID: UnsignedInt = 0xc4;

pub const CONTENT_ENCODINGS: UnsignedInt = 0x6D80;
pub const CONTENT_ENCODING: UnsignedInt = 0x6240;
pub const CONTENT_ENCODING_ORDER: UnsignedInt = 0x5031;
pub const CONTENT_ENCODING_SCOPE: UnsignedInt = 0x5032;
pub const CONTENT_ENCODING_TYPE: UnsignedInt = 0x5033;

pub const CONTENT_COMPRESSION: UnsignedInt = 0x5034;
pub const CONTENT_COMP_ALGO: UnsignedInt = 0x4254;
pub const CONTENT_COMP_SETTINGS: UnsignedInt = 0x4255;

pub const CONTENT_ENCRYPTION: UnsignedInt = 0x5035;
pub const CONTENT_ENC_ALGO: UnsignedInt = 0x47E1;
pub const CONTENT_ENC_KEY_ID: UnsignedInt = 0x47E2;
pub const CONTENT_SIGNATURE: UnsignedInt = 0x47E3;
pub const CONTENT_SIG_KEY_ID: UnsignedInt = 0x47E4;
pub const CONTENT_SIG_ALGO: UnsignedInt = 0x47E5;
pub const CONTENT_SIG_HASH_ALGO: UnsignedInt = 0x47E6;

pub const CUES: UnsignedInt = 0x1C53BB6B;
pub const CUE_POINT: UnsignedInt = 0xBB;
pub const CUE_TIME: UnsignedInt = 0xB3;
pub const CUE_TRACK_POSITIONS: UnsignedInt = 0xB7;
pub const CUE_TRACK: UnsignedInt = 0xF7;
pub const CUE_CLUSTER_POSITION: UnsignedInt = 0xF1;
pub const CUE_RELATIVE_POSITION: UnsignedInt = 0xF0;
pub const CUE_DURATION: UnsignedInt = 0xB2;
pub const CUE_BLOCK_NUMBER: UnsignedInt = 0x5378;
pub const CUE_CODEC_STATE: UnsignedInt = 0xEA;
pub const CUE_REFERENCE: UnsignedInt = 0xDB;
pub const CUE_REF_TIME: UnsignedInt = 0x96;
pub const CUE_REF_CLUSTER: UnsignedInt = 0x97;
pub const CUE_REF_NUMBER: UnsignedInt = 0x535F;
pub const CUE_REF_CODEC_STATE: UnsignedInt = 0xEB;

// Cluser data.

pub const CLUSTER: UnsignedInt = 0x1F43B675;
pub const TIMECODE: UnsignedInt = 0xE7;
pub const SILENT_TRACKS: UnsignedInt = 0x5854;
pub const SILENT_TRACK_NUMBER: UnsignedInt = 0x58D7;
pub const POSITION: UnsignedInt = 0xA7;
pub const PREV_SIZE: UnsignedInt = 0xAB;
pub const SIMPLE_BLOCK: UnsignedInt = 0xA3;
pub const BLOCK_GROUP: UnsignedInt = 0xA0;
pub const BLOCK: UnsignedInt = 0xA1;
pub const BLOCK_VIRTUAL: UnsignedInt = 0xA2;
pub const BLOCK_ADDITIONS: UnsignedInt = 0x75A1;
pub const BLOCK_MORE: UnsignedInt = 0xA6;
pub const BLOCK_ADD_ID: UnsignedInt = 0xEE;
pub const BLOCK_ADDITIONAL: UnsignedInt = 0xA5;
pub const BLOCK_DURATION: UnsignedInt = 0x9B;
pub const REFERENCE_PRIORITY: UnsignedInt = 0xFA;
pub const REFERENCE_BLOCK: UnsignedInt = 0xFB;
pub const REFERENCE_VIRTUAL: UnsignedInt = 0xFD;
pub const CODEC_STATE: UnsignedInt = 0xA4;
pub const DISCARD_PADDING: UnsignedInt = 0x75A2;
pub const SLICES: UnsignedInt = 0x8E;
pub const TIME_SLICE: UnsignedInt = 0xE8;
pub const LACE_NUMBER: UnsignedInt = 0xCC;
pub const FRAME_NUMBER: UnsignedInt = 0xCD;
pub const BLOCK_ADDITIONID: UnsignedInt = 0xCB;
pub const DELAY: UnsignedInt = 0xCE;
pub const SLICE_DURATION: UnsignedInt = 0xCF;
pub const REFERENCE_FRAME: UnsignedInt = 0xC8;
pub const REFERENCE_OFFSET: UnsignedInt = 0xC9;
pub const REFERENCE_TIMECODE: UnsignedInt = 0xCA;
pub const ENCRYPTED_BLOCK: UnsignedInt = 0xAF;
