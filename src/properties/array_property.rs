use std::io::{Cursor, Read, Write, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::CursorExt,
    error::{Error, SerializeError},
};

use super::{struct_property::StructProperty, Property, PropertyTrait};

pub struct ArrayProperty {
    pub property_type: String,
    field_name: Option<String>,
    pub properties: Vec<Property>,
}

impl ArrayProperty {
    pub fn new(
        property_type: String,
        field_name: Option<String>,
        properties: Vec<Property>,
    ) -> Self {
        ArrayProperty {
            property_type,
            field_name,
            properties,
        }
    }

    pub fn read(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let _length = cursor.read_u64::<LittleEndian>()?;

        let property_type = cursor.read_string()?;
        cursor.read_exact(&mut [0u8; 1])?;

        let properties_len = cursor.read_i32::<LittleEndian>()? as usize;
        let mut properties: Vec<Property> = Vec::with_capacity(properties_len);
        let mut field_name = None;

        match property_type.as_str() {
            "StructProperty" => {
                field_name = Some(cursor.read_string()?);
                let _dup_property_type = cursor.read_string()?;
                let _length_without_struct_name = cursor.read_u64::<LittleEndian>()?;

                let struct_name = cursor.read_string()?;
                let mut struct_guid = [0u8; 16];
                cursor.read_exact(&mut struct_guid)?;
                cursor.read_exact(&mut [0u8; 1])?;

                for _ in 0..properties_len {
                    let struct_property =
                        StructProperty::read(cursor, struct_name.clone(), struct_guid)?;
                    properties.push(struct_property.into());
                }
            }
            _ => {
                for _ in 0..properties_len {
                    properties.push(Property::new(cursor, &property_type, false)?)
                }
            }
        };

        Ok(ArrayProperty {
            property_type,
            field_name,
            properties,
        })
    }
}

impl PropertyTrait for ArrayProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if !include_header {
            panic!("Nested arrays not supported");
        }

        if self.properties.len() == 0 {
            return Ok(())
        }
        cursor.write_string(&String::from("ArrayProperty"))?;

        let begin = cursor.position();
        cursor.write_u64::<LittleEndian>(0)?;

        cursor.write_string(&self.property_type)?;
        cursor.write(&[0u8; 1])?;
        let begin_write = cursor.position();

        cursor.write_i32::<LittleEndian>(self.properties.len() as i32)?;

        match self.property_type.as_str() {
            "StructProperty" => {
                let struct_property: Result<&StructProperty, Error> = match &self.properties[0] {
                    Property::StructProperty(e) => Ok(e),
                    _ => Err(SerializeError::InvalidValue(String::from(
                        "Array property_type doesn't match property inside array",
                    ))
                    .into()),
                };
                let struct_property = struct_property?;

                cursor.write_string(
                    self.field_name.as_ref().ok_or::<Error>(
                        SerializeError::InvalidValue(String::from(
                            "Array type is StructProperty but field_name is None",
                        ))
                        .into(),
                    )?,
                )?;
                cursor.write_string(&self.property_type)?;

                let begin_without_name = cursor.position();
                cursor.write_u64::<LittleEndian>(0)?;
                cursor.write_string(&struct_property.name)?;
                cursor.write(&struct_property.guid)?;
                cursor.write(&[0u8; 1])?;

                for property in &self.properties {
                    let res: Result<(), Error> = match property {
                        Property::StructProperty(e) => {
                            e.write(cursor, false)?;
                            Ok(())
                        }
                        _ => Err(SerializeError::InvalidValue(String::from(
                            "Array property_type doesn't match property inside array",
                        ))
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
