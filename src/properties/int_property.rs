use std::{
    fmt::Debug,
    io::{Cursor, Read, Seek, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use ordered_float::OrderedFloat;
use unreal_helpers::{UnrealReadExt, UnrealWriteExt};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::{DeserializeError, Error},
};

use super::PropertyTrait;

macro_rules! check_size {
    ($cursor:ident, $expected:literal) => {
        let value_size = $cursor.read_u64::<LittleEndian>()?;
        if value_size != $expected {
            Err(DeserializeError::InvalidValueSize(
                $expected,
                value_size,
                $cursor.stream_position()?,
            ))?
        }
    };
}

macro_rules! impl_int_property {
    ($name:ident, $ty:ty, $read_method:ident, $write_method:ident, $size:literal) => {
        #[doc = "A property that stores a `"]
        #[doc = stringify!($ty)]
        #[doc = "`."]
        #[derive(Clone, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name {
            /// Integer value.
            pub value: $ty,
        }

        impl $name {
            #[doc = "Creates a new `"]
            #[doc = stringify!($name)]
            #[doc = "` instance."]
            #[inline]
            pub fn new(value: $ty) -> Self {
                $name { value }
            }

            #[inline]
            pub(crate) fn read<R: Read + Seek>(
                cursor: &mut R,
                include_header: bool,
            ) -> Result<Self, Error> {
                if include_header {
                    check_size!(cursor, $size);
                    let separator = cursor.read_u8()?;
                    assert_eq!(separator, 0);
                }
                Ok(Self {
                    value: cursor.$read_method::<LittleEndian>()?,
                })
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}{}", self.value, stringify!($ty))
            }
        }

        impl PropertyTrait for $name {
            #[inline]
            fn write<W: Write>(&self, cursor: &mut W, include_header: bool) -> Result<(), Error> {
                if !include_header {
                    return self.write_body(cursor);
                }

                let buf = &mut Cursor::new(Vec::new());
                self.write_body(buf)?;
                let buf = buf.get_ref();

                cursor.write_string(stringify!($name))?;
                cursor.write_u64::<LittleEndian>(buf.len() as u64)?;
                cursor.write_u8(0)?;
                cursor.write_all(buf)?;

                Ok(())
            }
        }

        impl $name {
            #[inline]
            fn write_body<W: Write>(&self, cursor: &mut W) -> Result<(), Error> {
                cursor.$write_method::<LittleEndian>(self.value)?;

                Ok(())
            }
        }
    };
}

/// A property that stores a `i8`.
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Int8Property {
    /// Integer value.
    pub value: i8,
}

impl Int8Property {
    /// Creates a new `Int8Property` instance.
    #[inline]
    pub fn new(value: i8) -> Self {
        Int8Property { value }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        include_header: bool,
    ) -> Result<Self, Error> {
        if include_header {
            check_size!(cursor, 1);
            let separator = cursor.read_u8()?;
            assert_eq!(separator, 0);
        }
        Ok(Int8Property {
            value: cursor.read_i8()?,
        })
    }
}

impl Debug for Int8Property {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}i8", self.value)
    }
}

impl PropertyTrait for Int8Property {
    #[inline]
    fn write<W: Write>(&self, cursor: &mut W, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string("Int8Property")?;
            cursor.write_u64::<LittleEndian>(1)?;
            cursor.write_u8(0)?;
        }
        cursor.write_i8(self.value)?;
        Ok(())
    }
}

/// A property that stores a `u8`.
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ByteProperty {
    /// Property name.
    pub name: Option<String>,
    /// Integer value.
    pub value: u8,
}

impl ByteProperty {
    /// Creates a new `ByteProperty` instance.
    #[inline]
    pub fn new(name: Option<String>, value: u8) -> Self {
        ByteProperty { name, value }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        include_header: bool,
    ) -> Result<Self, Error> {
        let mut name = None;
        if include_header {
            check_size!(cursor, 1);
            name = Some(cursor.read_string()?);
            let separator = cursor.read_u8()?;
            assert_eq!(separator, 0);
        }
        Ok(ByteProperty {
            name,
            value: cursor.read_u8()?,
        })
    }
}

impl Debug for ByteProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}u8", self.value)
    }
}

impl PropertyTrait for ByteProperty {
    #[inline]
    fn write<W: Write>(&self, cursor: &mut W, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string("ByteProperty")?;
            cursor.write_u64::<LittleEndian>(1)?;
            cursor.write_fstring(self.name.as_deref())?;
            cursor.write_u8(0)?;
        }
        cursor.write_u8(self.value)?;
        Ok(())
    }
}

/// A property that stores a `bool`.
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BoolProperty {
    /// Boolean value.
    pub value: bool,
}

impl BoolProperty {
    /// Creates a new `BoolProperty` instance.
    #[inline]
    pub fn new(value: bool) -> Self {
        BoolProperty { value }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        include_header: bool,
    ) -> Result<Self, Error> {
        if include_header {
            check_size!(cursor, 0);
        }
        let value = cursor.read_bool()?;
        if include_header {
            let indicator = cursor.read_u8()?;
            assert_eq!(indicator, 0);
        }
        Ok(BoolProperty { value })
    }
}

impl Debug for BoolProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PropertyTrait for BoolProperty {
    #[inline]
    fn write<W: Write>(&self, cursor: &mut W, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string("BoolProperty")?;
            cursor.write_u64::<LittleEndian>(0)?;
        }
        cursor.write_bool(self.value)?;
        if include_header {
            cursor.write_u8(0)?;
        }
        Ok(())
    }
}

/// A property that stores a `f32`.
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FloatProperty {
    /// Integer value.
    pub value: OrderedFloat<f32>,
}

impl FloatProperty {
    /// Creates a new `FloatProperty` instance.
    #[inline]
    pub fn new(value: f32) -> Self {
        FloatProperty {
            value: OrderedFloat(value),
        }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        include_header: bool,
    ) -> Result<Self, Error> {
        if include_header {
            check_size!(cursor, 4);
            let separator = cursor.read_u8()?;
            assert_eq!(separator, 0);
        }
        Ok(Self {
            value: OrderedFloat(cursor.read_f32::<LittleEndian>()?),
        })
    }
}

impl Debug for FloatProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}f32", self.value)
    }
}

impl PropertyTrait for FloatProperty {
    #[inline]
    fn write<W: Write>(&self, cursor: &mut W, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string("FloatProperty")?;
            cursor.write_u64::<LittleEndian>(4)?;
            cursor.write_u8(0)?;
        }
        cursor.write_f32::<LittleEndian>(self.value.0)?;
        Ok(())
    }
}

/// A property that stores a `f64`.
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DoubleProperty {
    /// Integer value.
    pub value: OrderedFloat<f64>,
}

impl DoubleProperty {
    /// Creates a new `DoubleProperty` instance.
    #[inline]
    pub fn new(value: f64) -> Self {
        DoubleProperty {
            value: OrderedFloat(value),
        }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        include_header: bool,
    ) -> Result<Self, Error> {
        if include_header {
            check_size!(cursor, 8);
            let separator = cursor.read_u8()?;
            assert_eq!(separator, 0);
        }
        Ok(Self {
            value: OrderedFloat(cursor.read_f64::<LittleEndian>()?),
        })
    }
}

impl Debug for DoubleProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}f64", self.value)
    }
}

impl PropertyTrait for DoubleProperty {
    #[inline]
    fn write<W: Write>(&self, cursor: &mut W, include_header: bool) -> Result<(), Error> {
        if include_header {
            cursor.write_string("DoubleProperty")?;
            cursor.write_u64::<LittleEndian>(8)?;
            cursor.write_u8(0)?;
        }
        cursor.write_f64::<LittleEndian>(self.value.0)?;
        Ok(())
    }
}

impl_int_property!(Int16Property, i16, read_i16, write_i16, 2);
impl_int_property!(UInt16Property, u16, read_u16, write_u16, 2);
impl_int_property!(IntProperty, i32, read_i32, write_i32, 4);
impl_int_property!(UInt32Property, u32, read_u32, write_u32, 4);
impl_int_property!(Int64Property, i64, read_i64, write_i64, 8);
impl_int_property!(UInt64Property, u64, read_u64, write_u64, 8);
