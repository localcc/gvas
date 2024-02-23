use std::io::{Read, Seek, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    error::{DeserializeError, Error},
    types::Guid,
};

/// Extensions for `Read`.
pub trait ReadExt {
    /// Reads a GVAS string.
    fn read_string(&mut self) -> Result<String, Error>;
    /// Reads a GVAS string.
    fn read_fstring(&mut self) -> Result<Option<String>, Error>;
    /// Reads a GUID.
    fn read_guid(&mut self) -> Result<Guid, Error>;
    /// Reads an 8bit boolean value.
    fn read_bool(&mut self) -> Result<bool, Error>;
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
    /// Writes a GVAS string.
    fn write_fstring(&mut self, v: Option<&str>) -> Result<usize, Error>;
    /// Writes a GUID.
    fn write_guid(&mut self, v: &Guid) -> Result<(), Error>;
    /// Writes an 8bit boolean value.
    fn write_bool(&mut self, v: bool) -> Result<(), Error>;
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

    fn read_fstring(&mut self) -> Result<Option<String>, Error> {
        let start_position = self.stream_position()?;
        let len = self.read_i32::<LittleEndian>()?;

        if !(-131072..=131072).contains(&len) {
            Err(DeserializeError::InvalidString(
                len,
                self.stream_position()?,
            ))?
        } else if len == 0 {
            Ok(None)
        } else if len < 0 {
            let mut buf = vec![0u16; -len as usize - 1];
            self.read_u16_into::<LittleEndian>(&mut buf)?;

            let terminator = self.read_u16::<LittleEndian>()?;
            if terminator != 0 {
                Err(DeserializeError::InvalidStringTerminator(
                    terminator,
                    self.stream_position()?,
                ))?
            }

            let string = String::from_utf16(&buf[..])
                .map_err(|e| DeserializeError::FromUtf16Error(e, start_position))?;

            Ok(Some(string))
        } else {
            let mut buf = vec![0u8; len as usize - 1];
            self.read_exact(&mut buf)?;

            let terminator = self.read_u8()?;
            if terminator != 0 {
                Err(DeserializeError::InvalidStringTerminator(
                    terminator as u16,
                    self.stream_position()?,
                ))?
            }

            let string = String::from_utf8(buf)
                .map_err(|e| DeserializeError::FromUtf8Error(e, start_position))?;

            Ok(Some(string))
        }
    }

    #[inline]
    fn read_guid(&mut self) -> Result<Guid, Error> {
        let mut guid = Guid::default();
        self.read_exact(&mut guid.0)?;
        Ok(guid)
    }

    #[inline]
    fn read_bool(&mut self) -> Result<bool, Error> {
        match self.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            value => Err(DeserializeError::InvalidBoolean(
                value as u32,
                self.stream_position()?,
            ))?,
        }
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
            let name = std::any::type_name::<T>();
            DeserializeError::invalid_enum_value(name, value, self)
        })?;
        Ok(result)
    }
}

impl<W: Write> WriteExt for W {
    #[inline]
    fn write_string<T: AsRef<str>>(&mut self, v: T) -> Result<usize, Error> {
        let v = v.as_ref();
        if v.is_ascii() {
            // ASCII strings do not require encoding
            let len = v.len() + 1;
            self.write_i32::<LittleEndian>(len as i32)?;
            let _ = self.write(v.as_bytes())?;
            let _ = self.write(&[0u8; 1])?;
            Ok(len * 2 + 4)
        } else {
            // Perform UTF-16 encoding when non-ASCII characters are detected
            let words: Vec<u16> = v.encode_utf16().collect();
            let len = words.len() + 1;
            self.write_i32::<LittleEndian>(-(len as i32))?;
            for word in words {
                self.write_u16::<LittleEndian>(word)?;
            }
            self.write_u16::<LittleEndian>(0u16)?;
            Ok(len * 2 + 4)
        }
    }

    fn write_fstring(&mut self, v: Option<&str>) -> Result<usize, Error> {
        match v {
            Some(str) => self.write_string(str),
            None => {
                self.write_i32::<LittleEndian>(0)?;
                Ok(4)
            }
        }
    }

    #[inline]
    fn write_guid(&mut self, v: &Guid) -> Result<(), Error> {
        Ok(self.write_all(&v.0)?)
    }

    #[inline]
    fn write_bool(&mut self, v: bool) -> Result<(), Error> {
        Ok(self.write_u8(if v { 1 } else { 0 })?)
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
