use std::io::{Cursor, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::error::{DeserializeError, Error};

pub trait CursorExt {
    fn read_string(&mut self) -> Result<String, Error>;
    fn read_string_opt(&mut self) -> Result<Option<String>, Error>;
    fn write_string(&mut self, v: &str) -> Result<(), Error>;
    fn write_string_opt(&mut self, v: Option<&str>) -> Result<(), Error>;
}

impl CursorExt for Cursor<Vec<u8>> {
    fn read_string(&mut self) -> Result<String, Error> {
        match CursorExt::read_string_opt(self)? {
            Some(str) => Ok(str),
            None => Err(DeserializeError::InvalidString(0, self.position()))?,
        }
    }

    fn read_string_opt(&mut self) -> Result<Option<String>, Error> {
        let len = self.read_i32::<LittleEndian>()?;

        if !(-131072..=131072).contains(&len) {
            Err(DeserializeError::InvalidString(len, self.position()))?
        } else if len == 0 {
            Ok(None)
        } else if len < 0 {
            let mut buf = vec![0u16; -len as usize - 1];
            self.read_u16_into::<LittleEndian>(&mut buf)?;

            let terminator = self.read_u16::<LittleEndian>()?;
            if terminator != 0 {
                Err(DeserializeError::InvalidStringTermination(
                    terminator,
                    self.position(),
                ))?
            }

            let string = String::from_utf16(&buf[..])?;

            Ok(Some(string))
        } else {
            let mut buf = vec![0u8; len as usize - 1];
            self.read_exact(&mut buf)?;

            let terminator = self.read_u8()?;
            if terminator != 0 {
                Err(DeserializeError::InvalidStringTermination(
                    terminator as u16,
                    self.position(),
                ))?
            }

            let string = String::from_utf8(buf)?;

            Ok(Some(string))
        }
    }

    fn write_string(&mut self, v: &str) -> Result<(), Error> {
        if v.is_ascii() {
            // ASCII strings do not require encoding
            let len = v.len() + 1;
            self.write_i32::<LittleEndian>(len as i32)?;
            let _ = self.write(v.as_bytes())?;
            let _ = self.write(&[0u8; 1])?;
        } else {
            // Perform UTF-16 encoding when non-ASCII characters are detected
            let words: Vec<u16> = v.encode_utf16().collect();
            let len = words.len() + 1;
            self.write_i32::<LittleEndian>(-(len as i32))?;
            for word in words {
                self.write_u16::<LittleEndian>(word)?;
            }
            self.write_u16::<LittleEndian>(0u16)?;
        }
        Ok(())
    }

    fn write_string_opt(&mut self, v: Option<&str>) -> Result<(), Error> {
        match v {
            Some(str) => self.write_string(str),
            None => {
                self.write_i32::<LittleEndian>(0)?;
                Ok(())
            }
        }
    }
}
