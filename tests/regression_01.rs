use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

use gvas::{
    cast,
    properties::{struct_property::StructProperty, Property},
    GvasFile,
};

#[test]
fn regression_01_guid() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/regression_01.bin");
    let mut file = File::open(path).expect("Failed to open test asset");

    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    let mut cursor = Cursor::new(data);

    let file = GvasFile::read(&mut cursor).expect("Failed to parse gvas file");

    let mut writer = Cursor::new(Vec::new());
    file.write(&mut writer).expect("Failed to write test asset");

    let mut cursor = Cursor::new(writer.get_ref().to_owned());
    let read_back = GvasFile::read(&mut cursor).expect("Failed to read written asset");

    let original_property: &StructProperty = cast!(
        Property,
        StructProperty,
        file.properties
            .get("Thing")
            .expect("Failed to get test property in original asset")
    )
    .expect("Failed to cast property from original asset to the correct type");

    let written_property: &StructProperty = cast!(
        Property,
        StructProperty,
        read_back
            .properties
            .get("Thing")
            .expect("Failed to get test property in written asset")
    )
    .expect("Failed to cast property from written asset to the correct type");

    let original_guid = original_property
        .get_guid()
        .expect("Failed to get property from original asset as Guid");
    let written_guid = written_property
        .get_guid()
        .expect("Failed to get property from written asset as Guid");

    assert_eq!(original_guid, written_guid);
}
