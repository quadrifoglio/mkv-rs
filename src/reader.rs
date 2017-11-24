//! Matroska stream reading functionality.

use std::io::Read;

/// The object that allows to retereive information from an MKV input source.
pub struct Reader<R: Read> {
    reader: R,
}

impl<R: Read> Reader<R> {
    fn new(reader: R) -> Reader<R> {
        Reader { reader: reader }
    }
}

impl<R: Read> ::std::convert::From<R> for Reader<R> {
    fn from(r: R) -> Reader<R> {
        Reader::new(r)
    }
}
