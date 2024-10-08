use std::io::Cursor;

use gvas::{
    cursor_ext::{ReadExt, WriteExt},
    error::Error,
};

#[test]
fn test_write_string() -> Result<(), Error> {
    // ASCII
    let mut cursor = Cursor::new(Vec::new());
    cursor.write_string("test")?;
    assert_eq!(
        cursor.get_ref(),
        &[5u8, 0u8, 0u8, 0u8, b't', b'e', b's', b't', 0u8],
    );

    // Non-ASCII
    let mut cursor = Cursor::new(Vec::new());
    cursor.write_string("\u{A7}")?;
    assert_eq!(
        cursor.get_ref(),
        &[0xfeu8, 0xffu8, 0xffu8, 0xffu8, 0xa7u8, 0u8, 0u8, 0u8],
    );

    Ok(())
}

#[test]
fn test_write_fstring() -> Result<(), Error> {
    // ASCII
    let mut cursor = Cursor::new(Vec::new());
    cursor.write_fstring(Some("test"))?;
    assert_eq!(
        cursor.get_ref(),
        &[5u8, 0u8, 0u8, 0u8, b't', b'e', b's', b't', 0u8],
    );

    // Non-ASCII
    let mut cursor = Cursor::new(Vec::new());
    cursor.write_fstring(Some("\u{A7}"))?;
    assert_eq!(
        cursor.get_ref(),
        &[0xfeu8, 0xffu8, 0xffu8, 0xffu8, 0xa7u8, 0u8, 0u8, 0u8],
    );

    // Null
    let mut cursor = Cursor::new(Vec::new());
    cursor.write_fstring(None)?;
    assert_eq!(cursor.get_ref(), &[0u8; 4],);

    Ok(())
}

#[test]
fn test_read_string() -> Result<(), Error> {
    // ASCII
    let mut cursor = Cursor::new(vec![5u8, 0u8, 0u8, 0u8, b't', b'e', b's', b't', 0u8]);
    let string = cursor.read_string()?;
    assert_eq!(string, "test");

    // Non-ASCII
    let mut cursor = Cursor::new(vec![0xfeu8, 0xffu8, 0xffu8, 0xffu8, 0xa7u8, 0u8, 0u8, 0u8]);
    let string = cursor.read_string()?;
    assert_eq!(string, "\u{A7}");

    // Null
    let mut cursor = Cursor::new(vec![0u8; 4]);
    let string = cursor.read_string().expect_err("Expected err").to_string();
    assert_eq!(string, "Invalid string size 0 at position 0x4");

    // Missing null terminator
    let mut cursor = Cursor::new(vec![1u8, 0u8, 0u8, 0u8, b't']);
    let string = cursor.read_string().expect_err("Expected err").to_string();
    assert_eq!(string, "Invalid string terminator 116 at position 0x5");

    // Missing null terminator, UTF-16
    let mut cursor = Cursor::new(vec![0xffu8, 0xffu8, 0xffu8, 0xffu8, b't', b'e']);
    let string = cursor.read_string().expect_err("Expected err").to_string();
    assert_eq!(string, "Invalid string terminator 25972 at position 0x6");

    Ok(())
}

#[test]
fn test_read_fstring() -> Result<(), Error> {
    // ASCII
    let mut cursor = Cursor::new(vec![5u8, 0u8, 0u8, 0u8, b't', b'e', b's', b't', 0u8]);
    let string = cursor.read_fstring()?.expect("Expected Some");
    assert_eq!(string, "test");

    // Non-ASCII
    let mut cursor = Cursor::new(vec![0xfeu8, 0xffu8, 0xffu8, 0xffu8, 0xa7u8, 0u8, 0u8, 0u8]);
    let string = cursor.read_fstring()?.expect("Expected Some");
    assert_eq!(string, "\u{A7}");

    // Null
    let mut cursor = Cursor::new(vec![0u8; 4]);
    let string = cursor.read_fstring()?;
    assert_eq!(string, None);

    // Missing null terminator
    let mut cursor = Cursor::new(vec![1u8, 0u8, 0u8, 0u8, b't']);
    let string = cursor.read_fstring().expect_err("Expected err").to_string();
    assert_eq!(string, "Invalid string terminator 116 at position 0x5");

    // Missing null terminator, UTF-16
    let mut cursor = Cursor::new(vec![0xffu8, 0xffu8, 0xffu8, 0xffu8, b't', b'e']);
    let string = cursor.read_fstring().expect_err("Expected err").to_string();
    assert_eq!(string, "Invalid string terminator 25972 at position 0x6");

    Ok(())
}
