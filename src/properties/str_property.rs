use std::io::{Cursor, Read, Seek, Write};

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
    #[inline]
    fn from(value: &str) -> Self {
        StrProperty::new(Some(value.into()))
    }
}

impl StrProperty {
    /// Creates a new `StrProperty` instance.
    #[inline]
    pub fn new(value: Option<String>) -> Self {
        StrProperty { value }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        include_header: bool,
    ) -> Result<Self, Error> {
        if include_header {
            let _length = cursor.read_u64::<LittleEndian>()?;
            let separator = cursor.read_u8()?;
            assert_eq!(separator, 0);
        }
        let value = cursor.read_fstring()?;
        Ok(StrProperty { value })
    }
}

impl PropertyTrait for StrProperty {
    #[inline]
    fn write<W: Write>(&self, cursor: &mut W, include_header: bool) -> Result<(), Error> {
        if !include_header {
            return self.write_body(cursor);
        }

        let buf = &mut Cursor::new(Vec::new());
        self.write_body(buf)?;
        let buf = buf.get_ref();

        cursor.write_string("StrProperty")?;
        cursor.write_u64::<LittleEndian>(buf.len() as u64)?;
        cursor.write_u8(0)?;
        cursor.write_all(buf)?;

        Ok(())
    }
}

impl StrProperty {
    #[inline]
    fn write_body<W: Write>(&self, cursor: &mut W) -> Result<(), Error> {
        cursor.write_fstring(self.value.as_deref())?;

        Ok(())
    }
}
