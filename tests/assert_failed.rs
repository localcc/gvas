use gvas::game_version::GameVersion;
use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

use gvas::GvasFile;

#[test]
fn assert_failed() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/assert_failed.sav");
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
    file.write(&mut writer).expect("Failed to write test asset");

    // Compare the two Vec<u8>s
    assert_eq!(cursor.get_ref(), writer.get_ref());
}
