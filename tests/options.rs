use gvas::{
    properties::{int_property::FloatProperty, Property},
    FCustomVersion, FEngineVersion, GvasFile, GvasHeader,
};
use indexmap::IndexMap;
use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
    str::FromStr,
};

#[test]
fn test_options() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/Options.sav");
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

    // Read the file back in again
    let mut reader = Cursor::new(writer.get_ref().to_owned());
    let file2 = GvasFile::read(&mut reader).expect("Failed to parse serialized save file");

    // Compare the two Vec<u8>s
    assert_eq!(cursor.get_ref(), writer.get_ref());

    // Compare the two GvasFiles
    assert_eq!(file, file2);

    // Compare the file to expected values
    assert_eq!(
        file,
        GvasFile {
            header: GvasHeader {
                file_type_tag: 1396790855,
                save_game_file_version: 2,
                package_file_ue4_version: 518,
                engine_version: FEngineVersion {
                    major: 4,
                    minor: 25,
                    patch: 3,
                    change_list: 13942748,
                    branch: "++UE4+Release-4.25".into(),
                },
                custom_version_format: 3,
                custom_versions: vec![
                    FCustomVersion {
                        key: FromStr::from_str("ED0A3111-614D-552E-A39A-67AF2C08A1C5").unwrap(),
                        version: 17
                    },
                    FCustomVersion {
                        key: FromStr::from_str("F37ABB24-834F-4656-C22D-2F1FFF96AD49").unwrap(),
                        version: 5
                    },
                    FCustomVersion {
                        key: FromStr::from_str("2923A576-B545-2309-41D8-AE98D86A2FCF").unwrap(),
                        version: 2
                    },
                    FCustomVersion {
                        key: FromStr::from_str("0769BC5F-AE40-C855-84F1-678E3FF1FF5E").unwrap(),
                        version: 1
                    },
                    FCustomVersion {
                        key: FromStr::from_str("12E426FB-4D4B-151F-0A55-7293702F1D96").unwrap(),
                        version: 3
                    },
                    FCustomVersion {
                        key: FromStr::from_str("FA7AF5FC-8342-7650-58E6-A9B9322DA0FF").unwrap(),
                        version: 61
                    },
                    FCustomVersion {
                        key: FromStr::from_str("22D5549C-BE4F-26A8-4607-2194D082B461").unwrap(),
                        version: 30
                    },
                    FCustomVersion {
                        key: FromStr::from_str("E432D8B0-0D4F-891F-B77E-CFACA24AFD36").unwrap(),
                        version: 10
                    },
                    FCustomVersion {
                        key: FromStr::from_str("2843C6E1-534D-2CA2-868E-6CA38CBD1764").unwrap(),
                        version: 0
                    },
                    FCustomVersion {
                        key: FromStr::from_str("3CC15E37-FB48-E406-F084-00B57E712A26").unwrap(),
                        version: 4
                    },
                    FCustomVersion {
                        key: FromStr::from_str("ED68B0E4-E942-94F4-0BDA-31A241BB462E").unwrap(),
                        version: 38
                    },
                    FCustomVersion {
                        key: FromStr::from_str("3F74FCCF-8044-B043-DF14-919373201D17").unwrap(),
                        version: 37
                    },
                    FCustomVersion {
                        key: FromStr::from_str("B5492BB0-E944-20BB-B732-04A36003E452").unwrap(),
                        version: 2
                    },
                    FCustomVersion {
                        key: FromStr::from_str("5C10E4A4-B549-A159-C440-C5A7EEDF7E54").unwrap(),
                        version: 0
                    },
                    FCustomVersion {
                        key: FromStr::from_str("C931C839-DC47-E65A-179C-449A7C8E1C3E").unwrap(),
                        version: 0
                    },
                    FCustomVersion {
                        key: FromStr::from_str("331BF078-984F-EAEB-EA84-B4B9A25AB9CC").unwrap(),
                        version: 4
                    },
                    FCustomVersion {
                        key: FromStr::from_str("0F383166-E043-4D2D-27CF-09805AA95669").unwrap(),
                        version: 0
                    },
                    FCustomVersion {
                        key: FromStr::from_str("9F8BF812-FC4A-7588-0CD9-7CA629BD3A38").unwrap(),
                        version: 43
                    },
                    FCustomVersion {
                        key: FromStr::from_str("4CE75A7B-104C-70D2-9857-58A95A2A210B").unwrap(),
                        version: 12
                    },
                    FCustomVersion {
                        key: FromStr::from_str("186929D7-DD4B-D61D-A864-E29D8438C13C").unwrap(),
                        version: 3
                    },
                    FCustomVersion {
                        key: FromStr::from_str("7852A1C2-FE4A-E7BF-FF90-176C55F71D53").unwrap(),
                        version: 1
                    },
                    FCustomVersion {
                        key: FromStr::from_str("D4A3AC6E-C14C-EC40-ED8B-86B7C58F4209").unwrap(),
                        version: 3
                    },
                    FCustomVersion {
                        key: FromStr::from_str("DD75E529-2746-A3E0-76D2-109DEADC2C23").unwrap(),
                        version: 17
                    },
                    FCustomVersion {
                        key: FromStr::from_str("5DA643AF-4749-D37F-8E3E-739805BBC1D9").unwrap(),
                        version: 7
                    },
                    FCustomVersion {
                        key: FromStr::from_str("EC6C266B-8F4B-C71E-D9E4-0BA307FC4209").unwrap(),
                        version: 1
                    },
                    FCustomVersion {
                        key: FromStr::from_str("613DF70D-EA47-3FA2-E989-27B79A49410C").unwrap(),
                        version: 1
                    },
                    FCustomVersion {
                        key: FromStr::from_str("86181D60-844F-64AC-DED3-16AAD6C7EA0D").unwrap(),
                        version: 31
                    },
                    FCustomVersion {
                        key: FromStr::from_str("D6BCFF9D-5801-4F49-8212-21E288A8923C").unwrap(),
                        version: 10
                    },
                    FCustomVersion {
                        key: FromStr::from_str("ACD0AEF2-6F41-FE9A-7FAA-6486FCD626FA").unwrap(),
                        version: 1
                    },
                    FCustomVersion {
                        key: FromStr::from_str("0B1F4F17-A545-C6B4-E82E-3FB17D91FBD0").unwrap(),
                        version: 10
                    },
                    FCustomVersion {
                        key: FromStr::from_str("834AF935-6C40-58E2-F509-18A37C241096").unwrap(),
                        version: 37
                    },
                    FCustomVersion {
                        key: FromStr::from_str("6EC18FB6-E242-1B8B-5C21-53B4FE448805").unwrap(),
                        version: 1
                    },
                    FCustomVersion {
                        key: FromStr::from_str("0685E1B2-C2CF-7342-BBF4-4EA507BA8B75").unwrap(),
                        version: 1
                    },
                    FCustomVersion {
                        key: FromStr::from_str("50326854-AF48-9980-9698-C88BB7F9ADFB").unwrap(),
                        version: 0
                    },
                    FCustomVersion {
                        key: FromStr::from_str("194D0C43-7049-5471-699B-6987E5B090DF").unwrap(),
                        version: 14
                    },
                    FCustomVersion {
                        key: FromStr::from_str("BD32FEAA-144C-9553-255E-6AB6DDD13210").unwrap(),
                        version: 1
                    },
                    FCustomVersion {
                        key: FromStr::from_str("8EE1AF23-584E-E14C-52C2-618DB7BE53B9").unwrap(),
                        version: 11
                    },
                    FCustomVersion {
                        key: FromStr::from_str("EAB762A4-3A4E-99F4-1FEC-C199B2E12482").unwrap(),
                        version: 2
                    },
                    FCustomVersion {
                        key: FromStr::from_str("BDFDB52E-104D-AC01-8FF3-3681DAA59333").unwrap(),
                        version: 5
                    },
                    FCustomVersion {
                        key: FromStr::from_str("4F359D50-2F49-E6F6-B285-49A71C633C07").unwrap(),
                        version: 0
                    },
                    FCustomVersion {
                        key: FromStr::from_str("E79E7F71-3A49-B0E9-3291-B3880781381B").unwrap(),
                        version: 6
                    },
                    FCustomVersion {
                        key: FromStr::from_str("40EB564A-DC11-F510-7E34-D392E76AC9B2").unwrap(),
                        version: 2
                    },
                    FCustomVersion {
                        key: FromStr::from_str("004A8AD7-9746-58E8-B519-A8BAB4467D48").unwrap(),
                        version: 17
                    },
                    FCustomVersion {
                        key: FromStr::from_str("86F87955-1F4C-3A93-7B08-BA832FB96163").unwrap(),
                        version: 1
                    },
                    FCustomVersion {
                        key: FromStr::from_str("52BE2F61-0B40-53DA-914F-0D917C85B19F").unwrap(),
                        version: 1
                    },
                    FCustomVersion {
                        key: FromStr::from_str("367A23A4-C941-EACA-F818-A28FF31B6858").unwrap(),
                        version: 4
                    },
                    FCustomVersion {
                        key: FromStr::from_str("753F4E80-494B-8870-068C-D6A4DCB67E3C").unwrap(),
                        version: 5
                    },
                    FCustomVersion {
                        key: FromStr::from_str("F20A68FB-A34B-EF59-B519-A8BA3D44C873").unwrap(),
                        version: 2
                    },
                    FCustomVersion {
                        key: FromStr::from_str("0EB75099-174E-1AB4-0DFA-CCBBD67F8157").unwrap(),
                        version: 1
                    },
                    FCustomVersion {
                        key: FromStr::from_str("965196AB-FC08-D845-8D22-D7B79E56AD78").unwrap(),
                        version: 1
                    },
                ],
                save_game_class_name: "/Game/UI/BP_SaveOptions.BP_SaveOptions_C".into(),
            },
            properties: IndexMap::from([
                (
                    "Slider1".into(),
                    Property::from(FloatProperty::new(0.16610672)),
                ),
                (
                    "Slider2".into(),
                    Property::from(FloatProperty::new(0.28251615)),
                ),
            ]),
        },
    );
}
