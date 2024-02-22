use std::{
    collections::HashMap,
    fs::File,
    io::{Cursor, Read},
    path::Path,
    str::FromStr,
};

use gvas::engine_version::FEngineVersion;
use gvas::game_version::GameVersion;
use gvas::{
    properties::{
        field_path_property::{FieldPath, FieldPathProperty},
        int_property::{FloatProperty, IntProperty},
        map_property::MapProperty,
        name_property::NameProperty,
        object_property::ObjectProperty,
        str_property::StrProperty,
        struct_property::{StructProperty, StructPropertyValue},
        struct_types::DateTime,
        Property,
    },
    types::Guid,
    GvasFile, GvasHeader,
};
use indexmap::IndexMap;

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

    // Read the file in to a GvasFile
    let hints = get_hints();
    let file = GvasFile::read_with_hints(&mut file, GameVersion::Default, &hints)
        .expect("Failed to parse gvas file");

    assert_eq!(
        file.header,
        GvasHeader::Version2 {
            package_file_version: 522,
            engine_version: FEngineVersion {
                major: 4,
                minor: 27,
                patch: 2,
                change_list: 18319896,
                branch: String::from("++UE4+Release-4.27")
            },
            custom_version_format: 3,
            custom_versions: IndexMap::from([
                (
                    Guid::from_str("FA7AF5FC-8342-7650-58E6-A9B9322DA0FF").unwrap(),
                    68
                ),
                (
                    Guid::from_str("FA7AF5FC-8342-7650-58E6-A9B9322DA0FF").unwrap(),
                    68
                ),
                (
                    Guid::from_str("12E426FB-4D4B-151F-0A55-7293702F1D96").unwrap(),
                    3
                ),
                (
                    Guid::from_str("FB0C82A7-5943-A720-142C-548C50CF2396").unwrap(),
                    6
                ),
                (
                    Guid::from_str("4E7CE782-A543-2333-C513-6BB4F30D3197").unwrap(),
                    0
                ),
                (
                    Guid::from_str("ED0A3111-614D-552E-A39A-67AF2C08A1C5").unwrap(),
                    17
                ),
                (
                    Guid::from_str("F37ABB24-834F-4656-C22D-2F1FFF96AD49").unwrap(),
                    5
                ),
                (
                    Guid::from_str("2923A576-B545-2309-41D8-AE98D86A2FCF").unwrap(),
                    5
                ),
                (
                    Guid::from_str("0769BC5F-AE40-C855-84F1-678E3FF1FF5E").unwrap(),
                    1
                ),
                (
                    Guid::from_str("22D5549C-BE4F-26A8-4607-2194D082B461").unwrap(),
                    43
                ),
                (
                    Guid::from_str("E432D8B0-0D4F-891F-B77E-CFACA24AFD36").unwrap(),
                    10
                ),
                (
                    Guid::from_str("2843C6E1-534D-2CA2-868E-6CA38CBD1764").unwrap(),
                    0
                ),
                (
                    Guid::from_str("3CC15E37-FB48-E406-F084-00B57E712A26").unwrap(),
                    4
                ),
                (
                    Guid::from_str("ED68B0E4-E942-94F4-0BDA-31A241BB462E").unwrap(),
                    40
                ),
                (
                    Guid::from_str("3F74FCCF-8044-B043-DF14-919373201D17").unwrap(),
                    37
                ),
                (
                    Guid::from_str("B5492BB0-E944-20BB-B732-04A36003E452").unwrap(),
                    3
                ),
                (
                    Guid::from_str("5C10E4A4-B549-A159-C440-C5A7EEDF7E54").unwrap(),
                    0
                ),
                (
                    Guid::from_str("C931C839-DC47-E65A-179C-449A7C8E1C3E").unwrap(),
                    0
                ),
                (
                    Guid::from_str("331BF078-984F-EAEB-EA84-B4B9A25AB9CC").unwrap(),
                    14
                ),
                (
                    Guid::from_str("0F383166-E043-4D2D-27CF-09805AA95669").unwrap(),
                    0
                ),
                (
                    Guid::from_str("9F8BF812-FC4A-7588-0CD9-7CA629BD3A38").unwrap(),
                    45
                ),
                (
                    Guid::from_str("4CE75A7B-104C-70D2-9857-58A95A2A210B").unwrap(),
                    13
                ),
                (
                    Guid::from_str("186929D7-DD4B-D61D-A864-E29D8438C13C").unwrap(),
                    3
                ),
                (
                    Guid::from_str("7852A1C2-FE4A-E7BF-FF90-176C55F71D53").unwrap(),
                    1
                ),
                (
                    Guid::from_str("D4A3AC6E-C14C-EC40-ED8B-86B7C58F4209").unwrap(),
                    3
                ),
                (
                    Guid::from_str("DD75E529-2746-A3E0-76D2-109DEADC2C23").unwrap(),
                    17
                ),
                (
                    Guid::from_str("5DA643AF-4749-D37F-8E3E-739805BBC1D9").unwrap(),
                    15
                ),
                (
                    Guid::from_str("EC6C266B-8F4B-C71E-D9E4-0BA307FC4209").unwrap(),
                    1
                ),
                (
                    Guid::from_str("613DF70D-EA47-3FA2-E989-27B79A49410C").unwrap(),
                    1
                ),
                (
                    Guid::from_str("86181D60-844F-64AC-DED3-16AAD6C7EA0D").unwrap(),
                    47
                ),
                (
                    Guid::from_str("686308E7-584C-236B-701B-3984915E2616").unwrap(),
                    1
                ),
                (
                    Guid::from_str("D6BCFF9D-5801-4F49-8212-21E288A8923C").unwrap(),
                    10
                ),
                (
                    Guid::from_str("ACD0AEF2-6F41-FE9A-7FAA-6486FCD626FA").unwrap(),
                    1
                ),
                (
                    Guid::from_str("0B1F4F17-A545-C6B4-E82E-3FB17D91FBD0").unwrap(),
                    10
                ),
                (
                    Guid::from_str("834AF935-6C40-58E2-F509-18A37C241096").unwrap(),
                    41
                ),
                (
                    Guid::from_str("6EC18FB6-E242-1B8B-5C21-53B4FE448805").unwrap(),
                    1
                ),
                (
                    Guid::from_str("0685E1B2-C2CF-7342-BBF4-4EA507BA8B75").unwrap(),
                    1
                ),
                (
                    Guid::from_str("3689F564-BA42-1BFD-8972-96BA4EFAD0D5").unwrap(),
                    1
                ),
                (
                    Guid::from_str("27D80E6F-9548-09A6-8D99-919CA40E1890").unwrap(),
                    2
                ),
                (
                    Guid::from_str("E79E7F71-3A49-B0E9-3291-B3880781381B").unwrap(),
                    8
                ),
                (
                    Guid::from_str("50326854-AF48-9980-9698-C88BB7F9ADFB").unwrap(),
                    0
                ),
                (
                    Guid::from_str("194D0C43-7049-5471-699B-6987E5B090DF").unwrap(),
                    15
                ),
                (
                    Guid::from_str("BD32FEAA-144C-9553-255E-6AB6DDD13210").unwrap(),
                    1
                ),
                (
                    Guid::from_str("8EE1AF23-584E-E14C-52C2-618DB7BE53B9").unwrap(),
                    11
                ),
                (
                    Guid::from_str("EAB762A4-3A4E-99F4-1FEC-C199B2E12482").unwrap(),
                    4
                ),
                (
                    Guid::from_str("BDFDB52E-104D-AC01-8FF3-3681DAA59333").unwrap(),
                    5
                ),
                (
                    Guid::from_str("4F359D50-2F49-E6F6-B285-49A71C633C07").unwrap(),
                    0
                ),
                (
                    Guid::from_str("40EB564A-DC11-F510-7E34-D392E76AC9B2").unwrap(),
                    2
                ),
                (
                    Guid::from_str("004A8AD7-9746-58E8-B519-A8BAB4467D48").unwrap(),
                    18
                ),
                (
                    Guid::from_str("86F87955-1F4C-3A93-7B08-BA832FB96163").unwrap(),
                    2
                ),
                (
                    Guid::from_str("52BE2F61-0B40-53DA-914F-0D917C85B19F").unwrap(),
                    1
                ),
                (
                    Guid::from_str("367A23A4-C941-EACA-F818-A28FF31B6858").unwrap(),
                    4
                ),
                (
                    Guid::from_str("753F4E80-494B-8870-068C-D6A4DCB67E3C").unwrap(),
                    5
                ),
                (
                    Guid::from_str("F20A68FB-A34B-EF59-B519-A8BA3D44C873").unwrap(),
                    2
                ),
                (
                    Guid::from_str("0EB75099-174E-1AB4-0DFA-CCBBD67F8157").unwrap(),
                    1
                )
            ]),
            save_game_class_name: String::from("/Script/CD.CDSave_GameState")
        }
    );

    assert_eq!(file.properties.len(), 6);
    for (count, (key, value)) in file.properties.iter().enumerate() {
        assert_eq!(
            value.to_owned(),
            match (count, key.as_str()) {
                (0, "LastSaveTime") => Property::from(StructProperty::from(DateTime {
                    ticks: 638160761644140000
                })),
                (1, "PlayerClass") => Property::from(ObjectProperty::from(
                    "/Game/Character/Player/Blueprints/BP_Soldier.BP_Soldier_C"
                )),
                (2, "Version") => Property::from(IntProperty::new(3)),
                (3, "GameplayDatabase") => Property::from(MapProperty::new(
                    String::from("NameProperty"),
                    String::from("StructProperty"),
                    0,
                    IndexMap::from([
                        (
                            Property::from(NameProperty::from("unlock.welcomescreen.seen")),
                            Property::from(StructProperty::new(
                                Guid::from(0),
                                StructPropertyValue::CustomStruct(
                                    String::from("Struct"),
                                    vec![
                                        (
                                            String::from("AsFloat"),
                                            Property::from(FloatProperty::new(0f32))
                                        ),
                                        (
                                            String::from("AsString"),
                                            Property::from(StrProperty::new(None))
                                        )
                                    ]
                                )
                            ))
                        ),
                        (
                            Property::from(NameProperty::from("game.tutorial.finished")),
                            Property::from(StructProperty::new(
                                Guid::from(0),
                                StructPropertyValue::CustomStruct(
                                    String::from("Struct"),
                                    vec![
                                        (
                                            String::from("AsFloat"),
                                            Property::from(FloatProperty::new(1f32))
                                        ),
                                        (
                                            String::from("AsString"),
                                            Property::from(StrProperty::new(None))
                                        )
                                    ],
                                )
                            )),
                        ),
                        (
                            Property::from(NameProperty::from("game.tutorial.skipped")),
                            Property::from(StructProperty::new(
                                Guid::from(0),
                                StructPropertyValue::CustomStruct(
                                    String::from("Struct"),
                                    vec![
                                        (
                                            String::from("AsFloat"),
                                            Property::from(FloatProperty::new(1f32))
                                        ),
                                        (
                                            String::from("AsString"),
                                            Property::from(StrProperty::new(None))
                                        )
                                    ]
                                )
                            )),
                        ),
                        (
                            Property::from(NameProperty::from("dialogs.messages.seen.Rumiko.0.50")),
                            Property::from(StructProperty::new(
                                Guid::from(0),
                                StructPropertyValue::CustomStruct(
                                    String::from("Struct"),
                                    vec![
                                        (
                                            String::from("AsFloat"),
                                            Property::from(FloatProperty::new(1f32))
                                        ),
                                        (
                                            String::from("AsString"),
                                            Property::from(StrProperty::new(None))
                                        )
                                    ]
                                )
                            )),
                        ),
                        (
                            Property::from(NameProperty::from("codex.Rumiko")),
                            Property::from(StructProperty::new(
                                Guid::from(0),
                                StructPropertyValue::CustomStruct(
                                    String::from("Struct"),
                                    vec![
                                        (
                                            String::from("AsFloat"),
                                            Property::from(FloatProperty::new(1f32))
                                        ),
                                        (
                                            String::from("AsString"),
                                            Property::from(StrProperty::new(None))
                                        )
                                    ]
                                )
                            )),
                        ),
                    ]),
                )),
                (4, "PlayerAttributes") => Property::from(MapProperty::Properties {
                    key_type: String::from("StructProperty"),
                    value_type: String::from("FloatProperty"),
                    allocation_flags: 0,
                    value: IndexMap::from([
                        (
                            Property::from(StructProperty {
                                guid: Guid::from(0),
                                value: StructPropertyValue::CustomStruct(
                                    String::from("Struct"),
                                    vec![
                                        (
                                            String::from("AttributeName"),
                                            Property::from(StrProperty::from(
                                                "Currency_Blueprints"
                                            ))
                                        ),
                                        (
                                            String::from("Attribute"),
                                            Property::from(FieldPathProperty::new(FieldPath::new(
                                                Vec::from([String::from("Currency_Blueprints")]),
                                                String::from("/Script/CD.CDPlayerAttributeSet")
                                            )))
                                        ),
                                        (
                                            String::from("AttributeOwner"),
                                            Property::from(ObjectProperty::from("None"))
                                        )
                                    ]
                                )
                            }),
                            Property::from(FloatProperty::new(0f32))
                        ),
                        (
                            Property::from(StructProperty {
                                guid: Guid::from(0),
                                value: StructPropertyValue::CustomStruct(
                                    String::from("Struct"),
                                    vec![
                                        (
                                            String::from("AttributeName"),
                                            Property::from(StrProperty::from("Currency_Electrum")),
                                        ),
                                        (
                                            String::from("Attribute"),
                                            Property::from(FieldPathProperty::new(FieldPath::new(
                                                Vec::from([String::from("Currency_Electrum")]),
                                                String::from("/Script/CD.CDPlayerAttributeSet")
                                            )))
                                        ),
                                        (
                                            String::from("AttributeOwner"),
                                            Property::from(ObjectProperty::from("None"))
                                        )
                                    ]
                                )
                            }),
                            Property::from(FloatProperty::new(0f32))
                        )
                    ])
                }),
                (5, "SecondaryWeaponClass") => Property::from(ObjectProperty::from(
                    "/Game/Weapons/RocketLauncher/Blueprints/BP_RocketLauncher.BP_RocketLauncher_C"
                )),

                _ => panic!("Unexpected key ({}, {})", count, key),
            }
        );
    }
}

#[test]
fn write_save_slot_03() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/SaveSlot_03.sav");
    let mut file = File::open(path).expect("Failed to open test asset");

    // Read the file in to a Vec<u8>
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    // Convert the Vec<u8> to a GvasFile
    let mut cursor = Cursor::new(data);
    let hints = get_hints();
    let file = GvasFile::read_with_hints(&mut cursor, GameVersion::Default, &hints)
        .expect("Failed to parse gvas file");

    // Convert the GvasFile back to a Vec<u8>
    let mut writer = Cursor::new(Vec::new());
    file.write(&mut writer)
        .expect("Failed to serialize gvas file");

    // Compare the two Vec<u8>s
    assert_eq!(cursor.get_ref(), writer.get_ref());

    // Read the file back in again
    let mut reader = Cursor::new(writer.get_ref().to_owned());
    let hints = get_hints();
    let read_back = GvasFile::read_with_hints(&mut reader, GameVersion::Default, &hints)
        .expect("Failed to read serialized gvas file");

    // Compare the two GvasFiles
    assert_eq!(file, read_back);
}
