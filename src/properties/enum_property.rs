use std::io::{Cursor, Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::Error,
};

use super::{impl_read_header, impl_write, impl_write_header_part, PropertyOptions, PropertyTrait};

/// A property that holds an enum value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumProperty {
    enum_type: String,
    value: String,
}

impl_write!(EnumProperty, (write_string, enum_type));

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
            Self::read_body(cursor, String::from("<array>"))
        }
    }

    impl_read_header!(enum_type);

    #[inline]
    fn read_body<R: Read + Seek>(cursor: &mut R, enum_type: String) -> Result<Self, Error> {
        let value = cursor.read_string()?;

        Ok(EnumProperty { enum_type, value })
    }

    #[inline]
    fn write_body<W: Write>(&self, cursor: &mut W) -> Result<(), Error> {
        cursor.write_string(&self.value)?;

        Ok(())
    }
}
