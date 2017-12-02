//! MKV Track information reading.

use std::io::Read;

use ebml::element::types::*;
use ebml::element::Element;
use ebml::reader::Reader;

use error::{ErrorKind, Result};
use elements as el;

/// Possible MKV track types.
pub enum TrackKind {
    Video(TrackVideo),
    Audio(TrackAudio),
    Complex,
    Logo,
    Subtitle,
    Buttons,
    Control,
}

/// Information about a video track.
pub struct TrackVideo {
    pub pixel_width: UnsignedInt,
    pub pixel_height: UnsignedInt,
}

/// Information about an audio track.
pub struct TrackAudio {
    pub channels: UnsignedInt,
    pub sampling_freq: Float,
    pub out_sampling_freq: Float
}

/// Contains parsed information about an MKV track.
pub struct Track {
    pub number: UnsignedInt,
    pub uid: UnsignedInt,
    pub kind: TrackKind,
    pub codec_id: Utf8,
}

/// Initialize the specified EBML reader to make it ready to read MKV track information.
pub fn init<R: Read>(ebml: &mut Reader<R>) {
    ebml.register::<el::Tracks>();
    ebml.register::<el::TrackEntry>();

    ebml.register::<el::TrackNumber>();
    ebml.register::<el::TrackUID>();
    ebml.register::<el::TrackType>();
    ebml.register::<el::FlagEnabled>();
    ebml.register::<el::FlagDefault>();
    ebml.register::<el::FlagForced>();
    ebml.register::<el::FlagLacing>();
    ebml.register::<el::MinCache>();
    ebml.register::<el::MaxCache>();
    ebml.register::<el::DefaultDuration>();
    ebml.register::<el::DefaultDecodedFieldDuration>();
    ebml.register::<el::TrackTimecodeScale>();
    ebml.register::<el::TrackOffset>();
    ebml.register::<el::MaxBlockAdditionID>();
    ebml.register::<el::Name>();
    ebml.register::<el::Language>();
    ebml.register::<el::CodecID>();
    ebml.register::<el::CodecPrivate>();
    ebml.register::<el::CodecName>();
    ebml.register::<el::AttachmentLink>();
    ebml.register::<el::CodecSettings>();
    ebml.register::<el::CodecInfoURL>();
    ebml.register::<el::CodecDownloadURL>();
    ebml.register::<el::CodecDecodeAll>();
    ebml.register::<el::TrackOverlay>();
    ebml.register::<el::CodecDelay>();
    ebml.register::<el::SeekPreRoll>();

    ebml.register::<el::TrackTranslate>();
    ebml.register::<el::TrackTranslateEditionUID>();
    ebml.register::<el::TrackTranslateCodec>();
    ebml.register::<el::TrackTranslateTrackID>();

    ebml.register::<el::Video>();
    ebml.register::<el::FlagInterlaced>();
    ebml.register::<el::FieldOrder>();
    ebml.register::<el::StereoMode>();
    ebml.register::<el::AlphaMode>();
    ebml.register::<el::OldStereoMode>();
    ebml.register::<el::PixelWidth>();
    ebml.register::<el::PixelHeight>();
    ebml.register::<el::PixelCropBottom>();
    ebml.register::<el::PixelCropTop>();
    ebml.register::<el::PixelCropLeft>();
    ebml.register::<el::PixelCropRight>();
    ebml.register::<el::DisplayWidth>();
    ebml.register::<el::DisplayHeight>();
    ebml.register::<el::DisplayUnit>();
    ebml.register::<el::AspectRatioType>();
    ebml.register::<el::ColourSpace>();
    ebml.register::<el::GammaValue>();
    ebml.register::<el::FrameRate>();

    ebml.register::<el::Colour>();
    ebml.register::<el::MatrixCoefficients>();
    ebml.register::<el::BitsPerChannel>();
    ebml.register::<el::ChromaSubsamplingHorz>();
    ebml.register::<el::ChromaSubsamplingVert>();
    ebml.register::<el::CbSubsamplingHorz>();
    ebml.register::<el::CbSubsamplingVert>();
    ebml.register::<el::ChromaSitingHorz>();
    ebml.register::<el::ChromaSitingVert>();
    ebml.register::<el::Range>();
    ebml.register::<el::TransferCharacteristics>();
    ebml.register::<el::Primaries>();
    ebml.register::<el::MaxCLL>();
    ebml.register::<el::MaxFALL>();

    ebml.register::<el::MasteringMetadata>();
    ebml.register::<el::PrimaryRChromaticityX>();
    ebml.register::<el::PrimaryRChromaticityY>();
    ebml.register::<el::PrimaryGChromaticityX>();
    ebml.register::<el::PrimaryGChromaticityY>();
    ebml.register::<el::PrimaryBChromaticityX>();
    ebml.register::<el::PrimaryBChromaticityY>();
    ebml.register::<el::WhitePointChromaticityX>();
    ebml.register::<el::WhitePointChromaticityY>();
    ebml.register::<el::LuminanceMax>();
    ebml.register::<el::LuminanceMin>();

    ebml.register::<el::Audio>();
    ebml.register::<el::SamplingFrequency>();
    ebml.register::<el::OutputSamplingFrequency>();
    ebml.register::<el::Channels>();
    ebml.register::<el::ChannelPositions>();
    ebml.register::<el::BitDepth>();

    ebml.register::<el::TrackOperation>();

    ebml.register::<el::TrackCombinePlanes>();
    ebml.register::<el::TrackPlane>();
    ebml.register::<el::TrackPlaneUID>();
    ebml.register::<el::TrackPlaneType>();

    ebml.register::<el::TrackJoinBlocks>();
    ebml.register::<el::TrackJoinUID>();
    ebml.register::<el::TrickTrackUID>();
    ebml.register::<el::TrickTrackSegmentUID>();
    ebml.register::<el::TrickTrackFlag>();
    ebml.register::<el::TrickMasterTrackUID>();
    ebml.register::<el::TrickMasterTrackSegmentUID>();

    ebml.register::<el::ContentEncodings>();
    ebml.register::<el::ContentEncoding>();
    ebml.register::<el::ContentEncodingOrder>();
    ebml.register::<el::ContentEncodingScope>();
    ebml.register::<el::ContentEncodingType>();

    ebml.register::<el::ContentCompression>();
    ebml.register::<el::ContentCompAlgo>();
    ebml.register::<el::ContentCompSettings>();

    ebml.register::<el::ContentEncryption>();
    ebml.register::<el::ContentEncAlgo>();
    ebml.register::<el::ContentEncKeyID>();
    ebml.register::<el::ContentSignature>();
    ebml.register::<el::ContentSigKeyID>();
    ebml.register::<el::ContentSigAlgo>();
    ebml.register::<el::ContentSigHashAlgo>();
}

/// Read information about all tracks in the MKV source.
pub fn read_track_information<R: Read>(ebml: &mut Reader<R>) -> Result<Vec<Track>> {
    let mut tracks = Vec::new();

    let (elem, _) = ebml.read_element(true)?;

    if elem.id() != el::TRACKS {
        bail!(ErrorKind::UnexpectedElement(el::TRACKS, elem.id()));
    }

    for entry in elem.children() {
        let kind = match find_child_uint!(entry, el::TrackType) {
            0x01 => {
                let video = find_child!(entry, el::Video);

                let pw = find_child!(video, el::PixelWidth);
                let ph = find_child!(video, el::PixelHeight);

                TrackKind::Video(TrackVideo {
                    pixel_width: pw.data().to_unsigned_int()?,
                    pixel_height: ph.data().to_unsigned_int()?,
                })
            },

            0x02 => {
                let audio = find_child!(entry, el::Audio);

                let channels = find_child_uint_or!(audio, el::Channels, 1);
                let sampling_freq = find_child_float_or!(audio, el::SamplingFrequency, 8000.0);
                let out_sampling_freq = find_child_float_or!(audio, el::OutputSamplingFrequency, sampling_freq);

                TrackKind::Audio(TrackAudio {
                    channels: channels,
                    sampling_freq: sampling_freq,
                    out_sampling_freq: out_sampling_freq
                })
            },

            0x03 => TrackKind::Complex,
            0x10 => TrackKind::Logo,
            0x11 => TrackKind::Subtitle,
            0x12 => TrackKind::Buttons,
            0x20 => TrackKind::Control,

            wtf => bail!(ErrorKind::InvalidElementValue(el::TRACK_TYPE, format!("{}", wtf))),
        };

        tracks.push(Track {
            number: find_child_uint!(entry, el::TrackNumber),
            uid: find_child_uint!(entry, el::TrackUID),
            codec_id: find_child_utf8!(entry, el::CodecID),
            kind: kind,
        });
    }

    Ok(tracks)
}
