//! MKV Track information reading.

use ebml::common::types::*;
use ebml::common::ElementArray as EbmlElementArray;

use error::{self, Result};
use elements as el;

/// Possible MKV track types.
pub enum TrackKind {
    Video(VideoTrack),
    Audio(AudioTrack),
    Complex,
    Logo,
    Subtitle,
    Buttons,
    Control,
}

/// Information about a video track.
pub struct VideoTrack {
    pixel_width: UnsignedInt,
    pixel_height: UnsignedInt,
}

impl VideoTrack {
    /// Return the width in pixel of this video track.
    pub fn width(&self) -> UnsignedInt {
        self.pixel_width
    }

    /// Return the height in pixel of this video track.
    pub fn height(&self) -> UnsignedInt {
        self.pixel_height
    }
}

/// Information about an audio track.
pub struct AudioTrack {
    channels: UnsignedInt,
    sampling_freq: Float,
    out_sampling_freq: Float
}

impl AudioTrack {
    /// Return the number of channels that this track is composed of.
    pub fn channel_count(&self) -> UnsignedInt {
        self.channels
    }

    /// Return the sampling frequency of the audio samples of this track.
    pub fn sampling_frequency(&self) -> Float {
        self.out_sampling_freq
    }
}

/// Contains parsed information about an MKV track.
pub struct Track {
    number: UnsignedInt,
    uid: UnsignedInt,
    kind: TrackKind,
    codec_id: Utf8,
}

impl Track {
    /// Return the number of this track.
    pub fn number(&self) -> UnsignedInt {
        self.number
    }

    /// Return the UID of this track.
    pub fn uid(&self) -> UnsignedInt {
        self.uid
    }

    /// Return the type of this track (Audio, Video, Subtitle...) and associated domain-specific
    /// information.
    pub fn kind<'a>(&'a self) -> &'a TrackKind {
        &self.kind
    }

    /// Returns a string that identifies the codec of this track.
    pub fn codec<'a>(&'a self) -> &'a str {
        self.codec_id.as_str()
    }
}

/// Read information about all tracks in the MKV source.
pub fn read_track_information(track_info: EbmlElementArray) -> Result<Vec<Track>> {
    let mut tracks = Vec::new();

    for track_entry in track_info.vec() {
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

                TrackKind::Video(VideoTrack {
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

                TrackKind::Audio(AudioTrack {
                    channels: channels,
                    sampling_freq: sampling_freq,
                    out_sampling_freq: out_sampling_freq,
                })
            },

            0x03 => TrackKind::Complex,
            0x10 => TrackKind::Logo,
            0x11 => TrackKind::Subtitle,
            0x12 => TrackKind::Buttons,
            0x20 => TrackKind::Control,

            wtf => bail!(error::invalid_value(el::TRACK_TYPE, wtf)),
        };

        tracks.push(Track {
            number: number,
            uid: uid,
            codec_id: codec_id,
            kind: kind,
        });
    }

    Ok(tracks)
}
