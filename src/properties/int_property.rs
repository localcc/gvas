use std::io::{Cursor, Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::CursorExt,
    error::{DeserializeError, Error, SerializeError},
};

use super::PropertyTrait;

macro_rules! check_size {
    ($cursor:ident, $expected:literal) => {
        let value_size = $cursor.read_u64::<LittleEndian>()?;
        if value_size != $expected {
            return Err(DeserializeError::InvalidValueSize($expected, value_size).into());
        }
    };
}

macro_rules! impl_int_property {
    ($name:ident, $ty:ty, $read_method:ident, $write_method:ident, $size:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            pub value: $ty,
        }

        impl $name {
            pub fn new(value: $ty) -> Self {
                $name { value }
            }

            pub fn read(cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<Self, Error> {
                if include_header {
                    check_size!(cursor, $size);
                    cursor.read_exact(&mut [0u8; 1])?;
                }
                Ok(Self {
                    value: cursor.$read_method::<LittleEndian>()?,
                })
            }
        }

        impl PropertyTrait for $name {
            fn write(
                &self,
                cursor: &mut Cursor<Vec<u8>>,
                include_header: bool,
            ) -> Result<(), Error> {
                if include_header {
                    cursor.write_string(&String::from(stringify!($name)))?;
                    cursor.write_i64::<LittleEndian>($size)?;
                    let _ = cursor.write(&[0u8; 1])?;
                }
                cursor.$write_method::<LittleEndian>(self.value)?;
                Ok(())
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Int8Property {
    pub value: i8,
}

impl Int8Property {
    pub fn new(value: i8) -> Self {
        Int8Property { value }
    }

    pub fn read(cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<Self, Error> {
        if include_header {
            check_size!(cursor, 1);
            cursor.read_exact(&mut [0u8; 1])?;
        }
        Ok(Int8Property {
            value: cursor.read_i8()?,
        })
    }
}

impl PropertyTrait for Int8Property {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string(&String::from("Int8Property"))?;
            cursor.write_i64::<LittleEndian>(1)?;
            let _ = cursor.write(&[0u8; 1])?;
        }
        cursor.write_i8(self.value)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteProperty {
    pub name: Option<String>,
    pub value: u8,
}

impl ByteProperty {
    pub fn new(name: Option<String>, value: u8) -> Self {
        ByteProperty { name, value }
    }

    pub fn read(cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<Self, Error> {
        let mut name = None;
        if include_header {
            check_size!(cursor, 1);
            name = Some(cursor.read_string()?);
            cursor.read_exact(&mut [0u8; 1])?;
        }
        Ok(ByteProperty {
            name,
            value: cursor.read_u8()?,
        })
    }
}

impl PropertyTrait for ByteProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string(&String::from("ByteProperty"))?;
            cursor.write_i64::<LittleEndian>(1)?;
            cursor.write_string(self.name.as_ref().ok_or_else(|| {
                SerializeError::InvalidValue(String::from("self.name None expected Some(...)"))
            })?)?;
            let _ = cursor.write(&[0u8; 1])?;
        }
        cursor.write_u8(self.value)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoolProperty {
    pub value: bool,
}

impl BoolProperty {
    pub fn new(value: bool) -> Self {
        BoolProperty { value }
    }

    pub fn read(cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<Self, Error> {
        if include_header {
            check_size!(cursor, 0);
        }
        let val = cursor.read_i16::<LittleEndian>()?;
        Ok(BoolProperty { value: val > 0 })
    }
}

impl PropertyTrait for BoolProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string(&String::from("BoolProperty"))?;
            cursor.write_i64::<LittleEndian>(0)?;
        }
        cursor.write_i16::<LittleEndian>(match self.value {
            true => 1,
            false => 0,
        })?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FloatProperty {
    pub value: f32,
}

impl FloatProperty {
    pub fn new(value: f32) -> Self {
        FloatProperty { value }
    }

    pub fn read(cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<Self, Error> {
        if include_header {
            check_size!(cursor, 4);
            cursor.read_exact(&mut [0u8; 1])?;
        }
        Ok(Self {
            value: cursor.read_f32::<LittleEndian>()?,
        })
    }
}

impl PropertyTrait for FloatProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string(&String::from("FloatProperty"))?;
            cursor.write_i64::<LittleEndian>(4)?;
            let _ = cursor.write(&[0u8; 1])?;
        }
        cursor.write_f32::<LittleEndian>(self.value)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoubleProperty {
    pub value: f64,
}

impl DoubleProperty {
    pub fn new(value: f64) -> Self {
        DoubleProperty { value }
    }

    pub fn read(cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<Self, Error> {
        if include_header {
            check_size!(cursor, 8);
            cursor.read_exact(&mut [0u8; 1])?;
        }
        Ok(Self {
            value: cursor.read_f64::<LittleEndian>()?,
        })
    }
}

impl PropertyTrait for DoubleProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string(&String::from("DoubleProperty"))?;
            cursor.write_i64::<LittleEndian>(8)?;
            let _ = cursor.write(&[0u8; 1])?;
        }
        cursor.write_f64::<LittleEndian>(self.value)?;
        Ok(())
    }
}

impl_int_property!(Int16Property, i16, read_i16, write_i16, 2);
impl_int_property!(UInt16Property, u16, read_u16, write_u16, 2);
impl_int_property!(IntProperty, i32, read_i32, write_i32, 4);
impl_int_property!(UInt32Property, u32, read_u32, write_u32, 4);
impl_int_property!(Int64Property, i64, read_i64, write_i64, 8);
impl_int_property!(UInt64Property, u64, read_u64, write_u64, 8);
