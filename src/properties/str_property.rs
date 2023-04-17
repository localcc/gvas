use std::io::{Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use unreal_helpers::{UnrealReadExt, UnrealWriteExt};

use crate::{cursor_ext::WriteExt, error::Error};

use super::PropertyTrait;

/// A property that holds a GVAS string value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StrProperty {
    /// Value of the GVAS string.
    pub value: Option<String>,
}

impl From<&str> for StrProperty {
    fn from(value: &str) -> Self {
        StrProperty::new(Some(value.into()))
    }
}

impl StrProperty {
    /// Creates a new `StrProperty` instance.
    pub fn new(value: Option<String>) -> Self {
        StrProperty { value }
    }

    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        include_header: bool,
    ) -> Result<Self, Error> {
        if include_header {
            let _length = cursor.read_u64::<LittleEndian>()?;
            cursor.read_exact(&mut [0u8; 1])?;
        }
        let value = cursor.read_fstring()?;
        Ok(StrProperty { value })
    }
}

impl PropertyTrait for StrProperty {
    fn write<W: Write + Seek>(&self, cursor: &mut W, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string("StrProperty")?;
            let property_length = match &self.value {
                Some(value) => value.len() + 1 + 4, // 1 is null-byte, 4 is string length field size
                None => 4,                          // 4 is string length field size
            };
            cursor.write_i64::<LittleEndian>(property_length as i64)?;
            let _ = cursor.write(&[0u8; 1])?;
        }

        cursor.write_fstring(self.value.as_deref())?;

        Ok(())
    }
}
