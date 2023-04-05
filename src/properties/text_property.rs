use std::{fmt::Debug, io::Cursor};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::CursorExt,
    error::{Error, SerializeError},
};

use super::PropertyTrait;

#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextProperty {
    Empty(),
    Rich(RichText),
    Simple(Vec<String>),
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
        if let Some(rich) = value {
            TextProperty::Rich(rich)
        } else if let Some(simple) = values {
            TextProperty::Simple(simple)
        } else {
            TextProperty::Empty()
        }
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

        if component_type == 0 {
            // Empty text
            let count = cursor.read_u32::<LittleEndian>()?;
            assert!(count == 0, "Unexpected count {}", count);

            Ok(TextProperty::Empty())
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

            Ok(TextProperty::Rich(RichText {
                id,
                pattern,
                text_format,
            }))
        } else if component_type == 2 {
            // Simple text
            let count = cursor.read_u32::<LittleEndian>()?;
            assert!(count > 0, "Unexpected count {}", count);

            let mut strings: Vec<String> = vec![];
            for _ in 0..count {
                let str = cursor.read_string()?;
                strings.push(str)
            }

            Ok(TextProperty::Simple(strings))
        } else {
            // Unknown text
            Err(SerializeError::InvalidValue(format!(
                "Unexpected component_type {}",
                component_type
            ))
            .into())
        }
    }
}

impl Debug for TextProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextProperty::Rich(value) => value.fmt(f),
            TextProperty::Simple(values) => f.debug_list().entries(values).finish(),
            TextProperty::Empty() => f.write_str("Empty"),
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

        match self {
            TextProperty::Empty() => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u8(255)?;
                cursor.write_u32::<LittleEndian>(0)?;
            }

            TextProperty::Rich(value) => {
                cursor.write_u32::<LittleEndian>(1)?;
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
            }

            TextProperty::Simple(values) => {
                cursor.write_u32::<LittleEndian>(2)?;
                cursor.write_u8(255)?;
                cursor.write_i32::<LittleEndian>(values.len() as i32)?;
                for value in values {
                    cursor.write_string(value)?;
                }
            }
        }

        Ok(())
    }
}

impl RichText {
    pub fn new(id: String, pattern: String, text_format: Vec<RichTextFormat>) -> Self {
        RichText {
            id,
            pattern,
            text_format,
        }
    }
}

impl RichTextFormat {
    pub fn new(format_key: String, content_type: u32, values: Vec<String>) -> Self {
        RichTextFormat {
            format_key,
            content_type,
            values,
        }
    }
}
