use std::{
    collections::HashMap,
    hash::Hash,
    io::{Cursor, Read, Seek, SeekFrom, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{cursor_ext::CursorExt, error::Error, scoped_stack_entry::ScopedStackEntry};

use super::{Property, PropertyTrait};

#[derive(Debug, Clone)]
pub struct MapProperty {
    pub key_type: String,
    pub value_type: String,
    pub allocation_flags: u32,
    pub value: HashMap<Property, Property>,
}

impl MapProperty {
    pub fn new(
        key_type: String,
        value_type: String,
        allocation_flags: u32,
        value: HashMap<Property, Property>,
    ) -> Self {
        MapProperty {
            key_type,
            value_type,
            allocation_flags,
            value,
        }
    }

    pub(crate) fn read(
        cursor: &mut Cursor<Vec<u8>>,
        hints: &HashMap<String, String>,
        properties_stack: &mut Vec<String>,
    ) -> Result<Self, Error> {
        let _length = cursor.read_u64::<LittleEndian>()?;

        let key_type = cursor.read_string()?;
        let value_type = cursor.read_string()?;

        cursor.read_exact(&mut [0u8; 1])?;

        let allocation_flags = cursor.read_u32::<LittleEndian>()?;
        let element_count = cursor.read_i32::<LittleEndian>()?;

        let mut map = HashMap::new();
        for _ in 0..element_count {
            let key_stack_entry = ScopedStackEntry::new(properties_stack, "Key".to_string());
            let key = Property::new(cursor, hints, properties_stack, &key_type, false, None)?;
            drop(key_stack_entry);

            let value_stack_entry = ScopedStackEntry::new(properties_stack, "Value".to_string());
            let value = Property::new(cursor, hints, properties_stack, &value_type, false, None)?;
            drop(value_stack_entry);

            map.insert(key, value);
        }

        Ok(MapProperty {
            key_type,
            value_type,
            allocation_flags,
            value: map,
        })
    }
}

impl PropertyTrait for MapProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if !include_header {
            panic!("Nested maps are not supported");
        }

        cursor.write_string("MapProperty")?;

        let begin = cursor.position();
        cursor.write_u64::<LittleEndian>(0)?;

        cursor.write_string(&self.key_type)?;
        cursor.write_string(&self.value_type)?;

        cursor.write_all(&[0u8; 1])?;

        cursor.write_u32::<LittleEndian>(self.allocation_flags)?;
        cursor.write_i32::<LittleEndian>(self.value.len() as i32)?;

        let write_begin = cursor.position();

        for (key, value) in &self.value {
            key.write(cursor, false)?;
            value.write(cursor, false)?;
        }

        let end = cursor.position();

        cursor.seek(SeekFrom::Start(begin))?;
        cursor.write_u64::<LittleEndian>(end - write_begin + 8)?;
        cursor.seek(SeekFrom::Start(end))?;

        Ok(())
    }
}

impl PartialEq for MapProperty {
    fn eq(&self, other: &Self) -> bool {
        self.key_type == other.key_type && self.value_type == other.value_type
    }
}

impl Eq for MapProperty {}

impl Hash for MapProperty {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key_type.hash(state);
        self.value_type.hash(state);
        self.allocation_flags.hash(state);
    }
}
