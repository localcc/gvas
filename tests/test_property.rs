mod test_file;

use std::{collections::HashMap, io::Cursor};

use gvas::{
    cast,
    cursor_ext::CursorExt,
    properties::{
        array_property::ArrayProperty,
        enum_property::EnumProperty,
        int_property::{
            BoolProperty, ByteProperty, DoubleProperty, FloatProperty, Int16Property,
            Int64Property, Int8Property, IntProperty, UInt16Property, UInt32Property,
            UInt64Property,
        },
        map_property::MapProperty,
        set_property::SetProperty,
        str_property::StrProperty,
        struct_property::StructProperty,
        struct_types::Vector,
        text_property::{RichText, RichTextFormat, TextProperty},
        Property, PropertyTrait,
    },
    types::Guid,
};

use indexmap::IndexMap;

macro_rules! test_property {
    ($function_name:ident, $type:ident, $property_value:expr) => {
        #[test]
        fn $function_name() {
            let property: $type = $property_value;

            // Export the property to a byte array
            let mut writer = Cursor::new(Vec::new());
            property
                .write(&mut writer, true)
                .expect(&format!("Failed to serialize {}", stringify!($ty)));

            // Import the property from a byte array
            let mut reader = Cursor::new(writer.get_ref().to_owned());
            let property_type = reader
                .read_string()
                .expect(&format!("Read {}", stringify!(property)));
            assert_eq!(property_type, stringify!($type));
            let imported = Property::new(
                &mut reader,
                &HashMap::new(),
                &mut Vec::new(),
                &property_type,
                true,
                None,
            )
            .expect(&format!("Reading {} from {:?}", property_type, reader));

            assert_eq!(writer, reader);
            assert_eq!(
                property,
                cast!(Property, $type, imported).expect(&format!("{} cast", stringify!($type)))
            );
        }
    };
}

test_property!(test_int8, Int8Property, Int8Property::new(i8::MAX));
test_property!(
    test_byte,
    ByteProperty,
    ByteProperty::new(Some("Test ByteProperty".into()), u8::MAX)
);
test_property!(test_int16, Int16Property, Int16Property::new(i16::MAX));
test_property!(test_uint16, UInt16Property, UInt16Property::new(u16::MAX));
test_property!(test_int, IntProperty, IntProperty::new(i32::MAX));
test_property!(test_uint32, UInt32Property, UInt32Property::new(u32::MAX));
test_property!(test_int64, Int64Property, Int64Property::new(i64::MAX));
test_property!(test_uint64, UInt64Property, UInt64Property::new(u64::MAX));
test_property!(test_float, FloatProperty, FloatProperty::new(1234f32));
test_property!(test_double, DoubleProperty, DoubleProperty::new(1234f64));
test_property!(test_bool, BoolProperty, BoolProperty::new(true));
test_property!(test_str, StrProperty, StrProperty::from("test string"));

// EnumProperty
test_property!(
    test_enum,
    EnumProperty,
    EnumProperty::new("type".into(), "value".into(), true)
);

// StructProperty
test_property!(
    test_struct,
    StructProperty,
    StructProperty::from(Vector::new(0f32, 1f32, 2f32))
);

// ArrayProperty
test_property!(
    test_array_empty,
    ArrayProperty,
    ArrayProperty::new("FloatProperty".into(), None, vec![],)
);

test_property!(
    test_array_float,
    ArrayProperty,
    ArrayProperty::new(
        "FloatProperty".into(),
        None,
        vec![
            Property::from(FloatProperty::new(0f32)),
            Property::from(FloatProperty::new(1f32)),
        ],
    )
);

test_property!(
    test_array_vector,
    ArrayProperty,
    ArrayProperty::new(
        "StructProperty".into(),
        Some(("FieldName".to_string(), "Vector".into(), Guid::from(0u128))),
        vec![
            Property::from(StructProperty::from(Vector::new(0f32, 1f32, 2f32))),
            Property::from(StructProperty::from(Vector::new(3f32, 4f32, 5f32))),
        ],
    )
);

// TextProperty
test_property!(
    test_array_text,
    ArrayProperty,
    ArrayProperty::new(
        "TextProperty".into(),
        None,
        vec![
            Property::from(TextProperty::new(None, None)),
            Property::from(TextProperty::new(
                Some(RichText::new(
                    "identifier".into(),
                    "{0}<br>{1}".into(),
                    vec![
                        RichTextFormat::new("0".into(), 0, vec!["line1".into()]),
                        RichTextFormat::new("1".into(), 0, vec!["line2".into()]),
                    ]
                )),
                None
            )),
            Property::from(TextProperty::new(None, Some(vec!["String".into()]))),
        ]
    )
);

// SetProperty
test_property!(
    test_set,
    SetProperty,
    SetProperty::new(
        "FloatProperty".into(),
        0,
        vec![Property::from(FloatProperty::new(4321f32))]
    )
);

// MapProperty
test_property!(
    test_map,
    MapProperty,
    MapProperty::new(
        "StrProperty".into(),
        "FloatProperty".into(),
        0,
        IndexMap::from([
            (
                Property::from(StrProperty::from("key1")),
                Property::from(FloatProperty::new(-1f32)),
            ),
            (
                Property::from(StrProperty::from("key2")),
                Property::from(FloatProperty::new(0.5f32)),
            ),
        ]),
    )
);
