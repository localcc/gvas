use std::{
    collections::HashMap,
    fmt::Debug,
    io::{Cursor, Read, Seek, SeekFrom, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::CursorExt,
    error::{Error, SerializeError},
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

#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayProperty {
    pub property_type: String,
    pub properties: Vec<Property>,

    array_struct_info: Option<ArrayStructInfo>,
}

impl ArrayProperty {
    pub fn new(
        property_type: String,
        field_name: Option<String>,
        properties: Vec<Property>,
    ) -> Self {
        let array_struct_info = field_name.map(|field_name| ArrayStructInfo {
            field_name,
            type_name: "".to_string(),
            guid: Guid([0u8; 16]),
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

        let properties_len = cursor.read_i32::<LittleEndian>()? as usize;
        let mut properties: Vec<Property> = Vec::with_capacity(properties_len);

        let mut array_struct_info = None;

        match property_type.as_str() {
            "StructProperty" => {
                let field_name = cursor.read_string()?;

                let _dup_property_type = cursor.read_string()?;
                let _length_without_struct_name = cursor.read_u64::<LittleEndian>()?;

                let struct_name = cursor.read_string()?;
                let mut struct_guid = [0u8; 16];
                cursor.read_exact(&mut struct_guid)?;
                cursor.read_exact(&mut [0u8; 1])?;

                for _ in 0..properties_len {
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

                array_struct_info = Some(ArrayStructInfo {
                    type_name: struct_name,
                    field_name,
                    guid: Guid(struct_guid),
                });
            }
            _ => {
                for _ in 0..properties_len {
                    properties.push(Property::new(
                        cursor,
                        hints,
                        properties_stack,
                        &property_type,
                        false,
                        Some((length - 4) / properties_len as u64 + length),
                    )?)
                }
            }
        };

        Ok(ArrayProperty {
            property_type,
            properties,

            array_struct_info,
        })
    }
}

impl Debug for ArrayProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (sep, ind) = match f.alternate() {
            true => ("\n", "    "),
            false => (" ", ""),
        };
        // if let Some(struct_info) = &self.array_struct_info {
        //     write!(f, "struct_info: {:?},{}", struct_info, sep)?;
        //     write!(f, "property_type: {:?},{}", self.property_type, sep)?;
        //     write!(f, "properties: ")?;
        // }
        match self.properties.len() {
            0 => write!(f, "[]"),
            1 => write!(f, "[{:?}]", self.properties.first().unwrap()),
            _ => {
                write!(f, "[")?;
                let mut first = true;
                for property in &self.properties {
                    if first {
                        first = false;
                    } else {
                        write!(f, ",")?;
                    }
                    write!(f, "{}{}{:?}", sep, ind, property)?;
                }
                if f.alternate() && !first {
                    write!(f, ",")?;
                }
                write!(f, "{}]", sep)
            }
        }
    }
}

impl PropertyTrait for ArrayProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if !include_header {
            return Err(SerializeError::invalid_value("Nested arrays not supported").into());
        }

        cursor.write_string(&String::from("ArrayProperty"))?;

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

                let begin_without_name = cursor.position();
                cursor.write_u64::<LittleEndian>(0)?;
                cursor.write_string(&array_struct_info.type_name)?;
                let _ = cursor.write(&array_struct_info.guid.0)?;
                let _ = cursor.write(&[0u8; 1])?;

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
                cursor.seek(SeekFrom::Start(begin_without_name))?;
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
