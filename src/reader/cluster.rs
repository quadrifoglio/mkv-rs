//! Read matroska cluster data.

use std::io::{Cursor, Read};

use ::ebml as libebml;
use self::libebml::types::*;

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

                el::SIMPLE_BLOCK => return Some(Block::from_binary(elem.content().into_binary())),

                el::BLOCK_GROUP => {
                    let mut elems = match elem.content().children() {
                        Ok(elems) => elems,
                        Err(err) => return Some(Err(Error::from(err))),
                    };

                    return Some(match elems.find(el::BLOCK) {
                        Some(elem) => Block::from_binary(elem.content().into_binary()),
                        None => Err(error::not_found(el::BLOCK)),
                    });
                },

                wtf => return Some(Err(error::unexpected(el::SIMPLE_BLOCK, wtf))),
            };
        }

        None
    }
}

/// Type alias to represent a Frame (basically just binary data).
pub type Frame = Vec<u8>;

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
    /// Parse a matroska block from its binary representation.
    fn from_binary(data: Vec<u8>) -> Result<Block> {
        let mut data_len = data.len();
        let mut cursor = Cursor::new(data);

        let (track_number, c) = libebml::reader::read_vint(&mut cursor, true)?;
        data_len -= c;

        let mut timecode_buf = vec![0u8; 2];
        let c = try_read(&mut cursor, &mut timecode_buf)?;
        data_len -= c;

        let timecode = ((timecode_buf[0] as i16) << 8) | (timecode_buf[1] as i16);

        let mut flags = vec![0u8; 1];
        let c = try_read(&mut cursor, &mut flags)?;
        data_len -= c;

        let mut keyframe = false;
        let mut discardable = false;
        let mut invisible = false;

        if flags[0] & 0x08 != 0 {
            invisible = true;
        }

        if flags[0] & 0x80 != 0 {
            keyframe = true;
        }

        if flags[0] & 0x01 != 0 {
            discardable = true;
        }

        let lacing = match (flags[0] & 0x6) >> 1 {
            0b00 => Lacing::None,
            0b01 => Lacing::Xiph,
            0b11 => Lacing::Ebml,
            0b10 => Lacing::FixedSize,

            wtf => bail!(error::invalid_value(0, wtf)),
        };

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

    /// Return the frames contained in the block. Consumes `self`.
    pub fn frames(self) -> Result<Vec<Frame>> {
        match self.lacing {
            Lacing::None => {
                let mut frames = Vec::with_capacity(1);
                frames.push(self.data);

                Ok(frames)
            },

            Lacing::Xiph => parse_xiph_frames(self.data),
            Lacing::Ebml => parse_ebml_frames(self.data),
            Lacing::FixedSize => parse_fixed_size_frames(self.data),
        }
    }
}

fn parse_xiph_frames(block: Vec<u8>) -> Result<Vec<Frame>> {
    let mut frames = Vec::new();
    let mut remaining = block.len();
    let mut cursor = Cursor::new(block);

    // Read the number of frames in the lace. The stored number is actually the number of frames in
    // the lace minus one.
    let mut number = vec![0u8; 1];

    let c = try_read(&mut cursor, &mut number)?;
    remaining -= c;

    let number = number[0];

    // Read the sizes of the laced frames. The last frame's size is not coded and is instead
    // deduced from the block size.
    let mut sizes = Vec::new();

    for _ in 0..number {
        let mut size = 0 as usize;

        let mut next = vec![0u8; 1];
        let c = try_read(&mut cursor, &mut next)?;
        remaining -= c;

        // The size is coded as a list of 255's, and terminated by some other byte value. When we
        // encounter a byte with a value other than 255, it means that the parsing of this size is
        // done and we can go on to read other sizes, if any.
        loop {
            size += next[0] as usize;

            if next[0] == 255 {
                let c = try_read(&mut cursor, &mut next)?;
                remaining -= c;
            } else {
                break;
            }
        }

        sizes.push(size);
    }

    // Read the actual frames in the lace based on the sizes that we read.
    for size in sizes {
        let mut frame = vec![0u8; size];
        let c = try_read(&mut cursor, &mut frame)?;
        remaining -= c;

        frames.push(frame);
    }

    // Read the last frame in the lace based on the remaining amout of bytes in the block.
    let mut frame = vec![0u8; remaining];
    try_read(&mut cursor, &mut frame)?;

    frames.push(frame);

    Ok(frames)
}

fn parse_ebml_frames(data: Vec<u8>) -> Result<Vec<Frame>> {
    let mut frames = Vec::new();
    let mut remaining = data.len();
    let mut cursor = Cursor::new(data);

    // Read the number of frames in the lace. The stored number is actually the number of frames in
    // the lace minus one.
    let mut number = vec![0u8; 1];

    let c = try_read(&mut cursor, &mut number)?;
    remaining -= c;

    let number = number[0];

    // Read the sizes of the laced frames. This first size is coded in EBML VINT format, and the
    // next ones are encoded as differences from that first size. The last frame's size is not
    // coded and is instead deduced from the total block size.
    let mut sizes = Vec::new();

    let (first_size, c) = libebml::reader::read_vint(&mut cursor, true)?;
    remaining -= c;

    sizes.push(first_size as usize);

    for _ in 0..number - 1 {
        let mut size = vec![0u8; 1];

        let c = try_read(&mut cursor, &mut size)?;
        remaining -= c;

        sizes.push(first_size as usize - size[0] as usize);
    }

    // Read the actual frames in the lace based on the sizes that we read.
    for size in sizes {
        let mut frame = vec![0u8; size];
        let c = try_read(&mut cursor, &mut frame)?;
        remaining -= c;

        frames.push(frame);
    }

    // Read the last frame in the lace based on the remaining amout of bytes in the block.
    let mut frame = vec![0u8; remaining];
    try_read(&mut cursor, &mut frame)?;

    frames.push(frame);

    Ok(frames)
}

fn parse_fixed_size_frames(data: Vec<u8>) -> Result<Vec<Frame>> {
    let len = data.len();
    let mut frames = Vec::new();
    let mut cursor = Cursor::new(data);

    let mut number = vec![0u8; 1];
    try_read(&mut cursor, &mut number)?;

    let number = number[0] as usize;
    let frame_size = len / number; // Is it really just a division ?!

    for _ in 0..number {
        let mut frame = vec![0u8; frame_size];
        try_read(&mut cursor, &mut frame)?;

        frames.push(frame);
    }

    Ok(frames)
}

fn try_read<R: Read>(r: &mut R, buf: &mut [u8]) -> Result<usize> {
    let c = r.read(buf)?;

    if c == 0 {
        Err(error::unexpected_eof())
    } else {
        Ok(c)
    }
}
