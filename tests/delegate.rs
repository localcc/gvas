use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
    str::FromStr,
};

use gvas::{
    properties::{
        delegate_property::{
            Delegate, DelegateProperty, MulticastInlineDelegateProperty, MulticastScriptDelegate,
            MulticastSparseDelegateProperty,
        },
        Property,
    },
    types::Guid,
    FCustomVersion, FEngineVersion, GvasFile, GvasHeader,
};

#[test]
fn read_delegate() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/Delegate.sav");
    let mut file = File::open(path).expect("Failed to open test asset");

    let file = GvasFile::read(&mut file).expect("Failed to parse gvas file");

    assert_eq!(
        file.header,
        GvasHeader::Version2 {
            package_file_version: 517,
            engine_version: FEngineVersion {
                major: 4,
                minor: 23,
                patch: 1,
                change_list: 9631420,
                branch: String::from("++UE4+Release-4.23"),
            },
            custom_version_format: 3,
            custom_versions: vec![
                FCustomVersion {
                    key: Guid::from_str("22D5549C-BE4F-26A8-4607-2194D082B461").unwrap(),
                    version: 23,
                },
                FCustomVersion {
                    key: Guid::from_str("E432D8B0-0D4F-891F-B77E-CFACA24AFD36").unwrap(),
                    version: 10,
                },
                FCustomVersion {
                    key: Guid::from_str("2843C6E1-534D-2CA2-868E-6CA38CBD1764").unwrap(),
                    version: 0,
                },
                FCustomVersion {
                    key: Guid::from_str("3CC15E37-FB48-E406-F084-00B57E712A26").unwrap(),
                    version: 3,
                },
                FCustomVersion {
                    key: Guid::from_str("ED68B0E4-E942-94F4-0BDA-31A241BB462E").unwrap(),
                    version: 34,
                },
                FCustomVersion {
                    key: Guid::from_str("3F74FCCF-8044-B043-DF14-919373201D17").unwrap(),
                    version: 35,
                },
                FCustomVersion {
                    key: Guid::from_str("B5492BB0-E944-20BB-B732-04A36003E452").unwrap(),
                    version: 2,
                },
                FCustomVersion {
                    key: Guid::from_str("5C10E4A4-B549-A159-C440-C5A7EEDF7E54").unwrap(),
                    version: 0,
                },
                FCustomVersion {
                    key: Guid::from_str("C931C839-DC47-E65A-179C-449A7C8E1C3E").unwrap(),
                    version: 0,
                },
                FCustomVersion {
                    key: Guid::from_str("331BF078-984F-EAEB-EA84-B4B9A25AB9CC").unwrap(),
                    version: 0,
                },
                FCustomVersion {
                    key: Guid::from_str("0F383166-E043-4D2D-27CF-09805AA95669").unwrap(),
                    version: 0,
                },
                FCustomVersion {
                    key: Guid::from_str("9F8BF812-FC4A-7588-0CD9-7CA629BD3A38").unwrap(),
                    version: 31,
                },
                FCustomVersion {
                    key: Guid::from_str("4CE75A7B-104C-70D2-9857-58A95A2A210B").unwrap(),
                    version: 11,
                },
                FCustomVersion {
                    key: Guid::from_str("186929D7-DD4B-D61D-A864-E29D8438C13C").unwrap(),
                    version: 2,
                },
                FCustomVersion {
                    key: Guid::from_str("7852A1C2-FE4A-E7BF-FF90-176C55F71D53").unwrap(),
                    version: 1,
                },
                FCustomVersion {
                    key: Guid::from_str("D4A3AC6E-C14C-EC40-ED8B-86B7C58F4209").unwrap(),
                    version: 3,
                },
                FCustomVersion {
                    key: Guid::from_str("DD75E529-2746-A3E0-76D2-109DEADC2C23").unwrap(),
                    version: 17,
                },
                FCustomVersion {
                    key: Guid::from_str("5DA643AF-4749-D37F-8E3E-739805BBC1D9").unwrap(),
                    version: 2,
                },
                FCustomVersion {
                    key: Guid::from_str("EC6C266B-8F4B-C71E-D9E4-0BA307FC4209").unwrap(),
                    version: 1,
                },
                FCustomVersion {
                    key: Guid::from_str("613DF70D-EA47-3FA2-E989-27B79A49410C").unwrap(),
                    version: 1,
                },
                FCustomVersion {
                    key: Guid::from_str("86181D60-844F-64AC-DED3-16AAD6C7EA0D").unwrap(),
                    version: 27,
                },
                FCustomVersion {
                    key: Guid::from_str("D6BCFF9D-5801-4F49-8212-21E288A8923C").unwrap(),
                    version: 6,
                },
                FCustomVersion {
                    key: Guid::from_str("ACD0AEF2-6F41-FE9A-7FAA-6486FCD626FA").unwrap(),
                    version: 1,
                },
                FCustomVersion {
                    key: Guid::from_str("0B1F4F17-A545-C6B4-E82E-3FB17D91FBD0").unwrap(),
                    version: 9,
                },
                FCustomVersion {
                    key: Guid::from_str("E79E7F71-3A49-B0E9-3291-B3880781381B").unwrap(),
                    version: 6,
                },
                FCustomVersion {
                    key: Guid::from_str("B3DC7D8E-BB47-DA80-A246-D39FF64D9893").unwrap(),
                    version: 1,
                },
                FCustomVersion {
                    key: Guid::from_str("CDB08ACB-DE4B-8CE7-9313-62A862EFE914").unwrap(),
                    version: 0,
                },
                FCustomVersion {
                    key: Guid::from_str("F20A68FB-A34B-EF59-B519-A8BA3D44C873").unwrap(),
                    version: 1,
                },
                FCustomVersion {
                    key: Guid::from_str("9186E0AF-5249-0D3A-3B67-73B61E2DF27C").unwrap(),
                    version: 2,
                },
                FCustomVersion {
                    key: Guid::from_str("BDFDB52E-104D-AC01-8FF3-3681DAA59333").unwrap(),
                    version: 5,
                },
                FCustomVersion {
                    key: Guid::from_str("4F359D50-2F49-E6F6-B285-49A71C633C07").unwrap(),
                    version: 0,
                },
                FCustomVersion {
                    key: Guid::from_str("EAB762A4-3A4E-99F4-1FEC-C199B2E12482").unwrap(),
                    version: 2,
                },
                FCustomVersion {
                    key: Guid::from_str("194D0C43-7049-5471-699B-6987E5B090DF").unwrap(),
                    version: 13,
                },
                FCustomVersion {
                    key: Guid::from_str("BD32FEAA-144C-9553-255E-6AB6DDD13210").unwrap(),
                    version: 1,
                },
                FCustomVersion {
                    key: Guid::from_str("8EE1AF23-584E-E14C-52C2-618DB7BE53B9").unwrap(),
                    version: 8,
                },
                FCustomVersion {
                    key: Guid::from_str("40EB564A-DC11-F510-7E34-D392E76AC9B2").unwrap(),
                    version: 2,
                },
                FCustomVersion {
                    key: Guid::from_str("004A8AD7-9746-58E8-B519-A8BAB4467D48").unwrap(),
                    version: 17,
                },
                FCustomVersion {
                    key: Guid::from_str("86F87955-1F4C-3A93-7B08-BA832FB96163").unwrap(),
                    version: 1,
                },
                FCustomVersion {
                    key: Guid::from_str("52BE2F61-0B40-53DA-914F-0D917C85B19F").unwrap(),
                    version: 1,
                },
                FCustomVersion {
                    key: Guid::from_str("367A23A4-C941-EACA-F818-A28FF31B6858").unwrap(),
                    version: 4,
                },
                FCustomVersion {
                    key: Guid::from_str("753F4E80-494B-8870-068C-D6A4DCB67E3C").unwrap(),
                    version: 5,
                },
                FCustomVersion {
                    key: Guid::from_str("ED0A3111-614D-552E-A39A-67AF2C08A1C5").unwrap(),
                    version: 17,
                },
                FCustomVersion {
                    key: Guid::from_str("965196AB-FC08-D845-8D22-D7B79E56AD78").unwrap(),
                    version: 1,
                },
                FCustomVersion {
                    key: Guid::from_str("F37ABB24-834F-4656-C22D-2F1FFF96AD49").unwrap(),
                    version: 4,
                },
                FCustomVersion {
                    key: Guid::from_str("12E426FB-4D4B-151F-0A55-7293702F1D96").unwrap(),
                    version: 3,
                },
            ],
            save_game_class_name: String::from("/Script/SaveFileTest.TestSaveGame"),
        }
    );

    assert_eq!(file.properties.len(), 3);

    for (count, (key, value)) in file.properties.iter().enumerate() {
        assert_eq!(
            value.to_owned(),
            match (count, key.as_str()) {
                (0, "DynamicDelegate") => Property::from(DelegateProperty::new(Delegate::new(
                    String::from(
                        "/Temp/UEDPIE_0_Untitled_1.Untitled_1:PersistentLevel.Saver1_Blueprint_2"
                    ),
                    String::from("FirstBinding")
                ))),
                (1, "MulticastDelegate") => Property::from(MulticastInlineDelegateProperty::new(MulticastScriptDelegate::new(vec![
                    Delegate::new(String::from("/Temp/UEDPIE_0_Untitled_1.Untitled_1:PersistentLevel.Saver1_Blueprint_2"), String::from("FirstBinding")),
                    Delegate::new(String::from("/Temp/UEDPIE_0_Untitled_1.Untitled_1:PersistentLevel.Saver1_Blueprint_2"), String::from("SecondBinding")),
                ]))),
                (2, "MulticastSparseDelegate") => Property::from(MulticastSparseDelegateProperty::new(MulticastScriptDelegate::new(
                    vec![
                        Delegate::new(String::from("/Temp/UEDPIE_0_Untitled_1.Untitled_1:PersistentLevel.Saver1_Blueprint_2"), String::from("FirstBinding")),
                    ]
                ))),
                _ => panic!("Unexpected key ({}, {})", count, key),
            }
        )
    }
}

#[test]
fn write_delegate() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/Delegate.sav");
    let mut file = File::open(path).expect("Failed to open test asset");

    // Read the file in to a Vec<u8>
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    // Convert the Vec<u8> to a GvasFile
    let mut cursor = Cursor::new(data);
    let file = GvasFile::read(&mut cursor).expect("Failed to parse gvas file");

    // Convert the GvasFile back to a Vec<u8>
    let mut writer = Cursor::new(Vec::new());
    file.write(&mut writer)
        .expect("Failed to serialize gvas file");

    // Compare the two Vec<u8>s
    assert_eq!(cursor.get_ref(), writer.get_ref());

    // Read the file back in again
    let mut reader = Cursor::new(writer.get_ref().to_owned());
    let read_back = GvasFile::read(&mut reader).expect("Failed to read serialized gvas file");

    // Compare the two GvasFiles
    assert_eq!(file, read_back);
}
