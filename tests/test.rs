use std::{
    fs::File,
    io::{Cursor, Read, Write},
    path::Path,
};

use gvas::{properties::Property, GvasFile};

macro_rules! get_or_panic {
    ($map:ident, $property_name:expr, $property_class:ident) => {
        match $map.get(&String::from($property_name)) {
            Some(e) => match e {
                Property::$property_class(e) => e,
                _ => panic!("Property {} doesn't match expected type", $property_name),
            },
            None => panic!("Property {} not found", $property_name),
        }
    };
}

macro_rules! verify_property {
    ($map:ident, $property_name:expr, $property_class:ident, $value:expr) => {
        let e = get_or_panic!($map, $property_name, $property_class);
        match e.value == $value {
            false => panic!(
                "Property {} value doesn't match, expected {} got {}",
                $property_name, $value, e.value
            ),
            true => {}
        };
    };
}

fn verify_file_data(file: &GvasFile) {
    let properties = &file.properties;

    let byte_property = get_or_panic!(properties, "u8_test", ByteProperty);
    if byte_property.name.as_ref().unwrap() != "None" {
        panic!(
            "Property u8_test name doesn't match, expected None got {:?}",
            byte_property.name
        );
    }
    if byte_property.value != 129 {
        panic!(
            "Property u8_test value doesn't match, expected 129 got {}",
            byte_property.value
        );
    }

    verify_property!(properties, "i8_test", Int8Property, -123);
    verify_property!(properties, "ushort_test", UInt16Property, 65530);
    verify_property!(properties, "short_test", Int16Property, -32764);
    verify_property!(properties, "uint32_test", UInt32Property, u32::MAX - 1);
    verify_property!(properties, "int32_test", IntProperty, i32::MIN + 1);
    verify_property!(properties, "ulong_test", UInt64Property, u64::MAX - 1);
    verify_property!(properties, "long_test", Int64Property, i64::MIN + 1);
    verify_property!(properties, "f_property", FloatProperty, 3.14159);
    verify_property!(properties, "d_property", DoubleProperty, 3.14159265358979);
    verify_property!(properties, "str_property", StrProperty, "Hello world");

    let struct_property = get_or_panic!(properties, "struct_property", StructProperty);
    if struct_property.name != "CustomStruct" {
        panic!(
            "Property struct_property name doesn't match, expected CustomStruct got {}",
            struct_property.name
        );
    }
    let struct_property_properties = &struct_property.properties;
    verify_property!(
        struct_property_properties,
        "test_field",
        UInt64Property,
        12345
    );

    let date_time_property = get_or_panic!(properties, "date_time_property", DateTimeProperty);
    if date_time_property.ticks != 0x8DA2624F3F62720 {
        panic!(
            "Property date_time_property ticks doesn't match, expected 0x8DA2624F3F62720 got {:#x}",
            date_time_property.ticks
        );
    }

    let array_of_structs = get_or_panic!(properties, "array_of_structs", ArrayProperty);
    for property in &array_of_structs.properties {
        match property {
            Property::StructProperty(e) => {
                let properties = &e.properties;
                verify_property!(properties, "test_field", UInt64Property, 10);
            }
            _ => panic!("array_of_structs elements are not StructProperty"),
        }
    }

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
                if &e.value != "Hello world from array" {
                    panic!("Property value value doesn't match, expected \"Hello world from array\" got {}", e.value);
                }
            }
            _ => panic!("array_of_strings elements are not StrProperty"),
        }
    }
}

#[test]
fn read_file() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/Slot1.sav");
    let mut file = File::open(&path).expect("Failed to open test asset");

    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    let mut cursor = Cursor::new(data);

    let file = GvasFile::read(&mut cursor).expect("Failed to parse gvas file");
    verify_file_data(&file);
}

#[test]
fn write_file() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/Slot1.sav");
    let mut file = File::open(&path).expect("Failed to open test asset");

    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    let mut cursor = Cursor::new(data);

    let file = GvasFile::read(&mut cursor).expect("Failed to parse gvas file");
    verify_file_data(&file);

    let mut writer = Cursor::new(Vec::new());
    file.write(&mut writer).expect("Failed to serialize gvas file");

    let mut reader = Cursor::new(writer.get_ref().to_owned());
    let file = GvasFile::read(&mut reader).expect("Failed to parse serialized save file");
    verify_file_data(&file);
}
