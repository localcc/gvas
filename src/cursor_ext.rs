use std::io::{Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use unreal_helpers::{UnrealReadExt, UnrealWriteExt};

use crate::{
    error::{DeserializeError, Error},
    types::Guid,
};

/// Extensions for `Read`.
pub trait ReadExt {
    /// Reads a GVAS string.
    fn read_string(&mut self) -> Result<String, Error>;
    /// Reads a GUID.
    fn read_guid(&mut self) -> Result<Guid, Error>;
    /// Reads a 32bit boolean value.
    fn read_b32(&mut self) -> Result<bool, Error>;
    /// Reads an 8bit enum value.
    fn read_enum<T>(&mut self) -> Result<T, Error>
    where
        T: TryFrom<i8>;
}

/// Extensions for `Write`.
pub trait WriteExt {
    /// Writes a GVAS string.
    fn write_string<T: AsRef<str>>(&mut self, v: T) -> Result<usize, Error>;
    /// Writes a GUID.
    fn write_guid(&mut self, v: &Guid) -> Result<(), Error>;
    /// Writes a 32bit boolean value.
    fn write_b32(&mut self, v: bool) -> Result<(), Error>;
    /// Writes an 8bit enum value.
    fn write_enum<T>(&mut self, v: T) -> Result<(), Error>
    where
        T: Into<i8> + std::fmt::Debug;
}

impl<R: Read + Seek> ReadExt for R {
    #[inline]
    fn read_string(&mut self) -> Result<String, Error> {
        match self.read_fstring()? {
            Some(str) => Ok(str),
            None => Err(DeserializeError::InvalidString(0, self.stream_position()?))?,
        }
    }

    #[inline]
    fn read_guid(&mut self) -> Result<Guid, Error> {
        let mut guid = Guid::default();
        self.read_exact(&mut guid.0)?;
        Ok(guid)
    }

    #[inline]
    fn read_b32(&mut self) -> Result<bool, Error> {
        match self.read_u32::<LittleEndian>()? {
            0 => Ok(false),
            1 => Ok(true),
            value => Err(DeserializeError::InvalidBoolean(
                value,
                self.stream_position()?,
            ))?,
        }
    }

    #[inline]
    fn read_enum<T>(&mut self) -> Result<T, Error>
    where
        T: TryFrom<i8>,
    {
        let value = self.read_i8()?;
        let result = T::try_from(value).map_err(|_| {
            DeserializeError::InvalidEnumValue(
                format!(
                    "No discriminant in enum `{}` matches the value `{}`",
                    std::any::type_name::<T>(),
                    value,
                )
                .into_boxed_str(),
            )
        })?;
        Ok(result)
    }
}

impl<W: Write> WriteExt for W {
    #[inline]
    fn write_string<T: AsRef<str>>(&mut self, v: T) -> Result<usize, Error> {
        Ok(self.write_fstring(Some(v.as_ref()))?)
    }

    #[inline]
    fn write_guid(&mut self, v: &Guid) -> Result<(), Error> {
        Ok(self.write_all(&v.0)?)
    }

    #[inline]
    fn write_b32(&mut self, v: bool) -> Result<(), Error> {
        Ok(self.write_u32::<LittleEndian>(if v { 1 } else { 0 })?)
    }

    #[inline]
    fn write_enum<T>(&mut self, v: T) -> Result<(), Error>
    where
        T: std::fmt::Debug + Into<i8>,
    {
        Ok(self.write_i8(v.into())?)
    }
}
