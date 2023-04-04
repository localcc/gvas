use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::CursorExt,
    error::{Error, SerializeError},
};

use super::PropertyTrait;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumProperty {
    enum_type: String,
    value: String,
    compact_name: bool,
}

impl EnumProperty {
    pub fn new(enum_type: String, value: String, compact_name: bool) -> Self {
        EnumProperty {
            enum_type,
            value,
            compact_name,
        }
    }

    pub(crate) fn read(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let _length = cursor.read_u64::<LittleEndian>()?;

        let read_enum_type = cursor.read_string()?;

        let compact_name = read_enum_type.contains("::");

        let mut enum_type = read_enum_type.clone();
        let value;
        if compact_name {
            let mut split = read_enum_type.split("::");
            if let Some(e) = split.next() {
                enum_type = e.to_string();
            } else {
                return Err(Error::from(SerializeError::InvalidValue(read_enum_type)));
            }
            if let Some(e) = split.next() {
                value = e.to_string();
            } else {
                return Err(Error::from(SerializeError::InvalidValue(read_enum_type)));
            }
        } else {
            cursor.read_exact(&mut [0u8; 1])?;
            value = cursor.read_string()?;
        }

        Ok(EnumProperty {
            enum_type,
            value,
            compact_name,
        })
    }
}

impl PropertyTrait for EnumProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string("EnumProperty")?;
        }

        let begin = cursor.position();
        cursor.write_u64::<LittleEndian>(0)?;

        if self.compact_name {
            //fixme: write compact name length
            cursor.write_string(&format!("{}::{}", self.enum_type, self.value))?;
        } else {
            cursor.write_string(&self.enum_type)?;
            cursor.write_all(&[0u8; 1])?;

            let value_begin = cursor.position();
            cursor.write_string(&self.value)?;
            let value_end = cursor.position();

            cursor.seek(SeekFrom::Start(begin))?;
            cursor.write_u64::<LittleEndian>(value_end - value_begin)?;
            cursor.seek(SeekFrom::Start(value_end))?;
        }

        Ok(())
    }
}
