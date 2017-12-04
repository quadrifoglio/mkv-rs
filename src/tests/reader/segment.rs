//! Segment information reading tests.

use std::io::Cursor;

use ebml;
use reader;

#[test]
fn read_seek_data() {
    let mut data = Cursor::new(vec![
        0x11, 0x4d, 0x9b, 0x74, 0xbb, 0x4d, 0xbb, 0x8b, 0x53, 0xab, 0x84, 0x15, 0x49, 0xa9, 0x66, 0x53,
        0xac, 0x81, 0x40, 0x4d, 0xbb, 0x8b, 0x53, 0xab, 0x84, 0x16, 0x54, 0xae, 0x6b, 0x53, 0xac, 0x81,
        0xac, 0x4d, 0xbb, 0x8d, 0x53, 0xab, 0x84, 0x11, 0x4d, 0x9b, 0x74, 0x53, 0xac, 0x83, 0x21, 0x09,
        0x73, 0x4d, 0xbb, 0x8c, 0x53, 0xab, 0x84, 0x1c, 0x53, 0xbb, 0x6b, 0x53, 0xac, 0x82, 0x0d, 0x41,
    ]);

    let (data, _) = ebml::reader::read_element(&mut data).unwrap();

    let seek_entries = reader::segment::read_seek_information(data.content().children().unwrap()).unwrap();

    assert_eq!(seek_entries.get(&0x1549a966).unwrap(), &64);
    assert_eq!(seek_entries.get(&0x1654ae6b).unwrap(), &172);
    assert_eq!(seek_entries.get(&0x114d9b74).unwrap(), &2165107);
    assert_eq!(seek_entries.get(&0x1c53bb6b).unwrap(), &3393);
}

#[test]
fn read_information() {
    let mut data = Cursor::new(vec![
        0x15, 0x49, 0xa9, 0x66, 0xe7, 0x73, 0xa4, 0x90, 0x46, 0xc6, 0x9d, 0x45, 0xa1, 0x85, 0xa9, 0x29,
        0x4d, 0x3d, 0x0a, 0x2f, 0x75, 0x00, 0x56, 0xbd, 0x2a, 0xd7, 0xb1, 0x83, 0x0f, 0x42, 0x40, 0x44,
        0x89, 0x84, 0x46, 0xfd, 0xc0, 0x00, 0x44, 0x61, 0x88, 0x04, 0x1b, 0xbb, 0x9a, 0x52, 0x1e, 0xb0,
        0x00, 0x4d, 0x80, 0xa5, 0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x73, 0x6f, 0x75, 0x72, 0x63,
        0x65, 0x66, 0x6f, 0x72, 0x67, 0x65, 0x2e, 0x6e, 0x65, 0x74, 0x2f, 0x70, 0x72, 0x6f, 0x6a, 0x65,
        0x63, 0x74, 0x73, 0x2f, 0x79, 0x61, 0x6d, 0x6b, 0x61, 0x57, 0x41, 0x90, 0x53, 0x6f, 0x72, 0x65,
        0x6e, 0x73, 0x6f, 0x6e, 0x20, 0x53, 0x71, 0x75, 0x65, 0x65, 0x7a, 0x65,
    ]);

    let (data, _) = ebml::reader::read_element(&mut data).unwrap();

    let segment = reader::segment::read_information(data.content().children().unwrap()).unwrap();
    let uid = vec![0x46, 0xc6, 0x9d, 0x45, 0xa1, 0x85, 0xa9, 0x29, 0x4d, 0x3d, 0x0a, 0x2f, 0x75, 0x00, 0x56, 0xbd];

    assert_eq!(segment.uid.unwrap(), uid);
    assert_eq!(segment.filename, None);
    assert_eq!(segment.timecode_scale, 1000000);
}
