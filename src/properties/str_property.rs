use std::io::{Cursor, Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use unreal_helpers::{UnrealReadExt, UnrealWriteExt};

use crate::{cursor_ext::WriteExt, error::Error};

use super::{impl_read, impl_read_header, impl_write, PropertyOptions, PropertyTrait};

/// A property that holds a GVAS string value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StrProperty {
    /// Value of the GVAS string.
    pub value: Option<String>,
}

impl From<&str> for StrProperty {
    #[inline]
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl From<String> for StrProperty {
    #[inline]
    fn from(value: String) -> Self {
        Self::new(Some(value))
    }
}

impl StrProperty {
    /// Creates a new `StrProperty` instance.
    #[inline]
    pub fn new(value: Option<String>) -> Self {
        StrProperty { value }
    }

    impl_read!();
    impl_read_header!();

    #[inline]
    fn read_body<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let value = cursor.read_fstring()?;
        Ok(StrProperty { value })
    }
}

impl PropertyTrait for StrProperty {
    impl_write!(StrProperty);

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        _: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        let len = cursor.write_fstring(self.value.as_deref())?;
        Ok(len)
    }
}
