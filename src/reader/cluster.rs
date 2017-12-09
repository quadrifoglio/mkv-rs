//! Read matroska cluster data.

use std::io::Read;

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
