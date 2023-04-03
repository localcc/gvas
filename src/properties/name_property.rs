use std::io::{Cursor, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{cursor_ext::CursorExt, error::Error};

use super::PropertyTrait;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NameProperty {
    pub value: String,
}

impl NameProperty {
    pub(crate) fn read(cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<Self, Error> {
        if include_header {
            let _length = cursor.read_u64::<LittleEndian>()?;
            cursor.read_exact(&mut [0u8; 1])?;
        }
        let value = cursor.read_string()?;
        Ok(NameProperty { value })
    }
}

impl PropertyTrait for NameProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string("NameProperty")?;
            let property_length = self.value.len() + 1 + 4; // 1 is null-byte, 4 is string length field size
            cursor.write_i64::<LittleEndian>(property_length as i64)?;
            let _ = cursor.write(&[0u8; 1])?;
        }

        cursor.write_string(&self.value)?;
        Ok(())
    }
}
