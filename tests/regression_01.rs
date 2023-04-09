use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
};

use gvas::GvasFile;

#[test]
fn regression_01_guid() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/regression_01.bin");
    let mut file = File::open(path).expect("Failed to open test asset");

    // Read the file in to a Vec<u8>
    let mut data = Vec::new();
    file.read_to_end(&mut data)
        .expect("Failed to read test asset");

    // Convert the Vec<u8> to a GvasFile
    let mut cursor = Cursor::new(data);
    let file = GvasFile::read(&mut cursor).expect("Failed to parse gvas file");

    // Convert the GvasFile back to a Vec<u8>
    let mut writer = Cursor::new(Vec::new());
    file.write(&mut writer).expect("Failed to write test asset");

    // Compare the two Vec<u8>s
    assert_eq!(cursor.get_ref(), writer.get_ref());

    // Read the file back in again
    let mut cursor = Cursor::new(writer.get_ref().to_owned());
    let read_back = GvasFile::read(&mut cursor).expect("Failed to read written asset");

    // Compare the two GvasFiles
    assert_eq!(file, read_back);

    let original_guid = file
        .properties
        .get("Thing")
        .expect("Failed to get test property in original asset")
        .get_struct()
        .expect("Failed to cast property from original asset to the correct type")
        .value
        .get_guid()
        .expect("Failed to get property from original asset as Guid");

    let written_guid = read_back
        .properties
        .get("Thing")
        .expect("Failed to get test property in written asset")
        .get_struct()
        .expect("Failed to cast property from written asset to the correct type")
        .value
        .get_guid()
        .expect("Failed to get property from written asset as Guid");

    assert_eq!(original_guid, written_guid);
}
