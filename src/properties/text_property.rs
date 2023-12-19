use std::{
    fmt::Debug,
    io::{Cursor, Read, Seek, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::{DeserializeError, Error},
};

use super::{impl_read, impl_read_header, impl_write, PropertyOptions, PropertyTrait};

/// A property that stores GVAS Text.
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextProperty {
    /// An empty `TextProperty`.
    Empty(
        // Workaround for https://github.com/serde-rs/json/issues/664
        [u8; 0],
    ),
    /// A rich `TextProperty`.
    Rich(RichText),
    /// A simple `TextProperty`.
    Simple(Vec<String>),
    /// `TextProperty` type 8.
    Type8(String, String, String),
}

/// A struct describing a rich `TextProperty`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RichText {
    /// A unique identifier.
    pub id: String,
    /// Text pattern.
    pub pattern: String,
    /// Text pattern substitutions.
    pub text_format: Vec<RichTextFormat>,
}

/// A struct describing a text_format entry in a rich `TextProperty`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RichTextFormat {
    /// Substituation key.
    pub format_key: String,
    /// Content type.
    pub content_type: u32,
    /// Substitution value.
    pub values: Vec<String>,
}

const RTF_UNKNOWN: u64 = 0x100000000;

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

impl_write!(TextProperty, options);

impl TextProperty {
    /// Creates a new `TextProperty` instance.
    #[inline]
    pub fn new(value: Option<RichText>, values: Option<Vec<String>>) -> Self {
        if let Some(rich) = value {
            TextProperty::Rich(rich)
        } else if let Some(simple) = values {
            TextProperty::Simple(simple)
        } else {
            TextProperty::Empty([])
        }
    }

    impl_read!(options);
    impl_read_header!(options);

    #[inline]
    pub(crate) fn read_body<R: Read + Seek>(
        cursor: &mut R,
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        let component_type = cursor.read_u32::<LittleEndian>()?;
        validate!(
            cursor,
            component_type <= 2 || component_type == 8,
            "Unexpected component {component_type}"
        );

        let expect_indicator = match component_type {
            1 => 3,
            8 => 0,
            _ => 255,
        };
        let indicator = cursor.read_u8()?;
        validate!(
            cursor,
            indicator == expect_indicator,
            "Unexpected indicator {} for component {}, expected {}",
            indicator,
            component_type,
            expect_indicator
        );

        if component_type == 0 {
            // Empty text
            let count = cursor.read_u32::<LittleEndian>()?;
            validate!(cursor, count == 0, "Unexpected count {count}");

            Ok(TextProperty::Empty([]))
        } else if component_type == 1 {
            // Rich text
            let num_flags = cursor.read_u8()?;
            validate!(cursor, num_flags == 8, "Unexpected num_flags {num_flags}");
            let flags = cursor.read_u64::<LittleEndian>()?;
            let expect_flags = if options.large_world_coordinates {
                RTF_UNKNOWN
            } else {
                0
            };
            validate!(cursor, flags == expect_flags, "Unexpected flags {flags:X}");

            if flags == RTF_UNKNOWN {
                let b = cursor.read_u8()?;
                assert_eq!(b, 0);
            }

            let id = cursor.read_string()?;
            let pattern = cursor.read_string()?;
            let arg_count = cursor.read_u32::<LittleEndian>()?;

            let mut text_format = vec![];
            for _ in 0..arg_count {
                let format_key = cursor.read_string()?;
                let separator = cursor.read_u8()?;
                validate!(cursor, separator == 4, "Unexpected separator {separator}");
                let content_type = cursor.read_u32::<LittleEndian>()?;
                let indicator = cursor.read_u8()?;
                validate!(cursor, indicator == 255, "Unexpected indicator {indicator}");
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

            let mut strings: Vec<String> = vec![];
            for _ in 0..count {
                let str = cursor.read_string()?;
                strings.push(str)
            }

            Ok(TextProperty::Simple(strings))
        } else if component_type == 8 {
            let unknown = cursor.read_string()?;
            let guid = cursor.read_string()?;
            let value = cursor.read_string()?;

            Ok(TextProperty::Type8(unknown, guid, value))
        } else {
            // Unknown text
            Err(DeserializeError::InvalidProperty(
                format!("Unexpected component_type {}", component_type),
                cursor.stream_position()?,
            ))?
        }
    }

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        match self {
            TextProperty::Empty(_) => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u8(255)?;
                cursor.write_u32::<LittleEndian>(0)?;
            }

            TextProperty::Rich(value) => {
                cursor.write_u32::<LittleEndian>(1)?;
                cursor.write_u8(3)?;
                cursor.write_u8(8)?;
                if options.large_world_coordinates {
                    cursor.write_u64::<LittleEndian>(RTF_UNKNOWN)?;
                    cursor.write_u8(0)?;
                } else {
                    cursor.write_u64::<LittleEndian>(0)?;
                }
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
                cursor.write_u32::<LittleEndian>(values.len() as u32)?;
                for value in values {
                    cursor.write_string(value)?;
                }
            }

            TextProperty::Type8(unknown, guid, value) => {
                cursor.write_u32::<LittleEndian>(8)?;
                cursor.write_u8(0)?;
                cursor.write_string(unknown)?;
                cursor.write_string(guid)?;
                cursor.write_string(value)?;
            }
        }

        Ok(())
    }
}

impl Debug for TextProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextProperty::Rich(value) => value.fmt(f),
            TextProperty::Simple(values) => f.debug_list().entries(values).finish(),
            TextProperty::Empty(_) => f.write_str("Empty"),
            TextProperty::Type8(_, _, value) => value.fmt(f),
        }
    }
}

impl RichText {
    /// Creates a new `RichText` instance.
    #[inline]
    pub fn new(id: String, pattern: String, text_format: Vec<RichTextFormat>) -> Self {
        RichText {
            id,
            pattern,
            text_format,
        }
    }
}

impl RichTextFormat {
    /// Creates a new `RichTextFormat` instance.
    #[inline]
    pub fn new(format_key: String, content_type: u32, values: Vec<String>) -> Self {
        RichTextFormat {
            format_key,
            content_type,
            values,
        }
    }
}
