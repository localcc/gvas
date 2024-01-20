use std::{
    hash::Hash,
    io::{Cursor, Read, Seek, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use indexmap::IndexMap;

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::{DeserializeError, Error},
    scoped_stack_entry::ScopedStackEntry,
};

use super::{
    impl_read_header, impl_write, impl_write_header_part, Property, PropertyOptions, PropertyTrait,
};

#[cfg(feature = "serde")]
use {
    super::{
        int_property::IntProperty,
        name_property::NameProperty,
        str_property::StrProperty,
        struct_property::{StructProperty, StructPropertyValue},
    },
    crate::types::Guid,
    serde::{
        de::{self, Deserializer, MapAccess, Visitor},
        ser::SerializeMap,
        Deserialize, Serialize, Serializer,
    },
};

/// A property that stores a map of properties to properties.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapProperty {
    /// Key type name.
    pub key_type: String,
    /// Value type name.
    pub value_type: String,
    /// Allocation flags.
    pub allocation_flags: u32,
    /// Map entries.
    pub value: IndexMap<Property, Property>,
}

impl_write!(
    MapProperty,
    options,
    (write_string, key_type),
    (write_string, value_type)
);

impl MapProperty {
    /// Creates a new `MapProperty` instance.
    #[inline]
    pub fn new(
        key_type: String,
        value_type: String,
        allocation_flags: u32,
        value: IndexMap<Property, Property>,
    ) -> Self {
        MapProperty {
            key_type,
            value_type,
            allocation_flags,
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
                "MapProperty is not supported in arrays",
                cursor,
            ))?
        }
    }

    impl_read_header!(options, key_type, value_type);

    #[inline]
    fn read_body<R: Read + Seek>(
        cursor: &mut R,
        options: &mut PropertyOptions,
        key_type: String,
        value_type: String,
    ) -> Result<Self, Error> {
        let allocation_flags = cursor.read_u32::<LittleEndian>()?;
        let element_count = cursor.read_u32::<LittleEndian>()?;

        let mut map = IndexMap::new();
        for _ in 0..element_count {
            let properties_stack = &mut options.properties_stack;
            let key_stack_entry = ScopedStackEntry::new(properties_stack, "Key".to_string());
            let key = Property::new(cursor, &key_type, false, options, None)?;
            drop(key_stack_entry);

            let properties_stack = &mut options.properties_stack;
            let value_stack_entry = ScopedStackEntry::new(properties_stack, "Value".to_string());
            let value = Property::new(cursor, &value_type, false, options, None)?;
            drop(value_stack_entry);

            map.insert(key, value);
        }

        Ok(MapProperty {
            key_type,
            value_type,
            allocation_flags,
            value: map,
        })
    }

    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        cursor.write_u32::<LittleEndian>(self.allocation_flags)?;
        cursor.write_u32::<LittleEndian>(self.value.len() as u32)?;

        for (key, value) in &self.value {
            key.write(cursor, false, options)?;
            value.write(cursor, false, options)?;
        }

        Ok(())
    }
}

impl Hash for MapProperty {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key_type.hash(state);
        self.value_type.hash(state);
        self.allocation_flags.hash(state);
    }
}

#[cfg(feature = "serde")]
impl Serialize for MapProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let entry_count = if self.allocation_flags == 0 { 3 } else { 4 };
        let mut map = serializer.serialize_map(Some(entry_count))?;
        map.serialize_entry("key_type", &self.key_type)?;
        map.serialize_entry("value_type", &self.value_type)?;
        if self.allocation_flags != 0 {
            map.serialize_entry("allocation_flags", &self.allocation_flags)?;
        }
        match (self.key_type.as_str(), self.value_type.as_str()) {
            ("NameProperty" | "StrProperty", "IntProperty") => {
                let values: IndexMap<String, i32> = self
                    .value
                    .iter()
                    .map(|(key, value)| match (key, value) {
                        (
                            Property::NameProperty(NameProperty {
                                value: Some(key),
                                array_index: 0,
                            }),
                            Property::IntProperty(IntProperty { value }),
                        ) => (key.to_string(), *value),
                        (
                            Property::StrProperty(StrProperty { value: Some(key) }),
                            Property::IntProperty(IntProperty { value }),
                        ) => (key.to_string(), *value),
                        _ => unreachable!("Unexpected combination of Property variants."),
                    })
                    .collect();
                map.serialize_entry("values", &values)?;
            }
            ("NameProperty" | "StrProperty", "StrProperty") => {
                let values: IndexMap<String, String> = self
                    .value
                    .iter()
                    .map(|(key, value)| match (key, value) {
                        (
                            Property::NameProperty(NameProperty {
                                value: Some(key),
                                array_index: 0,
                            }),
                            Property::StrProperty(StrProperty { value: Some(value) }),
                        ) => (key.to_string(), value.to_string()),
                        (
                            Property::StrProperty(StrProperty { value: Some(key) }),
                            Property::StrProperty(StrProperty { value: Some(value) }),
                        ) => (key.to_string(), value.to_string()),
                        _ => unreachable!("Unexpected combination of Property variants."),
                    })
                    .collect();
                map.serialize_entry("values", &values)?;
            }
            ("NameProperty" | "StrProperty", "StructProperty") => {
                let values: IndexMap<String, &StructPropertyValue> = self
                    .value
                    .iter()
                    .map(|(key, value)| match (key, value) {
                        (
                            Property::NameProperty(NameProperty {
                                value: Some(key),
                                array_index: 0,
                            }),
                            Property::StructProperty(StructProperty {
                                guid: Guid([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                                value,
                            }),
                        ) => (key.to_string(), value),
                        (
                            Property::StrProperty(StrProperty { value: Some(key) }),
                            Property::StructProperty(StructProperty {
                                guid: Guid([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                                value,
                            }),
                        ) => (key.to_string(), value),
                        _ => unreachable!("Unexpected combination of Property variants."),
                    })
                    .collect();
                map.serialize_entry("values", &values)?;
            }
            _ => Err(serde::ser::Error::custom(format!(
                "Unsupported key and value types ({}, {})",
                self.key_type, self.value_type
            )))?,
        };
        map.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for MapProperty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MapPropertyVisitor;

        impl<'de> Visitor<'de> for MapPropertyVisitor {
            type Value = MapProperty;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct MapProperty")
            }

            fn visit_map<M>(self, mut map: M) -> Result<MapProperty, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut key_type: Option<String> = None;
                let mut value_type: Option<String> = None;
                let mut allocation_flags: u32 = 0;
                let mut found_values = false;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "key_type" => {
                            key_type = Some(map.next_value()?);
                        }
                        "value_type" => {
                            value_type = Some(map.next_value()?);
                        }
                        "allocation_flags" => {
                            allocation_flags = map.next_value()?;
                        }
                        "values" => {
                            found_values = true;
                            break;
                        }
                        _ => Err(serde::de::Error::custom(format!("Unsupported key {}", key)))?,
                    }
                }

                let key_type = key_type.ok_or_else(|| de::Error::missing_field("key_type"))?;
                let value_type =
                    value_type.ok_or_else(|| de::Error::missing_field("value_type"))?;
                if !found_values {
                    Err(de::Error::missing_field("values"))?
                }

                // Convert values into the IndexMap<Property, Property>
                let value: IndexMap<Property, Property> =
                    match (key_type.as_str(), value_type.as_str()) {
                        ("NameProperty", "IntProperty") => map
                            .next_value::<IndexMap<String, i32>>()?
                            .into_iter()
                            .map(|(key, value)| {
                                (
                                    Property::NameProperty(NameProperty::from(Some(key))),
                                    Property::IntProperty(IntProperty { value }),
                                )
                            })
                            .collect(),
                        ("StrProperty", "IntProperty") => map
                            .next_value::<IndexMap<String, i32>>()?
                            .into_iter()
                            .map(|(key, value)| {
                                (
                                    Property::StrProperty(StrProperty { value: Some(key) }),
                                    Property::IntProperty(IntProperty { value }),
                                )
                            })
                            .collect(),
                        ("StrProperty", "StrProperty") => map
                            .next_value::<IndexMap<String, String>>()?
                            .into_iter()
                            .map(|(key, value)| {
                                (
                                    Property::StrProperty(StrProperty { value: Some(key) }),
                                    Property::StrProperty(StrProperty { value: Some(value) }),
                                )
                            })
                            .collect(),
                        ("NameProperty", "StructProperty") => map
                            .next_value::<IndexMap<String, StructPropertyValue>>()?
                            .into_iter()
                            .map(|(key, value)| {
                                (
                                    Property::NameProperty(NameProperty::from(Some(key))),
                                    Property::StructProperty(StructProperty {
                                        guid: Guid::from(0),
                                        value,
                                    }),
                                )
                            })
                            .collect(),
                        _ => Err(serde::de::Error::custom(format!(
                            "Unsupported key and value types ({}, {})",
                            key_type, value_type
                        )))?,
                    };

                Ok(MapProperty {
                    key_type,
                    value_type,
                    allocation_flags,
                    value,
                })
            }
        }

        deserializer.deserialize_map(MapPropertyVisitor)
    }
}
