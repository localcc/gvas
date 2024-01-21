use gvas::game_version::GameVersion;
use gvas::GvasFile;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;

#[test]
fn read_zlib_palworld() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/palworld_zlib.sav");
    let mut file = File::open(path).expect("Failed to open test asset");

    // Read the file in to a Vec<u8>
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    // Convert the Vec<u8> to a GvasFile
    let mut cursor = Cursor::new(data);
    let file =
        GvasFile::read(&mut cursor, GameVersion::Palworld).expect("Failed to parse gvas file");

    // Convert the GvasFile back to a Vec<u8>
    let mut writer = Cursor::new(Vec::new());
    file.write(&mut writer).expect("Failed to write test asset");

    let mut cursor = Cursor::new(writer.get_ref());
    let reparsed =
        GvasFile::read(&mut cursor, GameVersion::Palworld).expect("Failed to reparse gvas file");

    assert_eq!(file, reparsed);
}

#[test]
fn read_zlib_twice_palworld() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/palworld_zlib_twice.sav");
    let mut file = File::open(path).expect("Failed to open test asset");

    // Read the file in to a Vec<u8>
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    let hints = HashMap::from([(
        String::from(
            "worldSaveData.StructProperty.CharacterSaveParameterMap.MapProperty.Key.StructProperty",
        ),
        String::from("StructProperty"),
    ), (
        String::from(
            "worldSaveData.StructProperty.CharacterSaveParameterMap.MapProperty.Value.StructProperty",
        ),
        String::from("StructProperty"),
    ), (
        String::from(
            "worldSaveData.StructProperty.MapObjectSaveData.ArrayProperty.ConcreteModel.StructProperty.ModuleMap.MapProperty.Value.StructProperty",
        ),
        String::from("StructProperty"),
    ), (
        String::from("worldSaveData.StructProperty.FoliageGridSaveDataMap.MapProperty.Key.StructProperty"),
        String::from("StructProperty"),
    ), (
        String::from("worldSaveData.StructProperty.FoliageGridSaveDataMap.MapProperty.Value.StructProperty"),
        String::from("StructProperty"),
    ), (
        String::from("worldSaveData.StructProperty.FoliageGridSaveDataMap.MapProperty.Value.StructProperty.ModelMap.MapProperty.Value.StructProperty"),
        String::from("StructProperty")
    ), (
        String::from("worldSaveData.StructProperty.FoliageGridSaveDataMap.MapProperty.Value.StructProperty.ModelMap.MapProperty.Value.StructProperty.InstanceDataMap.MapProperty.Key.StructProperty"),
        String::from("StructProperty")
    ), (
        String::from("worldSaveData.StructProperty.FoliageGridSaveDataMap.MapProperty.Value.StructProperty.ModelMap.MapProperty.Value.StructProperty.InstanceDataMap.MapProperty.Value.StructProperty"),
        String::from("StructProperty")
    ), (
        String::from("worldSaveData.StructProperty.MapObjectSpawnerInStageSaveData.MapProperty.Key.StructProperty"), 
        String::from("StructProperty")
    ), (
        String::from("worldSaveData.StructProperty.MapObjectSpawnerInStageSaveData.MapProperty.Value.StructProperty"),
        String::from("StructProperty")
    ), (
        String::from("worldSaveData.StructProperty.MapObjectSpawnerInStageSaveData.MapProperty.Value.StructProperty.SpawnerDataMapByLevelObjectInstanceId.MapProperty.Key.StructProperty"),
        String::from("Guid")
    ), (
        String::from("worldSaveData.StructProperty.MapObjectSpawnerInStageSaveData.MapProperty.Value.StructProperty.SpawnerDataMapByLevelObjectInstanceId.MapProperty.Value.StructProperty"),
        String::from("StructProperty")
    ), (
        String::from("worldSaveData.StructProperty.MapObjectSpawnerInStageSaveData.MapProperty.Value.StructProperty.SpawnerDataMapByLevelObjectInstanceId.MapProperty.Value.StructProperty.ItemMap.MapProperty.Value.StructProperty"), 
        String::from("StructProperty")
    ), (
        String::from("worldSaveData.StructProperty.BaseCampSaveData.MapProperty.Key.StructProperty"),
        String::from("Guid")
    ), (
        String::from("worldSaveData.StructProperty.BaseCampSaveData.MapProperty.Value.StructProperty"),
        String::from("StructProperty")
    ), (
        String::from("worldSaveData.StructProperty.BaseCampSaveData.MapProperty.Value.StructProperty.ModuleMap.MapProperty.Value.StructProperty"),
        String::from("StructProperty")
    ), (
        String::from("worldSaveData.StructProperty.ItemContainerSaveData.MapProperty.Key.StructProperty"),
        String::from("StructProperty")
    ), (
        String::from("worldSaveData.StructProperty.ItemContainerSaveData.MapProperty.Value.StructProperty"),
        String::from("StructProperty")
    ), (
        String::from("worldSaveData.StructProperty.CharacterContainerSaveData.MapProperty.Key.StructProperty"),
        String::from("StructProperty")
    ), (
        String::from("worldSaveData.StructProperty.CharacterContainerSaveData.MapProperty.Value.StructProperty"),
        String::from("StructProperty")
    ), (
        String::from("worldSaveData.StructProperty.GroupSaveDataMap.MapProperty.Key.StructProperty"),
        String::from("Guid")
    ), (
        String::from("worldSaveData.StructProperty.GroupSaveDataMap.MapProperty.Value.StructProperty"),
        String::from("StructProperty")
    ), (
        String::from("worldSaveData.StructProperty.EnemyCampSaveData.StructProperty.EnemyCampStatusMap.MapProperty.Value.StructProperty"),
        String::from("StructProperty")
    )]);

    // Convert the Vec<u8> to a GvasFile
    let mut cursor = Cursor::new(data);
    let file = GvasFile::read_with_hints(&mut cursor, GameVersion::Palworld, &hints)
        .expect("Failed to parse gvas file");

    // Convert the GvasFile back to a Vec<u8>
    let mut writer = Cursor::new(Vec::new());
    file.write(&mut writer).expect("Failed to write test asset");

    let mut cursor = Cursor::new(writer.get_ref());
    let reparsed = GvasFile::read_with_hints(&mut cursor, GameVersion::Palworld, &hints)
        .expect("Failed to reparse gvas file");

    assert_eq!(file, reparsed);
}
