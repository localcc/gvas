use std::io::{Cursor, Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::Error,
};

use super::{impl_write, impl_write_header_part, Property, PropertyOptions, PropertyTrait};

/// A property that stores a set of properties.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetProperty {
    /// Property type.
    pub property_type: String,
    /// Allocation flags.
    pub allocation_flags: u32,
    /// Properties.
    pub properties: Vec<Property>,
}

impl_write!(SetProperty, options, (write_string, property_type));

impl SetProperty {
    /// Creates a new `SetProperty` instance.
    #[inline]
    pub fn new(property_type: String, allocation_flags: u32, properties: Vec<Property>) -> Self {
        SetProperty {
            property_type,
            allocation_flags,
            properties,
        }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        let length = cursor.read_u64::<LittleEndian>()?;

        let property_type = cursor.read_string()?;
        let separator = cursor.read_u8()?;
        assert_eq!(separator, 0);

        let allocation_flags = cursor.read_u32::<LittleEndian>()?;

        let element_count = cursor.read_u32::<LittleEndian>()? as usize;
        let mut properties: Vec<Property> = Vec::with_capacity(element_count);

        let total_bytes_per_property = (length - 8) / element_count as u64;

        for _ in 0..element_count {
            properties.push(Property::new(
                cursor,
                &property_type,
                false,
                options,
                Some(total_bytes_per_property),
            )?)
        }

        Ok(SetProperty {
            property_type,
            allocation_flags,
            properties,
        })
    }

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        cursor.write_u32::<LittleEndian>(self.allocation_flags)?;
        cursor.write_u32::<LittleEndian>(self.properties.len() as u32)?;
        for property in &self.properties {
            property.write(cursor, false, options)?;
        }

        Ok(())
    }
}
