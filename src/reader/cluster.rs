//! Read matroska cluster data.

use std::io::{Cursor, Read};

use ::ebml as libebml;
use self::libebml::common::types::*;
use self::libebml::common::ElementArray;

use elements as el;
use error::{self, Error, Result};

use super::Reader;

/// Represents a matroska cluster.
pub struct Cluster<'a, R: Read + 'a> {
    reader: &'a mut Reader<R>,
    pos: usize,
    size: usize,
}

impl<'a, R: Read + 'a> Cluster<'a, R> {
    pub(crate) fn new(reader: &'a mut Reader<R>, size: usize) -> Cluster<'a, R> {
        Cluster {
            reader: reader,
            pos: 0,
            size: size,
        }
    }

    /// Return an iterator over all the data blocks in the cluster.
    pub fn blocks(&'a mut self) -> Blocks<'a, R> {
        Blocks {
            cluster: self,
        }
    }
}

/// Iterator over Blocks.
pub struct Blocks<'a, R: Read + 'a> {
    cluster: &'a mut Cluster<'a, R>,
}

impl<'a, R: Read + 'a> ::std::iter::Iterator for Blocks<'a, R> {
    type Item = Result<Block>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cluster.pos < self.cluster.size {
            let (elem, c) = match libebml::reader::read_element(&mut self.cluster.reader.r) {
                Ok((elem, c)) => (elem, c),
                Err(err) => return Some(Err(Error::from(err))),
            };

            self.cluster.pos += c;
            self.cluster.reader.segment_position += c;

            match elem.id() {
                el::TIMECODE | el::SILENT_TRACKS | el::POSITION | el::PREV_SIZE | el::ENCRYPTED_BLOCK => continue,

                el::SIMPLE_BLOCK => return Some(simple_block(elem.content().into_binary())),

                el::BLOCK_GROUP => {
                    let elems = match elem.content().children() {
                        Ok(elems) => elems,
                        Err(err) => return Some(Err(Error::from(err))),
                    };

                    return Some(block_group(elems));
                },

                wtf => return Some(Err(error::unexpected(el::SIMPLE_BLOCK, wtf))),
            };
        }

        None
    }
}

/// Different lacing types available.
pub enum Lacing {
    None,
    Xiph,
    Ebml,
    FixedSize,
}

/// Represents a matroska data block.
pub struct Block {
    track_number: UnsignedInt,
    timecode: i16,
    keyframe: bool,
    invisible: bool,
    discardable: bool,
    lacing: Lacing,
    data: Binary,
}

impl Block {
    /// Return the index number of the track that the block is associated with.
    pub fn track(&self) -> u64 {
        self.track_number
    }

    /// Return the timecode of the block, relative to the cluster's timecode.
    pub fn timecode(&self) -> i16 {
        self.timecode
    }

    /// Return wether the block contains only keyframe(s).
    pub fn keyframe(&self) -> bool {
        self.keyframe
    }

    /// Return wether the block should be displayed by a player.
    pub fn invisible(&self) -> bool {
        self.invisible
    }

    /// Return wether the block can be discarded by a player.
    pub fn discardable(&self) -> bool {
        self.discardable
    }

    /// Return the lacing data of the block.
    pub fn lacing(&self) -> &Lacing {
        &self.lacing
    }

    /// Return the size in bytes of the block.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Return the data contained in the block. Consumes `self`.
    pub fn data(self) -> Vec<u8> {
        self.data
    }
}

fn simple_block(data: Binary) -> Result<Block> {
    parse_block(data, true)
}

fn block_group(mut elems: ElementArray) -> Result<Block> {
    let data = elems.find(el::BLOCK)
        .ok_or(error::not_found(el::BLOCK))?
        .content().into_binary();

    parse_block(data, false)
}

fn parse_block(data: Vec<u8>, simple_block_structure: bool) -> Result<Block> {
    let mut data_len = data.len();
    let mut cursor = Cursor::new(data);

    let (track_number, c) = libebml::reader::read_vint(&mut cursor, true)?;
    data_len -= c;

    let mut timecode_buf = vec![0u8; 2];
    let c = cursor.read(&mut timecode_buf)?;
    data_len -= c;

    let timecode = ((timecode_buf[0] as i16) << 8) | (timecode_buf[1] as i16);

    let mut flags = vec![0u8; 1];
    let c = cursor.read(&mut flags)?;
    data_len -= c;

    let mut keyframe = false;
    let mut discardable = false;
    let mut invisible = false;

    if flags[0] & 0x08 != 0 {
        invisible = true;
    }

    if simple_block_structure {
        if flags[0] & 0x80 != 0 {
            keyframe = true;
        }

        if flags[0] & 0x01 != 0 {
            discardable = true;
        }
    }

    let lacing = match (flags[0] & 0x6) >> 1 {
        0b00 => Lacing::None,
        0b01 => Lacing::Xiph,
        0b11 => Lacing::Ebml,
        0b10 => Lacing::FixedSize,

        wtf => bail!(error::invalid_value(0, wtf)),
    };

    // TODO: Handle lacing data.

    let mut data = vec![0u8; data_len];
    cursor.read(&mut data)?;

    Ok(Block {
        track_number: track_number as UnsignedInt,
        timecode: timecode,
        keyframe: keyframe,
        invisible: invisible,
        discardable: discardable,
        lacing: lacing,
        data: data,
    })
}
