use std::io::{Cursor, Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::Error,
};

use super::{impl_read, impl_read_header, PropertyOptions, PropertyTrait};

/// Field path
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FieldPath {
    /// Path
    pub path: Vec<String>,
    /// Resolved owner
    pub resolved_owner: String,
}

impl FieldPath {
    /// Creates a new `FieldPath` instance
    #[inline]
    pub fn new(path: Vec<String>, resolved_owner: String) -> Self {
        FieldPath {
            path,
            resolved_owner,
        }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let path_len = cursor.read_u32::<LittleEndian>()?;
        let mut path = Vec::with_capacity(path_len as usize);
        for _ in 0..path_len {
            path.push(cursor.read_string()?);
        }

        let resolved_owner = cursor.read_string()?;

        Ok(FieldPath {
            path,
            resolved_owner,
        })
    }

    #[inline]
    pub(crate) fn write<W: Write>(&self, cursor: &mut W) -> Result<(), Error> {
        cursor.write_u32::<LittleEndian>(self.path.len() as u32)?;

        for path_part in &self.path {
            cursor.write_string(path_part)?;
        }

        cursor.write_string(&self.resolved_owner)?;

        Ok(())
    }
}

/// Field path property
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FieldPathProperty {
    /// Field path
    pub value: FieldPath,
}

impl FieldPathProperty {
    /// Creates a new `FieldPathProperty` instance
    #[inline]
    pub fn new(value: FieldPath) -> Self {
        FieldPathProperty { value }
    }

    impl_read!();
    impl_read_header!();

    fn read_body<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let value = FieldPath::read(cursor)?;

        Ok(FieldPathProperty { value })
    }

    #[inline]
    fn write_body<W: Write>(&self, cursor: &mut W) -> Result<(), Error> {
        self.value.write(cursor)?;
        Ok(())
    }
}

impl PropertyTrait for FieldPathProperty {
    #[inline]
    fn write<W: Write>(
        &self,
        cursor: &mut W,
        include_header: bool,
        _options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        if !include_header {
            return self.write_body(cursor);
        }

        let buf = &mut Cursor::new(Vec::new());
        self.write_body(buf)?;
        let buf = buf.get_ref();

        cursor.write_string("FieldPathProperty")?;
        cursor.write_u64::<LittleEndian>(buf.len() as u64)?;
        cursor.write_u8(0)?;
        cursor.write_all(buf)?;

        Ok(())
    }
}
