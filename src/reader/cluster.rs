//! Cluster data reading.

use std::io::Read;

use ebml;
use ebml::common::types::*;
use ebml::common::ElementContent;

use error::{self, Result};
use elements as el;

/// The cluster is the the structure that contins the `Block`s.
pub struct Cluster {
    count: usize,
    size: usize,

    timecode: Option<UnsignedInt>,
    position: Option<UnsignedInt>,
    prev_size: Option<UnsignedInt>,

    first_block: Option<(ElementId, ElementContent)>,
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
            first_block: None,
        }
    }

    /// Read the next block of data from that cluster.
    pub fn block<R: Read>(&mut self, r: &mut R) -> Result<Option<Block>> {
        if self.first_block.is_some() {
            let (kind, content) = self.first_block.take().unwrap();

            return Ok(Some(read_block(kind, content)?));
        }

        if self.count >= self.size {
            return Ok(None);
        }

        let (elem, c) = ebml::reader::read_element(r)?;
        self.count += c;

        Ok(Some(read_block(elem.id(), elem.content())?))
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

pub fn read_cluster<R: Read>(r: &mut R, size: usize) -> Result<(Cluster, usize)> {
    let mut count = 0 as usize;
    let mut cluster = Cluster::new(size);

    loop {
        let (elem, c) = ebml::reader::read_element(r)?;
        count += c;
        cluster.count += c;

        match elem.id() {
            el::TIMECODE => cluster.timecode = Some(elem.content().into_uint()),
            el::POSITION => cluster.position = Some(elem.content().into_uint()),
            el::PREV_SIZE => cluster.prev_size = Some(elem.content().into_uint()),

            el::SIMPLE_BLOCK => {
                cluster.first_block = Some((el::SIMPLE_BLOCK, elem.content()));
                break;
            },

            el::BLOCK_GROUP => {
                cluster.first_block = Some((el::BLOCK_GROUP, elem.content()));
                break;
            },

            el::SILENT_TRACKS | el::ENCRYPTED_BLOCK => bail!(error::unexpected(0, 0)),

            _ => continue,
        };
    }

    Ok((cluster, count))
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

fn read_block(kind: ElementId, content: ElementContent) -> Result<Block> {
    let data: Vec<u8>;

    match kind {
        el::SIMPLE_BLOCK => data = content.into_binary(),

        el::BLOCK_GROUP => {
            let mut group = content.children()?;

            data = group.find(el::BLOCK)
                .ok_or(error::not_found(el::BLOCK))?
                .content().into_binary();
        },

        wtf => bail!(error::unexpected(kind, wtf)),
    };

    let block = Block {
        data: data
    };

    Ok(block)
}
