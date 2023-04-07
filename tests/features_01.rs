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
        "SeasonSave.StructProperty.Seasons.MapProperty.Key.StructProperty".to_string(),
        "Guid".to_string(),
    );

    hints.insert(
        "SeasonSave.StructProperty.Seasons.MapProperty.Value.StructProperty".to_string(),
        "Unk".to_string(),
    );

    hints.insert(
        "SeasonSave.StructProperty.Seasons.MapProperty.Value.StructProperty.CompletedSpecialChallenges.MapProperty.Key.StructProperty".to_string(), 
        "Guid".to_string()
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

    hints
}

#[test]
fn read_features_01() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/features_01.bin");
    let mut file = File::open(path).expect("Failed to open test asset");

    // Read the file in to a Vec<u8>
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    // Convert the Vec<u8> to a GvasFile
    let mut cursor = Cursor::new(data);
    let hints = get_hints();
    GvasFile::read_with_hints(&mut cursor, &hints).expect("Failed to parse gvas file");
}

#[test]
fn write_features_01() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/features_01.bin");
    let mut file = File::open(path).expect("Failed to open test asset!");

    // Read the file in to a Vec<u8>
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    // Convert the Vec<u8> to a GvasFile
    let mut cursor = Cursor::new(data);
    let hints = get_hints();
    let file = GvasFile::read_with_hints(&mut cursor, &hints).expect("Failed to parse gvas file");

    // Convert the GvasFile back to a Vec<u8>
    let mut writer = Cursor::new(Vec::new());
    file.write(&mut writer)
        .expect("Failed to serialize gvas file");

    // Compare the two Vec<u8>s
    assert_eq!(cursor.get_ref(), writer.get_ref());

    // Read the file back in again
    let mut reader = Cursor::new(writer.get_ref().to_owned());
    let file2 = GvasFile::read_with_hints(&mut reader, &hints)
        .expect("Failed to read serialized gvas file");

    // Compare the two GvasFiles
    assert_eq!(file, file2);
}
