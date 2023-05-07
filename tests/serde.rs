use std::{collections::HashMap, fs::File, path::Path};

use gvas::GvasFile;
use serde_json::json;

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
fn deserialize() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/SaveSlot_03.sav");
    let mut file = File::open(path).expect("Failed to open test asset");

    let file =
        GvasFile::read_with_hints(&mut file, &get_hints()).expect("Failed to parse gvas file");
    serde_json::to_value(&file).unwrap();
}
