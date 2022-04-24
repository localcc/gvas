use std::{path::Path, fs::File, io::{Read, Cursor}};

use gvas::{GvasFile, properties::{Property}};

macro_rules! verify_property {
    ($map:ident, $property_name:expr, $property_class:ident, $value:expr) => {
        match $map.get(&String::from($property_name)) {
            Some(e) => {
                match e {
                    Property::$property_class(e) => match e.value == $value {
                        false => panic!("Property {} value doesn't match, expected {} got {}", $property_name, $value, e.value),
                        true => {}
                    },
                    _ => panic!("Property {} doesn't match expected type", $property_name)
                }
            },
            None => panic!("Property {} not found", $property_name)
        };
    };
}

#[test]
fn read_file() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/Slot1.sav");
    let mut file = File::open(&path).expect("Failed to open test asset");

    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read test asset");

    let mut cursor = Cursor::new(data);
    
    let file = GvasFile::read(&mut cursor).expect("Failed to parse gvas file");
    let properties = &file.properties;

    match properties.get(&String::from("u8_test")) {
        Some(e) => match e {
            Property::ByteProperty(e) => {
                match e.name == "None" {
                    false => panic!("Property ByteProperty name doesn't match, expected {} got {}", "None", e.name),
                    true => {}
                };
                match e.value == 129 {
                    false => panic!("Property ByteProperty value doesn't match, expected {} got {}", 129, e.value),
                    true => {}
                }
            },
            _ => panic!("Property u8_test doesn't match expected type")
            }   
        None => panic!("Property u8_test not found")
    };

    verify_property!(properties, "i8_test", Int8Property, -123);
    verify_property!(properties, "ushort_test", UInt16Property, 65530);
    verify_property!(properties, "short_test", Int16Property, -32764);
    verify_property!(properties, "uint32_test", UInt32Property, u32::MAX - 1);
    verify_property!(properties, "int32_test", IntProperty, i32::MIN + 1);
    verify_property!(properties, "ulong_test", UInt64Property, u64::MAX - 1);
    verify_property!(properties, "long_test", Int64Property, i64::MIN + 1);
    verify_property!(properties, "f_property", FloatProperty, 3.14159);
    verify_property!(properties, "d_property", DoubleProperty, 3.14159265358979);
}