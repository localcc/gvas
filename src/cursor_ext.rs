use std::io::Cursor;

use unreal_helpers::{UnrealReadExt, UnrealWriteExt};

use crate::error::{DeserializeError, Error};

/// Extensions for `Cursor`
pub trait CursorExt {
    /// Reads a GVAS string.
    fn read_string(&mut self) -> Result<String, Error>;
    /// Writes a GVAS string.
    fn write_string(&mut self, v: &str) -> Result<(), Error>;
}

impl CursorExt for Cursor<Vec<u8>> {
    fn read_string(&mut self) -> Result<String, Error> {
        match self.read_fstring()? {
            Some(str) => Ok(str),
            None => Err(DeserializeError::InvalidString(0, self.position()))?,
        }
    }

    fn write_string(&mut self, v: &str) -> Result<(), Error> {
        self.write_fstring(Some(v))?;
        Ok(())
    }
}
