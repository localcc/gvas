use std::io::{Cursor, Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::Error,
};

use super::{impl_read, impl_read_header, impl_write, PropertyOptions, PropertyTrait};

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

    impl_read!();
    impl_read_header!();

    #[inline]
    fn read_body<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let value = cursor.read_string()?;
        Ok(ObjectProperty { value })
    }
}

impl PropertyTrait for ObjectProperty {
    impl_write!(ObjectProperty);

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        _: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        cursor.write_string(&self.value)
    }
}
