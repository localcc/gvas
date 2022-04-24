use std::{
    collections::HashMap,
    io::{Cursor, Read, Write},
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

    pub fn read(name: String, cursor: &mut Cursor<Vec<u8>>) -> Result<StructProperty, Error> {
        let mut guid = [0u8; 16];
        cursor.read_exact(&mut guid)?;
        cursor.read_exact(&mut [0u8; 1])?;

        let mut properties = HashMap::new();
        let mut key_name = cursor.read_string()?;
        while &key_name != "None" {
            let property = Property::new(cursor)?;
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
    fn get_length(&self) -> i64 {
        let mut len = 12;
        for (_, property) in &self.properties {
            len += property.get_length();
        }
        len
    }

    fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        cursor.write_i64::<LittleEndian>(self.get_length())?;
        cursor.write_string(&self.name)?;
        cursor.write(&self.guid)?;
        cursor.write(&[0u8; 1])?;

        for (key, value) in &self.properties {
            cursor.write_string(key)?;
            value.write(cursor)?;
        }
        cursor.write_string(&String::from("None"))?;

        Ok(())
    }
}

impl DateTimeProperty {
    pub fn new(guid: Guid, ticks: u64) -> Self {
        DateTimeProperty { guid, ticks }
    }

    pub fn read(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut guid = [0u8; 16];
        cursor.read_exact(&mut guid)?;
        cursor.read_exact(&mut [0u8; 1])?;

        let ticks = cursor.read_u64::<LittleEndian>()?;
        Ok(DateTimeProperty { guid, ticks })
    }
}

impl PropertyTrait for DateTimeProperty {
    fn get_length(&self) -> i64 {
        8
    }

    fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        cursor.write_i64::<LittleEndian>(self.get_length())?;
        cursor.write_string(&String::from("DateTime"))?;
        cursor.write(&self.guid)?;
        cursor.write(&[0u8; 1])?;

        cursor.write_u64::<LittleEndian>(self.ticks)?;
        Ok(())
    }
}
