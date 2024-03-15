use crate::common::*;
use gvas::{game_version::GameVersion, GvasFile};
use std::{collections::HashMap, fs::File, path::Path};

fn test_file_with_hints(path: &str, hints: &HashMap<String, String>) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    let mut file = File::open(path).expect("Open test asset");
    let file =
        GvasFile::read_with_hints(&mut file, GameVersion::Default, hints).expect("Parse gvas file");
    let value = serde_json::to_string(&file).expect("Deserialize");
    let from_value = serde_json::from_str::<GvasFile>(value.as_str()).expect("Serialize");
    assert_eq!(file, from_value);
}

fn test_file(path: &str) {
    test_file_with_hints(path, &HashMap::new());
}

#[test]
fn serde_assert_failed() {
    test_file(ASSERT_FAILED_PATH);
}

#[ignore] // This test takes ~5 seconds to run
#[test]
fn serde_component8() {
    test_file(COMPONENT8_PATH);
}

#[test]
fn serde_delgate() {
    test_file(DELEGATE_PATH);
}

#[ignore] // This test takes ~5 seconds to run
#[test]
fn serde_enum_array() {
    test_file(ENUM_ARRAY_PATH);
}

#[test]
fn serde_features_01() {
    test_file_with_hints(FEATURES_01_PATH, &features::hints());
}

#[test]
fn serde_options() {
    test_file(OPTIONS_PATH);
}

#[ignore] // This test takes ~5 seconds to run
#[test]
fn serde_package_version_524() {
    test_file(PACKAGE_VERSION_524_PATH);
}

#[test]
fn serde_regression_01() {
    test_file(REGRESSION_01_PATH);
}

#[test]
fn serde_saveslot_03() {
    test_file_with_hints(SAVESLOT_03_PATH, &saveslot3::hints());
}

#[test]
fn serde_slot1() {
    test_file(SLOT1_PATH);
}

#[test]
fn serde_slot2() {
    test_file(SLOT2_PATH);
}

#[ignore] // This test takes ~5 seconds to run
#[test]
fn serde_slot3() {
    test_file(SLOT3_PATH);
}

#[ignore] // This test takes ~5 seconds to run
#[test]
fn serde_transform() {
    test_file(TRANSFORM_PATH);
}

#[test]
fn serde_vector2d() {
    test_file(VECTOR2D_PATH);
}
