mod test_file;

use std::{collections::HashMap, io::Cursor};

use gvas::{
    cursor_ext::ReadExt,
    properties::{
        array_property::ArrayProperty,
        enum_property::EnumProperty,
        int_property::{
            BoolProperty, ByteProperty, BytePropertyValue, DoubleProperty, FloatProperty,
            Int16Property, Int64Property, Int8Property, IntProperty, UInt16Property,
            UInt32Property, UInt64Property,
        },
        map_property::MapProperty,
        set_property::SetProperty,
        str_property::StrProperty,
        struct_property::StructProperty,
        struct_types::VectorF,
        text_property::TextProperty,
        Property, PropertyOptions, PropertyTrait,
    },
    types::Guid,
};

use gvas::properties::text_property::FText;
use indexmap::IndexMap;

macro_rules! test_property {
    ($function_name:ident, $type:ident, $property_value:expr) => {
        #[test]
        fn $function_name() {
            let property: $type = $property_value;

            let mut options = PropertyOptions {
                hints: &HashMap::new(),
                properties_stack: &mut Vec::new(),
                large_world_coordinates: false,
                custom_versions: &[],
            };

            // Export the property to a byte array
            let mut writer = Cursor::new(Vec::new());
            property
                .write(&mut writer, true, &mut options)
                .expect(concat!("Failed to serialize {}", stringify!($ty)));

            // Import the property from a byte array
            let mut reader = Cursor::new(writer.get_ref().to_owned());
            let property_type = reader
                .read_string()
                .expect(&format!("Read {}", stringify!(property)));
            assert_eq!(property_type, stringify!($type));
            let imported = Property::new(&mut reader, &property_type, true, &mut options, None)
                .expect(&format!("Reading {} from {:?}", property_type, reader));

            assert_eq!(writer, reader);
            assert_eq!(Property::$type(property), imported);
        }
    };
}

test_property!(test_int8, Int8Property, Int8Property::new(i8::MAX));
test_property!(
    test_byte,
    ByteProperty,
    ByteProperty::new(
        Some(String::from("Test ByteProperty")),
        BytePropertyValue::Byte(2)
    )
);
test_property!(
    test_byte_namespaced,
    ByteProperty,
    ByteProperty::new(
        Some(String::from("Test NamespacedByteproperty")),
        BytePropertyValue::Namespaced(String::from("TestEnum::Value0"))
    )
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
    EnumProperty::new(String::from("type"), String::from("value"))
);

// StructProperty
test_property!(
    test_struct,
    StructProperty,
    StructProperty::from(VectorF::new(0f32, 1f32, 2f32))
);

// ArrayProperty
test_property!(
    test_array_empty,
    ArrayProperty,
    ArrayProperty::new(String::from("FloatProperty"), None, vec![],)
);

test_property!(
    test_array_float,
    ArrayProperty,
    ArrayProperty::new(
        String::from("FloatProperty"),
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
        String::from("StructProperty"),
        Some((
            "FieldName".to_string(),
            String::from("Vector"),
            Guid::from(0u128)
        )),
        vec![
            Property::from(StructProperty::from(VectorF::new(0f32, 1f32, 2f32))),
            Property::from(StructProperty::from(VectorF::new(3f32, 4f32, 5f32))),
        ],
    )
);

// TextProperty
test_property!(
    test_array_text,
    ArrayProperty,
    ArrayProperty::new(
        String::from("TextProperty"),
        None,
        vec![
            Property::from(TextProperty::new(FText::new_none(0, None))),
            Property::from(TextProperty::new(FText::new_base(
                0,
                Some(String::from("identifier")),
                Some(String::from("{0}<br>{1}")),
                Some(String::from("test<br>test"))
            ))),
        ]
    )
);

// SetProperty
test_property!(
    test_set,
    SetProperty,
    SetProperty::new(
        String::from("FloatProperty"),
        0,
        vec![Property::from(FloatProperty::new(4321f32))]
    )
);

// MapProperty
test_property!(
    test_map,
    MapProperty,
    MapProperty::new(
        String::from("StrProperty"),
        String::from("FloatProperty"),
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
