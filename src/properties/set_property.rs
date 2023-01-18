use std::{
    collections::HashMap,
    io::{Cursor, Read, Seek, SeekFrom, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{cursor_ext::CursorExt, error::Error};

use super::{Property, PropertyTrait};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetProperty {
    pub property_type: String,
    pub allocation_flags: u32,
    pub properties: Vec<Property>,
}

impl SetProperty {
    pub fn new(property_type: String, allocation_flags: u32, properties: Vec<Property>) -> Self {
        SetProperty {
            property_type,
            allocation_flags,
            properties,
        }
    }

    pub(crate) fn read(
        cursor: &mut Cursor<Vec<u8>>,
        hints: &HashMap<String, String>,
        properties_stack: &mut Vec<String>,
    ) -> Result<Self, Error> {
        let length = cursor.read_u64::<LittleEndian>()?;

        let property_type = cursor.read_string()?;
        cursor.read_exact(&mut [0u8; 1])?;

        let allocation_flags = cursor.read_u32::<LittleEndian>()?;

        let element_count = cursor.read_i32::<LittleEndian>()? as usize;
        let mut properties: Vec<Property> = Vec::with_capacity(element_count);

        let total_bytes_per_property = (length - 8) / element_count as u64;

        for _ in 0..element_count {
            properties.push(Property::new(
                cursor,
                hints,
                properties_stack,
                &property_type,
                false,
                Some(total_bytes_per_property),
            )?)
        }

        Ok(SetProperty {
            property_type,
            allocation_flags,
            properties,
        })
    }
}

impl PropertyTrait for SetProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if !include_header {
            panic!("Nested sets are not supported!"); // fixme: throw error
        }

        if self.properties.is_empty() {
            return Ok(());
        }

        cursor.write_string(&String::from("SetProperty"))?;

        let begin = cursor.position();
        cursor.write_u64::<LittleEndian>(0)?;

        cursor.write_string(&self.property_type)?;
        let _ = cursor.write(&[0u8; 1])?;

        let set_begin = cursor.position();

        cursor.write_u32::<LittleEndian>(self.allocation_flags)?;
        cursor.write_i32::<LittleEndian>(self.properties.len() as i32)?;

        for property in &self.properties {
            property.write(cursor, false)?;
        }

        let end_write = cursor.position();
        cursor.seek(SeekFrom::Start(begin))?;
        cursor.write_u64::<LittleEndian>(end_write - set_begin - 1)?;
        cursor.seek(SeekFrom::Start(end_write))?;

        Ok(())
    }
}
