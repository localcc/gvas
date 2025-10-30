use std::{
    fmt::Debug,
    io::{Cursor, Read, Seek, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use ordered_float::OrderedFloat;

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::{DeserializeError, Error, SerializeError},
    types::Guid,
};

use super::{
    Property, PropertyOptions, PropertyTrait,
    enum_property::EnumProperty,
    impl_read_header, impl_write, impl_write_header_part,
    int_property::{BoolProperty, ByteProperty, BytePropertyValue, FloatProperty, IntProperty},
    name_property::NameProperty,
    str_property::StrProperty,
    struct_property::{StructProperty, StructPropertyValue},
};

#[cfg(feature = "serde")]
use serde_with::{hex::Hex, serde_as};

/// A property that holds an array of values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ArrayProperty {
    /// An array of BoolProperty values.
    Bools {
        /// An array of values.
        bools: Vec<bool>,
    },
    /// An array of ByteProperty values.
    Bytes {
        /// An array of values.
        #[cfg_attr(feature = "serde", serde_as(as = "Hex"))]
        bytes: Vec<u8>,
    },
    /// An array of EnumProperty values.
    Enums {
        /// An array of values.
        enums: Vec<String>,
    },
    /// An array of FloatProperty values.
    Floats {
        /// An array of values.
        floats: Vec<OrderedFloat<f32>>,
    },
    /// An array of IntProperty values.
    Ints {
        /// An array of values.
        ints: Vec<i32>,
    },
    /// An array of NameProperty values.
    Names {
        /// An array of values.
        names: Vec<Option<String>>,
    },
    /// An array of StrProperty values.
    Strings {
        /// An array of values.
        strings: Vec<Option<String>>,
    },
    /// An array of StructProperty values.
    Structs {
        /// Field name.
        field_name: String,
        /// Type name.
        type_name: String,
        /// The unique identifier of the property.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Guid::is_zero"))]
        #[cfg_attr(feature = "serde", serde(default))]
        guid: Guid,
        /// An array of values.
        structs: Vec<StructPropertyValue>,
    },
    /// Any other Property value
    Properties {
        /// The type of Property in `properties`.
        property_type: String,
        /// An array of values.
        properties: Vec<Property>,
    },
}

macro_rules! validate {
    ($cursor:expr, $cond:expr, $($arg:tt)+) => {{
        if !$cond {
            Err(DeserializeError::InvalidProperty(
                format!($($arg)+).into_boxed_str(),
                $cursor.stream_position()?,
            ))?
        }
    }};
}

impl ArrayProperty {
    /// Creates a new `ArrayProperty` instance.
    #[inline]
    pub fn new(
        property_type: String,
        struct_info: Option<(String, String, Guid)>,
        properties: Vec<Property>,
    ) -> Result<Self, Error> {
        match (property_type.as_str(), struct_info) {
            ("BoolProperty", None) => match properties
                .iter()
                .map(|p| match p {
                    Property::BoolProperty(BoolProperty { value }) => Ok(*value),
                    _ => Err(()),
                })
                .collect::<Result<_, _>>()
            {
                Ok(bools) => Ok(ArrayProperty::Bools { bools }),
                Err(()) => Ok(ArrayProperty::Properties {
                    property_type,
                    properties,
                }),
            },

            ("ByteProperty", None) => match properties
                .iter()
                .map(|p| match p {
                    Property::ByteProperty(ByteProperty {
                        name: None,
                        value: BytePropertyValue::Byte(value),
                    }) => Ok(*value),
                    _ => Err(()),
                })
                .collect::<Result<_, _>>()
            {
                Ok(bytes) => Ok(ArrayProperty::Bytes { bytes }),
                Err(()) => Ok(ArrayProperty::Properties {
                    property_type,
                    properties,
                }),
            },

            ("EnumProperty", None) => match properties
                .iter()
                .map(|p| match p {
                    Property::EnumProperty(EnumProperty {
                        enum_type: None,
                        value,
                    }) => Ok(value.to_owned()),
                    _ => Err(()),
                })
                .collect::<Result<_, _>>()
            {
                Ok(enums) => Ok(ArrayProperty::Enums { enums }),
                Err(()) => Ok(ArrayProperty::Properties {
                    property_type,
                    properties,
                }),
            },

            ("IntProperty", None) => match properties
                .iter()
                .map(|p| match p {
                    Property::IntProperty(IntProperty { value }) => Ok(*value),
                    _ => Err(()),
                })
                .collect::<Result<_, _>>()
            {
                Ok(ints) => Ok(ArrayProperty::Ints { ints }),
                Err(()) => Ok(ArrayProperty::Properties {
                    property_type,
                    properties,
                }),
            },

            ("FloatProperty", None) => match properties
                .iter()
                .map(|p| match p {
                    Property::FloatProperty(FloatProperty { value }) => Ok(value.to_owned()),
                    _ => Err(()),
                })
                .collect::<Result<_, _>>()
            {
                Ok(floats) => Ok(ArrayProperty::Floats { floats }),
                Err(()) => Ok(ArrayProperty::Properties {
                    property_type,
                    properties,
                }),
            },

            ("NameProperty", None) => match properties
                .iter()
                .map(|p| match p {
                    Property::NameProperty(NameProperty {
                        array_index: 0,
                        value,
                    }) => Ok(value.to_owned()),
                    _ => Err(()),
                })
                .collect::<Result<_, _>>()
            {
                Ok(names) => Ok(ArrayProperty::Names { names }),
                Err(()) => Ok(ArrayProperty::Properties {
                    property_type,
                    properties,
                }),
            },

            ("StrProperty", None) => match properties
                .iter()
                .map(|p| match p {
                    Property::StrProperty(StrProperty { value }) => Ok(value.to_owned()),
                    _ => Err(()),
                })
                .collect::<Result<_, _>>()
            {
                Ok(strings) => Ok(ArrayProperty::Strings { strings }),
                Err(()) => Ok(ArrayProperty::Properties {
                    property_type,
                    properties,
                }),
            },

            ("StructProperty", Some((field_name, type_name, guid))) => match properties
                .iter()
                .map(|p| match p {
                    Property::StructPropertyValue(value) => Ok(value.clone()),
                    _ => Err(p),
                })
                .collect::<Result<_, _>>()
            {
                Ok(structs) => Ok(ArrayProperty::Structs {
                    field_name,
                    type_name,
                    guid,
                    structs,
                }),
                Err(p) => Err(SerializeError::invalid_value(format!(
                    "Array property_type {} doesn't match property inside array: {:#?}",
                    property_type, p
                )))?,
            },

            (_, Some(_)) => Err(SerializeError::invalid_value(
                "struct_info is only supported for StructProperty",
            ))?,

            (_, None) => Ok(ArrayProperty::Properties {
                property_type,
                properties,
            }),
        }
    }

    pub(crate) fn get_property_type(&self) -> Result<String, Error> {
        Ok(match self {
            ArrayProperty::Bools { bools: _ } => "BoolProperty".to_string(),
            ArrayProperty::Bytes { bytes: _ } => "ByteProperty".to_string(),
            ArrayProperty::Enums { enums: _ } => "EnumProperty".to_string(),
            ArrayProperty::Floats { floats: _ } => "FloatProperty".to_string(),
            ArrayProperty::Ints { ints: _ } => "IntProperty".to_string(),
            ArrayProperty::Names { names: _ } => "NameProperty".to_string(),
            ArrayProperty::Strings { strings: _ } => "StrProperty".to_string(),
            ArrayProperty::Structs {
                field_name: _,
                type_name: _,
                guid: _,
                structs: _,
            } => "StructProperty".to_string(),
            ArrayProperty::Properties {
                property_type,
                properties: _,
            } => property_type.clone(),
        })
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        include_header: bool,
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        if include_header {
            Self::read_header(cursor, options)
        } else {
            Err(DeserializeError::invalid_property(
                "ArrayProperty is not supported in arrays",
                cursor,
            ))?
        }
    }

    impl_read_header!(options, length, property_type);

    #[inline]
    pub(crate) fn read_body<R: Read + Seek>(
        cursor: &mut R,
        options: &mut PropertyOptions,
        length: u32,
        property_type: String,
    ) -> Result<Self, Error> {
        let property_count = cursor.read_u32::<LittleEndian>()?;
        let mut properties: Vec<Property> = Vec::with_capacity(property_count as usize);

        let mut array_struct_info = None;

        match property_type.as_str() {
            "StructProperty" => {
                let field_name = cursor.read_string()?;

                let property_type = cursor.read_string()?;
                assert_eq!(property_type, "StructProperty");
                let properties_size = cursor.read_u64::<LittleEndian>()?;

                let struct_name = cursor.read_string()?;
                let guid = cursor.read_guid()?;
                let terminator = cursor.read_u8()?;
                if terminator != 0 {
                    let position = cursor.stream_position()? - 1;
                    Err(DeserializeError::InvalidTerminator(terminator, position))?
                }

                let properties_start = cursor.stream_position()?;
                for _ in 0..property_count {
                    let value = StructProperty::read_body(cursor, &struct_name, options)?;
                    properties.push(Property::from(value));
                }
                let properties_end = cursor.stream_position()?;
                let actual_size = properties_end - properties_start;
                validate!(
                    cursor,
                    actual_size == properties_size,
                    "{actual_size} != {properties_size}",
                );

                array_struct_info = Some((field_name, struct_name, guid));
            }
            _ => {
                let suggested_length = if property_count > 0 && length >= 4 {
                    Some((length - 4) / property_count)
                } else {
                    None
                };
                for _ in 0..property_count {
                    properties.push(Property::new(
                        cursor,
                        &property_type,
                        false,
                        options,
                        suggested_length,
                    )?)
                }
            }
        };

        ArrayProperty::new(property_type, array_struct_info, properties)
    }
}

impl PropertyTrait for ArrayProperty {
    impl_write!(ArrayProperty, (write_string, fn, get_property_type));

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        match self {
            ArrayProperty::Bools { bools } => {
                let mut len = 4;
                cursor.write_u32::<LittleEndian>(bools.len() as u32)?;
                for b in bools {
                    let property = Property::from(BoolProperty::new(*b));
                    len += property.write(cursor, false, options)?;
                }
                Ok(len)
            }

            ArrayProperty::Bytes { bytes } => {
                let mut len = 4;
                cursor.write_u32::<LittleEndian>(bytes.len() as u32)?;
                for b in bytes {
                    let property = Property::from(ByteProperty::new_byte(None, *b));
                    len += property.write(cursor, false, options)?;
                }
                Ok(len)
            }

            ArrayProperty::Enums { enums } => {
                let mut len = 4;
                cursor.write_u32::<LittleEndian>(enums.len() as u32)?;
                for e in enums {
                    let property = Property::from(EnumProperty::new(None, e.to_owned()));
                    len += property.write(cursor, false, options)?;
                }
                Ok(len)
            }

            ArrayProperty::Floats { floats } => {
                let mut len = 4;
                cursor.write_u32::<LittleEndian>(floats.len() as u32)?;
                for f in floats {
                    let property = Property::from(FloatProperty::new(f.0));
                    len += property.write(cursor, false, options)?;
                }
                Ok(len)
            }

            ArrayProperty::Ints { ints } => {
                let mut len = 4;
                cursor.write_u32::<LittleEndian>(ints.len() as u32)?;
                for i in ints {
                    let property = Property::from(IntProperty::new(i.to_owned()));
                    len += property.write(cursor, false, options)?;
                }
                Ok(len)
            }

            ArrayProperty::Names { names } => {
                let mut len = 4;
                cursor.write_u32::<LittleEndian>(names.len() as u32)?;
                for n in names {
                    let property = Property::from(NameProperty::from(n.to_owned()));
                    len += property.write(cursor, false, options)?;
                }
                Ok(len)
            }

            ArrayProperty::Strings { strings } => {
                let mut len = 4;
                cursor.write_u32::<LittleEndian>(strings.len() as u32)?;
                for s in strings {
                    let property = Property::from(StrProperty::new(s.to_owned()));
                    len += property.write(cursor, false, options)?;
                }
                Ok(len)
            }

            ArrayProperty::Structs {
                field_name,
                type_name,
                guid,
                structs,
            } => {
                let mut len = 29;
                cursor.write_u32::<LittleEndian>(structs.len() as u32)?;
                len += cursor.write_string(field_name)?;
                len += cursor.write_string("StructProperty")?;

                let buf = &mut Cursor::new(Vec::new());
                for property in structs {
                    len += property.write(buf, false, options)?;
                }
                let buf = buf.get_ref();

                cursor.write_u64::<LittleEndian>(buf.len() as u64)?;
                len += cursor.write_string(type_name)?;
                cursor.write_guid(guid)?;
                cursor.write_u8(0)?;
                cursor.write_all(buf)?;
                Ok(len)
            }

            ArrayProperty::Properties {
                property_type: _,
                properties,
            } => {
                let mut len = 4;
                cursor.write_u32::<LittleEndian>(properties.len() as u32)?;
                for property in properties {
                    len += property.write(cursor, false, options)?;
                }
                Ok(len)
            }
        }
    }
}
