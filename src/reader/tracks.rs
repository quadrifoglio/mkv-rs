//! MKV Track information reading.

use std::io::Read;

use ebml;
use ebml::common::types::*;

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
    pub pixel_width: UnsignedInt,
    pub pixel_height: UnsignedInt,
}

/// Information about an audio track.
pub struct AudioTrack {
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

/// Read information about all tracks in the MKV source.
pub fn read_track_information<R: Read>(r: &mut R) -> Result<(Vec<Track>, usize)> {
    let mut tracks = Vec::new();

    let (elem, count) = ebml::reader::read_element(r)?;

    for track_entry in elem.content().children()?.vec() {
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
                    .map_or(8000.0, |elem| elem.content().into_float());

                let out_sampling_freq = audio.find(el::OUTPUT_SAMPLING_FREQUENCY)
                    .map_or(sampling_freq, |elem| elem.content().into_float());

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

    Ok((tracks, count))
}
