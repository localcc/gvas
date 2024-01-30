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
}

/// Extensions for `Write`.
pub trait WriteExt {
    /// Writes a GVAS string.
    fn write_string(&mut self, v: &str) -> Result<usize, Error>;
    /// Writes a GUID.
    fn write_guid(&mut self, v: &Guid) -> Result<(), Error>;
    /// Writes a 32bit boolean value.
    fn write_b32(&mut self, v: bool) -> Result<(), Error>;
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
}

impl<W: Write> WriteExt for W {
    #[inline]
    fn write_string(&mut self, v: &str) -> Result<usize, Error> {
        Ok(self.write_fstring(Some(v))?)
    }

    #[inline]
    fn write_guid(&mut self, v: &Guid) -> Result<(), Error> {
        Ok(self.write_all(&v.0)?)
    }

    fn write_b32(&mut self, v: bool) -> Result<(), Error> {
        Ok(self.write_u32::<LittleEndian>(if v { 1 } else { 0 })?)
    }
}
