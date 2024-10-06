use std::{
    fmt::Debug,
    hash::Hash,
    io::{Cursor, Read, Seek, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use indexmap::IndexMap;

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::{DeserializeError, Error, SerializeError},
    properties::{name_property::NameProperty, struct_types::LinearColor},
    scoped_stack_entry::ScopedStackEntry,
    types::map::HashableIndexMap,
    types::Guid,
};

use super::{
    impl_write, impl_write_header_part,
    int_property::{DoubleProperty, FloatProperty, IntProperty, UInt32Property, UInt64Property},
    make_matcher,
    struct_types::{
        DateTime, IntPoint, QuatD, QuatF, RotatorD, RotatorF, Vector2D, Vector2F, VectorD, VectorF,
    },
    Property, PropertyOptions, PropertyTrait,
};

macro_rules! validate {
    ($cond:expr, $($arg:tt)+) => {{
        if !$cond {
            Err(SerializeError::InvalidValue(
                format!($($arg)+).into_boxed_str(),
            ))?
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
    /// A `Vector2F` value.
    Vector2F(Vector2F),
    /// A `Vector2D` value.
    Vector2D(Vector2D),
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
    /// A `Timespan` value
    Timespan(DateTime),
    /// A `Guid` value.
    Guid(Guid),
    /// A `LinearColor` value.
    LinearColor(LinearColor),
    /// An `IntPoint` value.
    IntPoint(IntPoint),
    /// A custom struct value.
    CustomStruct {
        /// Type name.
        type_name: String,
        /// Properties.
        properties: HashableIndexMap<String, Vec<Property>>,
    },
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
                    "StructProperty".into(),
                    struct_path.into_boxed_str(),
                    cursor.stream_position()?,
                ))?
            };
            let hint = &hint.clone();
            Self::read_with_type_name(cursor, hint, options)
        }
    }

    #[inline]
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
            "Vector2D" => match options.large_world_coordinates {
                true => StructPropertyValue::Vector2D(Vector2D::new(
                    DoubleProperty::read(cursor, false)?.value.0,
                    DoubleProperty::read(cursor, false)?.value.0,
                )),
                false => StructPropertyValue::Vector2F(Vector2F::new(
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
            "Timespan" => StructPropertyValue::Timespan(DateTime {
                ticks: UInt64Property::read(cursor, false)?.value,
            }),
            "LinearColor" => StructPropertyValue::LinearColor(LinearColor::new(
                FloatProperty::read(cursor, false)?.value.0,
                FloatProperty::read(cursor, false)?.value.0,
                FloatProperty::read(cursor, false)?.value.0,
                FloatProperty::read(cursor, false)?.value.0,
            )),
            "IntPoint" => StructPropertyValue::IntPoint(IntPoint {
                x: IntProperty::read(cursor, false)?.value,
                y: IntProperty::read(cursor, false)?.value,
            }),
            "Guid" => StructPropertyValue::Guid(Guid::from([
                UInt32Property::read(cursor, false)?.value,
                UInt32Property::read(cursor, false)?.value,
                UInt32Property::read(cursor, false)?.value,
                UInt32Property::read(cursor, false)?.value,
            ])),
            _ => {
                let mut properties = HashableIndexMap::new();
                loop {
                    let property_name = cursor.read_string()?;
                    if property_name == "None" {
                        break;
                    }
                    let property_type = cursor.read_string()?;
                    let _property_stack_entry =
                        ScopedStackEntry::new(options.properties_stack, property_name.clone());

                    let property = Property::new(cursor, &property_type, true, options, None)?;
                    insert_property(&mut properties, property_name, property);
                }
                StructPropertyValue::CustomStruct {
                    type_name,
                    properties,
                }
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
    fn get_property_name(&self) -> Result<&str, Error> {
        let property_name = match &self.value {
            StructPropertyValue::Vector2F(_) | StructPropertyValue::Vector2D(_) => "Vector2D",
            StructPropertyValue::VectorF(_) | StructPropertyValue::VectorD(_) => "Vector",
            StructPropertyValue::RotatorF(_) | StructPropertyValue::RotatorD(_) => "Rotator",
            StructPropertyValue::QuatF(_) | StructPropertyValue::QuatD(_) => "Quat",
            StructPropertyValue::DateTime(_) => "DateTime",
            StructPropertyValue::Timespan(_) => "Timespan",
            StructPropertyValue::Guid(_) => "Guid",
            StructPropertyValue::LinearColor(_) => "LinearColor",
            StructPropertyValue::IntPoint(_) => "IntPoint",
            StructPropertyValue::CustomStruct { type_name, .. } => type_name,
        };
        Ok(property_name)
    }
}

fn insert_property(map: &mut IndexMap<String, Vec<Property>>, key: String, property: Property) {
    let entry = map.entry(key).or_default();
    #[cfg(debug_assertions)]
    {
        let array_index = match property {
            // TODO: Move array_index to the Property layer
            Property::NameProperty(NameProperty { array_index, .. }) => array_index,
            _ => 0,
        };
        let actual_array_index = entry.len() as u32;
        // Ensure that the position in the array matches the array_index value,
        // otherwise this conversion would cause data loss.
        assert_eq!(actual_array_index, array_index);
    }
    entry.push(property);
}

impl PropertyTrait for StructProperty {
    impl_write!(
        StructProperty,
        (write_string, fn, get_property_name),
        (write_guid, guid)
    );

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        match &self.value {
            StructPropertyValue::Vector2F(vector) => {
                validate!(
                    !options.large_world_coordinates,
                    "Vector2F not supported when LWC is enabled, use Vector2D",
                );
                cursor.write_f32::<LittleEndian>(vector.x.0)?;
                cursor.write_f32::<LittleEndian>(vector.y.0)?;
                Ok(8)
            }
            StructPropertyValue::Vector2D(vector) => {
                validate!(
                    options.large_world_coordinates,
                    "Vector2D not supported when LWC is disabled, use Vector2F",
                );
                cursor.write_f64::<LittleEndian>(vector.x.0)?;
                cursor.write_f64::<LittleEndian>(vector.y.0)?;
                Ok(16)
            }
            StructPropertyValue::VectorF(vector) => {
                validate!(
                    !options.large_world_coordinates,
                    "VectorF not supported when LWC is enabled, use VectorD",
                );
                cursor.write_f32::<LittleEndian>(vector.x.0)?;
                cursor.write_f32::<LittleEndian>(vector.y.0)?;
                cursor.write_f32::<LittleEndian>(vector.z.0)?;
                Ok(12)
            }
            StructPropertyValue::VectorD(vector) => {
                validate!(
                    options.large_world_coordinates,
                    "VectorD not supported when LWC is disabled, use VectorF",
                );
                cursor.write_f64::<LittleEndian>(vector.x.0)?;
                cursor.write_f64::<LittleEndian>(vector.y.0)?;
                cursor.write_f64::<LittleEndian>(vector.z.0)?;
                Ok(24)
            }
            StructPropertyValue::RotatorF(rotator) => {
                validate!(
                    !options.large_world_coordinates,
                    "RotatorF not supported when LWC is enabled, use RotatorD",
                );
                cursor.write_f32::<LittleEndian>(rotator.pitch.0)?;
                cursor.write_f32::<LittleEndian>(rotator.yaw.0)?;
                cursor.write_f32::<LittleEndian>(rotator.roll.0)?;
                Ok(12)
            }
            StructPropertyValue::RotatorD(rotator) => {
                validate!(
                    options.large_world_coordinates,
                    "RotatorD not supported when LWC is disabled, use RotatorF",
                );
                cursor.write_f64::<LittleEndian>(rotator.pitch.0)?;
                cursor.write_f64::<LittleEndian>(rotator.yaw.0)?;
                cursor.write_f64::<LittleEndian>(rotator.roll.0)?;
                Ok(24)
            }
            StructPropertyValue::QuatF(quat) => {
                validate!(
                    !options.large_world_coordinates,
                    "QuatF not supported when LWC is enabled, use QuatD",
                );
                cursor.write_f32::<LittleEndian>(quat.x.0)?;
                cursor.write_f32::<LittleEndian>(quat.y.0)?;
                cursor.write_f32::<LittleEndian>(quat.z.0)?;
                cursor.write_f32::<LittleEndian>(quat.w.0)?;
                Ok(16)
            }
            StructPropertyValue::QuatD(quat) => {
                validate!(
                    options.large_world_coordinates,
                    "QuatD not supported when LWC is disabled, use QuatF",
                );
                cursor.write_f64::<LittleEndian>(quat.x.0)?;
                cursor.write_f64::<LittleEndian>(quat.y.0)?;
                cursor.write_f64::<LittleEndian>(quat.z.0)?;
                cursor.write_f64::<LittleEndian>(quat.w.0)?;
                Ok(32)
            }
            StructPropertyValue::DateTime(date_time) => {
                cursor.write_u64::<LittleEndian>(date_time.ticks)?;
                Ok(8)
            }
            StructPropertyValue::Timespan(date_time) => {
                cursor.write_u64::<LittleEndian>(date_time.ticks)?;
                Ok(8)
            }
            StructPropertyValue::LinearColor(linear_color) => {
                cursor.write_f32::<LittleEndian>(linear_color.r.0)?;
                cursor.write_f32::<LittleEndian>(linear_color.g.0)?;
                cursor.write_f32::<LittleEndian>(linear_color.b.0)?;
                cursor.write_f32::<LittleEndian>(linear_color.a.0)?;
                Ok(16)
            }
            StructPropertyValue::IntPoint(int_point) => {
                cursor.write_i32::<LittleEndian>(int_point.x)?;
                cursor.write_i32::<LittleEndian>(int_point.y)?;
                Ok(8)
            }
            StructPropertyValue::Guid(guid) => {
                cursor.write_guid(guid)?;
                Ok(16)
            }
            StructPropertyValue::CustomStruct {
                properties: HashableIndexMap(properties),
                ..
            } => {
                let mut len = 0;
                for (key, values) in properties {
                    for value in values {
                        len += cursor.write_string(key)?;
                        len += value.write(cursor, true, options)?;
                    }
                }
                len += cursor.write_string("None")?;
                Ok(len)
            }
        }
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
    make_matcher!(VectorF, get_vector_f, get_vector_f_mut);
    make_matcher!(VectorD, get_vector_d, get_vector_d_mut);
    make_matcher!(RotatorF, get_rotator_f, get_rotator_f_mut);
    make_matcher!(RotatorD, get_rotator_d, get_rotator_d_mut);
    make_matcher!(QuatF, get_quat_f, get_quat_f_mut);
    make_matcher!(QuatD, get_quat_d, get_quat_d_mut);
    make_matcher!(DateTime, get_date_time, get_date_time_mut);
    make_matcher!(IntPoint, get_int_point, get_int_point_mut);
    make_matcher!(Guid, get_guid, get_guid_mut);

    /// Retrieves the enum value as a `CustomStruct`.
    #[inline]
    pub fn get_custom_struct(&self) -> Option<(&String, &IndexMap<String, Vec<Property>>)> {
        match self {
            Self::CustomStruct {
                type_name,
                properties: HashableIndexMap(properties),
            } => Some((type_name, properties)),
            _ => None,
        }
    }

    /// Retrieves the mutable enum value as a `CustomStruct`.
    #[inline]
    pub fn get_custom_struct_mut(
        &mut self,
    ) -> Option<(&mut String, &mut IndexMap<String, Vec<Property>>)> {
        match self {
            Self::CustomStruct {
                type_name,
                properties: HashableIndexMap(properties),
            } => Some((type_name, properties)),
            _ => None,
        }
    }
}
