use std::io::{Cursor, Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::Error,
};

use super::{PropertyOptions, PropertyTrait, impl_read, impl_read_header, impl_write};

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
    pub(crate) fn write<W: Write>(&self, cursor: &mut W) -> Result<usize, Error> {
        let mut len = 4;
        cursor.write_u32::<LittleEndian>(self.path.len() as u32)?;

        for path_part in &self.path {
            len += cursor.write_string(path_part)?;
        }

        len += cursor.write_string(&self.resolved_owner)?;

        Ok(len)
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

    #[inline]
    fn read_body<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let value = FieldPath::read(cursor)?;

        Ok(FieldPathProperty { value })
    }
}

impl PropertyTrait for FieldPathProperty {
    impl_write!(FieldPathProperty);

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        _: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        let len = self.value.write(cursor)?;
        Ok(len)
    }
}
