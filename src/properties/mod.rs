use std::{
    fmt::Debug,
    io::{Cursor, Read},
};

use byteorder::{LittleEndian, ReadBytesExt};
use enum_dispatch::enum_dispatch;

use crate::{
    cursor_ext::CursorExt,
    error::{DeserializeError, Error},
};

use self::{
    array_property::ArrayProperty,
    int_property::{
        BoolProperty, ByteProperty, DoubleProperty, FloatProperty, Int16Property, Int64Property,
        Int8Property, IntProperty, UInt16Property, UInt32Property, UInt64Property,
    },
    str_property::StrProperty,
    struct_property::{DateTimeProperty, StructProperty},
};

pub mod array_property;
pub mod int_property;
pub mod str_property;
pub mod struct_property;

#[enum_dispatch]
pub trait PropertyTrait: Debug + Clone + PartialEq {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error>;
}

#[enum_dispatch(PropertyTrait)]
pub enum Property {
    Int8Property,
    ByteProperty,
    Int16Property,
    UInt16Property,
    IntProperty,
    UInt32Property,
    Int64Property,
    UInt64Property,
    FloatProperty,
    DoubleProperty,
    BoolProperty,
    StrProperty,
    StructProperty,
    DateTimeProperty,
    ArrayProperty,
}

impl Property {
    pub fn new(
        cursor: &mut Cursor<Vec<u8>>,
        value_type: &str,
        include_header: bool,
    ) -> Result<Self, Error> {
        match value_type {
            "Int8Property" => Ok(Int8Property::read(cursor, include_header)?.into()),
            "ByteProperty" => Ok(ByteProperty::read(cursor, include_header)?.into()),
            "Int16Property" => Ok(Int16Property::read(cursor, include_header)?.into()),
            "UInt16Property" => Ok(UInt16Property::read(cursor, include_header)?.into()),
            "IntProperty" => Ok(IntProperty::read(cursor, include_header)?.into()),
            "UInt32Property" => Ok(UInt32Property::read(cursor, include_header)?.into()),
            "Int64Property" => Ok(Int64Property::read(cursor, include_header)?.into()),
            "UInt64Property" => Ok(UInt64Property::read(cursor, include_header)?.into()),
            "FloatProperty" => Ok(FloatProperty::read(cursor, include_header)?.into()),
            "DoubleProperty" => Ok(DoubleProperty::read(cursor, include_header)?.into()),
            "BoolProperty" => Ok(BoolProperty::read(cursor, include_header)?.into()),
            "StrProperty" => Ok(StrProperty::read(cursor, include_header)?.into()),
            "StructProperty" => {
                if !include_header {
                    panic!("include_header false should not be set on StructProperty");
                }

                let _struct_len = cursor.read_u64::<LittleEndian>()?;
                let struct_name = cursor.read_string()?;
                let mut guid = [0u8; 16];
                cursor.read_exact(&mut guid)?;
                cursor.read_exact(&mut [0u8; 1])?;

                match struct_name.as_str() {
                    "DateTime" => Ok(DateTimeProperty::read(cursor, guid)?.into()),
                    _ => Ok(StructProperty::read(cursor, struct_name, guid)?.into()),
                }
            }
            "ArrayProperty" => Ok(ArrayProperty::read(cursor)?.into()),
            _ => Err(DeserializeError::UnknownProperty(value_type.to_string()).into()),
        }
    }
}

impl Debug for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int8Property(arg0) => f.debug_tuple("Int8Property").field(arg0).finish(),
            Self::ByteProperty(arg0) => f.debug_tuple("ByteProperty").field(arg0).finish(),
            Self::Int16Property(arg0) => f.debug_tuple("Int16Property").field(arg0).finish(),
            Self::UInt16Property(arg0) => f.debug_tuple("UInt16Property").field(arg0).finish(),
            Self::IntProperty(arg0) => f.debug_tuple("IntProperty").field(arg0).finish(),
            Self::UInt32Property(arg0) => f.debug_tuple("UInt32Property").field(arg0).finish(),
            Self::Int64Property(arg0) => f.debug_tuple("Int64Property").field(arg0).finish(),
            Self::UInt64Property(arg0) => f.debug_tuple("UInt64Property").field(arg0).finish(),
            Self::FloatProperty(arg0) => f.debug_tuple("FloatProperty").field(arg0).finish(),
            Self::DoubleProperty(arg0) => f.debug_tuple("DoubleProperty").field(arg0).finish(),
            Self::BoolProperty(arg0) => f.debug_tuple("BoolProperty").field(arg0).finish(),
            Self::StrProperty(arg0) => f.debug_tuple("StrProperty").field(arg0).finish(),
            Self::StructProperty(arg0) => f.debug_tuple("StructProperty").field(arg0).finish(),
            Self::DateTimeProperty(arg0) => f.debug_tuple("DateTimeProperty").field(arg0).finish(),
            Self::ArrayProperty(arg0) => f.debug_tuple("ArrayProperty").field(arg0).finish(),
        }
    }
}

impl Clone for Property {
    fn clone(&self) -> Self {
        match self {
            Self::Int8Property(arg0) => Self::Int8Property(arg0.clone()),
            Self::ByteProperty(arg0) => Self::ByteProperty(arg0.clone()),
            Self::Int16Property(arg0) => Self::Int16Property(arg0.clone()),
            Self::UInt16Property(arg0) => Self::UInt16Property(arg0.clone()),
            Self::IntProperty(arg0) => Self::IntProperty(arg0.clone()),
            Self::UInt32Property(arg0) => Self::UInt32Property(arg0.clone()),
            Self::Int64Property(arg0) => Self::Int64Property(arg0.clone()),
            Self::UInt64Property(arg0) => Self::UInt64Property(arg0.clone()),
            Self::FloatProperty(arg0) => Self::FloatProperty(arg0.clone()),
            Self::DoubleProperty(arg0) => Self::DoubleProperty(arg0.clone()),
            Self::BoolProperty(arg0) => Self::BoolProperty(arg0.clone()),
            Self::StrProperty(arg0) => Self::StrProperty(arg0.clone()),
            Self::StructProperty(arg0) => Self::StructProperty(arg0.clone()),
            Self::DateTimeProperty(arg0) => Self::DateTimeProperty(arg0.clone()),
            Self::ArrayProperty(arg0) => Self::ArrayProperty(arg0.clone()),
        }
    }
}

impl PartialEq for Property {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int8Property(l0), Self::Int8Property(r0)) => l0 == r0,
            (Self::ByteProperty(l0), Self::ByteProperty(r0)) => l0 == r0,
            (Self::Int16Property(l0), Self::Int16Property(r0)) => l0 == r0,
            (Self::UInt16Property(l0), Self::UInt16Property(r0)) => l0 == r0,
            (Self::IntProperty(l0), Self::IntProperty(r0)) => l0 == r0,
            (Self::UInt32Property(l0), Self::UInt32Property(r0)) => l0 == r0,
            (Self::Int64Property(l0), Self::Int64Property(r0)) => l0 == r0,
            (Self::UInt64Property(l0), Self::UInt64Property(r0)) => l0 == r0,
            (Self::FloatProperty(l0), Self::FloatProperty(r0)) => l0 == r0,
            (Self::DoubleProperty(l0), Self::DoubleProperty(r0)) => l0 == r0,
            (Self::BoolProperty(l0), Self::BoolProperty(r0)) => l0 == r0,
            (Self::StrProperty(l0), Self::StrProperty(r0)) => l0 == r0,
            (Self::StructProperty(l0), Self::StructProperty(r0)) => l0 == r0,
            (Self::DateTimeProperty(l0), Self::DateTimeProperty(r0)) => l0 == r0,
            (Self::ArrayProperty(l0), Self::ArrayProperty(r0)) => l0 == r0,
            _ => false,
        }
    }
}
