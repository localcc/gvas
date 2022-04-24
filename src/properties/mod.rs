use std::io::{Cursor, Read};

use byteorder::{LittleEndian, ReadBytesExt};
use enum_dispatch::enum_dispatch;

use crate::{
    cursor_ext::CursorExt,
    error::{DeserializeError, Error},
};

use self::{
    int_property::{
        BoolProperty, ByteProperty, DoubleProperty, FloatProperty, Int16Property, Int64Property,
        Int8Property, IntProperty, UInt16Property, UInt32Property, UInt64Property,
    },
    str_property::StrProperty,
    struct_property::{DateTimeProperty, StructProperty},
    array_property::ArrayProperty
};

pub mod int_property;
pub mod str_property;
pub mod struct_property;
pub mod array_property;

#[enum_dispatch]
pub trait PropertyTrait {
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
    pub fn new(cursor: &mut Cursor<Vec<u8>>, value_type: &String, include_header: bool) -> Result<Self, Error> {
        match value_type.as_str() {
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
            },
            "ArrayProperty" => Ok(ArrayProperty::read(cursor)?.into()),
            _ => Err(DeserializeError::UnknownProperty(value_type.clone()).into()),
        }
    }
}
