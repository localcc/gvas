use std::io::{Cursor, Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use unreal_helpers::UnrealWriteExt;

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::Error,
};

use super::{impl_read_header, impl_write, impl_write_header_part, PropertyOptions, PropertyTrait};

/// A property that holds an enum value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumProperty {
    /// Enum Type.
    pub enum_type: Option<String>,
    /// Enum Value.
    pub value: String,
}

impl EnumProperty {
    /// Creates a new `EnumProperty` instance.
    #[inline]
    pub fn new(enum_type: Option<String>, value: String) -> Self {
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
            Self::read_body(cursor, None)
        }
    }

    impl_read_header!(enum_type);

    #[inline]
    fn read_body<R: Read + Seek>(cursor: &mut R, enum_type: Option<String>) -> Result<Self, Error> {
        let value = cursor.read_string()?;

        Ok(EnumProperty { enum_type, value })
    }
}

impl PropertyTrait for EnumProperty {
    impl_write!(EnumProperty, (write_fstring, enum_type));

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        _: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        let len = cursor.write_string(&self.value)?;

        Ok(len)
    }
}
