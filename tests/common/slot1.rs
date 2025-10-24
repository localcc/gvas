use gvas::{
    GvasFile, GvasHeader,
    engine_version::FEngineVersion,
    game_version::DeserializedGameVersion,
    properties::{
        Property,
        array_property::ArrayProperty,
        int_property::{
            ByteProperty, BytePropertyValue, DoubleProperty, FloatProperty, Int8Property,
            Int16Property, Int64Property, IntProperty, UInt16Property, UInt32Property,
            UInt64Property,
        },
        str_property::StrProperty,
        struct_property::{StructProperty, StructPropertyValue},
        struct_types::DateTime,
    },
    types::{Guid, map::HashableIndexMap},
};
use std::str::FromStr;

#[allow(clippy::approx_constant)]
pub(crate) fn expected() -> GvasFile {
    GvasFile {
        deserialized_game_version: DeserializedGameVersion::Default,
        header: GvasHeader::Version2 {
            package_file_version: 522,
            engine_version: FEngineVersion {
                major: 4,
                minor: 27,
                patch: 2,
                change_list: 18319896,
                branch: String::from("++UE4+Release-4.27"),
            },
            custom_version_format: 3,
            custom_versions: HashableIndexMap::from([
                (
                    Guid::from_str("22D5549C-BE4F-26A8-4607-2194D082B461").unwrap(),
                    43,
                ),
                (
                    Guid::from_str("E432D8B0-0D4F-891F-B77E-CFACA24AFD36").unwrap(),
                    10,
                ),
                (
                    Guid::from_str("2843C6E1-534D-2CA2-868E-6CA38CBD1764").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("3CC15E37-FB48-E406-F084-00B57E712A26").unwrap(),
                    4,
                ),
                (
                    Guid::from_str("ED68B0E4-E942-94F4-0BDA-31A241BB462E").unwrap(),
                    40,
                ),
                (
                    Guid::from_str("3F74FCCF-8044-B043-DF14-919373201D17").unwrap(),
                    37,
                ),
                (
                    Guid::from_str("B5492BB0-E944-20BB-B732-04A36003E452").unwrap(),
                    3,
                ),
                (
                    Guid::from_str("5C10E4A4-B549-A159-C440-C5A7EEDF7E54").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("C931C839-DC47-E65A-179C-449A7C8E1C3E").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("331BF078-984F-EAEB-EA84-B4B9A25AB9CC").unwrap(),
                    14,
                ),
                (
                    Guid::from_str("0F383166-E043-4D2D-27CF-09805AA95669").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("9F8BF812-FC4A-7588-0CD9-7CA629BD3A38").unwrap(),
                    45,
                ),
                (
                    Guid::from_str("4CE75A7B-104C-70D2-9857-58A95A2A210B").unwrap(),
                    13,
                ),
                (
                    Guid::from_str("186929D7-DD4B-D61D-A864-E29D8438C13C").unwrap(),
                    3,
                ),
                (
                    Guid::from_str("7852A1C2-FE4A-E7BF-FF90-176C55F71D53").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("D4A3AC6E-C14C-EC40-ED8B-86B7C58F4209").unwrap(),
                    3,
                ),
                (
                    Guid::from_str("DD75E529-2746-A3E0-76D2-109DEADC2C23").unwrap(),
                    17,
                ),
                (
                    Guid::from_str("5DA643AF-4749-D37F-8E3E-739805BBC1D9").unwrap(),
                    15,
                ),
                (
                    Guid::from_str("EC6C266B-8F4B-C71E-D9E4-0BA307FC4209").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("613DF70D-EA47-3FA2-E989-27B79A49410C").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("86181D60-844F-64AC-DED3-16AAD6C7EA0D").unwrap(),
                    47,
                ),
                (
                    Guid::from_str("686308E7-584C-236B-701B-3984915E2616").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("D6BCFF9D-5801-4F49-8212-21E288A8923C").unwrap(),
                    10,
                ),
                (
                    Guid::from_str("ACD0AEF2-6F41-FE9A-7FAA-6486FCD626FA").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("0B1F4F17-A545-C6B4-E82E-3FB17D91FBD0").unwrap(),
                    10,
                ),
                (
                    Guid::from_str("834AF935-6C40-58E2-F509-18A37C241096").unwrap(),
                    41,
                ),
                (
                    Guid::from_str("6EC18FB6-E242-1B8B-5C21-53B4FE448805").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("0685E1B2-C2CF-7342-BBF4-4EA507BA8B75").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("3689F564-BA42-1BFD-8972-96BA4EFAD0D5").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("27D80E6F-9548-09A6-8D99-919CA40E1890").unwrap(),
                    2,
                ),
                (
                    Guid::from_str("E79E7F71-3A49-B0E9-3291-B3880781381B").unwrap(),
                    8,
                ),
                (
                    Guid::from_str("50326854-AF48-9980-9698-C88BB7F9ADFB").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("B3DC7D8E-BB47-DA80-A246-D39FF64D9893").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("CDB08ACB-DE4B-8CE7-9313-62A862EFE914").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("965196AB-FC08-D845-8D22-D7B79E56AD78").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("0EB75099-174E-1AB4-0DFA-CCBBD67F8157").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("F20A68FB-A34B-EF59-B519-A8BA3D44C873").unwrap(),
                    2,
                ),
                (
                    Guid::from_str("9186E0AF-5249-0D3A-3B67-73B61E2DF27C").unwrap(),
                    2,
                ),
                (
                    Guid::from_str("BDFDB52E-104D-AC01-8FF3-3681DAA59333").unwrap(),
                    5,
                ),
                (
                    Guid::from_str("4F359D50-2F49-E6F6-B285-49A71C633C07").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("EAB762A4-3A4E-99F4-1FEC-C199B2E12482").unwrap(),
                    4,
                ),
                (
                    Guid::from_str("194D0C43-7049-5471-699B-6987E5B090DF").unwrap(),
                    15,
                ),
                (
                    Guid::from_str("BD32FEAA-144C-9553-255E-6AB6DDD13210").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("8EE1AF23-584E-E14C-52C2-618DB7BE53B9").unwrap(),
                    11,
                ),
                (
                    Guid::from_str("40EB564A-DC11-F510-7E34-D392E76AC9B2").unwrap(),
                    2,
                ),
                (
                    Guid::from_str("004A8AD7-9746-58E8-B519-A8BAB4467D48").unwrap(),
                    18,
                ),
                (
                    Guid::from_str("86F87955-1F4C-3A93-7B08-BA832FB96163").unwrap(),
                    2,
                ),
                (
                    Guid::from_str("52BE2F61-0B40-53DA-914F-0D917C85B19F").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("367A23A4-C941-EACA-F818-A28FF31B6858").unwrap(),
                    4,
                ),
                (
                    Guid::from_str("753F4E80-494B-8870-068C-D6A4DCB67E3C").unwrap(),
                    5,
                ),
                (
                    Guid::from_str("2923A576-B545-2309-41D8-AE98D86A2FCF").unwrap(),
                    5,
                ),
                (
                    Guid::from_str("0769BC5F-AE40-C855-84F1-678E3FF1FF5E").unwrap(),
                    1,
                ),
                (
                    Guid::from_str("FA7AF5FC-8342-7650-58E6-A9B9322DA0FF").unwrap(),
                    68,
                ),
                (
                    Guid::from_str("F37ABB24-834F-4656-C22D-2F1FFF96AD49").unwrap(),
                    5,
                ),
                (
                    Guid::from_str("ED0A3111-614D-552E-A39A-67AF2C08A1C5").unwrap(),
                    17,
                ),
                (
                    Guid::from_str("4E7CE782-A543-2333-C513-6BB4F30D3197").unwrap(),
                    0,
                ),
                (
                    Guid::from_str("12E426FB-4D4B-151F-0A55-7293702F1D96").unwrap(),
                    3,
                ),
            ]),
            save_game_class_name: String::from("/Script/UE4SaveFile.TestSaveGame"),
        },
        properties: HashableIndexMap::from([
            (
                String::from("u8_test"),
                Property::from(ByteProperty {
                    name: Some(String::from("None")),
                    value: BytePropertyValue::Byte(129),
                }),
            ),
            (
                String::from("i8_test"),
                Property::from(Int8Property::new(-123i8)),
            ),
            (
                String::from("ushort_test"),
                Property::from(UInt16Property::new(65530u16)),
            ),
            (
                String::from("short_test"),
                Property::from(Int16Property::new(-32764i16)),
            ),
            (
                String::from("uint32_test"),
                Property::from(UInt32Property::new(4294967294u32)),
            ),
            (
                String::from("int32_test"),
                Property::from(IntProperty::new(-2147483647i32)),
            ),
            (
                String::from("ulong_test"),
                Property::from(UInt64Property::new(18446744073709551614u64)),
            ),
            (
                String::from("long_test"),
                Property::from(Int64Property::new(-9223372036854775807i64)),
            ),
            (
                String::from("f_property"),
                Property::from(FloatProperty::new(3.14159f32)),
            ),
            (
                String::from("d_property"),
                Property::from(DoubleProperty::new(3.14159265358979f64)),
            ),
            (
                String::from("str_property"),
                Property::from(StrProperty::from("Hello world")),
            ),
            (
                String::from("struct_property"),
                Property::from(StructProperty {
                    type_name: String::from("CustomStruct"),
                    guid: Guid::default(),
                    value: StructPropertyValue::CustomStruct(HashableIndexMap::from([(
                        String::from("test_field"),
                        vec![Property::from(UInt64Property::new(12345u64))],
                    )])),
                }),
            ),
            (
                String::from("date_time_property"),
                Property::from(StructProperty {
                    type_name: String::from("DateTime"),
                    guid: Guid::default(),
                    value: StructPropertyValue::from(DateTime {
                        ticks: 637864237380020000,
                    }),
                }),
            ),
            (
                String::from("array_of_structs"),
                Property::from(ArrayProperty::Structs {
                    field_name: String::from("array_of_structs"),
                    type_name: String::from("CustomStruct"),
                    guid: Guid::default(),
                    structs: vec![
                        StructPropertyValue::CustomStruct(HashableIndexMap::from([(
                            String::from("test_field"),
                            vec![Property::from(UInt64Property::new(10u64))],
                        )])),
                        StructPropertyValue::CustomStruct(HashableIndexMap::from([(
                            String::from("test_field"),
                            vec![Property::from(UInt64Property::new(10u64))],
                        )])),
                    ],
                }),
            ),
            (
                String::from("array_of_ints"),
                Property::from(ArrayProperty::Ints {
                    ints: vec![12, 12, 12, 12, 12],
                }),
            ),
            (
                String::from("array_of_strings"),
                Property::from(ArrayProperty::Strings {
                    strings: vec![
                        Some(String::from("Hello world from array")),
                        Some(String::from("Hello world from array")),
                        Some(String::from("Hello world from array")),
                    ],
                }),
            ),
        ]),
    }
}

pub const SLOT1_JSON: &str = r#"{
  "header": {
    "type": "Version2",
    "package_file_version": 522,
    "engine_version": {
      "major": 4,
      "minor": 27,
      "patch": 2,
      "change_list": 18319896,
      "branch": "++UE4+Release-4.27"
    },
    "custom_version_format": 3,
    "custom_versions": {
      "22D5549C-BE4F-26A8-4607-2194D082B461": 43,
      "E432D8B0-0D4F-891F-B77E-CFACA24AFD36": 10,
      "2843C6E1-534D-2CA2-868E-6CA38CBD1764": 0,
      "3CC15E37-FB48-E406-F084-00B57E712A26": 4,
      "ED68B0E4-E942-94F4-0BDA-31A241BB462E": 40,
      "3F74FCCF-8044-B043-DF14-919373201D17": 37,
      "B5492BB0-E944-20BB-B732-04A36003E452": 3,
      "5C10E4A4-B549-A159-C440-C5A7EEDF7E54": 0,
      "C931C839-DC47-E65A-179C-449A7C8E1C3E": 0,
      "331BF078-984F-EAEB-EA84-B4B9A25AB9CC": 14,
      "0F383166-E043-4D2D-27CF-09805AA95669": 0,
      "9F8BF812-FC4A-7588-0CD9-7CA629BD3A38": 45,
      "4CE75A7B-104C-70D2-9857-58A95A2A210B": 13,
      "186929D7-DD4B-D61D-A864-E29D8438C13C": 3,
      "7852A1C2-FE4A-E7BF-FF90-176C55F71D53": 1,
      "D4A3AC6E-C14C-EC40-ED8B-86B7C58F4209": 3,
      "DD75E529-2746-A3E0-76D2-109DEADC2C23": 17,
      "5DA643AF-4749-D37F-8E3E-739805BBC1D9": 15,
      "EC6C266B-8F4B-C71E-D9E4-0BA307FC4209": 1,
      "613DF70D-EA47-3FA2-E989-27B79A49410C": 1,
      "86181D60-844F-64AC-DED3-16AAD6C7EA0D": 47,
      "686308E7-584C-236B-701B-3984915E2616": 1,
      "D6BCFF9D-5801-4F49-8212-21E288A8923C": 10,
      "ACD0AEF2-6F41-FE9A-7FAA-6486FCD626FA": 1,
      "0B1F4F17-A545-C6B4-E82E-3FB17D91FBD0": 10,
      "834AF935-6C40-58E2-F509-18A37C241096": 41,
      "6EC18FB6-E242-1B8B-5C21-53B4FE448805": 1,
      "0685E1B2-C2CF-7342-BBF4-4EA507BA8B75": 1,
      "3689F564-BA42-1BFD-8972-96BA4EFAD0D5": 1,
      "27D80E6F-9548-09A6-8D99-919CA40E1890": 2,
      "E79E7F71-3A49-B0E9-3291-B3880781381B": 8,
      "50326854-AF48-9980-9698-C88BB7F9ADFB": 0,
      "B3DC7D8E-BB47-DA80-A246-D39FF64D9893": 1,
      "CDB08ACB-DE4B-8CE7-9313-62A862EFE914": 0,
      "965196AB-FC08-D845-8D22-D7B79E56AD78": 1,
      "0EB75099-174E-1AB4-0DFA-CCBBD67F8157": 1,
      "F20A68FB-A34B-EF59-B519-A8BA3D44C873": 2,
      "9186E0AF-5249-0D3A-3B67-73B61E2DF27C": 2,
      "BDFDB52E-104D-AC01-8FF3-3681DAA59333": 5,
      "4F359D50-2F49-E6F6-B285-49A71C633C07": 0,
      "EAB762A4-3A4E-99F4-1FEC-C199B2E12482": 4,
      "194D0C43-7049-5471-699B-6987E5B090DF": 15,
      "BD32FEAA-144C-9553-255E-6AB6DDD13210": 1,
      "8EE1AF23-584E-E14C-52C2-618DB7BE53B9": 11,
      "40EB564A-DC11-F510-7E34-D392E76AC9B2": 2,
      "004A8AD7-9746-58E8-B519-A8BAB4467D48": 18,
      "86F87955-1F4C-3A93-7B08-BA832FB96163": 2,
      "52BE2F61-0B40-53DA-914F-0D917C85B19F": 1,
      "367A23A4-C941-EACA-F818-A28FF31B6858": 4,
      "753F4E80-494B-8870-068C-D6A4DCB67E3C": 5,
      "2923A576-B545-2309-41D8-AE98D86A2FCF": 5,
      "0769BC5F-AE40-C855-84F1-678E3FF1FF5E": 1,
      "FA7AF5FC-8342-7650-58E6-A9B9322DA0FF": 68,
      "F37ABB24-834F-4656-C22D-2F1FFF96AD49": 5,
      "ED0A3111-614D-552E-A39A-67AF2C08A1C5": 17,
      "4E7CE782-A543-2333-C513-6BB4F30D3197": 0,
      "12E426FB-4D4B-151F-0A55-7293702F1D96": 3
    },
    "save_game_class_name": "/Script/UE4SaveFile.TestSaveGame"
  },
  "properties": {
    "u8_test": {
      "type": "ByteProperty",
      "name": "None",
      "Byte": 129
    },
    "i8_test": {
      "type": "Int8Property",
      "value": -123
    },
    "ushort_test": {
      "type": "UInt16Property",
      "value": 65530
    },
    "short_test": {
      "type": "Int16Property",
      "value": -32764
    },
    "uint32_test": {
      "type": "UInt32Property",
      "value": 4294967294
    },
    "int32_test": {
      "type": "IntProperty",
      "value": -2147483647
    },
    "ulong_test": {
      "type": "UInt64Property",
      "value": 18446744073709551614
    },
    "long_test": {
      "type": "Int64Property",
      "value": -9223372036854775807
    },
    "f_property": {
      "type": "FloatProperty",
      "value": 3.14159
    },
    "d_property": {
      "type": "DoubleProperty",
      "value": 3.14159265358979
    },
    "str_property": {
      "type": "StrProperty",
      "value": "Hello world"
    },
    "struct_property": {
      "type": "StructProperty",
      "type_name": "CustomStruct",
      "CustomStruct": {
        "test_field": [
          {
            "type": "UInt64Property",
            "value": 12345
          }
        ]
      }
    },
    "date_time_property": {
      "type": "StructProperty",
      "type_name": "DateTime",
      "DateTime": {
        "ticks": 637864237380020000
      }
    },
    "array_of_structs": {
      "type": "ArrayProperty",
      "field_name": "array_of_structs",
      "type_name": "CustomStruct",
      "structs": [
        {
          "CustomStruct": {
            "test_field": [
              {
                "type": "UInt64Property",
                "value": 10
              }
            ]
          }
        },
        {
          "CustomStruct": {
            "test_field": [
              {
                "type": "UInt64Property",
                "value": 10
              }
            ]
          }
        }
      ]
    },
    "array_of_ints": {
      "type": "ArrayProperty",
      "ints": [
        12,
        12,
        12,
        12,
        12
      ]
    },
    "array_of_strings": {
      "type": "ArrayProperty",
      "strings": [
        "Hello world from array",
        "Hello world from array",
        "Hello world from array"
      ]
    }
  }
}"#;
