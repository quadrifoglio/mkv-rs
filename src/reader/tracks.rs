//! MKV Track information reading.

use std::io::Read;

use ebml::element::types::*;
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
    ebml.register_container(el::TRACKS);
    ebml.register_container(el::TRACK_ENTRY);
    ebml.register_container(el::TRACK_TRANSLATE);
    ebml.register_container(el::VIDEO);
    ebml.register_container(el::COLOUR);
    ebml.register_container(el::MASTERING_METADATA);
    ebml.register_container(el::AUDIO);
    ebml.register_container(el::TRACK_OPERATION);
    ebml.register_container(el::TRACK_COMBINE_PLANES);
    ebml.register_container(el::TRACK_PLANE);
    ebml.register_container(el::TRACK_JOIN_BLOCKS);
    ebml.register_container(el::CONTENT_ENCODINGS);
    ebml.register_container(el::CONTENT_ENCODING);
    ebml.register_container(el::CONTENT_COMPRESSION);
    ebml.register_container(el::CONTENT_ENCRYPTION);
}

/// Read information about all tracks in the MKV source.
pub fn read_track_information<R: Read>(ebml: &mut Reader<R>) -> Result<Vec<Track>> {
    let mut tracks = Vec::new();

    let (elem, _) = ebml.read_element(true)?;

    if elem.id() != el::TRACKS {
        bail!(ErrorKind::UnexpectedElement(el::TRACKS, elem.id()));
    }

    for entry in elem.children() {
        let kind = match find_child_uint!(entry, el::TRACK_TYPE) {
            0x01 => {
                let video = find_child!(entry, el::VIDEO);

                TrackKind::Video(TrackVideo {
                    pixel_width: find_child_uint!(video, el::PIXEL_WIDTH),
                    pixel_height: find_child_uint!(video, el::PIXEL_HEIGHT),
                })
            },

            0x02 => {
                let audio = find_child!(entry, el::AUDIO);

                let channels = find_child_uint_or!(audio, el::CHANNELS, 1);
                let sampling_freq = find_child_float_or!(audio, el::SAMPLING_FREQUENCY, 8000.0);
                let out_sampling_freq = find_child_float_or!(audio, el::OUTPUT_SAMPLING_FREQUENCY, sampling_freq);

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
            number: find_child_uint!(entry, el::TRACK_NUMBER),
            uid: find_child_uint!(entry, el::TRACK_UID),
            codec_id: find_child_utf8!(entry, el::CODEC_ID),
            kind: kind,
        });
    }

    Ok(tracks)
}
