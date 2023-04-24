use std::io::{Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::Error,
};

use super::PropertyTrait;

/// A property that describes a reference variable to another object which may be nil.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectProperty {
    /// Object reference
    pub value: String,
}

impl From<&str> for ObjectProperty {
    #[inline]
    fn from(value: &str) -> Self {
        ObjectProperty::new(value.into())
    }
}

impl ObjectProperty {
    /// Creates a new `ObjectProperty` instance
    #[inline]
    pub fn new(value: String) -> Self {
        ObjectProperty { value }
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
        let value = cursor.read_string()?;
        Ok(ObjectProperty { value })
    }
}

impl PropertyTrait for ObjectProperty {
    #[inline]
    fn write<W: Write>(&self, cursor: &mut W, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string("ObjectProperty")?;
            let property_length = self.value.len() + 1 + 4; // 1 is null-byte, 4 is string length field size
            cursor.write_u64::<LittleEndian>(property_length as u64)?;
            cursor.write_u8(0)?;
        }

        cursor.write_string(&self.value)?;
        Ok(())
    }
}
