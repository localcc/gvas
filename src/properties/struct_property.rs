use std::{
    fmt::Debug,
    hash::Hash,
    io::{Cursor, Read, Seek, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::{DeserializeError, Error, SerializeError},
    make_matcher,
    scoped_stack_entry::ScopedStackEntry,
    types::Guid,
};

use super::{
    int_property::{DoubleProperty, FloatProperty, IntProperty, UInt32Property, UInt64Property},
    struct_types::{DateTime, IntPoint, QuatD, QuatF, RotatorD, RotatorF, VectorD, VectorF},
    Property, PropertyOptions, PropertyTrait,
};

macro_rules! validate {
    ($cond:expr, $($arg:tt)+) => {{
        if !$cond {
            Err(SerializeError::InvalidValue(
                format!($($arg)+),
            ))?
        } else {
            Ok(())
        }
    }};
}

/// A property that holds a struct value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructProperty {
    /// The unique identifier of the property.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Guid::is_zero"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub guid: Guid,
    /// The value of the property.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub value: StructPropertyValue,
}

/// The possible values of a `StructProperty`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StructPropertyValue {
    /// A `VectorF` value.
    VectorF(VectorF),
    /// A `VectorD` value.
    VectorD(VectorD),
    /// A `RotatorF` value.
    RotatorF(RotatorF),
    /// A `RotatorD` value.
    RotatorD(RotatorD),
    /// A `QuatF` value.
    QuatF(QuatF),
    /// A `QuatD` value.
    QuatD(QuatD),
    /// A `DateTime` value.
    DateTime(DateTime),
    /// A `Guid` value.
    Guid(Guid),
    /// An `IntPoint` value.
    IntPoint(IntPoint),
    /// A custom struct value.
    CustomStruct(String, Vec<(String, Property)>),
}

impl StructProperty {
    /// Creates a new `StructProperty` instance.
    #[inline]
    pub fn new(guid: Guid, value: StructPropertyValue) -> Self {
        StructProperty { guid, value }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        include_header: bool,
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        if include_header {
            Ok(Self::read_real(cursor, true, None, options)?)
        } else {
            let struct_path = options.properties_stack.join(".");
            let Some(hint) = options.hints.get(&struct_path) else {
                        Err(DeserializeError::MissingHint(
                            "StructProperty".to_string(),
                            struct_path,
                            cursor.stream_position()?,
                        ))?
                    };
            let hint = &hint.clone();
            Self::read_with_type_name(cursor, hint, options)
        }
    }

    fn read_real<R: Read + Seek>(
        cursor: &mut R,
        include_header: bool,
        type_name: Option<String>,
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        if include_header {
            let _length = cursor.read_u64::<LittleEndian>()?;
        }

        let type_name = match include_header {
            true => cursor.read_string()?,
            false => match type_name {
                Some(t) => t,
                None => Err(DeserializeError::missing_argument("type_name", cursor))?,
            },
        };

        let guid = match include_header {
            true => cursor.read_guid()?,
            false => Guid::default(),
        };

        if include_header {
            let separator = cursor.read_u8()?;
            assert_eq!(separator, 0);
        }

        let value = match type_name.as_str() {
            "Vector" => match options.large_world_coordinates {
                true => StructPropertyValue::VectorD(VectorD::new(
                    DoubleProperty::read(cursor, false)?.value.0,
                    DoubleProperty::read(cursor, false)?.value.0,
                    DoubleProperty::read(cursor, false)?.value.0,
                )),
                false => StructPropertyValue::VectorF(VectorF::new(
                    FloatProperty::read(cursor, false)?.value.0,
                    FloatProperty::read(cursor, false)?.value.0,
                    FloatProperty::read(cursor, false)?.value.0,
                )),
            },
            "Rotator" => match options.large_world_coordinates {
                true => StructPropertyValue::RotatorD(RotatorD::new(
                    DoubleProperty::read(cursor, false)?.value.0,
                    DoubleProperty::read(cursor, false)?.value.0,
                    DoubleProperty::read(cursor, false)?.value.0,
                )),
                false => StructPropertyValue::RotatorF(RotatorF::new(
                    FloatProperty::read(cursor, false)?.value.0,
                    FloatProperty::read(cursor, false)?.value.0,
                    FloatProperty::read(cursor, false)?.value.0,
                )),
            },
            "Quat" => match options.large_world_coordinates {
                true => StructPropertyValue::QuatD(QuatD::new(
                    DoubleProperty::read(cursor, false)?.value.0,
                    DoubleProperty::read(cursor, false)?.value.0,
                    DoubleProperty::read(cursor, false)?.value.0,
                    DoubleProperty::read(cursor, false)?.value.0,
                )),
                false => StructPropertyValue::QuatF(QuatF::new(
                    FloatProperty::read(cursor, false)?.value.0,
                    FloatProperty::read(cursor, false)?.value.0,
                    FloatProperty::read(cursor, false)?.value.0,
                    FloatProperty::read(cursor, false)?.value.0,
                )),
            },
            "DateTime" => StructPropertyValue::DateTime(DateTime {
                ticks: UInt64Property::read(cursor, false)?.value,
            }),
            "IntPoint" => StructPropertyValue::IntPoint(IntPoint {
                x: IntProperty::read(cursor, false)?.value,
                y: IntProperty::read(cursor, false)?.value,
            }),
            "Guid" => StructPropertyValue::Guid(Guid::from((
                UInt32Property::read(cursor, false)?.value,
                UInt32Property::read(cursor, false)?.value,
                UInt32Property::read(cursor, false)?.value,
                UInt32Property::read(cursor, false)?.value,
            ))),
            _ => {
                let mut properties = Vec::new();
                loop {
                    let key_name = cursor.read_string()?;
                    if key_name == "None" {
                        break;
                    }
                    let value_type = cursor.read_string()?;
                    let _property_stack_entry =
                        ScopedStackEntry::new(options.properties_stack, key_name.clone());

                    let property = Property::new(cursor, &value_type, true, options, None)?;
                    properties.push((key_name, property));
                }
                StructPropertyValue::CustomStruct(type_name, properties)
            }
        };

        Ok(StructProperty { guid, value })
    }

    #[inline]
    pub(crate) fn read_with_type_name<R: Read + Seek>(
        cursor: &mut R,
        type_name: &str,
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        Self::read_real(cursor, false, Some(type_name.to_string()), options)
    }

    #[inline]
    fn validate_value(&self, options: &mut PropertyOptions) -> Result<(), Error> {
        match self.value {
            StructPropertyValue::VectorF(_) => validate!(
                !options.large_world_coordinates,
                "VectorF not supported when LWC is enabled, use VectorD",
            ),
            StructPropertyValue::VectorD(_) => validate!(
                options.large_world_coordinates,
                "VectorD not supported when LWC is disabled, use VectorF",
            ),
            StructPropertyValue::RotatorF(_) => validate!(
                !options.large_world_coordinates,
                "RotatorF not supported when LWC is enabled, use RotatorD",
            ),
            StructPropertyValue::RotatorD(_) => validate!(
                options.large_world_coordinates,
                "RotatorD not supported when LWC is disabled, use RotatorF",
            ),
            StructPropertyValue::QuatF(_) => validate!(
                !options.large_world_coordinates,
                "QuatF not supported when LWC is enabled, use QuatD",
            ),
            StructPropertyValue::QuatD(_) => validate!(
                options.large_world_coordinates,
                "QuatD not supported when LWC is disabled, use QuatF",
            ),
            StructPropertyValue::DateTime(_) => Ok(()),
            StructPropertyValue::Guid(_) => Ok(()),
            StructPropertyValue::IntPoint(_) => Ok(()),
            StructPropertyValue::CustomStruct(_, _) => Ok(()),
        }
    }

    #[inline]
    fn get_property_name(&self) -> Result<&str, Error> {
        let property_name = match &self.value {
            StructPropertyValue::VectorF(_) | StructPropertyValue::VectorD(_) => "Vector",
            StructPropertyValue::RotatorF(_) | StructPropertyValue::RotatorD(_) => "Rotator",
            StructPropertyValue::QuatF(_) | StructPropertyValue::QuatD(_) => "Quat",
            StructPropertyValue::DateTime(_) => "DateTime",
            StructPropertyValue::Guid(_) => "Guid",
            StructPropertyValue::IntPoint(_) => "IntPoint",
            StructPropertyValue::CustomStruct(type_name, _) => type_name,
        };
        Ok(property_name)
    }
}

impl PropertyTrait for StructProperty {
    fn write<W: Write>(
        &self,
        cursor: &mut W,
        include_header: bool,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        self.validate_value(options)?;
        if !include_header {
            return self.write_body(cursor, options);
        }

        let buf = &mut Cursor::new(Vec::new());
        self.write_body(buf, options)?;
        let buf = buf.get_ref();

        let property_name = self.get_property_name()?;

        cursor.write_string("StructProperty")?;
        cursor.write_u64::<LittleEndian>(buf.len() as u64)?;
        cursor.write_string(property_name)?;
        cursor.write_guid(&self.guid)?;
        cursor.write_u8(0)?;
        cursor.write_all(buf)?;

        Ok(())
    }
}

impl StructProperty {
    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        match &self.value {
            StructPropertyValue::VectorF(vector) => {
                cursor.write_f32::<LittleEndian>(vector.x.0)?;
                cursor.write_f32::<LittleEndian>(vector.y.0)?;
                cursor.write_f32::<LittleEndian>(vector.z.0)?;
            }
            StructPropertyValue::VectorD(vector) => {
                cursor.write_f64::<LittleEndian>(vector.x.0)?;
                cursor.write_f64::<LittleEndian>(vector.y.0)?;
                cursor.write_f64::<LittleEndian>(vector.z.0)?;
            }
            StructPropertyValue::RotatorF(rotator) => {
                cursor.write_f32::<LittleEndian>(rotator.pitch.0)?;
                cursor.write_f32::<LittleEndian>(rotator.yaw.0)?;
                cursor.write_f32::<LittleEndian>(rotator.roll.0)?;
            }
            StructPropertyValue::RotatorD(rotator) => {
                cursor.write_f64::<LittleEndian>(rotator.pitch.0)?;
                cursor.write_f64::<LittleEndian>(rotator.yaw.0)?;
                cursor.write_f64::<LittleEndian>(rotator.roll.0)?;
            }
            StructPropertyValue::QuatF(quat) => {
                cursor.write_f32::<LittleEndian>(quat.x.0)?;
                cursor.write_f32::<LittleEndian>(quat.y.0)?;
                cursor.write_f32::<LittleEndian>(quat.z.0)?;
                cursor.write_f32::<LittleEndian>(quat.w.0)?;
            }
            StructPropertyValue::QuatD(quat) => {
                cursor.write_f64::<LittleEndian>(quat.x.0)?;
                cursor.write_f64::<LittleEndian>(quat.y.0)?;
                cursor.write_f64::<LittleEndian>(quat.z.0)?;
                cursor.write_f64::<LittleEndian>(quat.w.0)?;
            }
            StructPropertyValue::DateTime(date_time) => {
                cursor.write_u64::<LittleEndian>(date_time.ticks)?;
            }
            StructPropertyValue::IntPoint(int_point) => {
                cursor.write_i32::<LittleEndian>(int_point.x)?;
                cursor.write_i32::<LittleEndian>(int_point.y)?;
            }
            StructPropertyValue::Guid(guid) => {
                cursor.write_guid(guid)?;
            }
            StructPropertyValue::CustomStruct(_, properties) => {
                for (key, value) in properties {
                    cursor.write_string(key)?;
                    value.write(cursor, true, options)?;
                }
                cursor.write_string("None")?;
            }
        };

        Ok(())
    }
}

impl From<VectorF> for StructProperty {
    #[inline]
    fn from(vector: VectorF) -> Self {
        Self::new(Guid([0u8; 16]), StructPropertyValue::VectorF(vector))
    }
}

impl From<VectorD> for StructProperty {
    #[inline]
    fn from(vector: VectorD) -> Self {
        Self::new(Guid([0u8; 16]), StructPropertyValue::VectorD(vector))
    }
}

impl From<RotatorF> for StructProperty {
    #[inline]
    fn from(rotator: RotatorF) -> Self {
        Self::new(Guid([0u8; 16]), StructPropertyValue::RotatorF(rotator))
    }
}

impl From<RotatorD> for StructProperty {
    #[inline]
    fn from(rotator: RotatorD) -> Self {
        Self::new(Guid([0u8; 16]), StructPropertyValue::RotatorD(rotator))
    }
}

impl From<QuatF> for StructProperty {
    #[inline]
    fn from(quat: QuatF) -> Self {
        Self::new(Guid([0u8; 16]), StructPropertyValue::QuatF(quat))
    }
}

impl From<QuatD> for StructProperty {
    #[inline]
    fn from(quat: QuatD) -> Self {
        Self::new(Guid([0u8; 16]), StructPropertyValue::QuatD(quat))
    }
}

impl From<DateTime> for StructProperty {
    #[inline]
    fn from(date_time: DateTime) -> Self {
        Self::new(Guid([0u8; 16]), StructPropertyValue::DateTime(date_time))
    }
}

impl From<IntPoint> for StructProperty {
    #[inline]
    fn from(int_point: IntPoint) -> Self {
        Self::new(Guid([0u8; 16]), StructPropertyValue::IntPoint(int_point))
    }
}

impl From<Guid> for StructProperty {
    #[inline]
    fn from(guid: Guid) -> Self {
        Self::new(Guid([0u8; 16]), StructPropertyValue::Guid(guid))
    }
}

impl StructPropertyValue {
    make_matcher!(VectorF, get_vector_f);
    make_matcher!(VectorD, get_vector_d);
    make_matcher!(RotatorF, get_rotator_f);
    make_matcher!(RotatorD, get_rotator_d);
    make_matcher!(QuatF, get_quat_f);
    make_matcher!(QuatD, get_quat_d);
    make_matcher!(DateTime, get_date_time);
    make_matcher!(IntPoint, get_int_point);
    make_matcher!(Guid, get_guid);
}
