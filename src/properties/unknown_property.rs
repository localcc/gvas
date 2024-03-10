use std::io::{Cursor, Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{cursor_ext::WriteExt, error::Error};

use super::{PropertyOptions, PropertyTrait};

/// This struct is read when a property is unknown to the deserializer
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnknownProperty {
    property_name: String,
    raw: Vec<u8>,
}

impl UnknownProperty {
    /// Creates a new `UnknownProperty` instance.
    #[inline]
    pub fn new(property_name: String, raw: Vec<u8>) -> Self {
        UnknownProperty { property_name, raw }
    }

    #[inline]
    pub(crate) fn read_with_length<R: Read + Seek>(
        cursor: &mut R,
        property_name: String,
        length: u32,
    ) -> Result<Self, Error> {
        let mut data = vec![0u8; length as usize];
        cursor.read_exact(&mut data)?;

        Ok(UnknownProperty {
            property_name,
            raw: data,
        })
    }

    #[inline]
    pub(crate) fn read_with_header<R: Read + Seek>(
        cursor: &mut R,
        property_name: String,
    ) -> Result<Self, Error> {
        let length = cursor.read_u32::<LittleEndian>()?;
        let array_index = cursor.read_u32::<LittleEndian>()?;
        assert_eq!(
            array_index,
            0,
            "Expected array_index value zero @ {:#x}",
            cursor.stream_position()? - 4
        );
        let separator = cursor.read_u8()?;
        assert_eq!(separator, 0);

        UnknownProperty::read_with_length(cursor, property_name, length)
    }
}

impl PropertyTrait for UnknownProperty {
    #[inline]
    fn write<W: Write>(
        &self,
        cursor: &mut W,
        include_header: bool,
        options: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        if !include_header {
            return self.write_body(cursor, options);
        }

        let buf = &mut Cursor::new(Vec::new());
        let body_len = self.write_body(buf, options)?;
        let buf = buf.get_ref();

        let name_len = cursor.write_string(&self.property_name)?;
        cursor.write_u32::<LittleEndian>(buf.len() as u32)?;
        cursor.write_u32::<LittleEndian>(0)?;
        cursor.write_u8(0)?;
        cursor.write_all(buf)?;

        Ok(9 + name_len + body_len)
    }

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        _: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        cursor.write_all(&self.raw)?;

        Ok(self.raw.len())
    }
}
