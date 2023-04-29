use std::io::{Cursor, Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::{DeserializeError, Error, SerializeError},
};

use super::{impl_read_header, PropertyOptions, PropertyTrait};

/// A property that holds an enum value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumProperty {
    enum_type: String,
    value: String,
}

impl EnumProperty {
    /// Creates a new `EnumProperty` instance.
    #[inline]
    pub fn new(enum_type: String, value: String) -> Self {
        EnumProperty { enum_type, value }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        include_header: bool,
    ) -> Result<Self, Error> {
        if include_header {
            Self::read_header(cursor)
        } else {
            Err(DeserializeError::invalid_property(
                "EnumProperty is not supported in arrays",
                cursor,
            ))?
        }
    }

    impl_read_header!(enum_type);

    #[inline]
    fn read_body<R: Read + Seek>(cursor: &mut R, enum_type: String) -> Result<Self, Error> {
        let value = cursor.read_string()?;

        Ok(EnumProperty { enum_type, value })
    }
}

impl PropertyTrait for EnumProperty {
    #[inline]
    fn write<W: Write>(
        &self,
        cursor: &mut W,
        include_header: bool,
        _options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        if !include_header {
            // return self.write_body(cursor);
            Err(SerializeError::invalid_value(
                "Enum without headers not supported!",
            ))?
        }

        let buf = &mut Cursor::new(Vec::new());
        self.write_body(buf)?;
        let buf = buf.get_ref();

        cursor.write_string("EnumProperty")?;
        cursor.write_u64::<LittleEndian>(buf.len() as u64)?;
        cursor.write_string(&self.enum_type)?;
        cursor.write_u8(0)?;
        cursor.write_all(buf)?;

        Ok(())
    }
}

impl EnumProperty {
    #[inline]
    fn write_body<W: Write>(&self, cursor: &mut W) -> Result<(), Error> {
        cursor.write_string(&self.value)?;

        Ok(())
    }
}
