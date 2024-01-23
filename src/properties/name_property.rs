use std::io::{Cursor, Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use unreal_helpers::{UnrealReadExt, UnrealWriteExt};

use crate::{cursor_ext::WriteExt, error::Error};

use super::{impl_read, impl_read_header, impl_write, PropertyOptions, PropertyTrait};

/// A property that holds a name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NameProperty {
    /// Array Index
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "is_zero"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub array_index: u32,
    /// Name value.
    pub value: Option<String>,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[cfg(feature = "serde")]
fn is_zero(num: &u32) -> bool {
    *num == 0
}

impl_write!(NameProperty, array_index);

impl From<&str> for NameProperty {
    #[inline]
    fn from(value: &str) -> Self {
        NameProperty::from(Some(value.into()))
    }
}

impl From<Option<String>> for NameProperty {
    #[inline]
    fn from(value: Option<String>) -> Self {
        let array_index: u32 = 0;
        NameProperty { array_index, value }
    }
}

impl NameProperty {
    impl_read!(array_index);
    impl_read_header!(array_index);

    #[inline]
    fn read_body<R: Read + Seek>(cursor: &mut R, array_index: u32) -> Result<Self, Error> {
        let value = cursor.read_fstring()?;
        Ok(NameProperty { array_index, value })
    }

    #[inline]
    fn write_body<W: Write>(&self, cursor: &mut W) -> Result<(), Error> {
        cursor.write_fstring(self.value.as_deref())?;
        Ok(())
    }
}
