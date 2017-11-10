/// Parsing tests

use std::io::Cursor;

use io;

#[test]
fn parse_header() {
    let mut data = Cursor::new(vec![
        0x1a, 0x45, 0xdf, 0xa3, 0xa3, 0x42, 0x86, 0x81, 0x01, 0x42, 0xf7, 0x81, 0x01, 0x42, 0xf2,
        0x81, 0x04, 0x42, 0xf3, 0x81, 0x08, 0x42, 0x82, 0x84, 0x77, 0x65, 0x62, 0x6d, 0xec, 0x82,
        0x00, 0x00, 0x42, 0x87, 0x81, 0x01, 0x42, 0x85, 0x81, 0x01,
    ]);

    let header = io::parse::header(&mut data).unwrap();

    assert_eq!(header.ebml_version, 1);
    assert_eq!(header.ebml_read_version, 1);
    assert_eq!(header.ebml_max_id_length, 4);
    assert_eq!(header.ebml_max_size_length, 8);
    assert_eq!(header.doc_type.as_str(), "webm");
    assert_eq!(header.doc_type_version, 1);
    assert_eq!(header.doc_type_read_version, 1);
}

#[test]
fn parse_seek_info() {
    let mut data = Cursor::new(vec![
        0x11, 0x4d, 0x9b, 0x74, 0xbb, 0x4d, 0xbb, 0x8b, 0x53, 0xab, 0x84, 0x15, 0x49, 0xa9, 0x66,
        0x53, 0xac, 0x81, 0x40, 0x4d, 0xbb, 0x8b, 0x53, 0xab, 0x84, 0x16, 0x54, 0xae, 0x6b, 0x53,
        0xac, 0x81, 0xac, 0x4d, 0xbb, 0x8d, 0x53, 0xab, 0x84, 0x11, 0x4d, 0x9b, 0x74, 0x53, 0xac,
        0x83, 0x21, 0x09, 0x73, 0x4d, 0xbb, 0x8c, 0x53, 0xab, 0x84, 0x1c, 0x53, 0xbb, 0x6b, 0x53,
        0xac, 0x82, 0x0d, 0x41,
    ]);

    let mut seek = io::parse::seek_info(&mut data).unwrap().into_iter();

    let entry = seek.next().unwrap();
    assert_eq!(entry.seek_id, vec![0x15, 0x49, 0xa9, 0x66]);
    assert_eq!(entry.seek_position, 64);

    let entry = seek.next().unwrap();
    assert_eq!(entry.seek_id, vec![0x16, 0x54, 0xae, 0x6b]);
    assert_eq!(entry.seek_position, 172);

    let entry = seek.next().unwrap();
    assert_eq!(entry.seek_id, vec![0x11, 0x4d, 0x9b, 0x74]);
    assert_eq!(entry.seek_position, 2165107);

    let entry = seek.next().unwrap();
    assert_eq!(entry.seek_id, vec![0x1c, 0x53, 0xbb, 0x6b]);
    assert_eq!(entry.seek_position, 3393);
}

#[test]
fn parse_segment_info() {
    let mut data = Cursor::new(vec![
        0x15, 0x49, 0xa9, 0x66, 0xe7, 0x73, 0xa4, 0x90, 0x46, 0xc6, 0x9d, 0x45, 0xa1, 0x85,
        0xa9, 0x29, 0x4d, 0x3d, 0x0a, 0x2f, 0x75, 0x00, 0x56, 0xbd, 0x2a, 0xd7, 0xb1, 0x83,
        0x0f, 0x42, 0x40, 0x44, 0x89, 0x84, 0x46, 0xfd, 0xc0, 0x00, 0x44, 0x61, 0x88, 0x04,
        0x1b, 0xbb, 0x9a, 0x52, 0x1e, 0xb0, 0x00, 0x4d, 0x80, 0xa5, 0x68, 0x74, 0x74, 0x70,
        0x3a, 0x2f, 0x2f, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x66, 0x6f, 0x72, 0x67, 0x65,
        0x2e, 0x6e, 0x65, 0x74, 0x2f, 0x70, 0x72, 0x6f, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x2f,
        0x79, 0x61, 0x6d, 0x6b, 0x61, 0x57, 0x41, 0x90, 0x53, 0x6f, 0x72, 0x65, 0x6e, 0x73,
        0x6f, 0x6e, 0x20, 0x53, 0x71, 0x75, 0x65, 0x65, 0x7a, 0x65,
    ]);

    let seg = io::parse::segment_info(&mut data).unwrap();

    assert_eq!(seg.uid, vec![0x46, 0xc6, 0x9d, 0x45, 0xa1, 0x85, 0xa9, 0x29, 0x4d, 0x3d, 0x0a, 0x2f, 0x75, 0x00, 0x56, 0xbd]);
    assert_eq!(seg.timecode_scale, 1000000);
    assert_eq!(seg.duration, 32480.0 as f32);
    assert_eq!(seg.muxing_app, String::from("http://sourceforge.net/projects/yamka"));
    assert_eq!(seg.writing_app, String::from("Sorenson Squeeze"));
}
