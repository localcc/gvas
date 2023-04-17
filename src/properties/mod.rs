use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    io::{Cursor, Seek, Write},
};

use enum_dispatch::enum_dispatch;

use crate::{
    error::{DeserializeError, Error},
    scoped_stack_entry::ScopedStackEntry,
};

use self::{
    array_property::ArrayProperty,
    enum_property::EnumProperty,
    int_property::{
        BoolProperty, ByteProperty, DoubleProperty, FloatProperty, Int16Property, Int64Property,
        Int8Property, IntProperty, UInt16Property, UInt32Property, UInt64Property,
    },
    map_property::MapProperty,
    name_property::NameProperty,
    set_property::SetProperty,
    str_property::StrProperty,
    struct_property::StructProperty,
    text_property::TextProperty,
    unknown_property::UnknownProperty,
};

/// Module for `ArrayProperty`.
pub mod array_property;
/// Module for `EnumProperty`.
pub mod enum_property;
/// Module for `IntProperty` and various integer properties.
pub mod int_property;
/// Module for `MapProperty`
pub mod map_property;
/// Module for `NameProperty`
pub mod name_property;
/// Module for `SetProperty`
pub mod set_property;
/// Module for `StrProperty`
pub mod str_property;
/// Module for `StructProperty`
pub mod struct_property;
/// Module for `StructProperty` sub-types.
pub mod struct_types;
/// Module for `TextProperty`
pub mod text_property;
/// Module for `UnknownProperty`
pub mod unknown_property;

/// Creates a match helper function for enums.
#[macro_export]
macro_rules! make_matcher {
    ($type:ident, $name:ident) => {
        #[doc = "Retrieves the enum value as a `"]
        #[doc = stringify!($type)]
        #[doc = "`."]
        pub fn $name(&self) -> Option<&$type> {
            match self {
                Self::$type(e) => Some(e),
                _ => None,
            }
        }
    };
}

/// Property traits.
#[enum_dispatch]
pub trait PropertyTrait: Debug + Clone + PartialEq + Eq + Hash {
    /// Serialize.
    fn write<W: Write + Seek>(&self, cursor: &mut W, include_header: bool) -> Result<(), Error>;
}

/// GVAS property types.
#[enum_dispatch(PropertyTrait)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type")
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Property {
    /// An `ArrayProperty`.
    ArrayProperty,
    /// A `BoolProperty`.
    BoolProperty,
    /// A `ByteProperty`.
    ByteProperty,
    /// A `DoubleProperty`.
    DoubleProperty,
    /// An `EnumProperty`.
    EnumProperty,
    /// A `FloatProperty`.
    FloatProperty,
    /// An `Int16Property`.
    Int16Property,
    /// An `Int64Property`.
    Int64Property,
    /// An `Int8Property`.
    Int8Property,
    /// An `IntProperty`.
    IntProperty,
    /// A `MapProperty`.
    MapProperty,
    /// A `NameProperty`.
    NameProperty,
    /// A `SetProperty`.
    SetProperty,
    /// A `StrProperty`.
    StrProperty,
    /// A `StructProperty`.
    StructProperty,
    /// A `TextProperty`.
    TextProperty,
    /// A `UInt16Property`.
    UInt16Property,
    /// A `UInt32Property`.
    UInt32Property,
    /// A `UInt64Property`.
    UInt64Property,
    /// An `UnknownProperty`.
    UnknownProperty,
}

impl Property {
    /// Creates a new `Property` instance.
    pub fn new(
        cursor: &mut Cursor<Vec<u8>>,
        hints: &HashMap<String, String>,
        properties_stack: &mut Vec<String>,
        value_type: &str,
        include_header: bool,
        suggested_length: Option<u64>,
    ) -> Result<Self, Error> {
        let _stack_entry = ScopedStackEntry::new(properties_stack, value_type.to_string());
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
            "EnumProperty" => Ok(EnumProperty::read(cursor)?.into()),
            "StrProperty" => Ok(StrProperty::read(cursor, include_header)?.into()),
            "TextProperty" => Ok(TextProperty::read(cursor, include_header)?.into()),
            "NameProperty" => Ok(NameProperty::read(cursor, include_header)?.into()),
            "StructProperty" => {
                if !include_header {
                    let struct_path = properties_stack.join(".");
                    let Some(hint) = hints.get(&struct_path) else {
                        Err(DeserializeError::MissingHint(
                            value_type.to_string(),
                            struct_path,
                            cursor.position(),
                        ))?
                    };

                    return Ok(StructProperty::read_with_type_name(
                        cursor,
                        hints,
                        properties_stack,
                        hint,
                    )?
                    .into());
                }

                Ok(StructProperty::read_with_header(cursor, hints, properties_stack)?.into())
            }
            "ArrayProperty" => Ok(ArrayProperty::read(cursor, hints, properties_stack)?.into()),
            "SetProperty" => Ok(SetProperty::read(cursor, hints, properties_stack)?.into()),
            "MapProperty" => Ok(MapProperty::read(cursor, hints, properties_stack)?.into()),
            _ => {
                if include_header {
                    return Ok(
                        UnknownProperty::read_with_header(cursor, value_type.to_string())?.into(),
                    );
                }

                if let Some(suggested_length) = suggested_length {
                    return Ok(UnknownProperty::read_with_length(
                        cursor,
                        value_type.to_string(),
                        suggested_length,
                    )?
                    .into());
                }

                Err(DeserializeError::invalid_property(
                    value_type,
                    cursor.position(),
                ))?
            }
        }
    }

    make_matcher!(ArrayProperty, get_array);
    make_matcher!(EnumProperty, get_enum);
    make_matcher!(BoolProperty, get_bool);
    make_matcher!(ByteProperty, get_byte);
    make_matcher!(DoubleProperty, get_f64);
    make_matcher!(FloatProperty, get_f32);
    make_matcher!(Int16Property, get_i16);
    make_matcher!(Int64Property, get_i64);
    make_matcher!(Int8Property, get_i8);
    make_matcher!(IntProperty, get_int);
    make_matcher!(UInt16Property, get_u16);
    make_matcher!(UInt32Property, get_u32);
    make_matcher!(UInt64Property, get_u64);
    make_matcher!(MapProperty, get_map);
    make_matcher!(NameProperty, get_name);
    make_matcher!(SetProperty, get_set);
    make_matcher!(StrProperty, get_str);
    make_matcher!(StructProperty, get_struct);
    make_matcher!(TextProperty, get_text);
    make_matcher!(UnknownProperty, get_unknown);
}
