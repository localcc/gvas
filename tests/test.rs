use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

#[macro_export]
macro_rules! get_or_panic {
    ($map:expr, $property_name:expr, $property_class:ident) => {
        match $map.get($property_name) {
            Some(e) => match e {
                Property::$property_class(e) => e,
                _ => panic!("Property {} doesn't match expected type", $property_name),
            },
            None => panic!("Property {} not found", $property_name),
        }
    };
}

#[macro_export]
macro_rules! verify_property {
    ($map:expr, $property_name:expr, $property_class:ident, $($value:expr),+) => {
        let actual = get_or_panic!($map, $property_name, $property_class).to_owned();
        let expected = $property_class::new($($value),+);
        assert_eq!(actual, expected);
    };
}

use gvas::{
    properties::{
        array_property::ArrayProperty,
        int_property::{
            ByteProperty, DoubleProperty, FloatProperty, Int16Property, Int64Property,
            Int8Property, IntProperty, UInt16Property, UInt32Property, UInt64Property,
        },
        str_property::StrProperty,
        struct_property::{StructProperty, StructPropertyValue},
        struct_types::DateTime,
        Property,
    },
    types::Guid,
    GvasFile,
};

#[allow(clippy::approx_constant)]
fn verify_file_data(file: &GvasFile) {
    let properties = &file.properties;

    let none = Some(String::from("None"));
    verify_property!(properties, "u8_test", ByteProperty, none, 129);
    verify_property!(properties, "i8_test", Int8Property, -123);
    verify_property!(properties, "ushort_test", UInt16Property, 65530);
    verify_property!(properties, "short_test", Int16Property, -32764);
    verify_property!(properties, "uint32_test", UInt32Property, u32::MAX - 1);
    verify_property!(properties, "int32_test", IntProperty, i32::MIN + 1);
    verify_property!(properties, "ulong_test", UInt64Property, u64::MAX - 1);
    verify_property!(properties, "long_test", Int64Property, i64::MIN + 1);
    verify_property!(properties, "f_property", FloatProperty, 3.14159);
    verify_property!(properties, "d_property", DoubleProperty, 3.14159265358979);
    let hello_world = Some(String::from("Hello world"));
    verify_property!(properties, "str_property", StrProperty, hello_world);

    let struct_property = get_or_panic!(properties, "struct_property", StructProperty);
    match &struct_property.value {
        StructPropertyValue::CustomStruct(name, props) => {
            assert_eq!(name, "CustomStruct");
            assert_eq!(
                props.to_owned(),
                vec![(
                    "test_field".to_string(),
                    Property::from(UInt64Property::new(12345))
                )]
            );
        }
        _ => {
            panic!(
                "Property struct_property name doesn't match, expected CustomStruct got {:?}",
                struct_property.value
            );
        }
    }

    let date_time_property = get_or_panic!(properties, "date_time_property", StructProperty);
    assert_eq!(
        date_time_property.to_owned(),
        StructProperty::from(DateTime {
            ticks: 0x8DA2624F3F62720u64
        }),
    );

    let array_of_structs = get_or_panic!(properties, "array_of_structs", ArrayProperty);
    assert_eq!(
        array_of_structs.to_owned(),
        ArrayProperty::new(
            String::from("StructProperty"),
            Some((
                String::from("array_of_structs"),
                String::from("CustomStruct"),
                Guid::from(0)
            )),
            vec![
                Property::from(StructProperty {
                    guid: Guid::from(0),
                    value: StructPropertyValue::CustomStruct(
                        String::from("CustomStruct"),
                        vec![(
                            String::from("test_field"),
                            UInt64Property { value: 10 }.into()
                        )]
                    )
                }),
                Property::from(StructProperty {
                    guid: Guid::from(0),
                    value: StructPropertyValue::CustomStruct(
                        String::from("CustomStruct"),
                        vec![(
                            String::from("test_field"),
                            UInt64Property { value: 10 }.into()
                        )]
                    )
                }),
            ]
        )
    );

    let array_of_ints = get_or_panic!(properties, "array_of_ints", ArrayProperty);
    for property in &array_of_ints.properties {
        match property {
            Property::IntProperty(e) => {
                if e.value != 12 {
                    panic!(
                        "Property value value doesn't match, expected 12 got {}",
                        e.value
                    );
                }
            }
            _ => panic!("array_of_ints elements are not IntProperty"),
        }
    }

    let array_of_strings = get_or_panic!(properties, "array_of_strings", ArrayProperty);
    for property in &array_of_strings.properties {
        match property {
            Property::StrProperty(e) => {
                assert_eq!(e.value, Some(String::from("Hello world from array")));
            }
            _ => panic!("array_of_strings elements are not StrProperty"),
        }
    }
}

#[test]
fn read_file() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/Slot1.sav");
    let mut file = File::open(path).expect("Failed to open test asset");

    // Read the file in to a GvasFile
    let file = GvasFile::read(&mut file).expect("Failed to parse gvas file");

    // Verify GvasFile contents
    verify_file_data(&file);
}

#[test]
fn write_file() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/Slot1.sav");
    let mut file = File::open(path).expect("Failed to open test asset");

    // Read the file in to a Vec<u8>
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    // Convert the Vec<u8> to a GvasFile
    let mut cursor = Cursor::new(data);
    let file = GvasFile::read(&mut cursor).expect("Failed to parse gvas file");

    // Verify GvasFile contents
    verify_file_data(&file);

    // Convert the GvasFile back to a Vec<u8>
    let mut writer = Cursor::new(Vec::new());
    file.write(&mut writer)
        .expect("Failed to serialize gvas file");

    // Compare the two Vec<u8>s
    assert_eq!(cursor.get_ref(), writer.get_ref());

    // Read the file back in again
    let mut reader = Cursor::new(writer.get_ref().to_owned());
    let file2 = GvasFile::read(&mut reader).expect("Failed to parse serialized save file");

    // Verify GvasFile contents
    verify_file_data(&file2);

    // Compare the two GvasFiles
    assert_eq!(file, file2);
}
