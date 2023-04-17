use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    io::{Cursor, Read, Seek, SeekFrom, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::{DeserializeError, Error},
    make_matcher,
    scoped_stack_entry::ScopedStackEntry,
    types::Guid,
};

use super::{
    int_property::{FloatProperty, IntProperty, UInt32Property, UInt64Property},
    struct_types::{DateTime, IntPoint, Quat, Rotator, Vector},
    Property, PropertyTrait,
};

/// A property that holds a struct value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructProperty {
    /// The unique identifier of the property.
    pub guid: Guid,
    /// The value of the property.
    pub value: StructPropertyValue,
}

/// The possible values of a `StructProperty`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StructPropertyValue {
    /// A `Vector` value.
    Vector(Vector),
    /// A `Rotator` value.
    Rotator(Rotator),
    /// A `Quat` value.
    Quat(Quat),
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
    pub fn new(guid: Guid, value: StructPropertyValue) -> Self {
        StructProperty { guid, value }
    }

    fn read(
        cursor: &mut Cursor<Vec<u8>>,
        hints: &HashMap<String, String>,
        properties_stack: &mut Vec<String>,
        include_header: bool,
        type_name: Option<String>,
    ) -> Result<Self, Error> {
        if include_header {
            let _length = cursor.read_u64::<LittleEndian>()?;
        }

        let type_name = match include_header {
            true => cursor.read_string()?,
            false => match type_name {
                Some(t) => t,
                None => Err(DeserializeError::missing_argument(
                    "type_name",
                    cursor.stream_position()?,
                ))?,
            },
        };

        let guid = match include_header {
            true => {
                let mut guid = [0u8; 16];
                cursor.read_exact(&mut guid)?;
                guid
            }
            false => [0u8; 16],
        };

        if include_header {
            cursor.read_exact(&mut [0u8; 1])?;
        }

        let value = match type_name.as_str() {
            "Vector" => StructPropertyValue::Vector(Vector::new(
                FloatProperty::read(cursor, false)?.value.0,
                FloatProperty::read(cursor, false)?.value.0,
                FloatProperty::read(cursor, false)?.value.0,
            )),
            "Rotator" => StructPropertyValue::Rotator(Rotator::new(
                FloatProperty::read(cursor, false)?.value.0,
                FloatProperty::read(cursor, false)?.value.0,
                FloatProperty::read(cursor, false)?.value.0,
            )),
            "Quat" => StructPropertyValue::Quat(Quat::new(
                FloatProperty::read(cursor, false)?.value.0,
                FloatProperty::read(cursor, false)?.value.0,
                FloatProperty::read(cursor, false)?.value.0,
                FloatProperty::read(cursor, false)?.value.0,
            )),
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
                let mut key_name = cursor.read_string()?;
                let mut properties = Vec::new();
                while &key_name != "None" {
                    let value_type = cursor.read_string()?;
                    let _property_stack_entry =
                        ScopedStackEntry::new(properties_stack, key_name.clone());

                    let property =
                        Property::new(cursor, hints, properties_stack, &value_type, true, None)?;
                    properties.push((key_name, property));
                    key_name = cursor.read_string()?;
                }
                StructPropertyValue::CustomStruct(type_name, properties)
            }
        };

        Ok(StructProperty {
            guid: Guid::new(guid),
            value,
        })
    }

    pub(crate) fn read_with_header(
        cursor: &mut Cursor<Vec<u8>>,
        hints: &HashMap<String, String>,
        properties_stack: &mut Vec<String>,
    ) -> Result<Self, Error> {
        Self::read(cursor, hints, properties_stack, true, None)
    }

    pub(crate) fn read_with_type_name(
        cursor: &mut Cursor<Vec<u8>>,
        hints: &HashMap<String, String>,
        properties_stack: &mut Vec<String>,
        type_name: &str,
    ) -> Result<Self, Error> {
        Self::read(
            cursor,
            hints,
            properties_stack,
            false,
            Some(type_name.to_string()),
        )
    }
}

impl PropertyTrait for StructProperty {
    fn write<W: Write + Seek>(&self, cursor: &mut W, include_header: bool) -> Result<(), Error> {
        let mut begin = 0;
        let mut write_begin = 0;
        if include_header {
            cursor.write_string("StructProperty")?;
            begin = cursor.stream_position()?;
            cursor.write_u64::<LittleEndian>(0)?;
            cursor.write_string(match &self.value {
                StructPropertyValue::Vector(_) => "Vector",
                StructPropertyValue::Rotator(_) => "Rotator",
                StructPropertyValue::Quat(_) => "Quat",
                StructPropertyValue::DateTime(_) => "DateTime",
                StructPropertyValue::Guid(_) => "Guid",
                StructPropertyValue::IntPoint(_) => "IntPoint",
                StructPropertyValue::CustomStruct(type_name, _) => type_name,
            })?;
            cursor.write_all(&self.guid.0)?;
            cursor.write_all(&[0u8; 1])?;
            write_begin = cursor.stream_position()?;
        }

        match &self.value {
            StructPropertyValue::Vector(vector) => {
                FloatProperty::new(f32::from(vector.x)).write(cursor, false)?;
                FloatProperty::new(f32::from(vector.y)).write(cursor, false)?;
                FloatProperty::new(f32::from(vector.z)).write(cursor, false)?;
            }
            StructPropertyValue::Rotator(rotator) => {
                FloatProperty::new(f32::from(rotator.pitch)).write(cursor, false)?;
                FloatProperty::new(f32::from(rotator.yaw)).write(cursor, false)?;
                FloatProperty::new(f32::from(rotator.roll)).write(cursor, false)?;
            }
            StructPropertyValue::Quat(quat) => {
                FloatProperty::new(f32::from(quat.x)).write(cursor, false)?;
                FloatProperty::new(f32::from(quat.y)).write(cursor, false)?;
                FloatProperty::new(f32::from(quat.z)).write(cursor, false)?;
                FloatProperty::new(f32::from(quat.w)).write(cursor, false)?;
            }
            StructPropertyValue::DateTime(date_time) => {
                UInt64Property::new(date_time.ticks).write(cursor, false)?;
            }
            StructPropertyValue::IntPoint(int_point) => {
                IntProperty::new(int_point.x).write(cursor, false)?;
                IntProperty::new(int_point.y).write(cursor, false)?;
            }
            StructPropertyValue::Guid(guid) => {
                let (a, b, c, d) = guid.to_owned().into();
                UInt32Property::new(a).write(cursor, false)?;
                UInt32Property::new(b).write(cursor, false)?;
                UInt32Property::new(c).write(cursor, false)?;
                UInt32Property::new(d).write(cursor, false)?;
            }
            StructPropertyValue::CustomStruct(_, properties) => {
                for (key, value) in properties {
                    cursor.write_string(key)?;
                    value.write(cursor, true)?;
                }
                cursor.write_string("None")?;
            }
        };

        if include_header {
            let write_end = cursor.stream_position()?;
            cursor.seek(SeekFrom::Start(begin))?;
            cursor.write_u64::<LittleEndian>(write_end - write_begin)?;
            cursor.seek(SeekFrom::Start(write_end))?;
        }

        Ok(())
    }
}

impl From<Vector> for StructProperty {
    fn from(vector: Vector) -> Self {
        Self::new(Guid([0u8; 16]), StructPropertyValue::Vector(vector))
    }
}

impl From<Rotator> for StructProperty {
    fn from(rotator: Rotator) -> Self {
        Self::new(Guid([0u8; 16]), StructPropertyValue::Rotator(rotator))
    }
}

impl From<Quat> for StructProperty {
    fn from(quat: Quat) -> Self {
        Self::new(Guid([0u8; 16]), StructPropertyValue::Quat(quat))
    }
}

impl From<DateTime> for StructProperty {
    fn from(date_time: DateTime) -> Self {
        Self::new(Guid([0u8; 16]), StructPropertyValue::DateTime(date_time))
    }
}

impl From<IntPoint> for StructProperty {
    fn from(int_point: IntPoint) -> Self {
        Self::new(Guid([0u8; 16]), StructPropertyValue::IntPoint(int_point))
    }
}

impl From<Guid> for StructProperty {
    fn from(guid: Guid) -> Self {
        Self::new(Guid([0u8; 16]), StructPropertyValue::Guid(guid))
    }
}

impl StructPropertyValue {
    make_matcher!(Vector, get_vector);
    make_matcher!(Rotator, get_rotator);
    make_matcher!(Quat, get_quat);
    make_matcher!(DateTime, get_date_time);
    make_matcher!(IntPoint, get_int_point);
    make_matcher!(Guid, get_guid);
}
