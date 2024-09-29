use std::hash::{Hash, Hasher};
use std::{
    fmt::Debug,
    io::{Cursor, Read, Seek, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use indexmap::IndexMap;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use ordered_float::OrderedFloat;
use unreal_helpers::{UnrealReadExt, UnrealWriteExt};

use crate::custom_version::FEditorObjectVersion;
use crate::properties::int_property::UInt64Property;
use crate::properties::struct_types::DateTime;
use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::Error,
};

use super::{impl_read, impl_read_header, impl_write, PropertyOptions, PropertyTrait};

/// A property that stores GVAS Text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextProperty {
    /// Value
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub value: FText,
}

impl TextProperty {
    /// Create a new [`TextProperty`]
    pub fn new(value: FText) -> Self {
        TextProperty { value }
    }

    #[inline]
    pub(crate) fn read_body<R: Read + Seek>(
        cursor: &mut R,
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        let value = FText::read(cursor, options)?;
        Ok(TextProperty { value })
    }

    impl_read!(options);
    impl_read_header!(options);
}

impl PropertyTrait for TextProperty {
    impl_write!(TextProperty);

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        let len = self.value.write(cursor, options)?;
        Ok(len)
    }
}

/// FText
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FText {
    /// Text flags
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "is_zero"))]
    pub flags: u32,
    /// Text history
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub history: FTextHistory,
}

#[cfg(feature = "serde")]
#[inline]
fn is_zero(num: &u32) -> bool {
    *num == 0
}

impl FText {
    /// Create a new [`FText`] of none type
    pub fn new_none(flags: u32, culture_invariant_string: Option<Option<String>>) -> Self {
        FText {
            flags,
            history: match culture_invariant_string {
                Some(culture_invariant_string) => FTextHistory::None {
                    culture_invariant_string,
                },
                None => FTextHistory::Empty {},
            },
        }
    }

    /// Create a new [`FText`] of base type
    pub fn new_base(
        flags: u32,
        namespace: Option<String>,
        key: Option<String>,
        source_string: Option<String>,
    ) -> Self {
        FText {
            flags,
            history: FTextHistory::Base {
                namespace,
                key,
                source_string,
            },
        }
    }

    /// Read [`FText`] from a cursor
    #[inline]
    pub fn read<R: Read + Seek>(cursor: &mut R, options: &PropertyOptions) -> Result<Self, Error> {
        let flags = cursor.read_u32::<LittleEndian>()?;
        let history = FTextHistory::read(cursor, options)?;

        Ok(FText { flags, history })
    }

    /// Write [`FText`] to a cursor
    #[inline]
    pub fn write<W: Write>(
        &self,
        cursor: &mut W,
        options: &PropertyOptions,
    ) -> Result<usize, Error> {
        let mut len = 4;
        cursor.write_u32::<LittleEndian>(self.flags)?;
        len += self.history.write(cursor, options)?;
        Ok(len)
    }
}

/// Text history type
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(i8)]
pub enum TextHistoryType {
    /// None
    #[default]
    None = -1,
    /// Base
    Base = 0,
    /// Named format
    NamedFormat,
    /// Ordered format
    OrderedFormat,
    /// Argument format
    ArgumentFormat,
    /// As number
    AsNumber,
    /// As percentage
    AsPercent,
    /// As currency
    AsCurrency,
    /// As date
    AsDate,
    /// As time
    AsTime,
    /// As datetime
    AsDateTime,
    /// Transform
    Transform,
    /// String table entry
    StringTableEntry,
    /// Text generator
    TextGenerator,
    /// Uncertain, Back 4 Blood specific serialization
    RawText,
}

/// FText history
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "history"))]
pub enum FTextHistory {
    /// Empty
    Empty {},
    /// None
    None {
        /// Culture invariant string
        culture_invariant_string: Option<String>,
    },
    /// Base text history
    Base {
        /// Namespace
        namespace: Option<String>,
        /// Key
        key: Option<String>,
        /// Source string
        source_string: Option<String>,
    },
    /// Named format text history
    NamedFormat {
        /// Source format
        source_format: Box<FText>,
        /// Arguments
        arguments: IndexMap<String, FormatArgumentValue>,
    },
    /// Ordered format text history
    OrderedFormat {
        /// Source format
        source_format: Box<FText>,
        /// Arguments
        arguments: Vec<FormatArgumentValue>,
    },
    /// Argument format text history
    ArgumentFormat {
        /// Source format
        source_format: Box<FText>,
        /// Arguments
        arguments: Vec<FormatArgumentData>,
    },
    /// Convert to number
    AsNumber {
        /// Source value
        source_value: Box<FormatArgumentValue>,
        /// Format options
        format_options: Option<NumberFormattingOptions>,
        /// Target culture
        target_culture: Option<String>,
    },
    /// Convert to percentage
    AsPercent {
        /// Source value
        source_value: Box<FormatArgumentValue>,
        /// Format options
        format_options: Option<NumberFormattingOptions>,
        /// Target culture
        target_culture: Option<String>,
    },
    /// Convert to currency
    AsCurrency {
        /// Currency code
        currency_code: Option<String>,
        /// Source value
        source_value: Box<FormatArgumentValue>,
        /// Format options
        format_options: Option<NumberFormattingOptions>,
        /// Target culture
        target_culture: Option<String>,
    },
    /// Convert to date
    AsDate {
        /// Date time
        date_time: DateTime,
        /// Date style
        date_style: DateTimeStyle,
        // todo: FTEXT_HISTORY_DATE_TIMEZONE support (needs object version)
        /// Target culture
        target_culture: String,
    },
    /// Convert to time
    AsTime {
        /// Source date time
        source_date_time: DateTime,
        /// Time style
        time_style: DateTimeStyle,
        /// Time zone
        time_zone: String,
        /// Target culture
        target_culture: String,
    },
    /// Convert to date time
    AsDateTime {
        /// Source date time
        source_date_time: DateTime,
        /// Date style
        date_style: DateTimeStyle,
        /// Time style
        time_style: DateTimeStyle,
        /// Time zone
        time_zone: String,
        /// Target culture
        target_culture: String,
    },
    /// Transform text
    Transform {
        /// Source text
        source_text: Box<FText>,
        /// Transform type
        #[cfg_attr(feature = "serde", serde(flatten))]
        transform_type: TransformType,
    },
    /// String table entry
    StringTableEntry {
        /// Table id
        table_id: Box<FText>,
        /// Key
        key: String,
    },
}

impl FTextHistory {
    /// Read [`FTextHistory`] from a cursor
    #[inline]
    pub fn read<R: Read + Seek>(cursor: &mut R, options: &PropertyOptions) -> Result<Self, Error> {
        let history_type = cursor.read_enum()?;

        Ok(match history_type {
            TextHistoryType::None => {
                if options.supports_version(
                    FEditorObjectVersion::CultureInvariantTextSerializationKeyStability,
                ) {
                    let has_culture_invariant_string = cursor.read_b32()?;
                    if has_culture_invariant_string {
                        let culture_invariant_string = cursor.read_fstring()?;
                        FTextHistory::None {
                            culture_invariant_string,
                        }
                    } else {
                        FTextHistory::Empty {}
                    }
                } else {
                    FTextHistory::Empty {}
                }
            }
            TextHistoryType::Base => {
                let namespace = cursor.read_fstring()?;
                let key = cursor.read_fstring()?;
                let source_string = cursor.read_fstring()?;

                FTextHistory::Base {
                    namespace,
                    key,
                    source_string,
                }
            }
            TextHistoryType::NamedFormat => {
                let source_format = Box::new(FText::read(cursor, options)?);

                let argument_count = cursor.read_i32::<LittleEndian>()?;
                let mut arguments = IndexMap::with_capacity(argument_count as usize);

                for _ in 0..argument_count {
                    let key = cursor.read_string()?;
                    let value = FormatArgumentValue::read(cursor, options)?;
                    arguments.insert(key, value);
                }

                FTextHistory::NamedFormat {
                    source_format,
                    arguments,
                }
            }
            TextHistoryType::OrderedFormat => {
                let source_format = Box::new(FText::read(cursor, options)?);

                let count = cursor.read_i32::<LittleEndian>()?;
                let mut arguments = Vec::with_capacity(count as usize);

                for _ in 0..count {
                    arguments.push(FormatArgumentValue::read(cursor, options)?);
                }

                FTextHistory::OrderedFormat {
                    source_format,
                    arguments,
                }
            }
            TextHistoryType::ArgumentFormat => {
                let source_format = Box::new(FText::read(cursor, options)?);
                let count = cursor.read_i32::<LittleEndian>()?;
                let mut arguments = Vec::with_capacity(count as usize);

                for _ in 0..count {
                    arguments.push(FormatArgumentData::read(cursor, options)?);
                }

                FTextHistory::ArgumentFormat {
                    source_format,
                    arguments,
                }
            }
            TextHistoryType::AsNumber => {
                let source_value = Box::new(FormatArgumentValue::read(cursor, options)?);

                let has_format_options = cursor.read_b32()?;
                let format_options = if has_format_options {
                    Some(NumberFormattingOptions::read(cursor)?)
                } else {
                    None
                };

                let target_culture = cursor.read_fstring()?;

                FTextHistory::AsNumber {
                    source_value,
                    format_options,
                    target_culture,
                }
            }
            TextHistoryType::AsPercent => {
                let source_value = Box::new(FormatArgumentValue::read(cursor, options)?);

                let has_format_options = cursor.read_b32()?;
                let format_options = if has_format_options {
                    Some(NumberFormattingOptions::read(cursor)?)
                } else {
                    None
                };

                let target_culture = cursor.read_fstring()?;

                FTextHistory::AsPercent {
                    source_value,
                    format_options,
                    target_culture,
                }
            }
            TextHistoryType::AsCurrency => {
                let currency_code = cursor.read_fstring()?;

                let source_value = Box::new(FormatArgumentValue::read(cursor, options)?);

                let has_format_options = cursor.read_b32()?;
                let format_options = if has_format_options {
                    Some(NumberFormattingOptions::read(cursor)?)
                } else {
                    None
                };

                let target_culture = cursor.read_fstring()?;

                FTextHistory::AsCurrency {
                    currency_code,
                    source_value,
                    format_options,
                    target_culture,
                }
            }
            TextHistoryType::AsDate => {
                let date_time = DateTime {
                    ticks: UInt64Property::read(cursor, false)?.value,
                };
                let date_style = cursor.read_enum()?;
                let target_culture = cursor.read_string()?;

                FTextHistory::AsDate {
                    date_time,
                    date_style,
                    target_culture,
                }
            }
            TextHistoryType::AsTime => {
                let source_date_time = DateTime {
                    ticks: UInt64Property::read(cursor, false)?.value,
                };
                let time_style = cursor.read_enum()?;
                let time_zone = cursor.read_string()?;
                let target_culture = cursor.read_string()?;

                FTextHistory::AsTime {
                    source_date_time,
                    time_style,
                    time_zone,
                    target_culture,
                }
            }
            TextHistoryType::AsDateTime => {
                let source_date_time = DateTime {
                    ticks: UInt64Property::read(cursor, false)?.value,
                };
                let date_style = cursor.read_enum()?;
                let time_style = cursor.read_enum()?;
                let time_zone = cursor.read_string()?;
                let target_culture = cursor.read_string()?;

                FTextHistory::AsDateTime {
                    source_date_time,
                    date_style,
                    time_style,
                    time_zone,
                    target_culture,
                }
            }
            TextHistoryType::Transform => {
                let source_text = Box::new(FText::read(cursor, options)?);
                let transform_type = cursor.read_enum()?;

                FTextHistory::Transform {
                    source_text,
                    transform_type,
                }
            }
            TextHistoryType::StringTableEntry => {
                let table_id = Box::new(FText::read(cursor, options)?);
                let key = cursor.read_string()?;

                FTextHistory::StringTableEntry { table_id, key }
            }
            _ => unimplemented!("unimplemented history type: {:?}", history_type),
        })
    }

    /// Write [`FTextHistory`] to a cursor
    #[inline]
    pub fn write<W: Write>(
        &self,
        cursor: &mut W,
        options: &PropertyOptions,
    ) -> Result<usize, Error> {
        match self {
            FTextHistory::Empty {} => {
                let mut len = 1;
                cursor.write_enum(TextHistoryType::None)?;
                if options.supports_version(
                    FEditorObjectVersion::CultureInvariantTextSerializationKeyStability,
                ) {
                    len += 4;
                    cursor.write_b32(false)?;
                }
                Ok(len)
            }

            FTextHistory::None {
                culture_invariant_string,
            } => {
                let mut len = 1;
                cursor.write_enum(TextHistoryType::None)?;
                if options.supports_version(
                    FEditorObjectVersion::CultureInvariantTextSerializationKeyStability,
                ) {
                    len += 4;
                    cursor.write_b32(true)?;
                    len += cursor.write_fstring(culture_invariant_string.as_deref())?;
                }
                Ok(len)
            }

            FTextHistory::Base {
                namespace,
                key,
                source_string,
            } => {
                let mut len = 1;
                cursor.write_enum(TextHistoryType::Base)?;
                len += cursor.write_fstring(namespace.as_deref())?;
                len += cursor.write_fstring(key.as_deref())?;
                len += cursor.write_fstring(source_string.as_deref())?;
                Ok(len)
            }

            FTextHistory::NamedFormat {
                source_format,
                arguments,
            } => {
                let mut len = 1;
                cursor.write_enum(TextHistoryType::NamedFormat)?;
                len += source_format.write(cursor, options)?;
                len += 4;
                cursor.write_i32::<LittleEndian>(arguments.len() as i32)?;
                for (key, value) in arguments {
                    len += cursor.write_string(key)?;
                    len += value.write(cursor, options)?;
                }
                Ok(len)
            }

            FTextHistory::OrderedFormat {
                source_format,
                arguments,
            } => {
                let mut len = 1;
                cursor.write_enum(TextHistoryType::OrderedFormat)?;
                len += source_format.write(cursor, options)?;
                len += 4;
                cursor.write_i32::<LittleEndian>(arguments.len() as i32)?;
                for argument in arguments {
                    len += argument.write(cursor, options)?;
                }
                Ok(len)
            }

            FTextHistory::ArgumentFormat {
                source_format,
                arguments,
            } => {
                let mut len = 1;
                cursor.write_enum(TextHistoryType::ArgumentFormat)?;
                len += source_format.write(cursor, options)?;
                len += 4;
                cursor.write_i32::<LittleEndian>(arguments.len() as i32)?;
                for argument in arguments {
                    len += argument.write(cursor, options)?;
                }
                Ok(len)
            }

            FTextHistory::AsNumber {
                source_value,
                format_options,
                target_culture,
            } => {
                let mut len = 1;
                cursor.write_enum(TextHistoryType::AsNumber)?;
                len += source_value.write(cursor, options)?;
                len += 4;
                cursor.write_b32(format_options.is_some())?;
                if let Some(format_options) = format_options {
                    len += format_options.write(cursor)?;
                };
                len += cursor.write_fstring(target_culture.as_deref())?;
                Ok(len)
            }

            FTextHistory::AsPercent {
                source_value,
                format_options,
                target_culture,
            } => {
                let mut len = 1;
                cursor.write_enum(TextHistoryType::AsPercent)?;
                len += source_value.write(cursor, options)?;
                len += 4;
                cursor.write_b32(format_options.is_some())?;
                if let Some(format_options) = format_options {
                    len += format_options.write(cursor)?;
                }
                len += cursor.write_fstring(target_culture.as_deref())?;
                Ok(len)
            }

            FTextHistory::AsCurrency {
                currency_code,
                source_value,
                format_options,
                target_culture,
            } => {
                let mut len = 0;
                len += cursor.write_fstring(currency_code.as_deref())?;
                len += source_value.write(cursor, options)?;
                len += 4;
                cursor.write_b32(format_options.is_some())?;
                if let Some(format_options) = format_options {
                    len += format_options.write(cursor)?;
                }
                len += cursor.write_fstring(target_culture.as_deref())?;
                Ok(len)
            }

            FTextHistory::AsDate {
                date_time,
                date_style,
                target_culture,
            } => {
                cursor.write_enum(TextHistoryType::AsDate)?;
                cursor.write_u64::<LittleEndian>(date_time.ticks)?;
                cursor.write_enum(*date_style)?;
                let mut len = 10;
                len += cursor.write_string(target_culture)?;
                Ok(len)
            }

            FTextHistory::AsTime {
                source_date_time,
                time_style,
                time_zone,
                target_culture,
            } => {
                cursor.write_enum(TextHistoryType::AsTime)?;
                cursor.write_u64::<LittleEndian>(source_date_time.ticks)?;
                cursor.write_enum(*time_style)?;
                let mut len = 10;
                len += cursor.write_string(time_zone)?;
                len += cursor.write_string(target_culture)?;
                Ok(len)
            }

            FTextHistory::AsDateTime {
                source_date_time,
                date_style,
                time_style,
                time_zone,
                target_culture,
            } => {
                cursor.write_enum(TextHistoryType::AsDateTime)?;
                cursor.write_u64::<LittleEndian>(source_date_time.ticks)?;
                cursor.write_enum(*date_style)?;
                cursor.write_enum(*time_style)?;
                let mut len = 11;
                len += cursor.write_string(time_zone.as_str())?;
                len += cursor.write_string(target_culture.as_str())?;
                Ok(len)
            }

            FTextHistory::Transform {
                source_text,
                transform_type,
            } => {
                cursor.write_enum(TextHistoryType::Transform)?;
                let mut len = 2;
                len += source_text.write(cursor, options)?;
                cursor.write_enum(*transform_type)?;
                Ok(len)
            }

            FTextHistory::StringTableEntry { table_id, key } => {
                let mut len = 0;
                len += table_id.write(cursor, options)?;
                len += cursor.write_string(key)?;
                Ok(len)
            }
        }
    }
}

impl Hash for FTextHistory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            FTextHistory::Empty {} => {
                state.write_u8(0);
            }
            FTextHistory::None {
                culture_invariant_string,
            } => {
                state.write_u8(0);
                culture_invariant_string.hash(state);
            }
            FTextHistory::Base {
                namespace,
                key,
                source_string,
            } => {
                state.write_u8(1);
                namespace.hash(state);
                key.hash(state);
                source_string.hash(state);
            }
            FTextHistory::NamedFormat {
                source_format,
                arguments,
            } => {
                state.write_u8(2);
                source_format.hash(state);
                for (key, value) in arguments {
                    key.hash(state);
                    value.hash(state);
                }
            }
            FTextHistory::OrderedFormat {
                source_format,
                arguments,
            } => {
                state.write_u8(3);
                source_format.hash(state);
                arguments.hash(state);
            }
            FTextHistory::ArgumentFormat {
                source_format,
                arguments,
            } => {
                state.write_u8(4);
                source_format.hash(state);
                arguments.hash(state);
            }
            FTextHistory::AsNumber {
                source_value,
                format_options,
                target_culture,
            } => {
                state.write_u8(5);
                source_value.hash(state);
                format_options.hash(state);
                target_culture.hash(state);
            }
            FTextHistory::AsPercent {
                source_value,
                format_options,
                target_culture,
            } => {
                state.write_u8(6);
                source_value.hash(state);
                format_options.hash(state);
                target_culture.hash(state);
            }
            FTextHistory::AsCurrency {
                currency_code,
                source_value,
                format_options,
                target_culture,
            } => {
                state.write_u8(7);
                currency_code.hash(state);
                source_value.hash(state);
                format_options.hash(state);
                target_culture.hash(state);
            }
            FTextHistory::AsDate {
                date_time,
                date_style,
                target_culture,
            } => {
                state.write_u8(8);
                date_time.hash(state);
                date_style.hash(state);
                target_culture.hash(state);
            }
            FTextHistory::AsTime {
                source_date_time,
                time_style,
                time_zone,
                target_culture,
            } => {
                state.write_u8(9);
                source_date_time.hash(state);
                time_style.hash(state);
                time_zone.hash(state);
                target_culture.hash(state);
            }
            FTextHistory::AsDateTime {
                source_date_time,
                date_style,
                time_style,
                time_zone,
                target_culture,
            } => {
                state.write_u8(10);
                source_date_time.hash(state);
                date_style.hash(state);
                time_style.hash(state);
                time_zone.hash(state);
                target_culture.hash(state);
            }
            FTextHistory::Transform {
                source_text,
                transform_type,
            } => {
                state.write_u8(11);
                source_text.hash(state);
                transform_type.hash(state);
            }
            FTextHistory::StringTableEntry { table_id, key } => {
                state.write_u8(12);
                table_id.hash(state);
                key.hash(state);
            }
        }
    }
}

/// Format argument type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(i8)]
pub enum FormatArgumentType {
    /// Integer (32 bit in most games, 64 bit in Hogwarts Legacy)
    Int,
    /// Unsigned integer (32 bit)
    UInt,
    /// Floating point number (32 bit)
    Float,
    /// Floating point number (64 bit)
    Double,
    /// FText
    Text,
    /// ?
    Gender,
}

/// Format argument data
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FormatArgumentData {
    /// Argument name
    pub name: String,
    /// Argument value
    pub value: FormatArgumentValue,
}

impl FormatArgumentData {
    /// Read [`FormatArgumentData`] from a cursor
    #[inline]
    pub fn read<R: Read + Seek>(cursor: &mut R, options: &PropertyOptions) -> Result<Self, Error> {
        let name = cursor.read_string()?;
        let value = FormatArgumentValue::read(cursor, options)?;

        Ok(FormatArgumentData { name, value })
    }

    /// Write [`FormatArgumentData`] to a cursor
    #[inline]
    pub fn write<W: Write>(
        &self,
        cursor: &mut W,
        options: &PropertyOptions,
    ) -> Result<usize, Error> {
        let mut len = 0;
        len += cursor.write_string(&self.name)?;
        len += self.value.write(cursor, options)?;
        Ok(len)
    }
}

/// Format argument value
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FormatArgumentValue {
    /// Integer
    Int(i32),
    /// Unsigned integer
    UInt(u32),
    /// Float
    Float(OrderedFloat<f32>),
    /// Double
    Double(OrderedFloat<f64>),
    /// FText
    Text(FText),
}

impl FormatArgumentValue {
    /// Read [`FormatArgumentValue`] from a cursor
    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        options: &PropertyOptions,
    ) -> Result<Self, Error> {
        let format_argument_type = cursor.read_enum()?;

        Ok(match format_argument_type {
            FormatArgumentType::Int => {
                // todo: hogwarts legacy support
                FormatArgumentValue::Int(cursor.read_i32::<LittleEndian>()?)
            }
            FormatArgumentType::UInt => {
                FormatArgumentValue::UInt(cursor.read_u32::<LittleEndian>()?)
            }
            FormatArgumentType::Float => {
                FormatArgumentValue::Float(cursor.read_f32::<LittleEndian>()?.into())
            }
            FormatArgumentType::Double => {
                FormatArgumentValue::Double(cursor.read_f64::<LittleEndian>()?.into())
            }
            FormatArgumentType::Text => FormatArgumentValue::Text(FText::read(cursor, options)?),
            FormatArgumentType::Gender => unimplemented!(),
        })
    }

    /// Write [`FormatArgumentValue`] to a cursor
    #[inline]
    pub fn write<W: Write>(
        &self,
        cursor: &mut W,
        options: &PropertyOptions,
    ) -> Result<usize, Error> {
        match self {
            FormatArgumentValue::Int(value) => {
                cursor.write_enum(FormatArgumentType::Int)?;
                cursor.write_i32::<LittleEndian>(*value)?;
                Ok(5)
            }
            FormatArgumentValue::UInt(value) => {
                cursor.write_enum(FormatArgumentType::UInt)?;
                cursor.write_u32::<LittleEndian>(*value)?;
                Ok(5)
            }
            FormatArgumentValue::Float(value) => {
                cursor.write_enum(FormatArgumentType::Float)?;
                cursor.write_f32::<LittleEndian>(value.0)?;
                Ok(5)
            }
            FormatArgumentValue::Double(value) => {
                cursor.write_enum(FormatArgumentType::Double)?;
                cursor.write_f64::<LittleEndian>(value.0)?;
                Ok(9)
            }
            FormatArgumentValue::Text(value) => {
                let mut len = 1;
                cursor.write_enum(FormatArgumentType::Text)?;
                len += value.write(cursor, options)?;
                Ok(len)
            }
        }
    }
}

/// Rounding mode
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "rounding"))]
#[repr(i8)]
pub enum RoundingMode {
    /// Rounds to the nearest place, equidistant ties go to the value which is closest to an even value: 1.5 becomes 2, 0.5 becomes 0
    HalfToEven,
    /// Rounds to nearest place, equidistant ties go to the value which is further from zero: -0.5 becomes -1.0, 0.5 becomes 1.0
    HalfFromZero,
    /// Rounds to nearest place, equidistant ties go to the value which is closer to zero: -0.5 becomes 0, 0.5 becomes 0.
    HalfToZero,
    /// Rounds to the value which is further from zero, "larger" in absolute value: 0.1 becomes 1, -0.1 becomes -1
    FromZero,
    /// Rounds to the value which is closer to zero, "smaller" in absolute value: 0.1 becomes 0, -0.1 becomes 0
    ToZero,
    /// Rounds to the value which is more negative: 0.1 becomes 0, -0.1 becomes -1
    ToNegativeInfinity,
    /// Rounds to the value which is more positive: 0.1 becomes 1, -0.1 becomes 0
    ToPositiveInfinity,
}

/// Number formatting options
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberFormattingOptions {
    /// Always include sign
    pub always_include_sign: bool,
    /// Use grouping
    pub use_grouping: bool,
    /// Rounding mode
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub rounding_mode: RoundingMode,
    /// Minimum integral digits
    pub minimum_integral_digits: i32,
    /// Maximum integral digits
    pub maximum_integral_digits: i32,
    /// Minimum fractional digits
    pub minimum_fractional_digits: i32,
    /// Maximum fractional digits
    pub maximum_fractional_digits: i32,
}

impl NumberFormattingOptions {
    /// Read [`NumberFormattingOptions`] from a cursor
    #[inline]
    pub fn read<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let always_include_sign = cursor.read_b32()?;
        let use_grouping = cursor.read_b32()?;
        let rounding_mode = cursor.read_enum()?;
        let minimum_integral_digits = cursor.read_i32::<LittleEndian>()?;
        let maximum_integral_digits = cursor.read_i32::<LittleEndian>()?;
        let minimum_fractional_digits = cursor.read_i32::<LittleEndian>()?;
        let maximum_fractional_digits = cursor.read_i32::<LittleEndian>()?;

        Ok(NumberFormattingOptions {
            always_include_sign,
            use_grouping,
            rounding_mode,
            minimum_integral_digits,
            maximum_integral_digits,
            minimum_fractional_digits,
            maximum_fractional_digits,
        })
    }

    /// Write [`NumberFormattingOptions`] to a cursor
    #[inline]
    pub fn write<W: Write>(&self, cursor: &mut W) -> Result<usize, Error> {
        cursor.write_b32(self.always_include_sign)?;
        cursor.write_b32(self.use_grouping)?;
        cursor.write_enum(self.rounding_mode)?;
        cursor.write_i32::<LittleEndian>(self.minimum_integral_digits)?;
        cursor.write_i32::<LittleEndian>(self.maximum_integral_digits)?;
        cursor.write_i32::<LittleEndian>(self.minimum_fractional_digits)?;
        cursor.write_i32::<LittleEndian>(self.maximum_fractional_digits)?;

        Ok(25)
    }
}

/// Date time style
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(i8)]
pub enum DateTimeStyle {
    /// Default
    Default,
    /// Short
    Short,
    /// Medium
    Medium,
    /// Long
    Long,
    /// Full
    Full,
}

/// Transform type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "transform"))]
#[repr(i8)]
pub enum TransformType {
    /// To lowercase
    ToLower = 0,
    /// To uppercase
    ToUpper,
}
