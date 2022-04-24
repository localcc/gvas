use std::io::{Cursor, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{error::Error, Guid};


pub trait CursorExt {
    fn read_string(&mut self) -> Result<String, Error>;
    fn write_string(&mut self, v: &String) -> Result<(), Error>;
}

impl CursorExt for Cursor<Vec<u8>> {
    fn read_string(&mut self) -> Result<String, Error> {
        let len = self.read_i32::<LittleEndian>()?;
        let mut bytes = vec![0u8; (len - 1) as usize];
        self.read_exact(&mut bytes)?;
        self.read_exact(&mut [0u8; 1])?; // null-byte terminator

        Ok(String::from_utf8(bytes)?)
    }

    fn write_string(&mut self, v: &String) -> Result<(), Error> {
        let len = v.len() + 1;
        self.write_i32::<LittleEndian>(len as i32)?;
        self.write(v.as_bytes())?;
        self.write(&[0u8; 1])?;
        Ok(())
    }
}
