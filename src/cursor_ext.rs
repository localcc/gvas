use std::io::{Read, Seek, Write};

use unreal_helpers::{UnrealReadExt, UnrealWriteExt};

use crate::error::{DeserializeError, Error};

/// Extensions for `Read`.
pub trait ReadExt {
    /// Reads a GVAS string.
    fn read_string(&mut self) -> Result<String, Error>;
}

/// Extensions for `Write`.
pub trait WriteExt {
    /// Writes a GVAS string.
    fn write_string(&mut self, v: &str) -> Result<usize, Error>;
}

impl<R: Read + Seek> ReadExt for R {
    fn read_string(&mut self) -> Result<String, Error> {
        match self.read_fstring()? {
            Some(str) => Ok(str),
            None => Err(DeserializeError::InvalidString(0, self.stream_position()?))?,
        }
    }
}

impl<W: Write> WriteExt for W {
    fn write_string(&mut self, v: &str) -> Result<usize, Error> {
        Ok(self.write_fstring(Some(v))?)
    }
}
