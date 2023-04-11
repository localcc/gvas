use std::{
    collections::HashMap,
    fmt::Debug,
    io::{Cursor, Read, Seek, SeekFrom, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::CursorExt,
    error::{DeserializeError, Error, SerializeError},
    types::Guid,
};

use super::{struct_property::StructProperty, Property, PropertyTrait};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct ArrayStructInfo {
    type_name: String,
    field_name: String,
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

    array_struct_info: Option<ArrayStructInfo>,
}

macro_rules! validate {
    ($cursor:expr, $cond:expr, $($arg:tt)+) => {{
        if !$cond {
            Err(DeserializeError::InvalidProperty(
                format!($($arg)+),
                $cursor.position(),
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

    pub(crate) fn read(
        cursor: &mut Cursor<Vec<u8>>,
        hints: &HashMap<String, String>,
        properties_stack: &mut Vec<String>,
    ) -> Result<Self, Error> {
        let length = cursor.read_u64::<LittleEndian>()?;

        let property_type = cursor.read_string()?;
        cursor.read_exact(&mut [0u8; 1])?;
        let start_position = cursor.position();

        let property_count = cursor.read_i32::<LittleEndian>()? as usize;
        let mut properties: Vec<Property> = Vec::with_capacity(property_count);

        let mut array_struct_info = None;

        match property_type.as_str() {
            "StructProperty" => {
                let field_name = cursor.read_string()?;

                let _dup_property_type = cursor.read_string()?;
                let properties_size = cursor.read_u64::<LittleEndian>()?;

                let struct_name = cursor.read_string()?;
                let mut struct_guid = [0u8; 16];
                cursor.read_exact(&mut struct_guid)?;
                cursor.read_exact(&mut [0u8; 1])?;

                let properties_start = cursor.position();
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
                let properties_end = cursor.position();
                validate!(
                    cursor,
                    properties_end == properties_start + properties_size,
                    "{properties_end} == {properties_start} + {properties_size}",
                );

                array_struct_info = Some(ArrayStructInfo {
                    type_name: struct_name,
                    field_name,
                    guid: Guid(struct_guid),
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
        let end_position = cursor.position();
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
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if !include_header {
            return Err(SerializeError::invalid_value("Nested arrays not supported").into());
        }

        cursor.write_string("ArrayProperty")?;

        let begin = cursor.position();
        cursor.write_u64::<LittleEndian>(0)?;

        cursor.write_string(&self.property_type)?;
        let _ = cursor.write(&[0u8; 1])?;
        let begin_write = cursor.position();

        cursor.write_i32::<LittleEndian>(self.properties.len() as i32)?;

        match self.property_type.as_str() {
            "StructProperty" => {
                let array_struct_info = self.array_struct_info.as_ref().ok_or_else(|| {
                    SerializeError::invalid_value(
                        "Array type is StructProperty but array_struct_info is None",
                    )
                })?;

                cursor.write_string(&array_struct_info.field_name)?;
                cursor.write_string(&self.property_type)?;

                let len_position = cursor.position();
                cursor.write_u64::<LittleEndian>(0)?;
                cursor.write_string(&array_struct_info.type_name)?;
                let _ = cursor.write(&array_struct_info.guid.0)?;
                let _ = cursor.write(&[0u8; 1])?;
                let begin_without_name = cursor.position();

                for property in &self.properties {
                    let res: Result<(), Error> = match property {
                        Property::StructProperty(e) => {
                            e.write(cursor, false)?;
                            Ok(())
                        }
                        _ => Err(SerializeError::invalid_value(
                            "Array property_type doesn't match property inside array",
                        )
                        .into()),
                    };
                    res?;
                }
                let end_without_name = cursor.position();
                cursor.seek(SeekFrom::Start(len_position))?;
                cursor.write_u64::<LittleEndian>(end_without_name - begin_without_name)?;
                cursor.seek(SeekFrom::Start(end_without_name))?;
            }
            _ => {
                for property in &self.properties {
                    property.write(cursor, false)?;
                }
            }
        }

        let end_write = cursor.position();
        cursor.seek(SeekFrom::Start(begin))?;
        cursor.write_u64::<LittleEndian>(end_write - begin_write)?;
        cursor.seek(SeekFrom::Start(end_write))?;
        Ok(())
    }
}
