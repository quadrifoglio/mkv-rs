//! Matroska stream reading functionality.

use std::io::Read;

use ebml;
use ebml::element::Element;

use elements as el;
use structures::*;
use error::{ErrorKind, Result};

/// The object that allows to retereive information from an MKV input source.
pub struct Reader<R: Read> {
    ebml: ebml::reader::Reader<R>,
}

impl<R: Read> Reader<R> {
    fn new(r: R) -> Reader<R> {
        let mut ebml = ebml::reader::Reader::from(r);

        // Meta Seek Information.

        ebml.register::<el::SeekHead>();
        ebml.register::<el::Seek>();
        ebml.register::<el::SeekID>();
        ebml.register::<el::SeekPosition>();

        // Segment Information.

        ebml.register::<el::Info>();
        ebml.register::<el::SegmentUID>();
        ebml.register::<el::SegmentFilename>();
        ebml.register::<el::PrevUID>();
        ebml.register::<el::PrevFilename>();
        ebml.register::<el::NextUID>();
        ebml.register::<el::NextFilename>();
        ebml.register::<el::SegmentFamily>();
        ebml.register::<el::ChapterTranslate>();
        ebml.register::<el::ChapterTranslateEditionUID>();
        ebml.register::<el::ChapterTranslateCodec>();
        ebml.register::<el::ChapterTranslateID>();
        ebml.register::<el::TimecodeScale>();
        ebml.register::<el::Duration>();
        ebml.register::<el::DateUTC>();
        ebml.register::<el::Title>();
        ebml.register::<el::MuxingApp>();
        ebml.register::<el::WritingApp>();

        // Tracks information.

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

        Reader { ebml: ebml }
    }

    /// Read seek data from the MKV input source.
    pub fn read_seek_data(&mut self) -> Result<Vec<SeekEntry>> {
        let mut entries = Vec::new();
        let (parent, _) = self.ebml.read_element(true)?;

        if parent.id() != el::SEEK_HEAD {
            bail!(ErrorKind::UnexpectedElement(el::SEEK_HEAD, parent.id()));
        }

        for child in parent.children() {
            let mut entry = SeekEntry::default();

            if child.id() != el::SEEK {
                bail!(ErrorKind::UnexpectedElement(el::SEEK, child.id()));
            }

            for value in child.children() {
                match value.id() {
                    el::SEEK_ID => entry.id = value.data().clone().take()?,
                    el::SEEK_POSITION => entry.position = value.data().to_unsigned_int()?,

                    _ => {}
                };
            }

            entries.push(entry);
        }

        Ok(entries)
    }

    /// Read segment informations from the MKV input source.
    pub fn read_segment_info(&mut self) -> Result<SegmentInfo> {
        let mut info = SegmentInfo::default();
        let (parent, _) = self.ebml.read_element(true)?;

        if parent.id() != el::INFO {
            bail!(ErrorKind::UnexpectedElement(el::INFO, parent.id()));
        }

        for child in parent.children() {
            match child.id() {
                el::SEGMENT_UID => info.uid = child.data().clone().take()?,
                el::SEGMENT_FILENAME => info.segment_filename = child.data().to_utf8()?,
                el::PREV_UID => info.prev_uid = child.data().clone().take()?,
                el::PREV_FILENAME => info.prev_filename = child.data().to_utf8()?,
                el::NEXT_UID => info.next_uid = child.data().clone().take()?,
                el::NEXT_FILENAME => info.next_filename = child.data().to_utf8()?,
                el::SEGMENT_FAMILY => info.segment_familys.push(child.data().clone().take()?),
                el::TIMECODE_SCALE => info.timecode_scale = child.data().to_unsigned_int()?,
                el::DURATION => info.duration = child.data().to_float()?,
                el::DATE_UTC => info.date_utc = child.data().clone().take()?,
                el::TITLE => info.title = child.data().to_utf8()?,
                el::MUXING_APP => info.muxing_app = child.data().to_utf8()?,
                el::WRITING_APP => info.writing_app = child.data().to_utf8()?,
                el::CHAPTER_TRANSLATE => info.chapter_translates.push(self.read_segment_info_chapter_translate()?),

                _ => {}
            };
        }

        Ok(info)
    }

    /// Read ChapterTranslate information.
    fn read_segment_info_chapter_translate(&mut self) -> Result<ChapterTranslate> {
        let mut ct = ChapterTranslate::default();

        let (parent, _) = self.ebml.read_element(true)?;

        if parent.id() != el::ChapterTranslate::id() {
            bail!(ErrorKind::UnexpectedElement(
                el::ChapterTranslate::id(),
                parent.id()
            ));
        }

        for child in parent.children() {
            match child.id() {
                el::CHAPTER_TRANSLATE_EDITION_UID => {
                    ct.edition_uids.push(child.data().to_unsigned_int()?)
                }
                el::CHAPTER_TRANSLATE_CODEC => ct.codec = child.data().to_unsigned_int()?,
                el::CHAPTER_TRANSLATE_ID => ct.id = child.data().clone().take()?,

                _ => {}
            };
        }

        Ok(ct)
    }
}

impl<R: Read> ::std::convert::From<R> for Reader<R> {
    fn from(r: R) -> Reader<R> {
        Reader::new(r)
    }
}
