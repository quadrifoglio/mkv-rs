//! Cluster data reading.

use std::io::Read;

use ebml;

use error::{self, Result};
use elements as el;

pub struct Cluster {
    pos: usize,
    size: usize,
}

impl Cluster {
    pub(crate) fn new(size: usize) -> Cluster {
        Cluster {
            pos: 0,
            size: size
        }
    }

    pub(crate) fn is_done(&self) -> bool {
        self.pos >= self.size
    }
}

pub struct Block {
    data: Vec<u8>,
}

impl Block {
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

pub fn read_block<R: Read>(r: &mut R, cluster: &mut Cluster) -> Result<(Block, usize)> {
    let mut count = 0 as usize;
    let mut data = Vec::new();

    loop {
        if cluster.pos >= cluster.size {
            break
        }

        let (elem, c) = ebml::reader::read_element(r)?;
        count += c;
        cluster.pos += c;

        match elem.id() {
            el::SIMPLE_BLOCK => {
                data = elem.content().into_binary();
                break;
            },

            el::BLOCK_GROUP => {
                let mut group = elem.content().children()?;

                data = group.find(el::BLOCK)
                    .ok_or(error::not_found(el::BLOCK))?
                    .content().into_binary();
            },

            _ => continue,
        };
    }

    let block = Block {
        data: data
    };

    Ok((block, count))
}
