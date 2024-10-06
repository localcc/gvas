use crate::common::*;
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
        struct_types::{
            DateTime, IntPoint, LinearColor, QuatD, QuatF, RotatorD, RotatorF, VectorD, VectorF,
        },
        text_property::{
            DateTimeStyle, FText, FTextHistory, FormatArgumentData, FormatArgumentValue,
            NumberFormattingOptions, RoundingMode, TextProperty, TransformType,
        },
        unknown_property::UnknownProperty,
        Property,
    },
    types::{map::HashableIndexMap, Guid},
    GvasFile,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
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

fn file<P: AsRef<Path>>(path: P, json: &str) {
    file_with_hints(path, &HashMap::new(), json)
}

fn file_with_hints<P: AsRef<Path>>(path: P, hints: &HashMap<String, String>, json: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    let mut file = File::open(path).expect("Failed to open test asset");

    // Read the file in to a Vec<u8>
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    // Convert the Vec<u8> to a GvasFile
    let mut cursor = Cursor::new(data);
    let file = GvasFile::read_with_hints(&mut cursor, GameVersion::Default, hints)
        .expect("Failed to parse gvas file");

    // Compare the GvasFile to its expected JSON representation
    serde_json(&file, json);
}

#[test]
fn file_profile_0() {
    file_with_hints(PROFILE_0_PATH, &profile0::hints(), profile0::PROFILE_0_JSON);
}

#[test]
fn file_regression_01() {
    file(REGRESSION_01_PATH, regression::REGRESSION_01_JSON);
}

#[test]
fn file_saveslot_03() {
    file_with_hints(
        SAVESLOT_03_PATH,
        &saveslot3::hints(),
        saveslot3::SAVESLOT_03_JSON,
    )
}

#[test]
fn file_vector2d() {
    file(VECTOR2D_PATH, vector2d::VECTOR2D_JSON)
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
  "ints": [
    0,
    1
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
                    Property::ByteProperty(ByteProperty::new_byte(None, 0xab)),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "bytes": "0001ab"
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
  "bools": [
    false,
    true
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
  "floats": [
    1.0,
    2.0
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
                    Property::EnumProperty(EnumProperty::new(None, "b".to_string())),
                ],
            )
            .expect("ArrayProperty::new"),
        ),
        r#"{
  "type": "ArrayProperty",
  "enums": [
    "a",
    "b"
  ]
}"#,
    )
}

#[test]
fn array_enum_ns() {
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
  "names": [
    null,
    "b"
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
  "strings": [
    null,
    "b"
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
                        HashableIndexMap::from([]),
                    )),
                    Property::MapProperty(MapProperty::new(
                        "ktb".to_string(),
                        "vtb".to_string(),
                        1,
                        HashableIndexMap::from([]),
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
      "DateTime": {
        "ticks": 0
      }
    },
    {
      "DateTime": {
        "ticks": 1
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
            HashableIndexMap::from([
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
            HashableIndexMap::from([
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
            HashableIndexMap::from([
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
            HashableIndexMap::from([
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
            HashableIndexMap::from([
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
            HashableIndexMap::from([
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
            HashableIndexMap::from([
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
            HashableIndexMap::from([
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
            HashableIndexMap::from([
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
            HashableIndexMap::from([
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
            HashableIndexMap::from([
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
            HashableIndexMap::from([
                (
                    Property::StructProperty(StructProperty::new(
                        Guid([0u8; 16]),
                        StructPropertyValue::VectorF(VectorF::new(0f32, 1f32, 2f32)),
                    )),
                    Property::FloatProperty(FloatProperty::new(0f32)),
                ),
                (
                    Property::StructProperty(StructProperty::new(
                        Guid([0x11u8; 16]),
                        StructPropertyValue::Timespan(DateTime::new(0)),
                    )),
                    Property::FloatProperty(FloatProperty::new(1f32)),
                ),
                (
                    Property::StructProperty(StructProperty::new(
                        Guid([0x22u8; 16]),
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
        "VectorF": {
          "x": 0.0,
          "y": 1.0,
          "z": 2.0
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
        "Timespan": {
          "ticks": 0
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
        "DateTime": {
          "ticks": 0
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
            Guid([0u8; 16]),
            StructPropertyValue::VectorF(VectorF::new(0f32, 1f32, 2f32)),
        )),
        r#"{
  "type": "StructProperty",
  "VectorF": {
    "x": 0.0,
    "y": 1.0,
    "z": 2.0
  }
}"#,
    )
}

#[test]
fn struct_vectord() {
    serde_json(
        &Property::StructProperty(StructProperty::new(
            Guid([0u8; 16]),
            StructPropertyValue::VectorD(VectorD::new(0f64, 1f64, 2f64)),
        )),
        r#"{
  "type": "StructProperty",
  "VectorD": {
    "x": 0.0,
    "y": 1.0,
    "z": 2.0
  }
}"#,
    )
}

#[test]
fn struct_rotatorf() {
    serde_json(
        &Property::StructProperty(StructProperty::new(
            Guid([0u8; 16]),
            StructPropertyValue::RotatorF(RotatorF::new(0f32, 1f32, 2f32)),
        )),
        r#"{
  "type": "StructProperty",
  "RotatorF": {
    "pitch": 0.0,
    "yaw": 1.0,
    "roll": 2.0
  }
}"#,
    )
}

#[test]
fn struct_rotatord() {
    serde_json(
        &Property::StructProperty(StructProperty::new(
            Guid([0u8; 16]),
            StructPropertyValue::RotatorD(RotatorD::new(0f64, 1f64, 2f64)),
        )),
        r#"{
  "type": "StructProperty",
  "RotatorD": {
    "pitch": 0.0,
    "yaw": 1.0,
    "roll": 2.0
  }
}"#,
    )
}

#[test]
fn struct_quatf() {
    serde_json(
        &Property::StructProperty(StructProperty::new(
            Guid([0u8; 16]),
            StructPropertyValue::QuatF(QuatF::new(0f32, 1f32, 2f32, 3f32)),
        )),
        r#"{
  "type": "StructProperty",
  "QuatF": {
    "x": 0.0,
    "y": 1.0,
    "z": 2.0,
    "w": 3.0
  }
}"#,
    )
}

#[test]
fn struct_quatd() {
    serde_json(
        &Property::StructProperty(StructProperty::new(
            Guid([0u8; 16]),
            StructPropertyValue::QuatD(QuatD::new(0f64, 1f64, 2f64, 3f64)),
        )),
        r#"{
  "type": "StructProperty",
  "QuatD": {
    "x": 0.0,
    "y": 1.0,
    "z": 2.0,
    "w": 3.0
  }
}"#,
    )
}

#[test]
fn struct_datetime() {
    serde_json(
        &Property::StructProperty(StructProperty::new(
            Guid([0u8; 16]),
            StructPropertyValue::QuatD(QuatD::new(0f64, 1f64, 2f64, 3f64)),
        )),
        r#"{
  "type": "StructProperty",
  "QuatD": {
    "x": 0.0,
    "y": 1.0,
    "z": 2.0,
    "w": 3.0
  }
}"#,
    )
}

#[test]
fn struct_linearcolor() {
    serde_json(
        &Property::StructProperty(StructProperty::new(
            Guid::default(),
            StructPropertyValue::LinearColor(LinearColor::new(0.0, 1.0, 2.0, 3.0)),
        )),
        r#"{
  "type": "StructProperty",
  "LinearColor": {
    "r": 0.0,
    "g": 1.0,
    "b": 2.0,
    "a": 3.0
  }
}"#,
    )
}

#[test]
fn struct_intpoint() {
    serde_json(
        &Property::StructProperty(StructProperty::new(
            Guid::default(),
            StructPropertyValue::IntPoint(IntPoint::new(0, 1)),
        )),
        r#"{
  "type": "StructProperty",
  "IntPoint": {
    "x": 0,
    "y": 1
  }
}"#,
    )
}

#[test]
fn struct_custom() {
    serde_json(
        &Property::StructProperty(StructProperty::new(
            Guid::default(),
            StructPropertyValue::CustomStruct {
                type_name: String::from("custom name"),
                properties: HashableIndexMap::from([(
                    String::from("key"),
                    vec![Property::from(StrProperty::from("value"))],
                )]),
            },
        )),
        r#"{
  "type": "StructProperty",
  "CustomStruct": {
    "type_name": "custom name",
    "properties": {
      "key": [
        {
          "type": "StrProperty",
          "value": "value"
        }
      ]
    }
  }
}"#,
    )
}

#[test]
fn struct_array_index() {
    serde_json(
        &Property::StructProperty(StructProperty {
            guid: Guid::default(),
            value: StructPropertyValue::CustomStruct {
                type_name: String::from("TowersTrackedQuests"),
                properties: HashableIndexMap::from([(
                    String::from("TrackedQuestsNames"),
                    vec![
                        Property::NameProperty(NameProperty {
                            array_index: 0,
                            value: Some(String::from("QU91_InvestigateTower_B2")),
                        }),
                        Property::NameProperty(NameProperty {
                            array_index: 1,
                            value: Some(String::from("QU91_InvestigateTower_B2")),
                        }),
                    ],
                )]),
            },
        }),
        r#"{
  "type": "StructProperty",
  "CustomStruct": {
    "type_name": "TowersTrackedQuests",
    "properties": {
      "TrackedQuestsNames": [
        {
          "type": "NameProperty",
          "value": "QU91_InvestigateTower_B2"
        },
        {
          "type": "NameProperty",
          "array_index": 1,
          "value": "QU91_InvestigateTower_B2"
        }
      ]
    }
  }
}"#,
    )
}

#[test]
fn text_empty() {
    serde_json(
        &Property::TextProperty(TextProperty::new(FText::new_none(0, None))),
        r#"{
  "type": "TextProperty",
  "history": "Empty"
}"#,
    );
}

#[test]
fn text_none_some_none() {
    serde_json(
        &Property::TextProperty(TextProperty::new(FText::new_none(1, Some(None)))),
        r#"{
  "type": "TextProperty",
  "flags": 1,
  "history": "None"
}"#,
    );
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
                arguments: HashableIndexMap::from([(
                    String::from("key"),
                    FormatArgumentValue::Int(2),
                )]),
            },
        })),
        r#"{
  "type": "TextProperty",
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
