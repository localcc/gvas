use gvas::game_version::GameVersion;
use std::{collections::HashMap, fs::File, path::Path};

use gvas::GvasFile;

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
    test_file("resources/test/assert_failed.sav");
}

#[ignore] // This test takes ~5 seconds to run
#[test]
fn serde_component8() {
    test_file("resources/test/component8.sav");
}

#[test]
fn serde_delgate() {
    test_file("resources/test/Delegate.sav");
}

#[ignore] // This test takes ~5 seconds to run
#[test]
fn serde_enum_array() {
    test_file("resources/test/enum_array.sav");
}

#[test]
fn serde_features_01() {
    let mut hints = HashMap::new();
    hints.insert(
        "SeasonSave.StructProperty.Seasons.MapProperty.Key.StructProperty".to_string(),
        "Guid".to_string(),
    );
    hints.insert(
        "SeasonSave.StructProperty.Seasons.MapProperty.Value.StructProperty".to_string(),
        "Unk".to_string(),
    );
    hints.insert(
        "SeasonSave.StructProperty.Seasons.MapProperty.Value.StructProperty.CompletedSpecialChallenges.MapProperty.Key.StructProperty".to_string(),
        "Guid".to_string(),
    );
    hints.insert(
        "UnLockedMissionParameters.MapProperty.Key.StructProperty".to_string(),
        "Guid".to_string(),
    );
    hints.insert(
        "UnLockedMissionParameters.MapProperty.Value.StructProperty".to_string(),
        "Unk".to_string(),
    );
    hints.insert(
        "ItemUpgradeSelections.MapProperty.Key.StructProperty".to_string(),
        "Guid".to_string(),
    );
    hints.insert(
        "ItemUpgradeSelections.MapProperty.Value.StructProperty".to_string(),
        "Unk".to_string(),
    );
    hints.insert(
        "ItemUpgradeLoadouts.ArrayProperty.Loadout.MapProperty.Key.StructProperty".to_string(),
        "Guid".to_string(),
    );
    hints.insert(
        "ItemUpgradeLoadouts.ArrayProperty.Loadout.MapProperty.Value.StructProperty".to_string(),
        "Unk".to_string(),
    );
    hints.insert(
        "EnemiesKilled.MapProperty.Key.StructProperty".to_string(),
        "Guid".to_string(),
    );
    hints.insert(
        "UnlockedItemSkins.MapProperty.Key.StructProperty".to_string(),
        "Guid".to_string(),
    );
    hints.insert(
        "UnlockedItemSkins.MapProperty.Value.StructProperty".to_string(),
        "Unk".to_string(),
    );
    hints.insert(
        "Resources.StructProperty.OwnedResources.MapProperty.Key.StructProperty".to_string(),
        "Guid".to_string(),
    );
    hints.insert(
        "FSDEventRewardsSave.StructProperty.EventsSeen.SetProperty.StructProperty".to_string(),
        "Guid".to_string(),
    );
    hints.insert(
        "GameDLCSave.StructProperty.AnnouncedIDs.SetProperty.StructProperty".to_string(),
        "Guid".to_string(),
    );
    hints.insert(
        "Drinks.StructProperty.UnlockedDrinks.SetProperty.StructProperty".to_string(),
        "Guid".to_string(),
    );
    hints.insert(
        "UnlockedItemSkins.MapProperty.Value.StructProperty.Skins.SetProperty.StructProperty"
            .to_string(),
        "Guid".to_string(),
    );
    hints.insert(
        "UnlockedPickaxeParts.SetProperty.StructProperty".to_string(),
        "Guid".to_string(),
    );
    hints.insert(
        "MinersManualKnownObjects.SetProperty.StructProperty".to_string(),
        "Guid".to_string(),
    );
    test_file_with_hints("resources/test/features_01.bin", &hints);
}

#[test]
fn serde_options() {
    test_file("resources/test/Options.sav");
}

#[ignore] // This test takes ~5 seconds to run
#[test]
fn serde_package_version_524() {
    test_file("resources/test/package_version_524.sav");
}

#[test]
fn serde_regression_01() {
    test_file("resources/test/regression_01.bin");
}

#[test]
fn serde_saveslot_03() {
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
    test_file_with_hints("resources/test/SaveSlot_03.sav", &hints);
}

#[test]
fn serde_slot1() {
    test_file("resources/test/Slot1.sav");
}

#[test]
fn serde_slot2() {
    test_file("resources/test/Slot2.sav");
}

#[ignore] // This test takes ~5 seconds to run
#[test]
fn serde_slot3() {
    test_file("resources/test/Slot3.sav");
}

#[ignore] // This test takes ~5 seconds to run
#[test]
fn serde_transform() {
    test_file("resources/test/transform.sav");
}
