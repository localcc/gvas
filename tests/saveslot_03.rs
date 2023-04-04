use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

use gvas::GvasFile;

fn get_hints() -> HashMap<String, String> {
    let mut hints = HashMap::new();

    hints.insert(
        "MinersManualKnownObjects.SetProperty.StructProperty".to_string(),
        "Struct".to_string(),
    );
    hints.insert(
        "GameplayDatabase.MapProperty.Value.StructProperty".to_string(),
        "Struct".to_string(),
    );
    hints.insert(
        "PlayerAttributes.MapProperty.Key.StructProperty".to_string(),
        "Struct".to_string(),
    );

    hints
}

#[test]
fn read_save_slot_03() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/SaveSlot_03.sav");
    let mut file = File::open(path).expect("Failed to open test asset");

    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    let mut cursor = Cursor::new(data);
    let hints = get_hints();

    GvasFile::read_with_hints(&mut cursor, &hints).expect("Failed to parse gvas file");
}

#[test]
fn write_save_slot_03() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/SaveSlot_03.sav");
    let mut file = File::open(path).expect("Failed to open test asset");

    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    let mut cursor = Cursor::new(data);
    let hints = get_hints();

    let file = GvasFile::read_with_hints(&mut cursor, &hints).expect("Failed to parse gvas file");

    let mut writer = Cursor::new(Vec::new());
    file.write(&mut writer)
        .expect("Failed to serialize gvas file");

    let mut reader = Cursor::new(writer.get_ref().to_owned());
    GvasFile::read_with_hints(&mut reader, &hints).expect("Failed to read serialized gvas file");
}
