use gvas::{
    error::{DeserializeError, Error},
    game_version::GameVersion,
    properties::{
        array_property::ArrayProperty, enum_property::EnumProperty, map_property::MapProperty,
        set_property::SetProperty, str_property::StrProperty, PropertyOptions,
    },
    types::map::HashableIndexMap,
    GvasFile,
};
use std::{collections::HashMap, io::Cursor};

const UNEXPECTED_EOF: [u8; 0] = [];

#[test]
fn test_unexpected_eof() {
    let mut reader = Cursor::new(UNEXPECTED_EOF);
    let result = GvasFile::read(&mut reader, GameVersion::Default);
    match result {
        Err(Error::Io(e)) => {
            assert_eq!(e.to_string(), "failed to fill whole buffer");
        }
        _ => panic!("Unexpected result {result:?}"),
    }
}

const INVALID_HEADER: [u8; 4] = [b'G', b'V', b'A', b'Z'];

#[test]
fn test_invalid_header() {
    let mut reader = Cursor::new(INVALID_HEADER);
    let result = GvasFile::read(&mut reader, GameVersion::Default);
    match result {
        Err(Error::Deserialize(DeserializeError::InvalidHeader(reason))) => {
            assert_eq!(reason.into_string(), "File type 1514231367 not recognized");
        }
        _ => panic!("Unexpected result {result:?}"),
    }
}

const INVALID_ARRAY_INDEX: [u8; 8] = [
    0, 0, 0, 0, // length
    1, 0, 0, 0, // array_index
];

#[test]
fn test_invalid_array_index() {
    // StrProperty
    let mut reader = Cursor::new(INVALID_ARRAY_INDEX);
    let result = StrProperty::read_header(&mut reader);
    match result {
        Err(Error::Deserialize(DeserializeError::InvalidArrayIndex(value, position))) => {
            assert_eq!(value, 1);
            assert_eq!(position, 4);
        }
        _ => panic!("Unexpected result {result:?}"),
    };

    // EnumProperty
    let mut reader = Cursor::new(INVALID_ARRAY_INDEX);
    let result = EnumProperty::read_header(&mut reader);
    match result {
        Err(Error::Deserialize(DeserializeError::InvalidArrayIndex(value, position))) => {
            assert_eq!(value, 1);
            assert_eq!(position, 4);
        }
        _ => panic!("Unexpected result {result:?}"),
    };

    let mut options = PropertyOptions {
        hints: &HashMap::new(),
        properties_stack: &mut Vec::new(),
        custom_versions: &HashableIndexMap::new(),
    };

    // ArrayProperty
    let mut reader = Cursor::new(INVALID_ARRAY_INDEX);
    let result = ArrayProperty::read_header(&mut reader, &mut options);
    match result {
        Err(Error::Deserialize(DeserializeError::InvalidArrayIndex(value, position))) => {
            assert_eq!(value, 1);
            assert_eq!(position, 4);
        }
        _ => panic!("Unexpected result {result:?}"),
    };

    // SetProperty
    let mut reader = Cursor::new(INVALID_ARRAY_INDEX);
    let result = SetProperty::read_header(&mut reader, &mut options);
    match result {
        Err(Error::Deserialize(DeserializeError::InvalidArrayIndex(value, position))) => {
            assert_eq!(value, 1);
            assert_eq!(position, 4);
        }
        _ => panic!("Unexpected result {result:?}"),
    };

    // MapProperty
    let mut reader = Cursor::new(INVALID_ARRAY_INDEX);
    let result = MapProperty::read_header(&mut reader, &mut options);
    match result {
        Err(Error::Deserialize(DeserializeError::InvalidArrayIndex(value, position))) => {
            assert_eq!(value, 1);
            assert_eq!(position, 4);
        }
        _ => panic!("Unexpected result {result:?}"),
    };
}

const INVALID_TERMINATOR: [u8; 9] = [
    0, 0, 0, 0, // length
    0, 0, 0, 0, // array_index
    1, // terminator
];

const INVALID_TERMINATOR_ENUM: [u8; 14] = [
    0, 0, 0, 0, // length
    0, 0, 0, 0, // array_index
    1, 0, 0, 0, 0, // enum_type
    1, // terminator
];

const INVALID_TERMINATOR_MAP: [u8; 19] = [
    0, 0, 0, 0, // length
    0, 0, 0, 0, // array_index
    1, 0, 0, 0, 0, // key_type
    1, 0, 0, 0, 0, // value_type
    1, // terminator
];

#[test]
fn test_invalid_terminator() {
    // StrProperty
    let mut reader = Cursor::new(INVALID_TERMINATOR);
    let result = StrProperty::read_header(&mut reader);
    match result {
        Err(Error::Deserialize(DeserializeError::InvalidTerminator(value, position))) => {
            assert_eq!(value, 1);
            assert_eq!(position, 8);
        }
        _ => panic!("Unexpected result {result:?}"),
    };

    // EnumProperty
    let mut reader = Cursor::new(INVALID_TERMINATOR_ENUM);
    let result = EnumProperty::read_header(&mut reader);
    match result {
        Err(Error::Deserialize(DeserializeError::InvalidTerminator(value, position))) => {
            assert_eq!(value, 1);
            assert_eq!(position, 13);
        }
        _ => panic!("Unexpected result {result:?}"),
    };

    let mut options = PropertyOptions {
        hints: &HashMap::new(),
        properties_stack: &mut Vec::new(),
        custom_versions: &HashableIndexMap::new(),
    };

    // ArrayProperty
    let mut reader = Cursor::new(INVALID_TERMINATOR_ENUM);
    let result = ArrayProperty::read_header(&mut reader, &mut options);
    match result {
        Err(Error::Deserialize(DeserializeError::InvalidTerminator(value, position))) => {
            assert_eq!(value, 1);
            assert_eq!(position, 13);
        }
        _ => panic!("Unexpected result {result:?}"),
    };

    // SetProperty
    let mut reader = Cursor::new(INVALID_TERMINATOR_ENUM);
    let result = SetProperty::read_header(&mut reader, &mut options);
    match result {
        Err(Error::Deserialize(DeserializeError::InvalidTerminator(value, position))) => {
            assert_eq!(value, 1);
            assert_eq!(position, 13);
        }
        _ => panic!("Unexpected result {result:?}"),
    };

    // MapProperty
    let mut reader = Cursor::new(INVALID_TERMINATOR_MAP);
    let result = MapProperty::read_header(&mut reader, &mut options);
    match result {
        Err(Error::Deserialize(DeserializeError::InvalidTerminator(value, position))) => {
            assert_eq!(value, 1);
            assert_eq!(position, 18);
        }
        _ => panic!("Unexpected result {result:?}"),
    };
}

const INVALID_LENGTH_STR: [u8; 13] = [
    0, 0, 0, 0, // length
    0, 0, 0, 0, // array_index
    0, // terminator
    0, 0, 0, 0, // string
];

const INVALID_LENGTH_ENUM: [u8; 19] = [
    0, 0, 0, 0, // length
    0, 0, 0, 0, // array_index
    1, 0, 0, 0, 0, // enum_type
    0, // terminator
    1, 0, 0, 0, 0, // string
];

const INVALID_LENGTH_ARRAY: [u8; 18] = [
    0, 0, 0, 0, // length
    0, 0, 0, 0, // array_index
    1, 0, 0, 0, 0, // property_type
    0, // terminator
    0, 0, 0, 0, // element_count
];

const INVALID_LENGTH_SET: [u8; 22] = [
    0, 0, 0, 0, // length
    0, 0, 0, 0, // array_index
    1, 0, 0, 0, 0, // property_type
    0, // terminator
    0, 0, 0, 0, // allocation_flags
    0, 0, 0, 0, // element_count
];

const INVALID_LENGTH_MAP: [u8; 27] = [
    0, 0, 0, 0, // length
    0, 0, 0, 0, // array_index
    1, 0, 0, 0, 0, // key_type
    1, 0, 0, 0, 0, // value_type
    0, // terminator
    0, 0, 0, 0, // allocation_flags
    0, 0, 0, 0, // element_count
];

#[test]
fn test_invalid_length() {
    // StrProperty
    let mut reader = Cursor::new(INVALID_LENGTH_STR);
    let result = StrProperty::read_header(&mut reader);
    match result {
        Err(Error::Deserialize(DeserializeError::InvalidValueSize(expected, read, position))) => {
            assert_eq!(expected, 0);
            assert_eq!(read, 4);
            assert_eq!(position, 9);
        }
        _ => panic!("Unexpected result {result:?}"),
    }

    // EnumProperty
    let mut reader = Cursor::new(INVALID_LENGTH_ENUM);
    let result = EnumProperty::read_header(&mut reader);
    match result {
        Err(Error::Deserialize(DeserializeError::InvalidValueSize(expected, read, position))) => {
            assert_eq!(expected, 0);
            assert_eq!(read, 5);
            assert_eq!(position, 14);
        }
        _ => panic!("Unexpected result {result:?}"),
    }

    let mut options = PropertyOptions {
        hints: &HashMap::new(),
        properties_stack: &mut Vec::new(),
        custom_versions: &HashableIndexMap::new(),
    };

    // ArrayProperty
    let mut reader = Cursor::new(INVALID_LENGTH_ARRAY);
    let result = ArrayProperty::read_header(&mut reader, &mut options);
    match result {
        Err(Error::Deserialize(DeserializeError::InvalidValueSize(expected, read, position))) => {
            assert_eq!(expected, 0);
            assert_eq!(read, 4);
            assert_eq!(position, 14);
        }
        _ => panic!("Unexpected result {result:?}"),
    };

    // SetProperty
    let mut reader = Cursor::new(INVALID_LENGTH_SET);
    let result = SetProperty::read_header(&mut reader, &mut options);
    match result {
        Err(Error::Deserialize(DeserializeError::InvalidValueSize(expected, read, position))) => {
            assert_eq!(expected, 0);
            assert_eq!(read, 8);
            assert_eq!(position, 14);
        }
        _ => panic!("Unexpected result {result:?}"),
    };

    // MapProperty
    let mut reader = Cursor::new(INVALID_LENGTH_MAP);
    let result = MapProperty::read_header(&mut reader, &mut options);
    match result {
        Err(Error::Deserialize(DeserializeError::InvalidValueSize(expected, read, position))) => {
            assert_eq!(expected, 0);
            assert_eq!(read, 8);
            assert_eq!(position, 19);
        }
        _ => panic!("Unexpected result {result:?}"),
    };
}
