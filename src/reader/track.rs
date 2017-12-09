//! Read matroska track information.

use ebml::common::types::*;
use ebml::common::ElementArray;

use elements as el;
use error::{self, Result};

/// Possible MKV track types.
pub enum Kind {
    Video(Video),
    Audio(Audio),
    Complex,
    Logo,
    Subtitle,
    Buttons,
    Control,
}

/// Information about a video track.
pub struct Video {
    pixel_width: UnsignedInt,
    pixel_height: UnsignedInt,
}

impl Video {
    /// Width of the video track in pixels.
    pub fn width(&self) -> u64 {
        self.pixel_width
    }

    /// Height of the video track in pixels.
    pub fn height(&self) -> u64 {
        self.pixel_height
    }
}

/// Information about an audio track.
pub struct Audio {
    channels: UnsignedInt,
    sampling_freq: Float,
    out_sampling_freq: Float
}

impl Audio {
    /// Number of audio channels.
    pub fn channels(&self) -> u64 {
        self.channels
    }

    /// Audio sampling frequency in hertz.
    pub fn sampling_frequency(&self) -> f64 {
        self.sampling_freq
    }

    /// Output audio sampling frequency in hertz.
    pub fn output_sampling_frequenct(&self) -> f64 {
        self.out_sampling_freq
    }
}

/// Contains parsed information about a matroka track.
pub struct Info {
    number: UnsignedInt,
    uid: UnsignedInt,
    kind: Kind,
    codec_id: Utf8,
}

impl Info {
    /// Index number of the track.
    pub fn index(&self) -> u64 {
        self.number
    }

    /// Uinique identifier of the track.
    pub fn uid(&self) -> u64 {
        self.uid
    }

    /// Domain-specific track type and associated data.
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    /// String identifier of the codec of the track.
    pub fn codec(&self) -> &str {
        self.codec_id.as_str()
    }
}

/// Read information about all tracks in the matroska file. Expected input: children of the
/// `Track` master element.
pub fn read(elems: ElementArray) -> Result<Vec<Info>> {
    let mut tracks = Vec::new();

    for track_entry in elems.vec() {
        let mut data = track_entry.content().children()?;

        let number = data.find(el::TRACK_NUMBER)
            .ok_or(error::not_found(el::TRACK_NUMBER))?
            .content().into_uint();

        let uid = data.find(el::TRACK_UID)
            .ok_or(error::not_found(el::TRACK_UID))?
            .content().into_uint();

        let codec_id = data.find(el::CODEC_ID)
            .ok_or(error::not_found(el::CODEC_ID))?
            .content().into_utf8()?;

        let track_type = data.find(el::TRACK_TYPE)
            .ok_or(error::not_found(el::TRACK_TYPE))?
            .content().into_uint();

        let kind = match track_type {
            0x01 => {
                let mut video = data.find(el::VIDEO)
                    .ok_or(error::not_found(el::VIDEO))?
                    .content().children()?;

                let pw = video.find(el::PIXEL_WIDTH)
                    .ok_or(error::not_found(el::PIXEL_WIDTH))?
                    .content().into_uint();

                let ph = video.find(el::PIXEL_HEIGHT)
                    .ok_or(error::not_found(el::PIXEL_HEIGHT))?
                    .content().into_uint();

                Kind::Video(Video {
                    pixel_width: pw,
                    pixel_height: ph,
                })
            },

            0x02 => {
                let mut audio = data.find(el::AUDIO)
                    .ok_or(error::not_found(el::AUDIO))?
                    .content().children()?;

                let channels = audio.find(el::CHANNELS)
                    .map_or(1, |elem| elem.content().into_uint());

                let sampling_freq = audio.find(el::SAMPLING_FREQUENCY)
                    .map_or_else(|| Ok(8000.0), |elem| elem.content().into_float())?;

                let out_sampling_freq = audio.find(el::OUTPUT_SAMPLING_FREQUENCY)
                    .map_or_else(|| Ok(sampling_freq), |elem| elem.content().into_float())?;

                Kind::Audio(Audio {
                    channels: channels,
                    sampling_freq: sampling_freq,
                    out_sampling_freq: out_sampling_freq,
                })
            },

            0x03 => Kind::Complex,
            0x10 => Kind::Logo,
            0x11 => Kind::Subtitle,
            0x12 => Kind::Buttons,
            0x20 => Kind::Control,

            wtf => bail!(error::invalid_value(el::TRACK_TYPE, wtf)),
        };

        tracks.push(Info {
            number: number,
            uid: uid,
            codec_id: codec_id,
            kind: kind,
        });
    }

    Ok(tracks)
}
