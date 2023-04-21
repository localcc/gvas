use std::{
    collections::HashMap,
    fmt::Debug,
    io::{Cursor, Read, Seek, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::{DeserializeError, Error, SerializeError},
    types::Guid,
};

use super::{struct_property::StructProperty, Property, PropertyTrait};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct ArrayStructInfo {
    type_name: String,
    field_name: String,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Guid::is_zero"))]
    #[cfg_attr(feature = "serde", serde(default))]
    guid: Guid,
}

/// A property that holds an array of values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayProperty {
    /// The type of Property in `properties`.
    pub property_type: String,
    /// An array of values.
    pub properties: Vec<Property>,

    #[cfg_attr(feature = "serde", serde(flatten))]
    array_struct_info: Option<ArrayStructInfo>,
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

impl ArrayProperty {
    /// Creates a new `ArrayProperty` instance.
    pub fn new(
        property_type: String,
        struct_info: Option<(String, String, Guid)>,
        properties: Vec<Property>,
    ) -> Self {
        let array_struct_info = struct_info.map(|(field_name, type_name, guid)| ArrayStructInfo {
            field_name,
            type_name,
            guid,
        });

        ArrayProperty {
            property_type,
            properties,

            array_struct_info,
        }
    }

    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        hints: &HashMap<String, String>,
        properties_stack: &mut Vec<String>,
    ) -> Result<Self, Error> {
        let length = cursor.read_u64::<LittleEndian>()?;

        let property_type = cursor.read_string()?;
        let separator = cursor.read_u8()?;
        assert_eq!(separator, 0);
        let start_position = cursor.stream_position()?;

        let property_count = cursor.read_u32::<LittleEndian>()? as usize;
        let mut properties: Vec<Property> = Vec::with_capacity(property_count);

        let mut array_struct_info = None;

        match property_type.as_str() {
            "StructProperty" => {
                let field_name = cursor.read_string()?;

                let _dup_property_type = cursor.read_string()?;
                let properties_size = cursor.read_u64::<LittleEndian>()?;

                let struct_name = cursor.read_string()?;
                let guid = cursor.read_guid()?;
                let separator = cursor.read_u8()?;
                assert_eq!(separator, 0);

                let properties_start = cursor.stream_position()?;
                for _ in 0..property_count {
                    properties.push(
                        StructProperty::read_with_type_name(
                            cursor,
                            hints,
                            properties_stack,
                            &struct_name,
                        )?
                        .into(),
                    );
                }
                let properties_end = cursor.stream_position()?;
                validate!(
                    cursor,
                    properties_end == properties_start + properties_size,
                    "{properties_end} == {properties_start} + {properties_size}",
                );

                array_struct_info = Some(ArrayStructInfo {
                    type_name: struct_name,
                    field_name,
                    guid,
                });
            }
            _ => {
                for _ in 0..property_count {
                    properties.push(Property::new(
                        cursor,
                        hints,
                        properties_stack,
                        &property_type,
                        false,
                        Some((length - 4) / property_count as u64 + length),
                    )?)
                }
            }
        };
        let end_position = cursor.stream_position()?;
        validate!(
            cursor,
            end_position == start_position + length,
            "{end_position} == {start_position} + {length}"
        );

        Ok(ArrayProperty {
            property_type,
            properties,

            array_struct_info,
        })
    }
}

impl PropertyTrait for ArrayProperty {
    fn write<W: Write>(&self, cursor: &mut W, include_header: bool) -> Result<(), Error> {
        if !include_header {
            // return self.write_body(cursor);
            Err(SerializeError::invalid_value("Nested arrays not supported"))?
        }

        let buf = &mut Cursor::new(Vec::new());
        self.write_body(buf)?;
        let buf = buf.get_ref();

        cursor.write_string("ArrayProperty")?;
        cursor.write_u64::<LittleEndian>(buf.len() as u64)?;
        cursor.write_string(&self.property_type)?;
        cursor.write_u8(0)?;
        cursor.write_all(buf)?;

        Ok(())
    }
}

impl ArrayProperty {
    fn write_body<W: Write>(&self, cursor: &mut W) -> Result<(), Error> {
        cursor.write_u32::<LittleEndian>(self.properties.len() as u32)?;

        match self.property_type.as_str() {
            "StructProperty" => {
                let array_struct_info = self.array_struct_info.as_ref().ok_or_else(|| {
                    SerializeError::invalid_value(
                        "Array type is StructProperty but array_struct_info is None",
                    )
                })?;

                cursor.write_string(&array_struct_info.field_name)?;
                cursor.write_string(&self.property_type)?;

                let buf = &mut Cursor::new(Vec::new());
                self.write_properties(buf)?;
                let buf = buf.get_ref();

                cursor.write_u64::<LittleEndian>(buf.len() as u64)?;
                cursor.write_string(&array_struct_info.type_name)?;
                cursor.write_guid(&array_struct_info.guid)?;
                cursor.write_u8(0)?;
                cursor.write_all(buf)?;
            }
            _ => {
                for property in &self.properties {
                    property.write(cursor, false)?;
                }
            }
        }

        Ok(())
    }

    fn write_properties<W: Write>(&self, cursor: &mut W) -> Result<(), Error> {
        for property in &self.properties {
            let res: Result<(), Error> = match property {
                Property::StructProperty(e) => {
                    e.write(cursor, false)?;
                    Ok(())
                }
                _ => Err(SerializeError::invalid_value(
                    "Array property_type doesn't match property inside array",
                ))?,
            };
            res?;
        }
        Ok(())
    }
}
