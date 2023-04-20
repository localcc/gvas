use std::io::{Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::{DeserializeError, Error},
};

use super::PropertyTrait;

/// A property that holds an enum value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EnumProperty {
    enum_type: String,
    value: String,
}

impl EnumProperty {
    /// Creates a new `EnumProperty` instance.
    pub fn new(enum_type: String, value: String) -> Self {
        EnumProperty { enum_type, value }
    }

    pub(crate) fn read<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let _length = cursor.read_u64::<LittleEndian>()?;

        let enum_type = cursor.read_string()?;

        let indicator = cursor.read_u8()?;
        if indicator != 0 {
            Err(DeserializeError::invalid_property(
                &format!("Unexpected indicator value {}", indicator),
                cursor,
            ))?
        }
        assert_eq!(indicator, 0);

        let value = cursor.read_string()?;

        Ok(EnumProperty { enum_type, value })
    }
}

impl PropertyTrait for EnumProperty {
    fn write<W: Write + Seek>(&self, cursor: &mut W, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string("EnumProperty")?;
        }

        let begin = cursor.stream_position()?;
        cursor.write_u64::<LittleEndian>(0)?;

        cursor.write_string(&self.enum_type)?;
        cursor.write_all(&[0u8; 1])?;

        let value_begin = cursor.stream_position()?;
        cursor.write_string(&self.value)?;
        let value_end = cursor.stream_position()?;

        cursor.seek(SeekFrom::Start(begin))?;
        cursor.write_u64::<LittleEndian>(value_end - value_begin)?;
        cursor.seek(SeekFrom::Start(value_end))?;

        Ok(())
    }
}
