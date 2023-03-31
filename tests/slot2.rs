use gvas::GvasFile;
use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

#[test]
fn read_slot2() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/Slot2.sav");
    let mut file = File::open(path).expect("Failed to open test asset");

    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    let mut cursor = Cursor::new(data);

    GvasFile::read(&mut cursor).expect("Failed to parse gvas file");
}

#[test]
fn write_slot2() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/Slot2.sav");
    let mut file = File::open(path).expect("Failed to open test asset");

    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    let mut cursor = Cursor::new(data);

    let file = GvasFile::read(&mut cursor).expect("Failed to parse gvas file");

    let mut writer = Cursor::new(Vec::new());
    file.write(&mut writer)
        .expect("Failed to serialize gvas file");

    let mut reader = Cursor::new(writer.get_ref().to_owned());
    GvasFile::read(&mut reader).expect("Failed to parse serialized save file");
}
