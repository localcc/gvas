use std::{
    fmt::Debug,
    io::{Cursor, Read, Seek, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use ordered_float::OrderedFloat;

use super::{
    PropertyOptions, PropertyTrait, impl_write,
    struct_types::{unwrap_value, wrap_type, wrap_value},
};
use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::{DeserializeError, Error},
};

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
    ($name:ident, $ty:ident, $read_method:ident, $write_method:ident, $size:literal) => {
        #[doc = concat!("A property that stores a `", stringify!($ty), "`.")]
        #[derive(Clone, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $name {
            /// Integer value.
            pub value: wrap_type!($ty),
        }

        impl $name {
            #[doc = concat!("Creates a new `", stringify!($name), "` instance.")]
            #[inline]
            pub fn new(value: $ty) -> Self {
                let value = wrap_value!($ty, value);
                Self { value }
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
                Ok(Self::new(cursor.$read_method::<LittleEndian>()?))
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}{}", self.value, stringify!($ty))
            }
        }

        impl PropertyTrait for $name {
            impl_write!($name);

            #[inline]
            fn write_body<W: Write>(
                &self,
                cursor: &mut W,
                _: &mut PropertyOptions,
            ) -> Result<usize, Error> {
                let value = self.value;
                let value = unwrap_value!($ty, value);
                cursor.$write_method::<LittleEndian>(value)?;

                Ok($size)
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

impl PropertyTrait for Int8Property {
    impl_write!(Int8Property);

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        _: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        cursor.write_i8(self.value)?;
        Ok(1)
    }
}

impl Debug for Int8Property {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}i8", self.value)
    }
}

/// Byte property value
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BytePropertyValue {
    /// Byte value
    Byte(u8),
    /// Namespaced enum value
    Namespaced(String),
}

/// A property that stores a `u8` or the property's namespaced name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ByteProperty {
    /// Property name.
    pub name: Option<String>,
    /// Property value.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub value: BytePropertyValue,
}

impl ByteProperty {
    /// Creates a new `ByteProperty` instance.
    #[inline]
    pub fn new(name: Option<String>, value: BytePropertyValue) -> Self {
        ByteProperty { name, value }
    }

    /// Creates a new `ByteProperty` instance for a u8 value
    #[inline]
    pub fn new_byte(name: Option<String>, value: u8) -> Self {
        ByteProperty {
            name,
            value: BytePropertyValue::Byte(value),
        }
    }

    /// Creates a new `ByteProperty` instance for a namespaced enum value
    #[inline]
    pub fn new_namespaced(name: Option<String>, value: String) -> Self {
        ByteProperty {
            name,
            value: BytePropertyValue::Namespaced(value),
        }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        include_header: bool,
        mut suggested_length: Option<u32>,
    ) -> Result<Self, Error> {
        let mut name = None;
        if include_header {
            let length = cursor.read_u32::<LittleEndian>()?;
            let array_index = cursor.read_u32::<LittleEndian>()?;
            assert_eq!(
                array_index,
                0,
                "Expected array_index value zero @ {:#x}",
                cursor.stream_position()? - 4
            );
            suggested_length = Some(length);

            name = Some(cursor.read_string()?);
            let separator = cursor.read_u8()?;
            assert_eq!(separator, 0);
        }

        // -1 to account for separator
        let length = suggested_length.map(|e| e - 1).unwrap_or(1);

        let value = match length {
            1 | 0 => BytePropertyValue::Byte(cursor.read_u8()?),
            _ => BytePropertyValue::Namespaced(cursor.read_string()?),
        };

        Ok(ByteProperty { name, value })
    }
}

impl PropertyTrait for ByteProperty {
    #[inline]
    fn write<W: Write>(
        &self,
        cursor: &mut W,
        include_header: bool,
        options: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        if !include_header {
            return self.write_body(cursor, options);
        }

        let mut len = 9;
        len += cursor.write_string("ByteProperty")?;

        let buf = &mut Cursor::new(Vec::new());
        len += self.write_body(buf, options)?;
        let buf = buf.get_ref();

        cursor.write_u64::<LittleEndian>(buf.len() as u64)?;
        len += cursor.write_fstring(self.name.as_deref())?;
        cursor.write_u8(0)?;
        cursor.write_all(buf)?;

        Ok(len)
    }

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        _: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        match &self.value {
            BytePropertyValue::Byte(value) => {
                cursor.write_u8(*value)?;
                Ok(1)
            }
            BytePropertyValue::Namespaced(name) => cursor.write_string(name),
        }
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
    fn write<W: Write>(
        &self,
        cursor: &mut W,
        include_header: bool,
        _options: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        let mut len = 0;
        if include_header {
            len += cursor.write_string("BoolProperty")?;
            len += 8;
            cursor.write_u64::<LittleEndian>(0)?;
        }
        len += 1;
        cursor.write_bool(self.value)?;
        if include_header {
            len += 1;
            cursor.write_u8(0)?;
        }
        Ok(len)
    }

    fn write_body<W: Write>(&self, _: &mut W, _: &mut PropertyOptions) -> Result<usize, Error> {
        unimplemented!()
    }
}

impl_int_property!(FloatProperty, f32, read_f32, write_f32, 4);
impl_int_property!(DoubleProperty, f64, read_f64, write_f64, 8);
impl_int_property!(Int16Property, i16, read_i16, write_i16, 2);
impl_int_property!(UInt16Property, u16, read_u16, write_u16, 2);
impl_int_property!(IntProperty, i32, read_i32, write_i32, 4);
impl_int_property!(UInt32Property, u32, read_u32, write_u32, 4);
impl_int_property!(Int64Property, i64, read_i64, write_i64, 8);
impl_int_property!(UInt64Property, u64, read_u64, write_u64, 8);
