use crate::common::{palworld, PALWORLD_ZLIB_PATH, PALWORLD_ZLIB_TWICE_PATH};
use gvas::game_version::GameVersion;
use gvas::GvasFile;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;

#[test]
fn read_zlib_palworld() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(PALWORLD_ZLIB_PATH);
    let mut file = File::open(path).expect("Failed to open test asset");

    // Read the file in to a Vec<u8>
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    // Convert the Vec<u8> to a GvasFile
    let mut cursor = Cursor::new(data);
    let file =
        GvasFile::read(&mut cursor, GameVersion::Palworld).expect("Failed to parse gvas file");

    // Convert the GvasFile back to a Vec<u8>
    let mut writer = Cursor::new(Vec::new());
    file.write(&mut writer).expect("Failed to write test asset");

    let mut cursor = Cursor::new(writer.get_ref());
    let reparsed =
        GvasFile::read(&mut cursor, GameVersion::Palworld).expect("Failed to reparse gvas file");

    assert_eq!(file, reparsed);
}

#[test]
fn read_zlib_twice_palworld() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(PALWORLD_ZLIB_TWICE_PATH);
    let mut file = File::open(path).expect("Failed to open test asset");

    // Read the file in to a Vec<u8>
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    let hints = palworld::hints();

    // Convert the Vec<u8> to a GvasFile
    let mut cursor = Cursor::new(data);
    let file = GvasFile::read_with_hints(&mut cursor, GameVersion::Palworld, &hints)
        .expect("Failed to parse gvas file");

    // Convert the GvasFile back to a Vec<u8>
    let mut writer = Cursor::new(Vec::new());
    file.write(&mut writer).expect("Failed to write test asset");

    let mut cursor = Cursor::new(writer.get_ref());
    let reparsed = GvasFile::read_with_hints(&mut cursor, GameVersion::Palworld, &hints)
        .expect("Failed to reparse gvas file");

    assert_eq!(file, reparsed);
}
