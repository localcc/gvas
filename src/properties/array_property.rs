use std::{
    fmt::Debug,
    io::{Cursor, Read, Seek, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::{DeserializeError, Error, SerializeError},
    types::Guid,
};

use super::{
    impl_read_header, impl_write, impl_write_header_part,
    int_property::{ByteProperty, BytePropertyValue},
    struct_property::StructProperty,
    Property, PropertyOptions, PropertyTrait,
};

#[cfg(feature = "serde")]
use serde_with::{hex::Hex, serde_as};

/// A property that holds an array of values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ArrayProperty {
    /// An array of ByteProperty values.
    Bytes {
        /// An array of values.
        #[cfg_attr(feature = "serde", serde_as(as = "Hex"))]
        bytes: Vec<u8>,
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
        structs: Vec<StructProperty>,
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
                format!($($arg)+),
                $cursor.stream_position()?,
            ))?
        }
    }};
}

impl_write!(ArrayProperty, options, (write_string, fn, get_property_type));

impl ArrayProperty {
    /// Creates a new `ArrayProperty` instance.
    #[inline]
    pub fn new(
        property_type: String,
        struct_info: Option<(String, String, Guid)>,
        properties: Vec<Property>,
    ) -> Result<Self, Error> {
        match (property_type.as_str(), struct_info) {
            ("StructProperty", Some((field_name, type_name, guid))) => match properties
                .iter()
                .map(|p| match p {
                    Property::StructProperty(struct_property) => Ok(struct_property.clone()),
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
                Err(p) => Err(SerializeError::invalid_value(&format!(
                    "Array property_type {} doesn't match property inside array: {:#?}",
                    property_type, p
                )))?,
            },

            (_, Some(_)) => Err(SerializeError::invalid_value(
                "struct_info is only supported for StructProperty",
            ))?,

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

            (_, None) => Ok(ArrayProperty::Properties {
                property_type,
                properties,
            }),
        }
    }

    pub(crate) fn get_property_type(&self) -> Result<String, Error> {
        Ok(match self {
            ArrayProperty::Bytes { bytes: _ } => "ByteProperty".to_string(),
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
                let separator = cursor.read_u8()?;
                assert_eq!(separator, 0);

                let properties_start = cursor.stream_position()?;
                for _ in 0..property_count {
                    properties.push(
                        StructProperty::read_with_type_name(cursor, &struct_name, options)?.into(),
                    );
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

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        match self {
            ArrayProperty::Structs {
                field_name,
                type_name,
                guid,
                structs,
            } => {
                cursor.write_u32::<LittleEndian>(structs.len() as u32)?;
                cursor.write_string(field_name)?;
                cursor.write_string("StructProperty")?;

                let buf = &mut Cursor::new(Vec::new());
                for property in structs {
                    property.write(buf, false, options)?;
                }
                let buf = buf.get_ref();

                cursor.write_u64::<LittleEndian>(buf.len() as u64)?;
                cursor.write_string(type_name)?;
                cursor.write_guid(guid)?;
                cursor.write_u8(0)?;
                cursor.write_all(buf)?;
            }

            ArrayProperty::Bytes { bytes } => {
                cursor.write_u32::<LittleEndian>(bytes.len() as u32)?;
                for b in bytes {
                    let property = Property::from(ByteProperty::new_byte(None, *b));
                    property.write(cursor, false, options)?;
                }
            }

            ArrayProperty::Properties {
                property_type: _,
                properties,
            } => {
                cursor.write_u32::<LittleEndian>(properties.len() as u32)?;
                for property in properties {
                    property.write(cursor, false, options)?;
                }
            }
        }

        Ok(())
    }
}
