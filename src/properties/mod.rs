use std::io::Cursor;

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
};

pub mod int_property;
pub mod str_property;
pub mod struct_property;

#[enum_dispatch]
pub trait PropertyTrait {
    fn get_length(&self) -> i64;
    fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error>;
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
}

impl Property {
    pub fn new(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let value_type = cursor.read_string()?;

        match value_type.as_str() {
            "Int8Property" => Ok(Int8Property::read(cursor)?.into()),
            "ByteProperty" => Ok(ByteProperty::read(cursor)?.into()),
            "Int16Property" => Ok(Int16Property::read(cursor)?.into()),
            "UInt16Property" => Ok(UInt16Property::read(cursor)?.into()),
            "IntProperty" => Ok(IntProperty::read(cursor)?.into()),
            "UInt32Property" => Ok(UInt32Property::read(cursor)?.into()),
            "Int64Property" => Ok(Int64Property::read(cursor)?.into()),
            "UInt64Property" => Ok(UInt64Property::read(cursor)?.into()),
            "FloatProperty" => Ok(FloatProperty::read(cursor)?.into()),
            "DoubleProperty" => Ok(DoubleProperty::read(cursor)?.into()),
            "BoolProperty" => Ok(BoolProperty::read(cursor)?.into()),
            "StrProperty" => Ok(StrProperty::read(cursor)?.into()),
            "StructProperty" => {
                let _struct_len = cursor.read_i64::<LittleEndian>()?;
                let struct_name = cursor.read_string()?;
                match struct_name.as_str() {
                    "DateTime" => Ok(DateTimeProperty::read(cursor)?.into()),
                    _ => Ok(StructProperty::read(struct_name, cursor)?.into()),
                }
            }
            _ => Err(DeserializeError::UnknownProperty(value_type).into()),
        }
    }
}
