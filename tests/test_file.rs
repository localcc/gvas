use std::io::Cursor;

use indexmap::IndexMap;

use gvas::{
    properties::{str_property::StrProperty, Property},
    FEngineVersion, GvasFile, GvasHeader,
};

#[test]
fn test_file() {
    let header = GvasHeader {
        file_type_tag: 0,
        save_game_file_version: 0,
        package_file_ue4_version: 0,
        engine_version: FEngineVersion {
            major: 1,
            minor: 2,
            patch: 3,
            change_list: 4,
            branch: "engine version branch".to_string(),
        },
        custom_version_format: 0,
        custom_versions: vec![],
        save_game_class_name: "save game class name".to_string(),
    };

    let mut properties: IndexMap<String, Property> = IndexMap::new();

    let str_property = StrProperty::from("Test for StrProperty");
    properties.insert("StrProperty".into(), str_property.into());

    let file = GvasFile { header, properties };

    // Serialize GvasFile to Vec<u8>
    let mut writer = Cursor::new(Vec::new());
    file.write(&mut writer)
        .expect("Failed to serialize gvas file");

    // Read GvasFile from Vec<u8>
    let mut reader = Cursor::new(writer.get_ref().to_owned());
    let imported = GvasFile::read(&mut reader).expect("Failed to parse serialized save file");

    // Compare the imported value to `file`
    assert_eq!(file, imported);
}
