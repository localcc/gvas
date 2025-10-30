use std::io::Cursor;

use byteorder::{LittleEndian, WriteBytesExt};

use gvas::game_version::GameVersion;
use gvas::{FILE_TYPE_GVAS, GvasFile, GvasHeader, error::Error};

#[test]
fn test_file_err() {
    let buf = [0; 4];

    // Read GvasFile from Vec<u8>
    let mut reader = Cursor::new(buf);
    let err =
        GvasFile::read(&mut reader, GameVersion::Default).expect_err("Expected file type error");
    assert_eq!(
        err.to_string(),
        "Invalid header: File type 0 not recognized"
    );

    // Read GvasHeader from Vec<u8>
    let mut reader = Cursor::new(buf);
    let err = GvasHeader::read(&mut reader).expect_err("Expected file type error");
    assert_eq!(
        err.to_string(),
        "Invalid header: File type 0 not recognized"
    );
}

#[test]
fn test_version_err() -> Result<(), Error> {
    let buf = {
        let mut cursor = Cursor::new(Vec::new());
        cursor.write_u32::<LittleEndian>(FILE_TYPE_GVAS)?;
        cursor.write_u32::<LittleEndian>(0x7DC2293F)?;
        cursor.into_inner()
    };
    let buf = buf.as_slice();

    // Read GvasFile from &[u8]
    let mut reader = Cursor::new(buf);
    let err =
        GvasFile::read(&mut reader, GameVersion::Default).expect_err("Expected file type error");
    assert_eq!(
        err.to_string(),
        "Invalid header: GVAS version 2109876543 not supported"
    );

    // Read GvasHeader from &[u8]
    let mut reader = Cursor::new(buf);
    let err = GvasHeader::read(&mut reader).expect_err("Expected file type error");
    assert_eq!(
        err.to_string(),
        "Invalid header: GVAS version 2109876543 not supported"
    );

    Ok(())
}
