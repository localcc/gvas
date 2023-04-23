use std::{
    collections::HashMap,
    hash::Hash,
    io::{Cursor, Read, Seek, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use indexmap::IndexMap;

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::{Error, SerializeError},
    scoped_stack_entry::ScopedStackEntry,
};

use super::{Property, PropertyTrait};

/// A property that stores a map of properties to properties.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MapProperty {
    /// Key type name.
    pub key_type: String,
    /// Value type name.
    pub value_type: String,
    /// Allocation flags.
    pub allocation_flags: u32,
    /// Map entries.
    #[cfg_attr(feature = "serde", serde(with = "indexmap::serde_seq"))]
    pub value: IndexMap<Property, Property>,
}

impl MapProperty {
    /// Creates a new `MapProperty` instance.
    #[inline]
    pub fn new(
        key_type: String,
        value_type: String,
        allocation_flags: u32,
        value: IndexMap<Property, Property>,
    ) -> Self {
        MapProperty {
            key_type,
            value_type,
            allocation_flags,
            value,
        }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        hints: &HashMap<String, String>,
        properties_stack: &mut Vec<String>,
    ) -> Result<Self, Error> {
        let _length = cursor.read_u64::<LittleEndian>()?;

        let key_type = cursor.read_string()?;
        let value_type = cursor.read_string()?;

        let separator = cursor.read_u8()?;
        assert_eq!(separator, 0);

        let allocation_flags = cursor.read_u32::<LittleEndian>()?;
        let element_count = cursor.read_u32::<LittleEndian>()?;

        let mut map = IndexMap::new();
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
    #[inline]
    fn write<W: Write>(&self, cursor: &mut W, include_header: bool) -> Result<(), Error> {
        if !include_header {
            return Err(SerializeError::invalid_value(
                "Nested maps are not supported",
            ))?;
        }

        let buf = &mut Cursor::new(Vec::new());
        self.write_body(buf)?;
        let buf = buf.get_ref();

        cursor.write_string("MapProperty")?;
        cursor.write_u64::<LittleEndian>(buf.len() as u64)?;
        cursor.write_string(&self.key_type)?;
        cursor.write_string(&self.value_type)?;
        cursor.write_u8(0)?;
        cursor.write_all(buf)?;

        Ok(())
    }
}

impl MapProperty {
    fn write_body<W: Write + Seek>(&self, cursor: &mut W) -> Result<(), Error> {
        cursor.write_u32::<LittleEndian>(self.allocation_flags)?;
        cursor.write_u32::<LittleEndian>(self.value.len() as u32)?;

        for (key, value) in &self.value {
            key.write(cursor, false)?;
            value.write(cursor, false)?;
        }

        Ok(())
    }
}

impl Hash for MapProperty {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key_type.hash(state);
        self.value_type.hash(state);
        self.allocation_flags.hash(state);
    }
}
