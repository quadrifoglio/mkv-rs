//! MKV Segment reading (Meta Seek Information & Segment Information).

use std::io::Read;
use std::collections::HashMap;

use ebml::element::Id as EbmlId;
use ebml::reader::{Reader, ReadElement};

use error::{Error, ErrorKind, Result};
use elements as el;

pub fn init<R: Read>(ebml: &mut Reader<R>) {
    // Meta Seek Information.
    ebml.register::<el::SeekHead>();
    ebml.register::<el::Seek>();
    ebml.register::<el::SeekID>();
    ebml.register::<el::SeekPosition>();

    // Segment Information.
    ebml.register::<el::Info>();
    ebml.register::<el::SegmentUID>();
    ebml.register::<el::SegmentFilename>();
    ebml.register::<el::PrevUID>();
    ebml.register::<el::PrevFilename>();
    ebml.register::<el::NextUID>();
    ebml.register::<el::NextFilename>();
    ebml.register::<el::SegmentFamily>();
    ebml.register::<el::ChapterTranslate>();
    ebml.register::<el::ChapterTranslateEditionUID>();
    ebml.register::<el::ChapterTranslateCodec>();
    ebml.register::<el::ChapterTranslateID>();
    ebml.register::<el::TimecodeScale>();
    ebml.register::<el::Duration>();
    ebml.register::<el::DateUTC>();
    ebml.register::<el::Title>();
    ebml.register::<el::MuxingApp>();
    ebml.register::<el::WritingApp>();
}

pub fn read_seek_information<R: Read>(ebml: &mut Reader<R>) -> Result<HashMap<EbmlId, u64>> {
    let mut entries = HashMap::new();
    let (elem, _) = ebml.read_element(true)?;

    for entry in elem.children() {
        let id = entry.find::<el::SeekID>().ok_or(Error::from(ErrorKind::ElementNotFound))?;
        let pos = entry.find::<el::SeekPosition>().ok_or(Error::from(ErrorKind::ElementNotFound))?;

        entries.insert(id.data().to_unsigned_int()?, pos.data().to_unsigned_int()?);
    }

    Ok(entries)
}
