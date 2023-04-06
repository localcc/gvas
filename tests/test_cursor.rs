use std::io::Cursor;

use gvas::{cursor_ext::CursorExt, error::Error};

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
