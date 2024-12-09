use gvas::cursor_ext::ReadExt;
use gvas::properties::{name_property::NameProperty, PropertyOptions, PropertyTrait};
use gvas::types::map::HashableIndexMap;
use std::collections::HashMap;
use std::io::Cursor;

#[test]
fn name_property_with_array_index() {
    let data = vec![
        0x0d, 0x00, 0x00, 0x00, 0x4e, 0x61, 0x6d, 0x65, 0x50, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74,
        0x79, 0x00, 0x1d, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x00,
        0x51, 0x55, 0x39, 0x31, 0x5f, 0x49, 0x6e, 0x76, 0x65, 0x73, 0x74, 0x69, 0x67, 0x61, 0x74,
        0x65, 0x54, 0x6f, 0x77, 0x65, 0x72, 0x5f, 0x42, 0x32, 0x00,
    ];

    // Convert the Vec<u8> to a NameProperty
    let mut cursor = Cursor::new(data);
    let property_type = cursor.read_fstring().expect("Failed to read property type");
    assert_eq!(Some(String::from("NameProperty")), property_type);
    let prop = NameProperty::read(&mut cursor, true).expect("Failed to read NameProperty");

    // Compare the parsed value to its expected value
    assert_eq!(
        NameProperty {
            array_index: 1,
            value: Some("QU91_InvestigateTower_B2".into()),
        },
        prop
    );

    // Convert the NameProperty back to a Vec<u8>
    let mut options = PropertyOptions {
        hints: &HashMap::new(),
        properties_stack: &mut Vec::new(),
        custom_versions: &HashableIndexMap::new(),
    };
    let mut writer = Cursor::new(Vec::new());
    prop.write(&mut writer, true, &mut options)
        .expect("Failed to serialize gvas file");

    // Compare the two Vec<u8>s
    assert_eq!(cursor.get_ref(), writer.get_ref());
}
