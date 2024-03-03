use crate::common::PACKAGE_VERSION_524_PATH;
use gvas::game_version::GameVersion;
use gvas::GvasFile;
use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

#[test]
fn write_slot3() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(PACKAGE_VERSION_524_PATH);
    let mut file = File::open(path).expect("Failed to open test asset");

    // Read the file in to a Vec<u8>
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    // Convert the Vec<u8> to a GvasFile
    let mut cursor = Cursor::new(data);
    let file =
        GvasFile::read(&mut cursor, GameVersion::Default).expect("Failed to parse gvas file");

    // Convert the GvasFile back to a Vec<u8>
    let mut writer = Cursor::new(Vec::new());
    file.write(&mut writer)
        .expect("Failed to serialize gvas file");

    // Compare the two Vec<u8>s
    assert_eq!(cursor.get_ref(), writer.get_ref());

    // Read the file back in again
    let mut reader = Cursor::new(writer.get_ref().to_owned());
    let read_back = GvasFile::read(&mut reader, GameVersion::Default)
        .expect("Failed to parse serialized save file");

    // Compare the two GvasFiles
    assert_eq!(file, read_back);
}
