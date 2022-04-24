use std::io::Cursor;

use enum_dispatch::enum_dispatch;

use crate::{error::{Error, DeserializeError}, cursor_ext::CursorExt};

use self::int_property::{Int8Property, ByteProperty, Int16Property, UInt16Property, IntProperty, UInt32Property, Int64Property, UInt64Property, FloatProperty, DoubleProperty, BoolProperty};

pub mod int_property;

#[enum_dispatch]
pub trait PropertyTrait {
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
            _ => Err(DeserializeError::UnknownProperty(value_type).into())
        }
    }
}