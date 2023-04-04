use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    io::{Cursor, Read, Seek, SeekFrom, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use indexmap::IndexMap;

use crate::{
    cast,
    cursor_ext::CursorExt,
    error::{Error, SerializeError},
    scoped_stack_entry::ScopedStackEntry,
    types::Guid,
};

use super::{
    int_property::{FloatProperty, IntProperty, UInt32Property, UInt64Property},
    struct_types::{DateTime, IntPoint, Quat, Rotator, Vector},
    Property, PropertyTrait,
};

macro_rules! write_flat_property {
    ($cursor:expr, $properties:expr, $struct_name:expr, $property_name:expr) => {
        $properties
            .get($property_name)
            .ok_or_else(|| SerializeError::struct_missing_field($struct_name, $property_name))?
            .write($cursor, false)?;
    };
}

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct StructProperty {
    pub type_name: String,
    pub guid: Guid,
    pub properties: IndexMap<String, Property>,
}

impl StructProperty {
    pub fn new(type_name: String, guid: Guid, properties: IndexMap<String, Property>) -> Self {
        StructProperty {
            type_name,
            guid,
            properties,
        }
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
            false => type_name.unwrap_or_default(),
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

        let mut properties: IndexMap<String, Property> = IndexMap::new();
        match type_name.as_str() {
            "Vector" => {
                properties.insert("X".to_string(), FloatProperty::read(cursor, false)?.into());
                properties.insert("Y".to_string(), FloatProperty::read(cursor, false)?.into());
                properties.insert("Z".to_string(), FloatProperty::read(cursor, false)?.into());
            }
            "Rotator" => {
                properties.insert(
                    "Pitch".to_string(),
                    FloatProperty::read(cursor, false)?.into(),
                );
                properties.insert(
                    "Yaw".to_string(),
                    FloatProperty::read(cursor, false)?.into(),
                );
                properties.insert(
                    "Roll".to_string(),
                    FloatProperty::read(cursor, false)?.into(),
                );
            }
            "Quat" => {
                properties.insert("X".to_string(), FloatProperty::read(cursor, false)?.into());
                properties.insert("Y".to_string(), FloatProperty::read(cursor, false)?.into());
                properties.insert("Z".to_string(), FloatProperty::read(cursor, false)?.into());
                properties.insert("W".to_string(), FloatProperty::read(cursor, false)?.into());
            }
            "DateTime" => {
                properties.insert(
                    "Ticks".to_string(),
                    UInt64Property::read(cursor, false)?.into(),
                );
            }
            "IntPoint" => {
                properties.insert("X".to_string(), IntProperty::read(cursor, false)?.into());
                properties.insert("Y".to_string(), IntProperty::read(cursor, false)?.into());
            }
            "Guid" => {
                properties.insert("A".to_string(), UInt32Property::read(cursor, false)?.into());
                properties.insert("B".to_string(), UInt32Property::read(cursor, false)?.into());
                properties.insert("C".to_string(), UInt32Property::read(cursor, false)?.into());
                properties.insert("D".to_string(), UInt32Property::read(cursor, false)?.into());
            }
            _ => {
                let mut key_name = cursor.read_string()?;
                while &key_name != "None" {
                    let value_type = cursor.read_string()?;
                    let _property_stack_entry =
                        ScopedStackEntry::new(properties_stack, key_name.clone());

                    let property =
                        Property::new(cursor, hints, properties_stack, &value_type, true, None)?;
                    properties.insert(key_name, property);
                    key_name = cursor.read_string()?;
                }
            }
        }

        Ok(StructProperty {
            type_name,
            guid: Guid::new(guid),
            properties,
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

    pub fn get_vector(&self) -> Option<Vector> {
        if self.type_name != "Vector" {
            return None;
        }

        Some(Vector::new(
            cast!(Property, FloatProperty, self.properties.get("X")?)?
                .value
                .0,
            cast!(Property, FloatProperty, self.properties.get("Y")?)?
                .value
                .0,
            cast!(Property, FloatProperty, self.properties.get("Z")?)?
                .value
                .0,
        ))
    }

    pub fn get_rotator(&self) -> Option<Rotator> {
        if self.type_name != "Rotator" {
            return None;
        }

        Some(Rotator::new(
            cast!(Property, FloatProperty, self.properties.get("Pitch")?)?
                .value
                .0,
            cast!(Property, FloatProperty, self.properties.get("Yaw")?)?
                .value
                .0,
            cast!(Property, FloatProperty, self.properties.get("Roll")?)?
                .value
                .0,
        ))
    }

    pub fn get_quat(&self) -> Option<Quat> {
        if self.type_name != "Quat" {
            return None;
        }

        Some(Quat::new(
            cast!(Property, FloatProperty, self.properties.get("X")?)?
                .value
                .0,
            cast!(Property, FloatProperty, self.properties.get("Y")?)?
                .value
                .0,
            cast!(Property, FloatProperty, self.properties.get("Z")?)?
                .value
                .0,
            cast!(Property, FloatProperty, self.properties.get("W")?)?
                .value
                .0,
        ))
    }

    pub fn get_date_time(&self) -> Option<DateTime> {
        if self.type_name != "DateTime" {
            return None;
        }

        Some(DateTime::new(
            cast!(Property, UInt64Property, self.properties.get("Ticks")?)?.value,
        ))
    }

    pub fn get_int_point(&self) -> Option<IntPoint> {
        if self.type_name != "IntPoint" {
            return None;
        }

        Some(IntPoint::new(
            cast!(Property, IntProperty, self.properties.get("X")?)?.value,
            cast!(Property, IntProperty, self.properties.get("Y")?)?.value,
        ))
    }

    pub fn get_guid(&self) -> Option<Guid> {
        if self.type_name != "Guid" {
            return None;
        }

        let a = u32::swap_bytes(cast!(Property, UInt32Property, self.properties.get("A")?)?.value); // ue4 saves GUID in BigEndian format
        let b = u32::swap_bytes(cast!(Property, UInt32Property, self.properties.get("B")?)?.value);
        let c = u32::swap_bytes(cast!(Property, UInt32Property, self.properties.get("C")?)?.value);
        let d = u32::swap_bytes(cast!(Property, UInt32Property, self.properties.get("D")?)?.value);

        Some(Guid::from((a, b, c, d)))
    }
}

impl PropertyTrait for StructProperty {
    fn write(&self, cursor: &mut Cursor<Vec<u8>>, include_header: bool) -> Result<(), Error> {
        let mut begin = 0;
        let mut write_begin = 0;
        if include_header {
            cursor.write_string(&String::from("StructProperty"))?;
            begin = cursor.position();
            cursor.write_u64::<LittleEndian>(0)?;
            cursor.write_string(&self.type_name)?;
            let _ = cursor.write(&self.guid.0)?;
            let _ = cursor.write(&[0u8; 1])?;
            write_begin = cursor.position();
        }

        match self.type_name.as_str() {
            "Vector" => {
                write_flat_property!(cursor, self.properties, "Vector", "X");
                write_flat_property!(cursor, self.properties, "Vector", "Y");
                write_flat_property!(cursor, self.properties, "Vector", "Z");
            }
            "Rotator" => {
                write_flat_property!(cursor, self.properties, "Rotator", "Pitch");
                write_flat_property!(cursor, self.properties, "Rotator", "Yaw");
                write_flat_property!(cursor, self.properties, "Rotator", "Roll");
            }
            "Quat" => {
                write_flat_property!(cursor, self.properties, "Quat", "X");
                write_flat_property!(cursor, self.properties, "Quat", "Y");
                write_flat_property!(cursor, self.properties, "Quat", "Z");
                write_flat_property!(cursor, self.properties, "Quat", "W");
            }
            "DateTime" => {
                write_flat_property!(cursor, self.properties, "DateTime", "Ticks");
            }
            "IntPoint" => {
                write_flat_property!(cursor, self.properties, "IntPoint", "X");
                write_flat_property!(cursor, self.properties, "IntPoint", "Y");
            }
            "Guid" => {
                write_flat_property!(cursor, self.properties, "Guid", "A");
                write_flat_property!(cursor, self.properties, "Guid", "B");
                write_flat_property!(cursor, self.properties, "Guid", "C");
                write_flat_property!(cursor, self.properties, "Guid", "D");
            }
            _ => {
                for (key, value) in &self.properties {
                    cursor.write_string(key)?;
                    value.write(cursor, true)?;
                }
                cursor.write_string(&String::from("None"))?;
            }
        };

        if include_header {
            let write_end = cursor.position();
            cursor.seek(SeekFrom::Start(begin))?;
            cursor.write_u64::<LittleEndian>(write_end - write_begin)?;
            cursor.seek(SeekFrom::Start(write_end))?;
        }

        Ok(())
    }
}

impl From<Vector> for StructProperty {
    fn from(vector: Vector) -> Self {
        Self::new(
            "Vector".to_string(),
            Guid([0u8; 16]),
            IndexMap::from([
                ("X".to_string(), FloatProperty::new(vector.x).into()),
                ("Y".to_string(), FloatProperty::new(vector.y).into()),
                ("Z".to_string(), FloatProperty::new(vector.z).into()),
            ]),
        )
    }
}

impl From<Rotator> for StructProperty {
    fn from(rotator: Rotator) -> Self {
        Self::new(
            "Rotator".to_string(),
            Guid([0u8; 16]),
            IndexMap::from([
                (
                    "Pitch".to_string(),
                    FloatProperty::new(rotator.pitch).into(),
                ),
                ("Yaw".to_string(), FloatProperty::new(rotator.yaw).into()),
                ("Roll".to_string(), FloatProperty::new(rotator.roll).into()),
            ]),
        )
    }
}

impl From<Quat> for StructProperty {
    fn from(quat: Quat) -> Self {
        Self::new(
            "Quat".to_string(),
            Guid([0u8; 16]),
            IndexMap::from([
                ("X".to_string(), FloatProperty::new(quat.x).into()),
                ("Y".to_string(), FloatProperty::new(quat.y).into()),
                ("Z".to_string(), FloatProperty::new(quat.z).into()),
                ("W".to_string(), FloatProperty::new(quat.w).into()),
            ]),
        )
    }
}

impl From<DateTime> for StructProperty {
    fn from(date_time: DateTime) -> Self {
        Self::new(
            "DateTime".to_string(),
            Guid([0u8; 16]),
            IndexMap::from([(
                "Ticks".to_string(),
                UInt64Property::new(date_time.ticks).into(),
            )]),
        )
    }
}

impl From<IntPoint> for StructProperty {
    fn from(int_point: IntPoint) -> Self {
        Self::new(
            "IntPoint".to_string(),
            Guid([0u8; 16]),
            IndexMap::from([
                ("X".to_string(), IntProperty::new(int_point.x).into()),
                ("Y".to_string(), IntProperty::new(int_point.y).into()),
            ]),
        )
    }
}

impl From<Guid> for StructProperty {
    fn from(guid: Guid) -> Self {
        let (a, b, c, d) = guid.into();
        Self::new(
            "Guid".to_string(),
            Guid([0u8; 16]),
            IndexMap::from([
                // swapping back from BigEndian to LittleEndian
                (
                    "A".to_string(),
                    UInt32Property::new(u32::swap_bytes(a)).into(),
                ),
                (
                    "B".to_string(),
                    UInt32Property::new(u32::swap_bytes(b)).into(),
                ),
                (
                    "C".to_string(),
                    UInt32Property::new(u32::swap_bytes(c)).into(),
                ),
                (
                    "D".to_string(),
                    UInt32Property::new(u32::swap_bytes(d)).into(),
                ),
            ]),
        )
    }
}

impl Debug for StructProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.guid.0.iter().all(|&x| x == 0) {
            // Call inner formatter for Guid(0)
            if let Some(vector) = self.get_vector() {
                return vector.fmt(f);
            } else if let Some(rotator) = self.get_rotator() {
                return rotator.fmt(f);
            } else if let Some(quat) = self.get_quat() {
                return quat.fmt(f);
            } else if let Some(date_time) = self.get_date_time() {
                return date_time.fmt(f);
            } else if let Some(int_point) = self.get_int_point() {
                return int_point.fmt(f);
            } else if let Some(guid) = self.get_guid() {
                return guid.fmt(f);
            }
        }

        let mut debug_struct = f.debug_struct("StructProperty");

        debug_struct.field("type_name", &self.type_name);
        debug_struct.field("guid", &self.guid);

        if let Some(vector) = self.get_vector() {
            debug_struct.field("vector", &vector);
        } else if let Some(rotator) = self.get_rotator() {
            debug_struct.field("rotator", &rotator);
        } else if let Some(quat) = self.get_quat() {
            debug_struct.field("quat", &quat);
        } else if let Some(date_time) = self.get_date_time() {
            debug_struct.field("date_time", &date_time);
        } else if let Some(int_point) = self.get_int_point() {
            debug_struct.field("int_point", &int_point);
        } else if let Some(guid) = self.get_guid() {
            debug_struct.field("value", &guid);
        } else {
            debug_struct.field("properties", &self.properties);
        }

        debug_struct.finish()
    }
}

impl PartialEq for StructProperty {
    fn eq(&self, other: &Self) -> bool {
        self.type_name == other.type_name
            && self.guid == other.guid
            && self.properties == other.properties
    }
}

impl Eq for StructProperty {}

impl Hash for StructProperty {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.type_name.hash(state);
        self.guid.hash(state);
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for StructProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        struct StructProperty<'a> {
            type_name: &'a String,
            guid: &'a Guid,
            properties: &'a IndexMap<String, Property>,

            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<Guid>,
        }

        StructProperty {
            type_name: &self.type_name,
            guid: &self.guid,
            properties: &self.properties,
            value: self.get_guid(),
        }
        .serialize(serializer)
    }
}
