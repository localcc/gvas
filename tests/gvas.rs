mod common;
use common::*;
mod gvas_tests;
use gvas::{
    game_version::{DeserializedGameVersion, GameVersion, PalworldCompressionType},
    GvasFile,
};
use std::{collections::HashMap, fs, io::Cursor, path::Path};

fn test_gvas_file(path: &str) -> GvasFile {
    test_gvas_file_(path, GameVersion::Default, &HashMap::new())
}

fn test_gvas_file_(
    path: &str,
    game_version: GameVersion,
    hints: &HashMap<String, String>,
) -> GvasFile {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);

    // Read the file in to a Vec<u8>
    let data = fs::read(path).expect("Read test asset");

    // Convert the Vec<u8> to a GvasFile
    let mut cursor = Cursor::new(data);
    let file = GvasFile::read_with_hints(&mut cursor, game_version, hints).expect("Read GvasFile");

    // Convert the GvasFile back to a Vec<u8>
    let mut writer = Cursor::new(Vec::new());
    file.write(&mut writer).expect("Write GvasFile");

    // Compare the two Vec<u8>s
    if match file.deserialized_game_version {
        DeserializedGameVersion::Default => true,
        DeserializedGameVersion::Palworld(PalworldCompressionType::Zlib) => false,
        DeserializedGameVersion::Palworld(PalworldCompressionType::ZlibTwice) => false,
        _ => unimplemented!(),
    } {
        assert_eq!(cursor.get_ref(), writer.get_ref());
    }

    // Read the file back in again
    let mut reader = Cursor::new(writer.into_inner());
    let file2 = GvasFile::read_with_hints(&mut reader, game_version, hints).expect("Read GvasFile");

    // Compare the two GvasFiles
    assert_eq!(file, file2);

    // Pass the file back for optional verification
    file
}

#[test]
fn assert_failed() {
    test_gvas_file(ASSERT_FAILED_PATH);
}

#[test]
fn component8() {
    test_gvas_file(COMPONENT8_PATH);
}

#[test]
fn delegate() {
    assert_eq!(test_gvas_file(DELEGATE_PATH), delegate::expected());
}

#[test]
fn enum_array() {
    test_gvas_file(ENUM_ARRAY_PATH);
}

#[test]
fn features_01() {
    test_gvas_file_(FEATURES_01_PATH, GameVersion::Default, &features::hints());
}

#[test]
fn options() {
    assert_eq!(test_gvas_file(OPTIONS_PATH), options::expected());
}

#[test]
fn package_version_524() {
    test_gvas_file(PACKAGE_VERSION_524_PATH);
}

#[test]
fn palworld_zlib() {
    test_gvas_file_(PALWORLD_ZLIB_PATH, GameVersion::Palworld, &HashMap::new());
}

#[test]
fn palworld_zlib_twice() {
    test_gvas_file_(
        PALWORLD_ZLIB_TWICE_PATH,
        GameVersion::Palworld,
        &palworld::hints(),
    );
}

#[test]
fn regression_01() {
    test_gvas_file(REGRESSION_01_PATH);
}

#[test]
fn saveslot03() {
    assert_eq!(
        test_gvas_file_(SAVESLOT_03_PATH, GameVersion::Default, &saveslot3::hints()),
        saveslot3::expected()
    );
}

#[test]
fn slot1() {
    assert_eq!(test_gvas_file(SLOT1_PATH), slot1::expected());
}

#[test]
fn slot2() {
    test_gvas_file(SLOT2_PATH);
}

#[test]
fn slot3() {
    test_gvas_file(SLOT3_PATH);
}

#[test]
fn text_property_noarray() {
    test_gvas_file(TEXT_PROPERTY_NOARRAY);
}

#[test]
fn transform() {
    test_gvas_file(TRANSFORM_PATH);
}

#[test]
fn vector2d() {
    assert_eq!(test_gvas_file(VECTOR2D_PATH), vector2d::expected());
}
