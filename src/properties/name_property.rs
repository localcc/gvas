use std::io::{Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::Error,
};

use super::{impl_read, impl_read_header, PropertyOptions, PropertyTrait};

/// A property that holds a name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NameProperty {
    /// Name value.
    pub value: String,
}

impl NameProperty {
    impl_read!();
    impl_read_header!();

    #[inline]
    fn read_body<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let value = cursor.read_string()?;
        Ok(NameProperty { value })
    }
}

impl PropertyTrait for NameProperty {
    #[inline]
    fn write<W: Write>(
        &self,
        cursor: &mut W,
        include_header: bool,
        _options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        if include_header {
            cursor.write_string("NameProperty")?;
            let property_length = self.value.len() + 1 + 4; // 1 is null-byte, 4 is string length field size
            cursor.write_u64::<LittleEndian>(property_length as u64)?;
            cursor.write_u8(0)?;
        }

        cursor.write_string(&self.value)?;
        Ok(())
    }
}
