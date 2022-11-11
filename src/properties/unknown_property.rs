use std::io::{Cursor, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{cursor_ext::CursorExt, error::Error};

use super::PropertyTrait;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnknownProperty {
    property_name: String,
    raw: Vec<u8>,
}

impl UnknownProperty {
    pub fn new(property_name: String, raw: Vec<u8>) -> Self {
        UnknownProperty { property_name, raw }
    }

    pub fn read_with_length(
        cursor: &mut Cursor<Vec<u8>>,
        property_name: String,
        length: u64,
    ) -> Result<Self, Error> {
        let mut data = vec![0u8; length as usize];
        cursor.read_exact(&mut data)?;

        Ok(UnknownProperty {
            property_name,
            raw: data,
        })
    }

    pub fn read_with_header(
        cursor: &mut Cursor<Vec<u8>>,
        property_name: String,
    ) -> Result<Self, Error> {
        let length = cursor.read_u64::<LittleEndian>()?;
        cursor.read_exact(&mut [0u8; 1])?;

        let mut data = vec![0u8; length as usize];
        cursor.read_exact(&mut data)?;

        Ok(UnknownProperty {
            property_name,
            raw: data,
        })
    }
}

impl PropertyTrait for UnknownProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string(&self.property_name)?;
            cursor.write_u64::<LittleEndian>(self.raw.len() as u64)?;
            cursor.write_all(&[0u8; 1])?;
        }

        cursor.write_all(&self.raw)?;

        Ok(())
    }
}
