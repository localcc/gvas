use std::{
    collections::HashMap,
    io::{Cursor, Read, Write, SeekFrom, Seek},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{cursor_ext::CursorExt, error::Error, Guid};

use super::{Property, PropertyTrait};

pub struct StructProperty {
    pub name: String,
    pub guid: Guid,
    pub properties: HashMap<String, Property>,
}

pub struct DateTimeProperty {
    pub guid: Guid,
    pub ticks: u64,
}

impl StructProperty {
    pub fn new(name: String, guid: Guid, properties: HashMap<String, Property>) -> Self {
        StructProperty {
            name,
            guid,
            properties,
        }
    }

    pub fn read(cursor: &mut Cursor<Vec<u8>>, name: String, guid: [u8; 16]) -> Result<StructProperty, Error> {
        let mut properties = HashMap::new();
        let mut key_name = cursor.read_string()?;
        while &key_name != "None" {
            let value_type = cursor.read_string()?;
            let property = Property::new(cursor, &value_type, true)?;
            properties.insert(key_name, property);
            key_name = cursor.read_string()?;
        }

        Ok(StructProperty {
            name,
            guid,
            properties,
        })
    }
}

impl PropertyTrait for StructProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        let mut begin = 0;
        let mut write_begin = 0;
        if include_header {
            cursor.write_string(&String::from("StructProperty"))?;
            begin = cursor.position();
            cursor.write_u64::<LittleEndian>(0)?;
            cursor.write_string(&self.name)?;
            cursor.write(&self.guid)?;
            cursor.write(&[0u8; 1])?;
            write_begin = cursor.position();
        }

        for (key, value) in &self.properties {
            cursor.write_string(key)?;
            value.write(cursor, true)?;
        }
        cursor.write_string(&String::from("None"))?;

        if include_header {
            let write_end = cursor.position();
            cursor.seek(SeekFrom::Start(begin))?;
            cursor.write_u64::<LittleEndian>(write_end - write_begin)?;
            cursor.seek(SeekFrom::Start(write_end))?;
        }

        Ok(())
    }
}

impl DateTimeProperty {
    pub fn new(guid: Guid, ticks: u64) -> Self {
        DateTimeProperty { guid, ticks }
    }

    pub fn read(cursor: &mut Cursor<Vec<u8>>, guid: [u8; 16]) -> Result<Self, Error> {
        let ticks = cursor.read_u64::<LittleEndian>()?;
        Ok(DateTimeProperty { guid, ticks })
    }
}

impl PropertyTrait for DateTimeProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string(&String::from("StructProperty"))?;
            cursor.write_u64::<LittleEndian>(8)?;
            cursor.write_string(&String::from("DateTime"))?;
            cursor.write(&self.guid)?;
            cursor.write(&[0u8; 1])?;
        }

        cursor.write_u64::<LittleEndian>(self.ticks)?;
        Ok(())
    }
}
