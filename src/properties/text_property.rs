use std::{fmt::Debug, io::Cursor};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::CursorExt,
    error::{Error, SerializeError},
};

use super::PropertyTrait;

#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextProperty {
    value: Option<RichText>,
    values: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RichText {
    id: String,
    pattern: String,
    text_format: Vec<RichTextFormat>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RichTextFormat {
    format_key: String,
    content_type: u32,
    values: Vec<String>,
}

impl TextProperty {
    pub fn new(value: Option<RichText>, values: Option<Vec<String>>) -> Self {
        TextProperty { value, values }
    }

    pub(crate) fn read(cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<Self, Error> {
        if include_header {
            return Err(
                SerializeError::invalid_value("TextProperty only supported in arrays").into(),
            );
        }

        let component_type = cursor.read_u32::<LittleEndian>()?;
        assert!(component_type <= 2, "component_type {} > 2", component_type);

        let expect_indicator = if component_type == 1 { 3 } else { 255 };
        let indicator = cursor.read_u8()?;
        assert!(
            indicator == expect_indicator,
            "Unexpected indicator {} for component {}, expected {}",
            indicator,
            component_type,
            expect_indicator
        );

        let mut value: Option<_> = None;
        let mut values: Option<_> = None;

        if component_type == 0 {
            // Empty text
            let count = cursor.read_u32::<LittleEndian>()?;
            assert!(count == 0, "Unexpected count {}", count);
        } else if component_type == 1 {
            // Rich text
            let num_flags = cursor.read_u8()?;
            assert!(num_flags == 8, "Unexpected num_flags {}", num_flags);
            let flags = cursor.read_u64::<LittleEndian>()?;
            assert!(flags == 0, "Unexpected flags {}", flags);

            let id = cursor.read_string()?;
            let pattern = cursor.read_string()?;
            let arg_count = cursor.read_u32::<LittleEndian>()?;

            let mut text_format = vec![];
            for _ in 0..arg_count {
                let format_key = cursor.read_string()?;
                let separator = cursor.read_u8()?;
                assert!(separator == 4, "Unexpected separator {}", separator);
                let content_type = cursor.read_u32::<LittleEndian>()?;
                let indicator = cursor.read_u8()?;
                assert!(indicator == 255, "Unexpected indicator {}", indicator);
                let count = cursor.read_u32::<LittleEndian>()?;

                let mut values = vec![];
                for _ in 0..count {
                    let value = cursor.read_string()?;
                    values.push(value);
                }

                text_format.push(RichTextFormat {
                    format_key,
                    content_type,
                    values,
                });
            }

            value = Some(RichText {
                id,
                pattern,
                text_format,
            });
        } else if component_type == 2 {
            // Simple text
            let count = cursor.read_u32::<LittleEndian>()?;
            assert!(count > 0, "Unexpected count {}", count);

            let mut strings: Vec<String> = vec![];
            for _ in 0..count {
                let str = cursor.read_string()?;
                strings.push(str)
            }

            values = Some(strings);
        } else {
            // Unknown text
            return Err(SerializeError::InvalidValue(format!(
                "Unexpected component_type {}",
                component_type
            ))
            .into());
        }

        Ok(TextProperty { value, values })
    }
}

impl Debug for TextProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.values.is_some(), self.value.is_some()) {
            (false, false) => {
                // Empty text
                f.write_str("Empty")
            }
            (false, true) => {
                // Rich text
                let value = self.value.as_ref().unwrap();
                Debug::fmt(value, f)
            }
            (true, false) => {
                // Simple text
                let values = self.values.as_ref().unwrap();
                f.debug_list().entries(values).finish()
            }
            _ => {
                // Unknown text
                f.write_str("Unknown type")
            }
        }
    }
}

impl PropertyTrait for TextProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        if include_header {
            return Err(
                SerializeError::invalid_value("TextProperty only supported in arrays").into(),
            );
        }

        let component_type = match (self.values.is_some(), self.value.is_some()) {
            (false, false) => 0,
            (false, true) => 1,
            (true, false) => 2,
            _ => return Err(SerializeError::invalid_value("value and values are both set").into()),
        };

        cursor.write_u32::<LittleEndian>(component_type)?;
        if component_type == 0 {
            cursor.write_u8(255)?;
            cursor.write_u32::<LittleEndian>(0)?;
        } else if component_type == 1 {
            let value = self.value.as_ref().unwrap();
            cursor.write_u8(3)?;
            cursor.write_u8(8)?;
            cursor.write_u64::<LittleEndian>(0)?;
            cursor.write_string(&value.id)?;
            cursor.write_string(&value.pattern)?;
            cursor.write_u32::<LittleEndian>(value.text_format.len() as u32)?;
            for rtf in &value.text_format {
                cursor.write_string(&rtf.format_key)?;
                cursor.write_u8(4)?;
                cursor.write_u32::<LittleEndian>(rtf.content_type)?;
                cursor.write_u8(255)?;
                cursor.write_u32::<LittleEndian>(rtf.values.len() as u32)?;
                for value in &rtf.values {
                    cursor.write_string(value)?;
                }
            }
        } else if component_type == 2 {
            let values = self.values.as_ref().unwrap();
            cursor.write_u8(255)?;
            cursor.write_i32::<LittleEndian>(values.len() as i32)?;
            for value in values {
                cursor.write_string(value)?;
            }
        } else {
            return Err(SerializeError::InvalidValue(format!(
                "Unexpected component_type {}",
                component_type
            ))
            .into());
        }
        Ok(())
    }
}