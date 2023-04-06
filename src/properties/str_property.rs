use std::io::{Cursor, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{cursor_ext::CursorExt, error::Error};

use super::PropertyTrait;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StrProperty {
    pub value: Option<String>,
}

impl From<&str> for StrProperty {
    fn from(value: &str) -> Self {
        StrProperty::new(Some(value.into()))
    }
}

impl StrProperty {
    pub fn new(value: Option<String>) -> Self {
        StrProperty { value }
    }

    pub(crate) fn read(cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<Self, Error> {
        if include_header {
            let _length = cursor.read_u64::<LittleEndian>()?;
            cursor.read_exact(&mut [0u8; 1])?;
        }
        let value = cursor.read_string_opt()?;
        Ok(StrProperty { value })
    }
}

impl PropertyTrait for StrProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string(&String::from("StrProperty"))?;
            let property_length = match &self.value {
                Some(value) => value.len() + 1 + 4, // 1 is null-byte, 4 is string length field size
                None => 4,                          // 4 is string length field size
            };
            cursor.write_i64::<LittleEndian>(property_length as i64)?;
            let _ = cursor.write(&[0u8; 1])?;
        }

        match &self.value {
            Some(value) => cursor.write_string(value),
            None => {
                cursor.write_all(&[0u8; 4])?;
                Ok(())
            }
        }
    }
}
