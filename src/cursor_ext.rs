use std::io::{Read, Seek, Write};

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
}

/// Extensions for `Write`.
pub trait WriteExt {
    /// Writes a GVAS string.
    fn write_string(&mut self, v: &str) -> Result<usize, Error>;
    /// Writes a GUID.
    fn write_guid(&mut self, v: &Guid) -> Result<(), Error>;
}

impl<R: Read + Seek> ReadExt for R {
    fn read_string(&mut self) -> Result<String, Error> {
        match self.read_fstring()? {
            Some(str) => Ok(str),
            None => Err(DeserializeError::InvalidString(0, self.stream_position()?))?,
        }
    }

    fn read_guid(&mut self) -> Result<Guid, Error> {
        let mut guid = Guid::default();
        self.read_exact(&mut guid.0)?;
        Ok(guid)
    }
}

impl<W: Write> WriteExt for W {
    fn write_string(&mut self, v: &str) -> Result<usize, Error> {
        Ok(self.write_fstring(Some(v))?)
    }

    fn write_guid(&mut self, v: &Guid) -> Result<(), Error> {
        Ok(self.write_all(&v.0)?)
    }
}
