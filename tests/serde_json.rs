mod common;
use common::REGRESSION_01_PATH;
use gvas::{
    game_version::GameVersion,
    properties::{
        array_property::ArrayProperty,
        delegate_property::{Delegate, DelegateProperty},
        enum_property::EnumProperty,
        field_path_property::{FieldPath, FieldPathProperty},
        int_property::{
            BoolProperty, ByteProperty, BytePropertyValue, DoubleProperty, FloatProperty,
            Int16Property, Int64Property, Int8Property, IntProperty, UInt16Property,
            UInt32Property, UInt64Property,
        },
        map_property::MapProperty,
        name_property::NameProperty,
        object_property::ObjectProperty,
        set_property::SetProperty,
        str_property::StrProperty,
        struct_property::{StructProperty, StructPropertyValue},
        struct_types::{DateTime, VectorF},
        text_property::{
            DateTimeStyle, FText, FTextHistory, FormatArgumentData, FormatArgumentValue,
            NumberFormattingOptions, RoundingMode, TextProperty, TransformType,
        },
        unknown_property::UnknownProperty,
        Property,
    },
    types::Guid,
    GvasFile,
};
use indexmap::{indexmap, IndexMap};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Debug,
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

fn serde_json<T>(value: &T, json: &str)
where
    T: Debug + for<'a> Deserialize<'a> + PartialEq + Serialize,
{
    assert_eq!(
        serde_json::to_string_pretty(value)
            .expect("serde_json::to_string")
            .as_str(),
        json
    );
    assert_eq!(
        &serde_json::from_str::<T>(json).expect("serde_json::from_str"),
        value
    );
}

#[test]
fn file_regression_01() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(REGRESSION_01_PATH);
    let mut file = File::open(path).expect("Failed to open test asset");

    // Read the file in to a Vec<u8>
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    // Convert the Vec<u8> to a GvasFile
    let mut cursor = Cursor::new(data);
    let file =
        GvasFile::read(&mut cursor, GameVersion::Default).expect("Failed to parse gvas file");

    // Compare the GvasFile to its expected JSON representation
    serde_json(
        &file,
        r#"{
  "header": {
    "type": "Version2",
    "package_file_version": 517,
    "engine_version": {
      "major": 4,
      "minor": 23,
      "patch": 1,
      "change_list": 9631420,
      "branch": "++UE4+Release-4.23"
    },
    "custom_version_format": 3,
    "custom_versions": {
      "22D5549C-BE4F-26A8-4607-2194D082B461": 23,
      "E432D8B0-0D4F-891F-B77E-CFACA24AFD36": 10,
      "2843C6E1-534D-2CA2-868E-6CA38CBD1764": 0,
      "3CC15E37-FB48-E406-F084-00B57E712A26": 3,
      "ED68B0E4-E942-94F4-0BDA-31A241BB462E": 34,
      "3F74FCCF-8044-B043-DF14-919373201D17": 35,
      "B5492BB0-E944-20BB-B732-04A36003E452": 2,
      "5C10E4A4-B549-A159-C440-C5A7EEDF7E54": 0,
      "C931C839-DC47-E65A-179C-449A7C8E1C3E": 0,
      "331BF078-984F-EAEB-EA84-B4B9A25AB9CC": 0,
      "0F383166-E043-4D2D-27CF-09805AA95669": 0,
      "9F8BF812-FC4A-7588-0CD9-7CA629BD3A38": 31,
      "4CE75A7B-104C-70D2-9857-58A95A2A210B": 11,
      "186929D7-DD4B-D61D-A864-E29D8438C13C": 2,
      "7852A1C2-FE4A-E7BF-FF90-176C55F71D53": 1,
      "D4A3AC6E-C14C-EC40-ED8B-86B7C58F4209": 3,
      "DD75E529-2746-A3E0-76D2-109DEADC2C23": 17,
      "5DA643AF-4749-D37F-8E3E-739805BBC1D9": 2,
      "EC6C266B-8F4B-C71E-D9E4-0BA307FC4209": 1,
      "613DF70D-EA47-3FA2-E989-27B79A49410C": 1,
      "86181D60-844F-64AC-DED3-16AAD6C7EA0D": 27,
      "D6BCFF9D-5801-4F49-8212-21E288A8923C": 6,
      "ACD0AEF2-6F41-FE9A-7FAA-6486FCD626FA": 1,
      "0B1F4F17-A545-C6B4-E82E-3FB17D91FBD0": 9,
      "E79E7F71-3A49-B0E9-3291-B3880781381B": 6,
      "B3DC7D8E-BB47-DA80-A246-D39FF64D9893": 1,
      "CDB08ACB-DE4B-8CE7-9313-62A862EFE914": 0,
      "F20A68FB-A34B-EF59-B519-A8BA3D44C873": 1,
      "9186E0AF-5249-0D3A-3B67-73B61E2DF27C": 2,
      "BDFDB52E-104D-AC01-8FF3-3681DAA59333": 5,
      "4F359D50-2F49-E6F6-B285-49A71C633C07": 0,
      "EAB762A4-3A4E-99F4-1FEC-C199B2E12482": 2,
      "194D0C43-7049-5471-699B-6987E5B090DF": 13,
      "BD32FEAA-144C-9553-255E-6AB6DDD13210": 1,
      "8EE1AF23-584E-E14C-52C2-618DB7BE53B9": 8,
      "40EB564A-DC11-F510-7E34-D392E76AC9B2": 2,
      "004A8AD7-9746-58E8-B519-A8BAB4467D48": 17,
      "86F87955-1F4C-3A93-7B08-BA832FB96163": 1,
      "52BE2F61-0B40-53DA-914F-0D917C85B19F": 1,
      "367A23A4-C941-EACA-F818-A28FF31B6858": 4,
      "753F4E80-494B-8870-068C-D6A4DCB67E3C": 5,
      "ED0A3111-614D-552E-A39A-67AF2C08A1C5": 17,
      "965196AB-FC08-D845-8D22-D7B79E56AD78": 1,
      "F37ABB24-834F-4656-C22D-2F1FFF96AD49": 4,
      "12E426FB-4D4B-151F-0A55-7293702F1D96": 3
    },
    "save_game_class_name": "/Script/SaveTest.TestSaveGame"
  },
  "properties": {
    "Thing": {
      "type": "StructProperty",
      "value": {
        "Guid": "D49982B3-DF3D-D549-B4AE-57C71D5838E4"
      }
    }
  }
}"#,
    );
}

#[test]
fn array_int8() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("Int8Property"),
                None,
                vec![
                    Property::Int8Property(Int8Property { value: 0 }),
                    Property::Int8Property(Int8Property { value: 1 }),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "property_type": "Int8Property",
  "properties": [
    {
      "type": "Int8Property",
      "value": 0
    },
    {
      "type": "Int8Property",
      "value": 1
    }
  ]
}"#,
    )
}

#[test]
fn array_int16() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("Int16Property"),
                None,
                vec![
                    Property::Int16Property(Int16Property { value: 0 }),
                    Property::Int16Property(Int16Property { value: 1 }),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "property_type": "Int16Property",
  "properties": [
    {
      "type": "Int16Property",
      "value": 0
    },
    {
      "type": "Int16Property",
      "value": 1
    }
  ]
}"#,
    )
}

#[test]
fn array_int32() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("IntProperty"),
                None,
                vec![
                    Property::IntProperty(IntProperty { value: 0 }),
                    Property::IntProperty(IntProperty { value: 1 }),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "property_type": "IntProperty",
  "properties": [
    {
      "type": "IntProperty",
      "value": 0
    },
    {
      "type": "IntProperty",
      "value": 1
    }
  ]
}"#,
    )
}

#[test]
fn array_int64() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("Int64Property"),
                None,
                vec![
                    Property::Int64Property(Int64Property { value: 0 }),
                    Property::Int64Property(Int64Property { value: 1 }),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "property_type": "Int64Property",
  "properties": [
    {
      "type": "Int64Property",
      "value": 0
    },
    {
      "type": "Int64Property",
      "value": 1
    }
  ]
}"#,
    )
}

#[test]
fn array_uint8() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("ByteProperty"),
                None,
                vec![
                    Property::ByteProperty(ByteProperty::new_byte(None, 0)),
                    Property::ByteProperty(ByteProperty::new_byte(None, 1)),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "bytes": [
    0,
    1
  ]
}"#,
    )
}

#[test]
fn array_uint16() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("UInt16Property"),
                None,
                vec![
                    Property::UInt16Property(UInt16Property { value: 0 }),
                    Property::UInt16Property(UInt16Property { value: 1 }),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "property_type": "UInt16Property",
  "properties": [
    {
      "type": "UInt16Property",
      "value": 0
    },
    {
      "type": "UInt16Property",
      "value": 1
    }
  ]
}"#,
    )
}

#[test]
fn array_uint32() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("UInt32Property"),
                None,
                vec![
                    Property::UInt32Property(UInt32Property { value: 0 }),
                    Property::UInt32Property(UInt32Property { value: 1 }),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "property_type": "UInt32Property",
  "properties": [
    {
      "type": "UInt32Property",
      "value": 0
    },
    {
      "type": "UInt32Property",
      "value": 1
    }
  ]
}"#,
    )
}

#[test]
fn array_uint64() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("UInt64Property"),
                None,
                vec![
                    Property::UInt64Property(UInt64Property { value: 0 }),
                    Property::UInt64Property(UInt64Property { value: 1 }),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "property_type": "UInt64Property",
  "properties": [
    {
      "type": "UInt64Property",
      "value": 0
    },
    {
      "type": "UInt64Property",
      "value": 1
    }
  ]
}"#,
    )
}

#[test]
fn array_bool() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("BoolProperty"),
                None,
                vec![
                    Property::BoolProperty(BoolProperty::new(false)),
                    Property::BoolProperty(BoolProperty::new(true)),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "property_type": "BoolProperty",
  "properties": [
    {
      "type": "BoolProperty",
      "value": false
    },
    {
      "type": "BoolProperty",
      "value": true
    }
  ]
}"#,
    )
}

#[test]
fn array_double() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("DoubleProperty"),
                None,
                vec![
                    Property::DoubleProperty(DoubleProperty::new(1f64)),
                    Property::DoubleProperty(DoubleProperty::new(2f64)),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "property_type": "DoubleProperty",
  "properties": [
    {
      "type": "DoubleProperty",
      "value": 1.0
    },
    {
      "type": "DoubleProperty",
      "value": 2.0
    }
  ]
}"#,
    )
}

#[test]
fn array_float() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("FloatProperty"),
                None,
                vec![
                    Property::FloatProperty(FloatProperty::new(1f32)),
                    Property::FloatProperty(FloatProperty::new(2f32)),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "property_type": "FloatProperty",
  "properties": [
    {
      "type": "FloatProperty",
      "value": 1.0
    },
    {
      "type": "FloatProperty",
      "value": 2.0
    }
  ]
}"#,
    )
}

#[test]
fn array_enum() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("EnumProperty"),
                None,
                vec![
                    Property::EnumProperty(EnumProperty::new(None, "a".to_string())),
                    Property::EnumProperty(EnumProperty::new(
                        Some("ns".to_string()),
                        "b".to_string(),
                    )),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "property_type": "EnumProperty",
  "properties": [
    {
      "type": "EnumProperty",
      "value": "a"
    },
    {
      "type": "EnumProperty",
      "enum_type": "ns",
      "value": "b"
    }
  ]
}"#,
    )
}

#[test]
fn array_name() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("NameProperty"),
                None,
                vec![
                    Property::NameProperty(NameProperty::from(None)),
                    Property::NameProperty(NameProperty::from("b")),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "property_type": "NameProperty",
  "properties": [
    {
      "type": "NameProperty"
    },
    {
      "type": "NameProperty",
      "value": "b"
    }
  ]
}"#,
    )
}

#[test]
fn array_object() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("ObjectProperty"),
                None,
                vec![
                    Property::ObjectProperty(ObjectProperty::from("a")),
                    Property::ObjectProperty(ObjectProperty::from("b")),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "property_type": "ObjectProperty",
  "properties": [
    {
      "type": "ObjectProperty",
      "value": "a"
    },
    {
      "type": "ObjectProperty",
      "value": "b"
    }
  ]
}"#,
    )
}

#[test]
fn array_str() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("StrProperty"),
                None,
                vec![
                    Property::StrProperty(StrProperty::new(None)),
                    Property::StrProperty(StrProperty::from("b")),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "property_type": "StrProperty",
  "properties": [
    {
      "type": "StrProperty"
    },
    {
      "type": "StrProperty",
      "value": "b"
    }
  ]
}"#,
    )
}

#[test]
fn array_map() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("MapProperty"),
                None,
                vec![
                    Property::MapProperty(MapProperty::new(
                        "kta".to_string(),
                        "vta".to_string(),
                        0,
                        indexmap! {},
                    )),
                    Property::MapProperty(MapProperty::new(
                        "ktb".to_string(),
                        "vtb".to_string(),
                        1,
                        indexmap! {},
                    )),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "property_type": "MapProperty",
  "properties": [
    {
      "type": "MapProperty",
      "key_type": "kta",
      "value_type": "vta",
      "allocation_flags": 0,
      "value": []
    },
    {
      "type": "MapProperty",
      "key_type": "ktb",
      "value_type": "vtb",
      "allocation_flags": 1,
      "value": []
    }
  ]
}"#,
    )
}

#[test]
fn array_struct() {
    serde_json(
        &Property::ArrayProperty(
            ArrayProperty::new(
                String::from("StructProperty"),
                Some((String::from("fn"), String::from("tn"), Guid([0x11u8; 16]))),
                vec![
                    Property::StructProperty(StructProperty::new(
                        Guid([0x22u8; 16]),
                        StructPropertyValue::DateTime(DateTime { ticks: 0 }),
                    )),
                    Property::StructProperty(StructProperty::new(
                        Guid::default(),
                        StructPropertyValue::DateTime(DateTime { ticks: 1 }),
                    )),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "field_name": "fn",
  "type_name": "tn",
  "guid": "11111111-1111-1111-1111-111111111111",
  "structs": [
    {
      "guid": "22222222-2222-2222-2222-222222222222",
      "value": {
        "DateTime": {
          "ticks": 0
        }
      }
    },
    {
      "value": {
        "DateTime": {
          "ticks": 1
        }
      }
    }
  ]
}"#,
    )
}

#[test]
fn delegate() {
    serde_json(
        &Property::DelegateProperty(DelegateProperty::new(Delegate::new(
            String::from("o"),
            String::from("fn"),
        ))),
        r#"{
  "type": "DelegateProperty",
  "value": {
    "object": "o",
    "function_name": "fn"
  }
}"#,
    )
}

#[test]
fn enum_property() {
    serde_json(
        &Property::EnumProperty(EnumProperty::new(
            Some(String::from("a")),
            String::from("b"),
        )),
        r#"{
  "type": "EnumProperty",
  "enum_type": "a",
  "value": "b"
}"#,
    );
    serde_json(
        &Property::EnumProperty(EnumProperty::new(None, String::from("a"))),
        r#"{
  "type": "EnumProperty",
  "value": "a"
}"#,
    );
}

#[test]
fn field_path() {
    serde_json(
        &Property::FieldPathProperty(FieldPathProperty::new(FieldPath::new(
            vec![String::from("a"), String::from("b")],
            String::from("owner"),
        ))),
        r#"{
  "type": "FieldPathProperty",
  "value": {
    "path": [
      "a",
      "b"
    ],
    "resolved_owner": "owner"
  }
}"#,
    )
}

#[test]
fn map_enum_bool() {
    serde_json(
        &Property::MapProperty(MapProperty::new(
            String::from("EnumProperty"),
            String::from("BoolProperty"),
            0,
            IndexMap::from([
                (
                    Property::from(EnumProperty::new(None, String::from("a"))),
                    Property::from(BoolProperty::new(false)),
                ),
                (
                    Property::from(EnumProperty::new(None, String::from("b"))),
                    Property::from(BoolProperty::new(true)),
                ),
            ]),
        )),
        r#"{
  "type": "MapProperty",
  "enum_bools": {
    "a": false,
    "b": true
  }
}"#,
    )
}

#[test]
fn map_enum_int() {
    serde_json(
        &Property::MapProperty(MapProperty::new(
            String::from("EnumProperty"),
            String::from("IntProperty"),
            0,
            IndexMap::from([
                (
                    Property::from(EnumProperty::new(None, String::from("a"))),
                    Property::from(IntProperty::new(0)),
                ),
                (
                    Property::from(EnumProperty::new(None, String::from("b"))),
                    Property::from(IntProperty::new(1)),
                ),
            ]),
        )),
        r#"{
  "type": "MapProperty",
  "enum_ints": {
    "a": 0,
    "b": 1
  }
}"#,
    )
}

#[test]
fn map_enum_unknown() {
    serde_json(
        &Property::MapProperty(MapProperty::new(
            String::from("EnumProperty"),
            String::from("UnknownProperty"),
            0,
            IndexMap::from([
                (
                    Property::from(EnumProperty::new(None, String::from("a"))),
                    Property::from(UnknownProperty::new(String::from("n"), vec![])),
                ),
                (
                    Property::from(EnumProperty::new(None, String::from("b"))),
                    Property::from(UnknownProperty::new(String::from("m"), vec![1])),
                ),
            ]),
        )),
        r#"{
  "type": "MapProperty",
  "value_type": "UnknownProperty",
  "enum_props": {
    "a": {
      "type": "UnknownProperty",
      "property_name": "n",
      "raw": []
    },
    "b": {
      "type": "UnknownProperty",
      "property_name": "m",
      "raw": [
        1
      ]
    }
  }
}"#,
    )
}

#[test]
fn map_int_bool() {
    serde_json(
        &Property::MapProperty(MapProperty::new(
            String::from("IntProperty"),
            String::from("BoolProperty"),
            0,
            IndexMap::from([
                (
                    Property::IntProperty(IntProperty::new(0)),
                    Property::BoolProperty(BoolProperty::new(false)),
                ),
                (
                    Property::IntProperty(IntProperty::new(1)),
                    Property::BoolProperty(BoolProperty::new(true)),
                ),
                (
                    Property::IntProperty(IntProperty::new(2)),
                    Property::BoolProperty(BoolProperty::new(false)),
                ),
            ]),
        )),
        r#"{
  "type": "MapProperty",
  "key_type": "IntProperty",
  "value_type": "BoolProperty",
  "allocation_flags": 0,
  "value": [
    [
      {
        "type": "IntProperty",
        "value": 0
      },
      {
        "type": "BoolProperty",
        "value": false
      }
    ],
    [
      {
        "type": "IntProperty",
        "value": 1
      },
      {
        "type": "BoolProperty",
        "value": true
      }
    ],
    [
      {
        "type": "IntProperty",
        "value": 2
      },
      {
        "type": "BoolProperty",
        "value": false
      }
    ]
  ]
}"#,
    )
}

#[test]
fn map_name_bool() {
    serde_json(
        &Property::MapProperty(MapProperty::new(
            String::from("NameProperty"),
            String::from("BoolProperty"),
            0,
            IndexMap::from([
                (
                    Property::NameProperty(NameProperty::from("a")),
                    Property::BoolProperty(BoolProperty::new(false)),
                ),
                (
                    Property::NameProperty(NameProperty::from("b")),
                    Property::BoolProperty(BoolProperty::new(true)),
                ),
            ]),
        )),
        r#"{
  "type": "MapProperty",
  "name_bools": {
    "a": false,
    "b": true
  }
}"#,
    );
}

#[test]
fn map_name_int() {
    serde_json(
        &Property::MapProperty(MapProperty::new(
            String::from("NameProperty"),
            String::from("IntProperty"),
            0,
            IndexMap::from([
                (
                    Property::NameProperty(NameProperty::from("a")),
                    Property::IntProperty(IntProperty::new(0)),
                ),
                (
                    Property::NameProperty(NameProperty::from("b")),
                    Property::IntProperty(IntProperty::new(1)),
                ),
            ]),
        )),
        r#"{
  "type": "MapProperty",
  "name_ints": {
    "a": 0,
    "b": 1
  }
}"#,
    );
}

#[test]
fn map_name_property() {
    serde_json(
        &Property::MapProperty(MapProperty::new(
            String::from("NameProperty"),
            String::from("UnknownProperty"),
            0,
            IndexMap::from([
                (
                    Property::NameProperty(NameProperty::from("a")),
                    Property::UnknownProperty(UnknownProperty::new(String::from("b"), vec![])),
                ),
                (
                    Property::NameProperty(NameProperty::from("c")),
                    Property::UnknownProperty(UnknownProperty::new(String::from("d"), vec![1])),
                ),
            ]),
        )),
        r#"{
  "type": "MapProperty",
  "value_type": "UnknownProperty",
  "name_props": {
    "a": {
      "type": "UnknownProperty",
      "property_name": "b",
      "raw": []
    },
    "c": {
      "type": "UnknownProperty",
      "property_name": "d",
      "raw": [
        1
      ]
    }
  }
}"#,
    );
}

#[test]
fn map_str_bool() {
    serde_json(
        &Property::MapProperty(MapProperty::new(
            String::from("StrProperty"),
            String::from("BoolProperty"),
            0,
            IndexMap::from([
                (
                    Property::StrProperty(StrProperty::from("a")),
                    Property::BoolProperty(BoolProperty::new(false)),
                ),
                (
                    Property::StrProperty(StrProperty::from("b")),
                    Property::BoolProperty(BoolProperty::new(true)),
                ),
            ]),
        )),
        r#"{
  "type": "MapProperty",
  "str_bools": {
    "a": false,
    "b": true
  }
}"#,
    );
}

#[test]
fn map_str_int() {
    serde_json(
        &Property::MapProperty(MapProperty::new(
            String::from("StrProperty"),
            String::from("IntProperty"),
            0,
            IndexMap::from([
                (
                    Property::StrProperty(StrProperty::from("zero")),
                    Property::IntProperty(IntProperty::new(0)),
                ),
                (
                    Property::StrProperty(StrProperty::from("one")),
                    Property::IntProperty(IntProperty::new(1)),
                ),
                (
                    Property::StrProperty(StrProperty::from("two")),
                    Property::IntProperty(IntProperty::new(2)),
                ),
            ]),
        )),
        r#"{
  "type": "MapProperty",
  "str_ints": {
    "zero": 0,
    "one": 1,
    "two": 2
  }
}"#,
    )
}

#[test]
fn map_str_property() {
    serde_json(
        &Property::MapProperty(MapProperty::new(
            String::from("StrProperty"),
            String::from("UnknownProperty"),
            0,
            IndexMap::from([
                (
                    Property::StrProperty(StrProperty::from("a")),
                    Property::UnknownProperty(UnknownProperty::new(String::from("b"), vec![])),
                ),
                (
                    Property::StrProperty(StrProperty::from("c")),
                    Property::UnknownProperty(UnknownProperty::new(String::from("d"), vec![1])),
                ),
            ]),
        )),
        r#"{
  "type": "MapProperty",
  "value_type": "UnknownProperty",
  "str_props": {
    "a": {
      "type": "UnknownProperty",
      "property_name": "b",
      "raw": []
    },
    "c": {
      "type": "UnknownProperty",
      "property_name": "d",
      "raw": [
        1
      ]
    }
  }
}"#,
    );
}

#[test]
fn map_str_str() {
    serde_json(
        &Property::MapProperty(MapProperty::new(
            String::from("StrProperty"),
            String::from("StrProperty"),
            0,
            IndexMap::from([
                (
                    Property::StrProperty(StrProperty::from("a")),
                    Property::StrProperty(StrProperty::from("b")),
                ),
                (
                    Property::StrProperty(StrProperty::from("c")),
                    Property::StrProperty(StrProperty::from("d")),
                ),
            ]),
        )),
        r#"{
  "type": "MapProperty",
  "str_strs": {
    "a": "b",
    "c": "d"
  }
}"#,
    );
}

#[test]
fn map_struct_float() {
    serde_json(
        &Property::MapProperty(MapProperty::new(
            String::from("StructProperty"),
            String::from("FloatProperty"),
            0,
            IndexMap::from([
                (
                    Property::StructProperty(StructProperty::new(
                        Guid::new([0u8; 16]),
                        StructPropertyValue::VectorF(VectorF::new(0f32, 1f32, 2f32)),
                    )),
                    Property::FloatProperty(FloatProperty::new(0f32)),
                ),
                (
                    Property::StructProperty(StructProperty::new(
                        Guid::new([0x11u8; 16]),
                        StructPropertyValue::Timespan(DateTime::new(0)),
                    )),
                    Property::FloatProperty(FloatProperty::new(1f32)),
                ),
                (
                    Property::StructProperty(StructProperty::new(
                        Guid::new([0x22u8; 16]),
                        StructPropertyValue::DateTime(DateTime::new(0)),
                    )),
                    Property::FloatProperty(FloatProperty::new(2f32)),
                ),
            ]),
        )),
        r#"{
  "type": "MapProperty",
  "key_type": "StructProperty",
  "value_type": "FloatProperty",
  "allocation_flags": 0,
  "value": [
    [
      {
        "type": "StructProperty",
        "value": {
          "VectorF": {
            "x": 0.0,
            "y": 1.0,
            "z": 2.0
          }
        }
      },
      {
        "type": "FloatProperty",
        "value": 0.0
      }
    ],
    [
      {
        "type": "StructProperty",
        "guid": "11111111-1111-1111-1111-111111111111",
        "value": {
          "Timespan": {
            "ticks": 0
          }
        }
      },
      {
        "type": "FloatProperty",
        "value": 1.0
      }
    ],
    [
      {
        "type": "StructProperty",
        "guid": "22222222-2222-2222-2222-222222222222",
        "value": {
          "DateTime": {
            "ticks": 0
          }
        }
      },
      {
        "type": "FloatProperty",
        "value": 2.0
      }
    ]
  ]
}"#,
    )
}

#[test]
fn name_array_index() {
    serde_json(
        &Property::NameProperty(NameProperty {
            array_index: 1,
            value: None,
        }),
        r#"{
  "type": "NameProperty",
  "array_index": 1
}"#,
    )
}

#[test]
fn name_none() {
    serde_json(
        &Property::NameProperty(NameProperty {
            array_index: 0,
            value: None,
        }),
        r#"{
  "type": "NameProperty"
}"#,
    )
}

#[test]
fn name_some() {
    serde_json(
        &Property::NameProperty(NameProperty::from("a")),
        r#"{
  "type": "NameProperty",
  "value": "a"
}"#,
    )
}

#[test]
fn float() {
    serde_json(
        &Property::FloatProperty(FloatProperty::new(0f32)),
        r#"{
  "type": "FloatProperty",
  "value": 0.0
}"#,
    )
}

#[test]
fn double() {
    serde_json(
        &Property::DoubleProperty(DoubleProperty::new(0f64)),
        r#"{
  "type": "DoubleProperty",
  "value": 0.0
}"#,
    )
}

#[test]
fn byte_none_byte() {
    serde_json(
        &Property::ByteProperty(ByteProperty::new(None, BytePropertyValue::Byte(0))),
        r#"{
  "type": "ByteProperty",
  "Byte": 0
}"#,
    )
}

#[test]
fn byte_some_ns() {
    serde_json(
        &Property::ByteProperty(ByteProperty::new_namespaced(
            Some(String::from("test name")),
            String::from("ns"),
        )),
        r#"{
  "type": "ByteProperty",
  "name": "test name",
  "Namespaced": "ns"
}"#,
    )
}

#[test]
fn int8() {
    serde_json(
        &Property::from(Int8Property::new(0i8)),
        r#"{
  "type": "Int8Property",
  "value": 0
}"#,
    )
}

#[test]
fn int16() {
    serde_json(
        &Property::Int16Property(Int16Property::new(0)),
        r#"{
  "type": "Int16Property",
  "value": 0
}"#,
    )
}

#[test]
fn uint16() {
    serde_json(
        &Property::UInt16Property(UInt16Property::new(0)),
        r#"{
  "type": "UInt16Property",
  "value": 0
}"#,
    )
}

#[test]
fn int() {
    serde_json(
        &Property::IntProperty(IntProperty::new(0)),
        r#"{
  "type": "IntProperty",
  "value": 0
}"#,
    )
}

#[test]
fn uint32() {
    serde_json(
        &Property::UInt32Property(UInt32Property::new(0)),
        r#"{
  "type": "UInt32Property",
  "value": 0
}"#,
    )
}

#[test]
fn int64() {
    serde_json(
        &Property::Int64Property(Int64Property::new(0)),
        r#"{
  "type": "Int64Property",
  "value": 0
}"#,
    )
}

#[test]
fn uint64() {
    serde_json(
        &Property::UInt64Property(UInt64Property::new(0)),
        r#"{
  "type": "UInt64Property",
  "value": 0
}"#,
    )
}

#[test]
fn object() {
    serde_json(
        &Property::ObjectProperty(ObjectProperty::from("a")),
        r#"{
  "type": "ObjectProperty",
  "value": "a"
}"#,
    )
}

#[test]
fn set_int() {
    serde_json(
        &Property::SetProperty(SetProperty::new(
            String::from("IntProperty"),
            0,
            vec![
                Property::IntProperty(IntProperty { value: 0 }),
                Property::IntProperty(IntProperty { value: 1 }),
            ],
        )),
        r#"{
  "type": "SetProperty",
  "property_type": "IntProperty",
  "allocation_flags": 0,
  "properties": [
    {
      "type": "IntProperty",
      "value": 0
    },
    {
      "type": "IntProperty",
      "value": 1
    }
  ]
}"#,
    )
}

#[test]
fn str_none() {
    serde_json(
        &Property::StrProperty(StrProperty { value: None }),
        r#"{
  "type": "StrProperty"
}"#,
    )
}

#[test]
fn str_some() {
    serde_json(
        &Property::StrProperty(StrProperty::from("a")),
        r#"{
  "type": "StrProperty",
  "value": "a"
}"#,
    )
}

#[test]
fn struct_vectorf() {
    serde_json(
        &Property::StructProperty(StructProperty::new(
            Guid::new([0u8; 16]),
            StructPropertyValue::VectorF(VectorF::new(0f32, 1f32, 2f32)),
        )),
        r#"{
  "type": "StructProperty",
  "value": {
    "VectorF": {
      "x": 0.0,
      "y": 1.0,
      "z": 2.0
    }
  }
}"#,
    )
}

#[test]
fn text_none_none() {
    serde_json(
        &Property::TextProperty(TextProperty::new(FText::new_none(0, None))),
        r#"{
  "type": "TextProperty",
  "flags": 0,
  "history": "None"
}"#,
    );
}

#[ignore] // This test fails
#[test]
fn text_none_some_none() {
    // serde_json(
    //     &Property::TextProperty(TextProperty::new(FText::new_none(1, Some(None)))),
    //     "{\n  \"type\": \"TextProperty\",\n  \"flags\": 1,\n  \"history\": \"None\"\n}",
    // );
}

#[test]
fn text_none_some_some() {
    serde_json(
        &Property::TextProperty(TextProperty::new(FText::new_none(
            2,
            Some(Some(String::from("a"))),
        ))),
        r#"{
  "type": "TextProperty",
  "flags": 2,
  "history": "None",
  "culture_invariant_string": "a"
}"#,
    );
}

#[test]
fn text_base_none() {
    serde_json(
        &Property::TextProperty(TextProperty::new(FText::new_base(0, None, None, None))),
        r#"{
  "type": "TextProperty",
  "flags": 0,
  "history": "Base"
}"#,
    );
}

#[test]
fn text_base_filled() {
    serde_json(
        &Property::TextProperty(TextProperty::new(FText::new_base(
            1,
            Some(String::from("ns")),
            Some(String::from("k")),
            Some(String::from("ss")),
        ))),
        r#"{
  "type": "TextProperty",
  "flags": 1,
  "history": "Base",
  "namespace": "ns",
  "key": "k",
  "source_string": "ss"
}"#,
    );
}

#[test]
fn text_namedformat() {
    serde_json(
        &Property::TextProperty(TextProperty::new(FText {
            flags: 0,
            history: FTextHistory::NamedFormat {
                source_format: Box::new(FText {
                    flags: 1,
                    history: FTextHistory::None {
                        culture_invariant_string: None,
                    },
                }),
                arguments: IndexMap::from([(String::from("key"), FormatArgumentValue::Int(2))]),
            },
        })),
        r#"{
  "type": "TextProperty",
  "flags": 0,
  "history": "NamedFormat",
  "source_format": {
    "flags": 1,
    "history": "None"
  },
  "arguments": {
    "key": {
      "Int": 2
    }
  }
}"#,
    );
}

#[test]
fn text_orderedformat() {
    serde_json(
        &Property::TextProperty(TextProperty::new(FText {
            flags: 0,
            history: FTextHistory::OrderedFormat {
                source_format: Box::new(FText {
                    flags: 1,
                    history: FTextHistory::None {
                        culture_invariant_string: None,
                    },
                }),
                arguments: vec![FormatArgumentValue::UInt(2)],
            },
        })),
        r#"{
  "type": "TextProperty",
  "flags": 0,
  "history": "OrderedFormat",
  "source_format": {
    "flags": 1,
    "history": "None"
  },
  "arguments": [
    {
      "UInt": 2
    }
  ]
}"#,
    );
}

#[test]
fn text_argumentformat() {
    serde_json(
        &Property::TextProperty(TextProperty::new(FText {
            flags: 0,
            history: FTextHistory::ArgumentFormat {
                source_format: Box::new(FText {
                    flags: 1,
                    history: FTextHistory::None {
                        culture_invariant_string: None,
                    },
                }),
                arguments: vec![FormatArgumentData {
                    name: String::from("key"),
                    value: FormatArgumentValue::UInt(2),
                }],
            },
        })),
        r#"{
  "type": "TextProperty",
  "flags": 0,
  "history": "ArgumentFormat",
  "source_format": {
    "flags": 1,
    "history": "None"
  },
  "arguments": [
    {
      "name": "key",
      "value": {
        "UInt": 2
      }
    }
  ]
}"#,
    );
}

#[test]
fn text_asnumber() {
    serde_json(
        &Property::TextProperty(TextProperty::new(FText {
            flags: 0,
            history: FTextHistory::AsNumber {
                source_value: Box::new(FormatArgumentValue::Text(FText {
                    flags: 1,
                    history: FTextHistory::None {
                        culture_invariant_string: None,
                    },
                })),
                format_options: Some(NumberFormattingOptions {
                    always_include_sign: true,
                    use_grouping: true,
                    rounding_mode: RoundingMode::ToZero,
                    minimum_integral_digits: 2,
                    maximum_integral_digits: 3,
                    minimum_fractional_digits: 4,
                    maximum_fractional_digits: 5,
                }),
                target_culture: Some(String::from("culture")),
            },
        })),
        r#"{
  "type": "TextProperty",
  "flags": 0,
  "history": "AsNumber",
  "source_value": {
    "Text": {
      "flags": 1,
      "history": "None"
    }
  },
  "format_options": {
    "always_include_sign": true,
    "use_grouping": true,
    "rounding": "ToZero",
    "minimum_integral_digits": 2,
    "maximum_integral_digits": 3,
    "minimum_fractional_digits": 4,
    "maximum_fractional_digits": 5
  },
  "target_culture": "culture"
}"#,
    );
}

#[test]
fn text_ascurrency() {
    serde_json(
        &Property::TextProperty(TextProperty::new(FText {
            flags: 0,
            history: FTextHistory::AsNumber {
                source_value: Box::new(FormatArgumentValue::Text(FText {
                    flags: 1,
                    history: FTextHistory::None {
                        culture_invariant_string: None,
                    },
                })),
                format_options: Some(NumberFormattingOptions {
                    always_include_sign: true,
                    use_grouping: true,
                    rounding_mode: RoundingMode::ToZero,
                    minimum_integral_digits: 2,
                    maximum_integral_digits: 3,
                    minimum_fractional_digits: 4,
                    maximum_fractional_digits: 5,
                }),
                target_culture: Some(String::from("culture")),
            },
        })),
        r#"{
  "type": "TextProperty",
  "flags": 0,
  "history": "AsNumber",
  "source_value": {
    "Text": {
      "flags": 1,
      "history": "None"
    }
  },
  "format_options": {
    "always_include_sign": true,
    "use_grouping": true,
    "rounding": "ToZero",
    "minimum_integral_digits": 2,
    "maximum_integral_digits": 3,
    "minimum_fractional_digits": 4,
    "maximum_fractional_digits": 5
  },
  "target_culture": "culture"
}"#,
    );
}

#[test]
fn text_asdate() {
    serde_json(
        &Property::TextProperty(TextProperty::new(FText {
            flags: 0,
            history: FTextHistory::AsDate {
                date_time: DateTime { ticks: 1 },
                date_style: DateTimeStyle::Default,
                target_culture: String::from("culture"),
            },
        })),
        r#"{
  "type": "TextProperty",
  "flags": 0,
  "history": "AsDate",
  "date_time": {
    "ticks": 1
  },
  "date_style": "Default",
  "target_culture": "culture"
}"#,
    );
}

#[test]
fn text_astime() {
    serde_json(
        &Property::TextProperty(TextProperty::new(FText {
            flags: 0,
            history: FTextHistory::AsTime {
                source_date_time: DateTime { ticks: 1 },
                time_style: DateTimeStyle::Default,
                time_zone: String::from("zone"),
                target_culture: String::from("culture"),
            },
        })),
        r#"{
  "type": "TextProperty",
  "flags": 0,
  "history": "AsTime",
  "source_date_time": {
    "ticks": 1
  },
  "time_style": "Default",
  "time_zone": "zone",
  "target_culture": "culture"
}"#,
    );
}

#[test]
fn text_asdatetime() {
    serde_json(
        &Property::TextProperty(TextProperty::new(FText {
            flags: 0,
            history: FTextHistory::AsDateTime {
                source_date_time: DateTime { ticks: 1 },
                date_style: DateTimeStyle::Default,
                time_style: DateTimeStyle::Default,
                time_zone: String::from("zone"),
                target_culture: String::from("culture"),
            },
        })),
        r#"{
  "type": "TextProperty",
  "flags": 0,
  "history": "AsDateTime",
  "source_date_time": {
    "ticks": 1
  },
  "date_style": "Default",
  "time_style": "Default",
  "time_zone": "zone",
  "target_culture": "culture"
}"#,
    );
}

#[test]
fn text_transform() {
    serde_json(
        &Property::TextProperty(TextProperty::new(FText {
            flags: 0,
            history: FTextHistory::Transform {
                source_text: Box::new(FText {
                    flags: 1,
                    history: FTextHistory::None {
                        culture_invariant_string: None,
                    },
                }),
                transform_type: TransformType::ToLower,
            },
        })),
        r#"{
  "type": "TextProperty",
  "flags": 0,
  "history": "Transform",
  "source_text": {
    "flags": 1,
    "history": "None"
  },
  "transform": "ToLower"
}"#,
    );
}

#[test]
fn unknown() {
    serde_json(
        &Property::UnknownProperty(UnknownProperty::new(String::from("name"), vec![0, 1, 2])),
        r#"{
  "type": "UnknownProperty",
  "property_name": "name",
  "raw": [
    0,
    1,
    2
  ]
}"#,
    )
}
