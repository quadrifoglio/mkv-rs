//! Read matroska cluster data.

use std::io::Read;

use ::ebml as libebml;
use self::libebml::common::types::*;
use self::libebml::common::ElementArray;

use elements as el;
use error::{self, Result};

/// Represents a matroska cluster.
pub struct Cluster<'a, R: Read + 'a> {
    r: &'a mut R,
    pos: usize,
    size: usize,
}

impl<'a, R: Read + 'a> Cluster<'a, R> {
    pub(crate) fn new(r: &'a mut R, size: usize) -> Cluster<'a, R> {
        Cluster {
            r: r,
            pos: 0,
            size: size,
        }
    }

    pub fn block(&mut self) -> Result<Option<Block>> {
        while self.pos < self.size {
            let (elem, c) = libebml::reader::read_element(&mut self.r)?;
            self.pos += c;

            match elem.id() {
                el::TIMECODE | el::SILENT_TRACKS | el::POSITION | el::PREV_SIZE | el::ENCRYPTED_BLOCK => continue,

                el::SIMPLE_BLOCK => return Ok(Some(simple_block(elem.content().into_binary())?)),
                el::BLOCK_GROUP => return Ok(Some(block_group(elem.content().children()?)?)),

                wtf => bail!(error::unexpected(el::SIMPLE_BLOCK, wtf)),
            };
        }

        Ok(None)
    }
}

/// Represents a matroska data block.
pub struct Block {
    data: Vec<u8>,
}

impl Block {
    /// Return the size in bytes of the block.
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

fn simple_block(data: Binary) -> Result<Block> {
    Ok(Block {
        data: data,
    })
}

fn block_group(mut elems: ElementArray) -> Result<Block> {
    let data = elems.find(el::BLOCK)
        .ok_or(error::not_found(el::BLOCK))?
        .content().into_binary();

    Ok(Block {
        data: data,
    })
}
