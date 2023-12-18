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
    error::{DeserializeError, Error},
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

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        self.value.write(cursor, options)
    }
}
impl_write!(TextProperty, options);

/// FText
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FText {
    /// Text flags
    pub flags: u32,
    /// Text history
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub history: FTextHistory,
}

impl FText {
    /// Create a new [`FText`] of none type
    pub fn new_none(flags: u32, culture_invariant_string: Option<Option<String>>) -> Self {
        FText {
            flags,
            history: FTextHistory::None {
                culture_invariant_string,
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
    pub fn read<R: Read + Seek>(
        cursor: &mut R,
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        let flags = cursor.read_u32::<LittleEndian>()?;
        let history = FTextHistory::read(cursor, options)?;

        Ok(FText { flags, history })
    }

    /// Write [`FText`] to a cursor
    pub fn write<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        cursor.write_u32::<LittleEndian>(self.flags)?;
        self.history.write(cursor, options)
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
    /// None
    None {
        /// Culture invariant string
        culture_invariant_string: Option<Option<String>>,
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
    pub fn read<R: Read + Seek>(
        cursor: &mut R,
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        let history_type =
            TextHistoryType::try_from(cursor.read_i8()?).map_err(DeserializeError::from)?;

        Ok(match history_type {
            TextHistoryType::None => {
                let culture_invariant_string = if options.supports_version(
                    FEditorObjectVersion::CultureInvariantTextSerializationKeyStability,
                ) {
                    let has_culture_invariant_string = cursor.read_b32()?;
                    if has_culture_invariant_string {
                        Some(cursor.read_fstring()?)
                    } else {
                        None
                    }
                } else {
                    None
                };

                FTextHistory::None {
                    culture_invariant_string,
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
                let date_style =
                    DateTimeStyle::try_from(cursor.read_i8()?).map_err(DeserializeError::from)?;
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
                let time_style =
                    DateTimeStyle::try_from(cursor.read_i8()?).map_err(DeserializeError::from)?;
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
                let date_style =
                    DateTimeStyle::try_from(cursor.read_i8()?).map_err(DeserializeError::from)?;
                let time_style =
                    DateTimeStyle::try_from(cursor.read_i8()?).map_err(DeserializeError::from)?;
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
                let transform_type =
                    TransformType::try_from(cursor.read_u8()?).map_err(DeserializeError::from)?;

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
    pub fn write<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        match self {
            FTextHistory::None {
                culture_invariant_string,
            } => {
                cursor.write_i8(TextHistoryType::None as i8)?;

                if options.supports_version(
                    FEditorObjectVersion::CultureInvariantTextSerializationKeyStability,
                ) {
                    cursor.write_b32(culture_invariant_string.is_some())?;

                    if let Some(culture_invariant_string) = culture_invariant_string {
                        cursor.write_fstring(culture_invariant_string.as_deref())?;
                    }
                }
            }
            FTextHistory::Base {
                namespace,
                key,
                source_string,
            } => {
                cursor.write_i8(TextHistoryType::Base as i8)?;

                cursor.write_fstring(namespace.as_deref())?;
                cursor.write_fstring(key.as_deref())?;
                cursor.write_fstring(source_string.as_deref())?;
            }
            FTextHistory::NamedFormat {
                source_format,
                arguments,
            } => {
                cursor.write_i8(TextHistoryType::NamedFormat as i8)?;

                source_format.write(cursor, options)?;

                cursor.write_i32::<LittleEndian>(arguments.len() as i32)?;
                for (key, value) in arguments {
                    cursor.write_string(key.as_ref())?;
                    value.write(cursor, options)?;
                }
            }
            FTextHistory::OrderedFormat {
                source_format,
                arguments,
            } => {
                cursor.write_i8(TextHistoryType::OrderedFormat as i8)?;

                source_format.write(cursor, options)?;

                cursor.write_i32::<LittleEndian>(arguments.len() as i32)?;
                for argument in arguments {
                    argument.write(cursor, options)?;
                }
            }
            FTextHistory::ArgumentFormat {
                source_format,
                arguments,
            } => {
                cursor.write_i8(TextHistoryType::ArgumentFormat as i8)?;

                source_format.write(cursor, options)?;

                cursor.write_i32::<LittleEndian>(arguments.len() as i32)?;
                for argument in arguments {
                    argument.write(cursor, options)?;
                }
            }
            FTextHistory::AsNumber {
                source_value,
                format_options,
                target_culture,
            } => {
                cursor.write_i8(TextHistoryType::AsNumber as i8)?;

                source_value.write(cursor, options)?;

                cursor.write_b32(format_options.is_some())?;
                if let Some(format_options) = format_options {
                    format_options.write(cursor)?;
                };

                cursor.write_fstring(target_culture.as_deref())?;
            }
            FTextHistory::AsPercent {
                source_value,
                format_options,
                target_culture,
            } => {
                cursor.write_i8(TextHistoryType::AsPercent as i8)?;

                source_value.write(cursor, options)?;

                cursor.write_b32(format_options.is_some())?;
                if let Some(format_options) = format_options {
                    format_options.write(cursor)?;
                }

                cursor.write_fstring(target_culture.as_deref())?;
            }
            FTextHistory::AsCurrency {
                currency_code,
                source_value,
                format_options,
                target_culture,
            } => {
                cursor.write_fstring(currency_code.as_deref())?;

                source_value.write(cursor, options)?;

                cursor.write_b32(format_options.is_some())?;
                if let Some(format_options) = format_options {
                    format_options.write(cursor)?;
                }

                cursor.write_fstring(target_culture.as_deref())?;
            }
            FTextHistory::AsDate {
                date_time,
                date_style,
                target_culture,
            } => {
                cursor.write_i8(TextHistoryType::AsDate as i8)?;

                cursor.write_u64::<LittleEndian>(date_time.ticks)?;
                cursor.write_i8(*date_style as i8)?;

                cursor.write_string(target_culture.as_ref())?;
            }
            FTextHistory::AsTime {
                source_date_time,
                time_style,
                time_zone,
                target_culture,
            } => {
                cursor.write_i8(TextHistoryType::AsTime as i8)?;

                cursor.write_u64::<LittleEndian>(source_date_time.ticks)?;
                cursor.write_i8(*time_style as i8)?;
                cursor.write_string(time_zone.as_str())?;
                cursor.write_string(target_culture.as_str())?;
            }
            FTextHistory::AsDateTime {
                source_date_time,
                date_style,
                time_style,
                time_zone,
                target_culture,
            } => {
                cursor.write_i8(TextHistoryType::AsDateTime as i8)?;

                cursor.write_u64::<LittleEndian>(source_date_time.ticks)?;
                cursor.write_i8(*date_style as i8)?;
                cursor.write_i8(*time_style as i8)?;
                cursor.write_string(time_zone.as_str())?;
                cursor.write_string(target_culture.as_str())?;
            }
            FTextHistory::Transform {
                source_text,
                transform_type,
            } => {
                cursor.write_i8(TextHistoryType::Transform as i8)?;

                source_text.write(cursor, options)?;
                cursor.write_u8(*transform_type as u8)?;
            }
            FTextHistory::StringTableEntry { .. } => {}
        };

        Ok(())
    }
}

impl Hash for FTextHistory {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
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
    pub fn read<R: Read + Seek>(
        cursor: &mut R,
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        let name = cursor.read_string()?;
        let value = FormatArgumentValue::read(cursor, options)?;

        Ok(FormatArgumentData { name, value })
    }

    /// Write [`FormatArgumentData`] to a cursor
    pub fn write<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        cursor.write_string(self.name.as_ref())?;
        self.value.write(cursor, options)
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
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        let format_argument_type =
            FormatArgumentType::try_from(cursor.read_i8()?).map_err(DeserializeError::from)?;

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
    pub fn write<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        match self {
            FormatArgumentValue::Int(value) => {
                cursor.write_i8(FormatArgumentType::Int as i8)?;
                cursor.write_i32::<LittleEndian>(*value)?;
            }
            FormatArgumentValue::UInt(value) => {
                cursor.write_i8(FormatArgumentType::UInt as i8)?;
                cursor.write_u32::<LittleEndian>(*value)?;
            }
            FormatArgumentValue::Float(value) => {
                cursor.write_i8(FormatArgumentType::Float as i8)?;
                cursor.write_f32::<LittleEndian>(value.0)?;
            }
            FormatArgumentValue::Double(value) => {
                cursor.write_i8(FormatArgumentType::Double as i8)?;
                cursor.write_f64::<LittleEndian>(value.0)?;
            }
            FormatArgumentValue::Text(value) => {
                cursor.write_i8(FormatArgumentType::Text as i8)?;
                value.write(cursor, options)?;
            }
        };

        Ok(())
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
    pub fn read<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let always_include_sign = cursor.read_b32()?;
        let use_grouping = cursor.read_b32()?;
        let rounding_mode =
            RoundingMode::try_from(cursor.read_i8()?).map_err(DeserializeError::from)?;
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
    pub fn write<W: Write>(&self, cursor: &mut W) -> Result<(), Error> {
        cursor.write_b32(self.always_include_sign)?;
        cursor.write_b32(self.use_grouping)?;
        cursor.write_i8(self.rounding_mode as i8)?;
        cursor.write_i32::<LittleEndian>(self.minimum_integral_digits)?;
        cursor.write_i32::<LittleEndian>(self.maximum_integral_digits)?;
        cursor.write_i32::<LittleEndian>(self.minimum_fractional_digits)?;
        cursor.write_i32::<LittleEndian>(self.maximum_fractional_digits)?;

        Ok(())
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
#[repr(u8)]
pub enum TransformType {
    /// To lowercase
    ToLower = 0,
    /// To uppercase
    ToUpper,
}
