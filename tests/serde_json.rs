use gvas::{
    game_version::GameVersion,
    properties::{
        array_property::ArrayProperty,
        delegate_property::{Delegate, DelegateProperty},
        enum_property::EnumProperty,
        field_path_property::{FieldPath, FieldPathProperty},
        int_property::{
            DoubleProperty, FloatProperty, Int16Property, Int64Property, IntProperty,
            UInt16Property, UInt32Property, UInt64Property,
        },
        map_property::MapProperty,
        name_property::NameProperty,
        object_property::ObjectProperty,
        set_property::SetProperty,
        str_property::StrProperty,
        struct_property::{StructProperty, StructPropertyValue},
        struct_types::{DateTime, VectorF},
        text_property::{FText, TextProperty},
        unknown_property::UnknownProperty,
        Property,
    },
    types::Guid,
    GvasFile,
};
use indexmap::IndexMap;
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
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/regression_01.bin");
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
  "deserialized_game_version": "Default",
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
    "custom_versions": [
      {
        "key": "22D5549C-BE4F-26A8-4607-2194D082B461",
        "version": 23
      },
      {
        "key": "E432D8B0-0D4F-891F-B77E-CFACA24AFD36",
        "version": 10
      },
      {
        "key": "2843C6E1-534D-2CA2-868E-6CA38CBD1764",
        "version": 0
      },
      {
        "key": "3CC15E37-FB48-E406-F084-00B57E712A26",
        "version": 3
      },
      {
        "key": "ED68B0E4-E942-94F4-0BDA-31A241BB462E",
        "version": 34
      },
      {
        "key": "3F74FCCF-8044-B043-DF14-919373201D17",
        "version": 35
      },
      {
        "key": "B5492BB0-E944-20BB-B732-04A36003E452",
        "version": 2
      },
      {
        "key": "5C10E4A4-B549-A159-C440-C5A7EEDF7E54",
        "version": 0
      },
      {
        "key": "C931C839-DC47-E65A-179C-449A7C8E1C3E",
        "version": 0
      },
      {
        "key": "331BF078-984F-EAEB-EA84-B4B9A25AB9CC",
        "version": 0
      },
      {
        "key": "0F383166-E043-4D2D-27CF-09805AA95669",
        "version": 0
      },
      {
        "key": "9F8BF812-FC4A-7588-0CD9-7CA629BD3A38",
        "version": 31
      },
      {
        "key": "4CE75A7B-104C-70D2-9857-58A95A2A210B",
        "version": 11
      },
      {
        "key": "186929D7-DD4B-D61D-A864-E29D8438C13C",
        "version": 2
      },
      {
        "key": "7852A1C2-FE4A-E7BF-FF90-176C55F71D53",
        "version": 1
      },
      {
        "key": "D4A3AC6E-C14C-EC40-ED8B-86B7C58F4209",
        "version": 3
      },
      {
        "key": "DD75E529-2746-A3E0-76D2-109DEADC2C23",
        "version": 17
      },
      {
        "key": "5DA643AF-4749-D37F-8E3E-739805BBC1D9",
        "version": 2
      },
      {
        "key": "EC6C266B-8F4B-C71E-D9E4-0BA307FC4209",
        "version": 1
      },
      {
        "key": "613DF70D-EA47-3FA2-E989-27B79A49410C",
        "version": 1
      },
      {
        "key": "86181D60-844F-64AC-DED3-16AAD6C7EA0D",
        "version": 27
      },
      {
        "key": "D6BCFF9D-5801-4F49-8212-21E288A8923C",
        "version": 6
      },
      {
        "key": "ACD0AEF2-6F41-FE9A-7FAA-6486FCD626FA",
        "version": 1
      },
      {
        "key": "0B1F4F17-A545-C6B4-E82E-3FB17D91FBD0",
        "version": 9
      },
      {
        "key": "E79E7F71-3A49-B0E9-3291-B3880781381B",
        "version": 6
      },
      {
        "key": "B3DC7D8E-BB47-DA80-A246-D39FF64D9893",
        "version": 1
      },
      {
        "key": "CDB08ACB-DE4B-8CE7-9313-62A862EFE914",
        "version": 0
      },
      {
        "key": "F20A68FB-A34B-EF59-B519-A8BA3D44C873",
        "version": 1
      },
      {
        "key": "9186E0AF-5249-0D3A-3B67-73B61E2DF27C",
        "version": 2
      },
      {
        "key": "BDFDB52E-104D-AC01-8FF3-3681DAA59333",
        "version": 5
      },
      {
        "key": "4F359D50-2F49-E6F6-B285-49A71C633C07",
        "version": 0
      },
      {
        "key": "EAB762A4-3A4E-99F4-1FEC-C199B2E12482",
        "version": 2
      },
      {
        "key": "194D0C43-7049-5471-699B-6987E5B090DF",
        "version": 13
      },
      {
        "key": "BD32FEAA-144C-9553-255E-6AB6DDD13210",
        "version": 1
      },
      {
        "key": "8EE1AF23-584E-E14C-52C2-618DB7BE53B9",
        "version": 8
      },
      {
        "key": "40EB564A-DC11-F510-7E34-D392E76AC9B2",
        "version": 2
      },
      {
        "key": "004A8AD7-9746-58E8-B519-A8BAB4467D48",
        "version": 17
      },
      {
        "key": "86F87955-1F4C-3A93-7B08-BA832FB96163",
        "version": 1
      },
      {
        "key": "52BE2F61-0B40-53DA-914F-0D917C85B19F",
        "version": 1
      },
      {
        "key": "367A23A4-C941-EACA-F818-A28FF31B6858",
        "version": 4
      },
      {
        "key": "753F4E80-494B-8870-068C-D6A4DCB67E3C",
        "version": 5
      },
      {
        "key": "ED0A3111-614D-552E-A39A-67AF2C08A1C5",
        "version": 17
      },
      {
        "key": "965196AB-FC08-D845-8D22-D7B79E56AD78",
        "version": 1
      },
      {
        "key": "F37ABB24-834F-4656-C22D-2F1FFF96AD49",
        "version": 4
      },
      {
        "key": "12E426FB-4D4B-151F-0A55-7293702F1D96",
        "version": 3
      }
    ],
    "save_game_class_name": "/Script/SaveTest.TestSaveGame"
  },
  "properties": [
    [
      "Thing",
      {
        "type": "StructProperty",
        "value": {
          "Guid": "D49982B3-DF3D-D549-B4AE-57C71D5838E4"
        }
      }
    ]
  ]
}"#,
    );
}

#[test]
fn array_int() {
    serde_json(
        &Property::ArrayProperty(ArrayProperty::new(
            String::from("IntProperty"),
            None,
            vec![
                Property::IntProperty(IntProperty { value: 0 }),
                Property::IntProperty(IntProperty { value: 1 }),
            ],
        )),
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
fn array_str() {
    serde_json(
        &Property::ArrayProperty(ArrayProperty::new(
            String::from("StrProperty"),
            None,
            vec![
                Property::StrProperty(StrProperty::from("a")),
                Property::StrProperty(StrProperty::from("b")),
            ],
        )),
        r#"{
  "type": "ArrayProperty",
  "property_type": "StrProperty",
  "properties": [
    {
      "type": "StrProperty",
      "value": "a"
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
  "enum_type": null,
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
fn map_str_int() {
    let mut index_map = IndexMap::new();
    index_map.insert(
        Property::StrProperty(StrProperty::from("zero")),
        Property::IntProperty(IntProperty::new(0)),
    );
    index_map.insert(
        Property::StrProperty(StrProperty::from("one")),
        Property::IntProperty(IntProperty::new(1)),
    );
    index_map.insert(
        Property::StrProperty(StrProperty::from("two")),
        Property::IntProperty(IntProperty::new(2)),
    );
    serde_json(
        &Property::MapProperty(MapProperty::new(
            String::from("StrProperty"),
            String::from("IntProperty"),
            0,
            index_map,
        )),
        r#"{
  "type": "MapProperty",
  "key_type": "StrProperty",
  "value_type": "IntProperty",
  "allocation_flags": 0,
  "value": [
    [
      {
        "type": "StrProperty",
        "value": "zero"
      },
      {
        "type": "IntProperty",
        "value": 0
      }
    ],
    [
      {
        "type": "StrProperty",
        "value": "one"
      },
      {
        "type": "IntProperty",
        "value": 1
      }
    ],
    [
      {
        "type": "StrProperty",
        "value": "two"
      },
      {
        "type": "IntProperty",
        "value": 2
      }
    ]
  ]
}"#,
    )
}

#[test]
fn map_struct_float() {
    let mut index_map = IndexMap::new();
    index_map.insert(
        Property::StructProperty(StructProperty::new(
            Guid::new([0u8; 16]),
            StructPropertyValue::VectorF(VectorF::new(0f32, 1f32, 2f32)),
        )),
        Property::FloatProperty(FloatProperty::new(0f32)),
    );
    index_map.insert(
        Property::StructProperty(StructProperty::new(
            Guid::new([0x11u8; 16]),
            StructPropertyValue::Timespan(DateTime::new(0)),
        )),
        Property::FloatProperty(FloatProperty::new(1f32)),
    );
    index_map.insert(
        Property::StructProperty(StructProperty::new(
            Guid::new([0x22u8; 16]),
            StructPropertyValue::DateTime(DateTime::new(0)),
        )),
        Property::FloatProperty(FloatProperty::new(2f32)),
    );
    serde_json(
        &Property::MapProperty(MapProperty::new(
            String::from("StructProperty"),
            String::from("FloatProperty"),
            0,
            index_map,
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
fn name() {
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
fn str() {
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
  "history": {
    "None": {
      "culture_invariant_string": null
    }
  }
}"#,
    );
}

#[ignore] // This test fails
#[test]
fn text_none_some_none() {
    // serde_json(
    //     &Property::TextProperty(TextProperty::new(FText::new_none(1, Some(None)))),
    //     "{\n  \"type\": \"TextProperty\",\n  \"flags\": 1,\n  \"history\": {\n    \"None\": {\n      \"culture_invariant_string\": null\n    }\n  }\n}",
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
  "history": {
    "None": {
      "culture_invariant_string": "a"
    }
  }
}"#,
    );
}

#[test]
fn text_base() {
    serde_json(
        &Property::TextProperty(TextProperty::new(FText::new_base(0, None, None, None))),
        r#"{
  "type": "TextProperty",
  "flags": 0,
  "history": {
    "Base": {
      "namespace": null,
      "key": null,
      "source_string": null
    }
  }
}"#,
    );
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
  "history": {
    "Base": {
      "namespace": "ns",
      "key": "k",
      "source_string": "ss"
    }
  }
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
