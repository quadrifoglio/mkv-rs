//! Cluster data reading.

use std::io::Read;

use ebml;
use ebml::common::types::UnsignedInt;

use error::{self, Result};
use elements as el;

/// The cluster is the the structure that contins the `Block`s.
pub struct Cluster {
    count: usize,
    size: usize,

    timecode: Option<UnsignedInt>,
    position: Option<UnsignedInt>,
    prev_size: Option<UnsignedInt>,
}

impl Cluster {
    /// Create a new `Cluster` object with the specified size.
    pub(crate) fn new(size: usize) -> Cluster {
        Cluster {
            count: 0,
            size: size,
            timecode: None,
            position: None,
            prev_size: None,
        }
    }

    /// Returns `true` if there is no more data to read from that cluster.
    pub(crate) fn is_done(&self) -> bool {
        self.count >= self.size
    }

    /// Return the absolute timestamp of the cluster, if specified.
    pub fn timecode(&self) -> Option<u64> {
        self.timecode
    }

    /// Return the position of this cluster in the segment, if specified.
    pub fn position(&self) -> Option<u64> {
        self.position
    }

    /// Return the size of the previous cluster in bytes, if specified.
    pub fn previous_cluster_size(&self) -> Option<u64> {
        self.prev_size
    }
}

/// A `Block` is a structure that contains the acutal media data.
pub struct Block {
    data: Vec<u8>,
}

impl Block {
    /// Returns the size of the data contained within that block.
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

/// Read a data block from the specified `Cluster`.
pub fn read_block<R: Read>(r: &mut R, cluster: &mut Cluster) -> Result<(Block, usize)> {
    let mut count = 0 as usize;
    let mut data = Vec::new();

    while cluster.count < cluster.size {
        let (elem, c) = ebml::reader::read_element(r)?;
        count += c;
        cluster.count += c;

        match elem.id() {
            el::TIMECODE => cluster.timecode = Some(elem.content().into_uint()),
            el::POSITION => cluster.position = Some(elem.content().into_uint()),
            el::PREV_SIZE => cluster.prev_size = Some(elem.content().into_uint()),

            el::SIMPLE_BLOCK => {
                data = elem.content().into_binary();
                break;
            },

            el::BLOCK_GROUP => {
                let mut group = elem.content().children()?;

                data = group.find(el::BLOCK)
                    .ok_or(error::not_found(el::BLOCK))?
                    .content().into_binary();

                break;
            },

            _ => continue,
        };
    }

    let block = Block {
        data: data
    };

    Ok((block, count))
}
