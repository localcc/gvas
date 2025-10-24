use std::{
    fmt::Debug,
    hash::Hash,
    io::{Cursor, Read, Seek, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use indexmap::IndexMap;

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    custom_version::FUE5ReleaseStreamObjectVersion,
    error::{DeserializeError, Error, SerializeError},
    properties::{name_property::NameProperty, struct_types::LinearColor},
    scoped_stack_entry::ScopedStackEntry,
    types::{Guid, map::HashableIndexMap},
};

use super::{
    Property, PropertyOptions, PropertyTrait, impl_write, impl_write_header_part, make_matcher,
    struct_types::{
        DateTime, IntPoint, QuatD, QuatF, RotatorD, RotatorF, Timespan, Vector2D, Vector2F,
        VectorD, VectorF,
    },
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
    /// Type name.
    pub type_name: String,
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
    Timespan(Timespan),
    /// A `Guid` value.
    Guid(Guid),
    /// A `LinearColor` value.
    LinearColor(LinearColor),
    /// An `IntPoint` value.
    IntPoint(IntPoint),
    /// A custom struct value.
    CustomStruct(HashableIndexMap<String, Vec<Property>>),
}

impl StructProperty {
    /// Creates a new `StructProperty` instance.
    #[inline]
    pub fn new(guid: Guid, type_name: String, value: StructPropertyValue) -> Self {
        StructProperty {
            guid,
            type_name,
            value,
        }
    }

    #[inline]
    pub(crate) fn read<R: Read + Seek>(
        cursor: &mut R,
        include_header: bool,
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        if include_header {
            Self::read_header(cursor, options)
        } else {
            Err(DeserializeError::invalid_property(
                "StructProperty::read() include_header must be true, use read_body() instead",
                cursor,
            ))?
        }
    }

    #[inline]
    fn read_header<R: Read + Seek>(
        cursor: &mut R,
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        let length = cursor.read_u32::<LittleEndian>()?;

        let array_index = cursor.read_u32::<LittleEndian>()?;
        if array_index != 0 {
            let position = cursor.stream_position()? - 4;
            Err(DeserializeError::InvalidArrayIndex(array_index, position))?
        }

        let type_name = cursor.read_string()?;

        let guid = cursor.read_guid()?;

        let terminator = cursor.read_u8()?;
        if terminator != 0 {
            let position = cursor.stream_position()? - 1;
            Err(DeserializeError::InvalidTerminator(terminator, position))?
        }

        let start = cursor.stream_position()?;
        let value = Self::read_body(cursor, &type_name, options)?;
        let end = cursor.stream_position()?;
        if end - start != length as u64 {
            Err(DeserializeError::InvalidValueSize(
                length as u64,
                end - start,
                start,
            ))?
        }

        Ok(StructProperty {
            guid,
            type_name,
            value,
        })
    }

    #[inline]
    pub(crate) fn read_body<R: Read + Seek>(
        cursor: &mut R,
        type_name: &str,
        options: &mut PropertyOptions,
    ) -> Result<StructPropertyValue, Error> {
        let value = match type_name {
            "Vector" => StructPropertyValue::read_vector(cursor, options)?,
            "Vector2D" => StructPropertyValue::read_vector2(cursor, options)?,
            "Rotator" => StructPropertyValue::read_rotator(cursor, options)?,
            "Quat" => StructPropertyValue::read_quat(cursor, options)?,
            "DateTime" => StructPropertyValue::read_datetime(cursor)?,
            "Timespan" => StructPropertyValue::read_timespan(cursor)?,
            "LinearColor" => StructPropertyValue::read_linearcolor(cursor)?,
            "IntPoint" => StructPropertyValue::read_intpoint(cursor)?,
            "Guid" => StructPropertyValue::read_guid(cursor)?,
            _ => StructPropertyValue::read_custom(cursor, options)?,
        };
        Ok(value)
    }

    #[inline]
    fn get_property_type(&self) -> Result<&str, Error> {
        Ok(&self.type_name)
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
        (write_string, fn, get_property_type),
        (write_guid, guid)
    );

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        self.value.write_body(cursor, options)
    }
}

impl PropertyTrait for StructPropertyValue {
    #[inline]
    fn write<W: Write>(
        &self,
        writer: &mut W,
        include_header: bool,
        options: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        if !include_header {
            return self.write_body(writer, options);
        }
        Err(SerializeError::invalid_value(
            "StructPropertyValue can not be serialized with a header",
        ))?
    }

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        match self {
            StructPropertyValue::Vector2F(vector) => {
                validate!(
                    !options
                        .supports_version(FUE5ReleaseStreamObjectVersion::LargeWorldCoordinates),
                    "Vector2F not supported when LWC is enabled, use Vector2D",
                );
                cursor.write_f32::<LittleEndian>(vector.x.0)?;
                cursor.write_f32::<LittleEndian>(vector.y.0)?;
                Ok(8)
            }
            StructPropertyValue::Vector2D(vector) => {
                validate!(
                    options.supports_version(FUE5ReleaseStreamObjectVersion::LargeWorldCoordinates),
                    "Vector2D not supported when LWC is disabled, use Vector2F",
                );
                cursor.write_f64::<LittleEndian>(vector.x.0)?;
                cursor.write_f64::<LittleEndian>(vector.y.0)?;
                Ok(16)
            }
            StructPropertyValue::VectorF(vector) => {
                validate!(
                    !options
                        .supports_version(FUE5ReleaseStreamObjectVersion::LargeWorldCoordinates),
                    "VectorF not supported when LWC is enabled, use VectorD",
                );
                cursor.write_f32::<LittleEndian>(vector.x.0)?;
                cursor.write_f32::<LittleEndian>(vector.y.0)?;
                cursor.write_f32::<LittleEndian>(vector.z.0)?;
                Ok(12)
            }
            StructPropertyValue::VectorD(vector) => {
                validate!(
                    options.supports_version(FUE5ReleaseStreamObjectVersion::LargeWorldCoordinates),
                    "VectorD not supported when LWC is disabled, use VectorF",
                );
                cursor.write_f64::<LittleEndian>(vector.x.0)?;
                cursor.write_f64::<LittleEndian>(vector.y.0)?;
                cursor.write_f64::<LittleEndian>(vector.z.0)?;
                Ok(24)
            }
            StructPropertyValue::RotatorF(rotator) => {
                validate!(
                    !options
                        .supports_version(FUE5ReleaseStreamObjectVersion::LargeWorldCoordinates),
                    "RotatorF not supported when LWC is enabled, use RotatorD",
                );
                cursor.write_f32::<LittleEndian>(rotator.pitch.0)?;
                cursor.write_f32::<LittleEndian>(rotator.yaw.0)?;
                cursor.write_f32::<LittleEndian>(rotator.roll.0)?;
                Ok(12)
            }
            StructPropertyValue::RotatorD(rotator) => {
                validate!(
                    options.supports_version(FUE5ReleaseStreamObjectVersion::LargeWorldCoordinates),
                    "RotatorD not supported when LWC is disabled, use RotatorF",
                );
                cursor.write_f64::<LittleEndian>(rotator.pitch.0)?;
                cursor.write_f64::<LittleEndian>(rotator.yaw.0)?;
                cursor.write_f64::<LittleEndian>(rotator.roll.0)?;
                Ok(24)
            }
            StructPropertyValue::QuatF(quat) => {
                validate!(
                    !options
                        .supports_version(FUE5ReleaseStreamObjectVersion::LargeWorldCoordinates),
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
                    options.supports_version(FUE5ReleaseStreamObjectVersion::LargeWorldCoordinates),
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
            StructPropertyValue::CustomStruct(properties) => {
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

impl StructPropertyValue {
    fn read_custom<R: Read + Seek>(
        cursor: &mut R,
        options: &mut PropertyOptions,
    ) -> Result<StructPropertyValue, Error> {
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
        Ok(StructPropertyValue::CustomStruct(properties))
    }

    fn read_guid<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        Ok(Self::Guid(cursor.read_guid()?))
    }

    fn read_intpoint<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        Ok(Self::IntPoint(IntPoint::new(
            cursor.read_i32::<LittleEndian>()?,
            cursor.read_i32::<LittleEndian>()?,
        )))
    }

    fn read_linearcolor<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        Ok(Self::LinearColor(LinearColor::new(
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
            cursor.read_f32::<LittleEndian>()?,
        )))
    }

    fn read_timespan<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        Ok(Self::Timespan(Timespan::new(
            cursor.read_u64::<LittleEndian>()?,
        )))
    }

    fn read_datetime<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        Ok(Self::DateTime(DateTime::new(
            cursor.read_u64::<LittleEndian>()?,
        )))
    }

    fn read_quat<R: Read + Seek>(
        cursor: &mut R,
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        match options.supports_version(FUE5ReleaseStreamObjectVersion::LargeWorldCoordinates) {
            true => Ok(Self::QuatD(QuatD::new(
                cursor.read_f64::<LittleEndian>()?,
                cursor.read_f64::<LittleEndian>()?,
                cursor.read_f64::<LittleEndian>()?,
                cursor.read_f64::<LittleEndian>()?,
            ))),
            false => Ok(Self::QuatF(QuatF::new(
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
            ))),
        }
    }

    fn read_rotator<R: Read + Seek>(
        cursor: &mut R,
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        match options.supports_version(FUE5ReleaseStreamObjectVersion::LargeWorldCoordinates) {
            true => Ok(Self::RotatorD(RotatorD::new(
                cursor.read_f64::<LittleEndian>()?,
                cursor.read_f64::<LittleEndian>()?,
                cursor.read_f64::<LittleEndian>()?,
            ))),
            false => Ok(Self::RotatorF(RotatorF::new(
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
            ))),
        }
    }

    fn read_vector2<R: Read + Seek>(
        cursor: &mut R,
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        match options.supports_version(FUE5ReleaseStreamObjectVersion::LargeWorldCoordinates) {
            true => Ok(Self::Vector2D(Vector2D::new(
                cursor.read_f64::<LittleEndian>()?,
                cursor.read_f64::<LittleEndian>()?,
            ))),
            false => Ok(Self::Vector2F(Vector2F::new(
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
            ))),
        }
    }

    fn read_vector<R: Read + Seek>(
        cursor: &mut R,
        options: &mut PropertyOptions,
    ) -> Result<Self, Error> {
        match options.supports_version(FUE5ReleaseStreamObjectVersion::LargeWorldCoordinates) {
            true => Ok(Self::VectorD(VectorD::new(
                cursor.read_f64::<LittleEndian>()?,
                cursor.read_f64::<LittleEndian>()?,
                cursor.read_f64::<LittleEndian>()?,
            ))),
            false => Ok(Self::VectorF(VectorF::new(
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
                cursor.read_f32::<LittleEndian>()?,
            ))),
        }
    }
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
    pub fn get_custom_struct(&self) -> Option<&HashableIndexMap<String, Vec<Property>>> {
        match self {
            Self::CustomStruct(properties) => Some(properties),
            _ => None,
        }
    }

    /// Retrieves the mutable enum value as a `CustomStruct`.
    #[inline]
    pub fn get_custom_struct_mut(
        &mut self,
    ) -> Option<&mut HashableIndexMap<String, Vec<Property>>> {
        match self {
            Self::CustomStruct(properties) => Some(properties),
            _ => None,
        }
    }
}

impl From<Vector2F> for StructPropertyValue {
    #[inline]
    fn from(value: Vector2F) -> Self {
        StructPropertyValue::Vector2F(value)
    }
}

impl From<Vector2D> for StructPropertyValue {
    #[inline]
    fn from(value: Vector2D) -> Self {
        StructPropertyValue::Vector2D(value)
    }
}

impl From<VectorF> for StructPropertyValue {
    #[inline]
    fn from(vector: VectorF) -> Self {
        StructPropertyValue::VectorF(vector)
    }
}

impl From<VectorD> for StructPropertyValue {
    #[inline]
    fn from(vector: VectorD) -> Self {
        StructPropertyValue::VectorD(vector)
    }
}

impl From<RotatorF> for StructPropertyValue {
    #[inline]
    fn from(rotator: RotatorF) -> Self {
        StructPropertyValue::RotatorF(rotator)
    }
}

impl From<RotatorD> for StructPropertyValue {
    #[inline]
    fn from(rotator: RotatorD) -> Self {
        StructPropertyValue::RotatorD(rotator)
    }
}

impl From<QuatF> for StructPropertyValue {
    #[inline]
    fn from(quat: QuatF) -> Self {
        StructPropertyValue::QuatF(quat)
    }
}

impl From<QuatD> for StructPropertyValue {
    #[inline]
    fn from(quat: QuatD) -> Self {
        StructPropertyValue::QuatD(quat)
    }
}

impl From<DateTime> for StructPropertyValue {
    #[inline]
    fn from(date_time: DateTime) -> Self {
        StructPropertyValue::DateTime(date_time)
    }
}

impl From<Timespan> for StructPropertyValue {
    #[inline]
    fn from(timespan: Timespan) -> Self {
        StructPropertyValue::Timespan(timespan)
    }
}

impl From<Guid> for StructPropertyValue {
    #[inline]
    fn from(guid: Guid) -> Self {
        StructPropertyValue::Guid(guid)
    }
}

impl From<LinearColor> for StructPropertyValue {
    #[inline]
    fn from(linear_color: LinearColor) -> Self {
        StructPropertyValue::LinearColor(linear_color)
    }
}

impl From<IntPoint> for StructPropertyValue {
    #[inline]
    fn from(int_point: IntPoint) -> Self {
        StructPropertyValue::IntPoint(int_point)
    }
}
