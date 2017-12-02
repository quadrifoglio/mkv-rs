//! Cluster data reading.

use std::io::Read;

use ebml::element::types::*;
use ebml::reader::Reader;

use elements as el;
use error::{ErrorKind, Result};

/// The element that contains all the data blocks (frames).
pub struct Cluster<'a, R: Read + 'a> {
    ebml: &'a mut Reader<R>,

    pub timecode: Option<UnsignedInt>,
    pub position: Option<UnsignedInt>,
}

/// A block that contains data.
pub struct Block {
    pub data: Binary,
    pub additions: Option<Binary>,
}

/// Initialize the specified EBML reader to make it ready to read MKV segment information.
pub fn init<R: Read>(ebml: &mut Reader<R>) {
    ebml.register_container(el::CLUSTER);
    ebml.register_container(el::SILENT_TRACKS);
    ebml.register_container(el::BLOCK_GROUP);
    ebml.register_container(el::BLOCK);
    ebml.register_container(el::BLOCK_ADDITIONS);
    ebml.register_container(el::BLOCK_MORE);
    ebml.register_container(el::SLICES);
    ebml.register_container(el::TIME_SLICE);
    ebml.register_container(el::BLOCK_ADDITIONID);
    ebml.register_container(el::DELAY);
    ebml.register_container(el::SLICE_DURATION);
    ebml.register_container(el::REFERENCE_FRAME);
}

/// Read a cluster (the element that contains the blocks).
pub fn read_cluster<'a, R: Read>(ebml: &'a mut Reader<R>) -> Result<Cluster<'a, R>> {
    let (elem, _) = ebml.read_element(false)?;

    if elem.id() != el::CLUSTER {
        bail!(ErrorKind::UnexpectedElement(el::CLUSTER, elem.id()));
    }

    Ok(Cluster {
        ebml: ebml,
        timecode: None,
        position: None
    })
}

impl<'a, R: Read> Cluster<'a, R> {
    /// Read a data block from the cluster.
    pub fn read_block(&mut self) -> Result<Block> {
        let (elem, _) = self.ebml.read_element(false)?;
        let data = self.ebml.read_data(elem.size())?;

        match elem.id() {
            el::TIMECODE => {
                self.timecode = Some(data.to_unsigned_int()?);
                return self.read_block();
            },

            el::POSITION => {
                self.position = Some(data.to_unsigned_int()?);
                return self.read_block();
            },

            el::SIMPLE_BLOCK => {
                return Ok(Block {
                    data: data.take()?,
                    additions: None
                });
            },

            _ => bail!(ErrorKind::UnexpectedElement(el::BLOCK, elem.id())),
        };
    }
}
