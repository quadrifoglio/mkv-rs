//! This module defines all the MKV/EBML elements that are used in Matroska files.

use ebml::element::types::*;

// Meta Seek Information.

pub const SEEK_HEAD: UnsignedInt = 0x114D9B74;
pub const SEEK: UnsignedInt = 0x4DBB;
pub const SEEK_ID: UnsignedInt = 0x53AB;
pub const SEEK_POSITION: UnsignedInt = 0x53AC;

ebml_element_container!(SeekHead => SEEK_HEAD);
ebml_element_container!(Seek => SEEK);
ebml_element_mandatory!(SeekID => SEEK_ID, Binary);
ebml_element_mandatory!(SeekPosition => SEEK_POSITION, UnsignedInt);

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

ebml_element_container!(Info => INFO);

ebml_element_mandatory!(SegmentUID => SEGMENT_UID, Binary);
ebml_element_mandatory!(SegmentFilename => SEGMENT_FILENAME, Utf8);
ebml_element_mandatory!(PrevUID => PREV_UID, Binary);
ebml_element_mandatory!(PrevFilename => PREV_FILENAME, Utf8);
ebml_element_mandatory!(NextUID => NEXT_UID, Binary);
ebml_element_mandatory!(NextFilename => NEXT_FILENAME, Utf8);
ebml_element_mandatory!(SegmentFamily => SEGMENT_FAMILY, Binary);

ebml_element_container!(ChapterTranslate => CHAPTER_TRANSLATE);
ebml_element_mandatory!(ChapterTranslateEditionUID => CHAPTER_TRANSLATE_EDITION_UID, UnsignedInt);
ebml_element_mandatory!(ChapterTranslateCodec => CHAPTER_TRANSLATE_CODEC, UnsignedInt);
ebml_element_mandatory!(ChapterTranslateID => CHAPTER_TRANSLATE_ID, Binary);

ebml_element_mandatory!(TimecodeScale => TIMECODE_SCALE, UnsignedInt);
ebml_element_mandatory!(Duration => DURATION, Float);
ebml_element_mandatory!(DateUTC => DATE_UTC, Binary);
ebml_element_mandatory!(Title => TITLE, Utf8);
ebml_element_mandatory!(MuxingApp => MUXING_APP, Utf8);
ebml_element_mandatory!(WritingApp => WRITING_APP, Utf8);

// Tracks information.

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

ebml_element_container!(Tracks => TRACKS);
ebml_element_container!(TrackEntry => TRACK_ENTRY);

ebml_element_mandatory!(TrackNumber => TRACK_NUMBER, UnsignedInt);
ebml_element_mandatory!(TrackUID => TRACK_UID, UnsignedInt);
ebml_element_mandatory!(TrackType => TRACK_TYPE, UnsignedInt);
ebml_element_mandatory!(FlagEnabled => FLAG_ENABLED, UnsignedInt);
ebml_element_mandatory!(FlagDefault => FLAG_DEFAULT, UnsignedInt);
ebml_element_mandatory!(FlagForced => FLAG_FORCED, UnsignedInt);
ebml_element_mandatory!(FlagLacing => FLAG_LACING, UnsignedInt);
ebml_element_mandatory!(MinCache => MIN_CACHE, UnsignedInt);
ebml_element_mandatory!(MaxCache => MAX_CACHE, UnsignedInt);
ebml_element_mandatory!(DefaultDuration => DEFAULT_DURATION, UnsignedInt);
ebml_element_mandatory!(DefaultDecodedFieldDuration => DEFAULT_DECODED_FIELD_DURATION, UnsignedInt);
ebml_element_mandatory!(TrackTimecodeScale => TRACK_TIMECODE_SCALE, Float);
ebml_element_mandatory!(TrackOffset => TRACK_OFFSET, SignedInt);
ebml_element_mandatory!(MaxBlockAdditionID => MAX_BLOCK_ADDITION_ID, UnsignedInt);
ebml_element_mandatory!(Name => NAME, Utf8);
ebml_element_mandatory!(Language => LANGUAGE, Utf8);
ebml_element_mandatory!(CodecID => CODEC_ID, Utf8);
ebml_element_mandatory!(CodecPrivate => CODEC_PRIVATE, Binary);
ebml_element_mandatory!(CodecName => CODEC_NAME, Utf8);
ebml_element_mandatory!(AttachmentLink => ATTACHMENT_LINK, UnsignedInt);
ebml_element_mandatory!(CodecSettings => CODEC_SETTINGS, Utf8);
ebml_element_mandatory!(CodecInfoURL => CODEC_INFO_URL, Utf8);
ebml_element_mandatory!(CodecDownloadURL => CODEC_DOWNLOAD_URL, Utf8);
ebml_element_mandatory!(CodecDecodeAll => CODEC_DECODE_ALL, UnsignedInt);
ebml_element_mandatory!(TrackOverlay => TRACK_OVERLAY, UnsignedInt);
ebml_element_mandatory!(CodecDelay => CODEC_DELAY, UnsignedInt);
ebml_element_mandatory!(SeekPreRoll => SEEK_PRE_ROLL, UnsignedInt);

ebml_element_container!(TrackTranslate => TRACK_TRANSLATE);
ebml_element_mandatory!(TrackTranslateEditionUID => TRACK_TRANSLATE_EDITION_UID, UnsignedInt);
ebml_element_mandatory!(TrackTranslateCodec => TRACK_TRANSLATE_CODEC, UnsignedInt);
ebml_element_mandatory!(TrackTranslateTrackID => TRACK_TRANSLATE_TRACK_ID, Binary);

ebml_element_container!(Video => VIDEO);
ebml_element_mandatory!(FlagInterlaced => FLAG_INTERLACED, UnsignedInt);
ebml_element_mandatory!(FieldOrder => FIELD_ORDER, UnsignedInt);
ebml_element_mandatory!(StereoMode => STEREO_MODE, UnsignedInt);
ebml_element_mandatory!(AlphaMode => ALPHA_MODE, UnsignedInt);
ebml_element_mandatory!(OldStereoMode => OLD_STEREO_MODE, UnsignedInt);
ebml_element_mandatory!(PixelWidth => PIXEL_WIDTH, UnsignedInt);
ebml_element_mandatory!(PixelHeight => PIXEL_HEIGHT, UnsignedInt);
ebml_element_mandatory!(PixelCropBottom => PIXEL_CROP_BOTTOM, UnsignedInt);
ebml_element_mandatory!(PixelCropTop => PIXEL_CROP_TOP, UnsignedInt);
ebml_element_mandatory!(PixelCropLeft => PIXEL_CROP_LEFT, UnsignedInt);
ebml_element_mandatory!(PixelCropRight => PIXEL_CROP_RIGHT, UnsignedInt);
ebml_element_mandatory!(DisplayWidth => DISPLAY_WIDTH, UnsignedInt);
ebml_element_mandatory!(DisplayHeight => DISPLAY_HEIGHT, UnsignedInt);
ebml_element_mandatory!(DisplayUnit => DISPLAY_UNIT, UnsignedInt);
ebml_element_mandatory!(AspectRatioType => ASPECT_RATIO_TYPE, UnsignedInt);
ebml_element_mandatory!(ColourSpace => COLOUR_SPACE, Binary);
ebml_element_mandatory!(GammaValue => GAMMA_VALUE, Float);
ebml_element_mandatory!(FrameRate => FRAME_RATE, Float);

ebml_element_container!(Colour => COLOUR);
ebml_element_mandatory!(MatrixCoefficients => MATRIX_COEFFICIENTS, UnsignedInt);
ebml_element_mandatory!(BitsPerChannel => BITS_PER_CHANNEL, UnsignedInt);
ebml_element_mandatory!(ChromaSubsamplingHorz => CHROMA_SUBSAMPLING_HORZ, UnsignedInt);
ebml_element_mandatory!(ChromaSubsamplingVert => CHROMA_SUBSAMPLING_VERT, UnsignedInt);
ebml_element_mandatory!(CbSubsamplingHorz => CB_SUBSAMPLING_HORZ, UnsignedInt);
ebml_element_mandatory!(CbSubsamplingVert => CB_SUBSAMPLING_VERT, UnsignedInt);
ebml_element_mandatory!(ChromaSitingHorz => CHROMA_SITING_HORZ, UnsignedInt);
ebml_element_mandatory!(ChromaSitingVert => CHROMA_SITING_VERT, UnsignedInt);
ebml_element_mandatory!(Range => RANGE, UnsignedInt);
ebml_element_mandatory!(TransferCharacteristics => TRANSFER_CHARACTERISTICS, UnsignedInt);
ebml_element_mandatory!(Primaries => PRIMARIES, UnsignedInt);
ebml_element_mandatory!(MaxCLL => MAX_CLL, UnsignedInt);
ebml_element_mandatory!(MaxFALL => MAX_FALL, UnsignedInt);

ebml_element_container!(MasteringMetadata => MASTERING_METADATA);
ebml_element_mandatory!(PrimaryRChromaticityX => PRIMARY_RCHROMATICITY_X, Float);
ebml_element_mandatory!(PrimaryRChromaticityY => PRIMARY_RCHROMATICITY_Y, Float);
ebml_element_mandatory!(PrimaryGChromaticityX => PRIMARY_GCHROMATICITY_X, Float);
ebml_element_mandatory!(PrimaryGChromaticityY => PRIMARY_GCHROMATICITY_Y, Float);
ebml_element_mandatory!(PrimaryBChromaticityX => PRIMARY_BCHROMATICITY_X, Float);
ebml_element_mandatory!(PrimaryBChromaticityY => PRIMARY_BCHROMATICITY_Y, Float);
ebml_element_mandatory!(WhitePointChromaticityX => WHITE_POINT_CHROMATICITY_X, Float);
ebml_element_mandatory!(WhitePointChromaticityY => WHITE_POINT_CHROMATICITY_Y, Float);
ebml_element_mandatory!(LuminanceMax => LUMINANCE_MAX, Float);
ebml_element_mandatory!(LuminanceMin => LUMINANCE_MIN, Float);

ebml_element_container!(Audio => AUDIO);
ebml_element_mandatory!(SamplingFrequency => SAMPLING_FREQUENCY, Float);
ebml_element_mandatory!(OutputSamplingFrequency => OUTPUT_SAMPLING_FREQUENCY, Float);
ebml_element_mandatory!(Channels => CHANNELS, UnsignedInt);
ebml_element_mandatory!(ChannelPositions => CHANNEL_POSITIONS, Binary);
ebml_element_mandatory!(BitDepth => BIT_DEPTH, UnsignedInt);

ebml_element_container!(TrackOperation => TRACK_OPERATION);

ebml_element_container!(TrackCombinePlanes => TRACK_COMBINE_PLANES);
ebml_element_container!(TrackPlane => TRACK_PLANE);
ebml_element_mandatory!(TrackPlaneUID => TRACK_PLANE_UID, UnsignedInt);
ebml_element_mandatory!(TrackPlaneType => TRACK_PLANE_TYPE, UnsignedInt);

ebml_element_container!(TrackJoinBlocks => TRACK_JOIN_BLOCKS);
ebml_element_mandatory!(TrackJoinUID => TRACK_JOIN_UID, UnsignedInt);
ebml_element_mandatory!(TrickTrackUID => TRICK_TRACK_UID, UnsignedInt);
ebml_element_mandatory!(TrickTrackSegmentUID => TRICK_TRACK_SEGMENT_UID, Binary);
ebml_element_mandatory!(TrickTrackFlag => TRICK_TRACK_FLAG, UnsignedInt);
ebml_element_mandatory!(TrickMasterTrackUID => TRICK_MASTER_TRACK_UID, UnsignedInt);
ebml_element_mandatory!(TrickMasterTrackSegmentUID => TRICK_MASTER_TRACK_SEGMENT_UID, Binary);

ebml_element_container!(ContentEncodings => CONTENT_ENCODINGS);
ebml_element_container!(ContentEncoding => CONTENT_ENCODING);
ebml_element_mandatory!(ContentEncodingOrder => CONTENT_ENCODING_ORDER, UnsignedInt);
ebml_element_mandatory!(ContentEncodingScope => CONTENT_ENCODING_SCOPE, UnsignedInt);
ebml_element_mandatory!(ContentEncodingType => CONTENT_ENCODING_TYPE, UnsignedInt);

ebml_element_container!(ContentCompression => CONTENT_COMPRESSION);
ebml_element_mandatory!(ContentCompAlgo => CONTENT_COMP_ALGO, UnsignedInt);
ebml_element_mandatory!(ContentCompSettings => CONTENT_COMP_SETTINGS, Binary);

ebml_element_container!(ContentEncryption => CONTENT_ENCRYPTION);
ebml_element_mandatory!(ContentEncAlgo => CONTENT_ENC_ALGO, UnsignedInt);
ebml_element_mandatory!(ContentEncKeyID => CONTENT_ENC_KEY_ID, Binary);
ebml_element_mandatory!(ContentSignature => CONTENT_SIGNATURE, Binary);
ebml_element_mandatory!(ContentSigKeyID => CONTENT_SIG_KEY_ID, Binary);
ebml_element_mandatory!(ContentSigAlgo => CONTENT_SIG_ALGO, UnsignedInt);
ebml_element_mandatory!(ContentSigHashAlgo => CONTENT_SIG_HASH_ALGO, UnsignedInt);
