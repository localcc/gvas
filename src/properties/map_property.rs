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
    enum_property::EnumProperty,
    impl_read_header, impl_write, impl_write_header_part,
    int_property::{BoolProperty, IntProperty},
    name_property::NameProperty,
    str_property::StrProperty,
    Property, PropertyOptions, PropertyTrait,
};

/// A property that stores a map of properties to properties.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MapProperty {
    /// Map<EnumProperty, BoolProperty>
    EnumBool {
        /// Map entries.
        enum_bools: IndexMap<String, bool>,
    },
    /// Map<EnumProperty, IntProperty>
    EnumInt {
        /// Map entries.
        enum_ints: IndexMap<String, i32>,
    },
    /// Map<EnumProperty, Property>
    EnumProperty {
        /// Value type.
        value_type: String,
        /// Map entries.
        enum_props: IndexMap<String, Property>,
    },
    /// Map<NameProperty, BoolProperty>
    NameBool {
        /// Map entries.
        name_bools: IndexMap<String, bool>,
    },
    /// Map<NameProperty, IntProperty>
    NameInt {
        /// Map entries.
        name_ints: IndexMap<String, i32>,
    },
    /// Map<NameProperty, Property>
    NameProperty {
        /// Value type.
        value_type: String,
        /// Map entries.
        name_props: IndexMap<String, Property>,
    },
    /// Map<Property, Property>
    Properties {
        /// Key type name.
        key_type: String,
        /// Value type name.
        value_type: String,
        /// Allocation flags.
        allocation_flags: u32,
        /// Map entries.
        #[cfg_attr(feature = "serde", serde(with = "indexmap::map::serde_seq"))]
        value: IndexMap<Property, Property>,
    },
    /// Map<StrProperty, BoolProperty>
    StrBool {
        /// Map entries.
        str_bools: IndexMap<String, bool>,
    },
    /// Map<StrProperty, IntProperty>
    StrInt {
        /// Map entries.
        str_ints: IndexMap<String, i32>,
    },
    /// Map<StrProperty, Property>
    StrProperty {
        /// Value type.
        value_type: String,
        /// Map entries.
        str_props: IndexMap<String, Property>,
    },
    /// Map<StrProperty, StrProperty>
    StrStr {
        /// Map entries.
        str_strs: IndexMap<String, Option<String>>,
    },
}

impl_write!(
    MapProperty,
    options,
    (write_string, fn, get_key_type),
    (write_string, fn, get_value_type)
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
        match (key_type.as_str(), value_type.as_str(), allocation_flags) {
            ("EnumProperty", "BoolProperty", 0) => match value
                .iter()
                .map(|e| match e {
                    (
                        Property::EnumProperty(EnumProperty {
                            enum_type: None,
                            value: key,
                        }),
                        Property::BoolProperty(BoolProperty { value }),
                    ) => Ok((key.clone(), *value)),
                    // _ => Err(e),
                    _ => Err(()),
                })
                .collect::<Result<_, _>>()
            {
                Ok(enum_bools) => MapProperty::EnumBool { enum_bools },
                // Err(e) => Err(SerializeError::invalid_value(&format!(
                //     "Map entry type does not match container type ({}, {}): {:#?}",
                //     key_type, value_type, e
                // )))?,
                Err(_) => MapProperty::Properties {
                    key_type,
                    value_type,
                    allocation_flags,
                    value,
                },
            },

            ("EnumProperty", "IntProperty", 0) => match value
                .iter()
                .map(|e| match e {
                    (
                        Property::EnumProperty(EnumProperty {
                            enum_type: None,
                            value: key,
                        }),
                        Property::IntProperty(IntProperty { value }),
                    ) => Ok((key.clone(), *value)),
                    _ => Err(()),
                })
                .collect::<Result<_, _>>()
            {
                Ok(enum_ints) => MapProperty::EnumInt { enum_ints },
                Err(_) => MapProperty::Properties {
                    key_type,
                    value_type,
                    allocation_flags,
                    value,
                },
            },

            ("EnumProperty", _, 0) => {
                match value
                    .iter()
                    .map(|e| match e {
                        (
                            Property::EnumProperty(EnumProperty {
                                enum_type: None,
                                value: key,
                            }),
                            value,
                        ) => Ok((key.clone(), value.clone())),
                        _ => Err(()),
                    })
                    .collect::<Result<_, _>>()
                {
                    Ok(enum_props) => MapProperty::EnumProperty {
                        value_type,
                        enum_props,
                    },
                    Err(_) => MapProperty::Properties {
                        key_type,
                        value_type,
                        allocation_flags,
                        value,
                    },
                }
            }

            ("NameProperty", "BoolProperty", 0) => match value
                .iter()
                .map(|e| match e {
                    (
                        Property::NameProperty(NameProperty {
                            array_index: 0,
                            value: Some(key),
                        }),
                        Property::BoolProperty(BoolProperty { value }),
                    ) => Ok((key.clone(), *value)),
                    _ => Err(()),
                })
                .collect::<Result<_, _>>()
            {
                Ok(name_bools) => MapProperty::NameBool { name_bools },
                Err(_) => MapProperty::Properties {
                    key_type,
                    value_type,
                    allocation_flags,
                    value,
                },
            },

            ("NameProperty", "IntProperty", 0) => match value
                .iter()
                .map(|e| match e {
                    (
                        Property::NameProperty(NameProperty {
                            array_index: 0,
                            value: Some(key),
                        }),
                        Property::IntProperty(IntProperty { value }),
                    ) => Ok((key.clone(), *value)),
                    _ => Err(()),
                })
                .collect::<Result<_, _>>()
            {
                Ok(name_ints) => MapProperty::NameInt { name_ints },
                Err(_) => MapProperty::Properties {
                    key_type,
                    value_type,
                    allocation_flags,
                    value,
                },
            },

            ("NameProperty", _, 0) => {
                match value
                    .iter()
                    .map(|e| match e {
                        (
                            Property::NameProperty(NameProperty {
                                array_index: 0,
                                value: Some(key),
                            }),
                            value,
                        ) => Ok((key.clone(), value.clone())),
                        _ => Err(()),
                    })
                    .collect::<Result<_, _>>()
                {
                    Ok(name_props) => MapProperty::NameProperty {
                        value_type,
                        name_props,
                    },
                    Err(_) => MapProperty::Properties {
                        key_type,
                        value_type,
                        allocation_flags,
                        value,
                    },
                }
            }

            ("StrProperty", "BoolProperty", 0) => match value
                .iter()
                .map(|e| match e {
                    (
                        Property::StrProperty(StrProperty { value: Some(key) }),
                        Property::BoolProperty(BoolProperty { value }),
                    ) => Ok((key.clone(), *value)),
                    _ => Err(()),
                })
                .collect::<Result<_, _>>()
            {
                Ok(str_bools) => MapProperty::StrBool { str_bools },
                Err(_) => MapProperty::Properties {
                    key_type,
                    value_type,
                    allocation_flags,
                    value,
                },
            },

            ("StrProperty", "IntProperty", 0) => match value
                .iter()
                .map(|e| match e {
                    (
                        Property::StrProperty(StrProperty { value: Some(key) }),
                        Property::IntProperty(IntProperty { value }),
                    ) => Ok((key.clone(), *value)),
                    _ => Err(()),
                })
                .collect::<Result<_, _>>()
            {
                Ok(str_ints) => MapProperty::StrInt { str_ints },
                Err(_) => MapProperty::Properties {
                    key_type,
                    value_type,
                    allocation_flags,
                    value,
                },
            },

            ("StrProperty", "StrProperty", 0) => match value
                .iter()
                .map(|e| match e {
                    (
                        Property::StrProperty(StrProperty { value: Some(key) }),
                        Property::StrProperty(StrProperty { value }),
                    ) => Ok((key.clone(), value.clone())),
                    _ => Err(()),
                })
                .collect::<Result<_, _>>()
            {
                Ok(str_strs) => MapProperty::StrStr { str_strs },
                Err(_) => MapProperty::Properties {
                    key_type,
                    value_type,
                    allocation_flags,
                    value,
                },
            },

            ("StrProperty", _, 0) => {
                match value
                    .iter()
                    .map(|e| match e {
                        (Property::StrProperty(StrProperty { value: Some(key) }), value) => {
                            Ok((key.clone(), value.clone()))
                        }
                        _ => Err(()),
                    })
                    .collect::<Result<_, _>>()
                {
                    Ok(str_props) => MapProperty::StrProperty {
                        value_type,
                        str_props,
                    },
                    Err(_) => MapProperty::Properties {
                        key_type,
                        value_type,
                        allocation_flags,
                        value,
                    },
                }
            }

            _ => MapProperty::Properties {
                key_type,
                value_type,
                allocation_flags,
                value,
            },
        }
    }

    #[inline]
    pub(crate) fn get_key_type(&self) -> Result<&str, Error> {
        Ok(self.key_type())
    }

    #[inline]
    pub(crate) fn get_value_type(&self) -> Result<&str, Error> {
        Ok(self.value_type())
    }

    #[inline]
    fn key_type(&self) -> &str {
        match self {
            MapProperty::EnumBool { enum_bools: _ } => "EnumProperty",
            MapProperty::EnumInt { enum_ints: _ } => "EnumProperty",
            MapProperty::EnumProperty {
                value_type: _,
                enum_props: _,
            } => "EnumProperty",
            MapProperty::NameBool { name_bools: _ } => "NameProperty",
            MapProperty::NameInt { name_ints: _ } => "NameProperty",
            MapProperty::NameProperty {
                value_type: _,
                name_props: _,
            } => "NameProperty",
            MapProperty::Properties {
                key_type,
                value_type: _,
                allocation_flags: _,
                value: _,
            } => key_type,
            MapProperty::StrBool { str_bools: _ } => "StrProperty",
            MapProperty::StrInt { str_ints: _ } => "StrProperty",
            MapProperty::StrProperty {
                value_type: _,
                str_props: _,
            } => "StrProperty",
            MapProperty::StrStr { str_strs: _ } => "StrProperty",
        }
    }

    #[inline]
    fn value_type(&self) -> &str {
        match self {
            MapProperty::EnumBool { enum_bools: _ } => "BoolProperty",
            MapProperty::EnumInt { enum_ints: _ } => "IntProperty",
            MapProperty::EnumProperty {
                value_type,
                enum_props: _,
            } => value_type,
            MapProperty::NameBool { name_bools: _ } => "BoolProperty",
            MapProperty::NameInt { name_ints: _ } => "IntProperty",
            MapProperty::NameProperty {
                value_type,
                name_props: _,
            } => value_type,
            MapProperty::Properties {
                key_type: _,
                value_type,
                allocation_flags: _,
                value: _,
            } => value_type,
            MapProperty::StrBool { str_bools: _ } => "BoolProperty",
            MapProperty::StrInt { str_ints: _ } => "IntProperty",
            MapProperty::StrProperty {
                value_type,
                str_props: _,
            } => value_type,
            MapProperty::StrStr { str_strs: _ } => "StrProperty",
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

        Ok(MapProperty::new(
            key_type,
            value_type,
            allocation_flags,
            map,
        ))
    }

    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<(), Error> {
        match self {
            MapProperty::EnumBool { enum_bools } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(enum_bools.len() as u32)?;
                for (key, value) in enum_bools {
                    EnumProperty::new(None, key.clone()).write(cursor, false, options)?;
                    BoolProperty::new(*value).write(cursor, false, options)?;
                }
            }

            MapProperty::EnumInt { enum_ints } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(enum_ints.len() as u32)?;
                for (key, value) in enum_ints {
                    EnumProperty::new(None, key.clone()).write(cursor, false, options)?;
                    IntProperty::new(*value).write(cursor, false, options)?;
                }
            }

            MapProperty::EnumProperty {
                value_type: _,
                enum_props,
            } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(enum_props.len() as u32)?;
                for (key, value) in enum_props {
                    EnumProperty::new(None, key.clone()).write(cursor, false, options)?;
                    value.write(cursor, false, options)?;
                }
            }

            MapProperty::NameBool { name_bools } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(name_bools.len() as u32)?;
                for (key, value) in name_bools {
                    NameProperty::from(key.clone()).write(cursor, false, options)?;
                    BoolProperty::new(*value).write(cursor, false, options)?;
                }
            }

            MapProperty::NameInt { name_ints } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(name_ints.len() as u32)?;
                for (key, value) in name_ints {
                    NameProperty::from(key.clone()).write(cursor, false, options)?;
                    IntProperty::new(*value).write(cursor, false, options)?;
                }
            }

            MapProperty::NameProperty {
                value_type: _,
                name_props,
            } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(name_props.len() as u32)?;
                for (key, value) in name_props {
                    NameProperty::from(key.clone()).write(cursor, false, options)?;
                    value.write(cursor, false, options)?;
                }
            }

            MapProperty::Properties {
                key_type: _,
                value_type: _,
                allocation_flags,
                value,
            } => {
                cursor.write_u32::<LittleEndian>(*allocation_flags)?;
                cursor.write_u32::<LittleEndian>(value.len() as u32)?;
                for (key, value) in value {
                    key.write(cursor, false, options)?;
                    value.write(cursor, false, options)?;
                }
            }

            MapProperty::StrBool { str_bools } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(str_bools.len() as u32)?;
                for (key, value) in str_bools {
                    StrProperty::from(key.clone()).write(cursor, false, options)?;
                    BoolProperty::new(*value).write(cursor, false, options)?;
                }
            }

            MapProperty::StrInt { str_ints } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(str_ints.len() as u32)?;
                for (key, value) in str_ints {
                    StrProperty::from(key.clone()).write(cursor, false, options)?;
                    IntProperty::new(*value).write(cursor, false, options)?;
                }
            }

            MapProperty::StrProperty {
                value_type: _,
                str_props,
            } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(str_props.len() as u32)?;
                for (key, value) in str_props {
                    StrProperty::from(key.clone()).write(cursor, false, options)?;
                    value.write(cursor, false, options)?;
                }
            }

            MapProperty::StrStr { str_strs } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(str_strs.len() as u32)?;
                for (key, value) in str_strs {
                    StrProperty::from(key.clone()).write(cursor, false, options)?;
                    StrProperty::new(value.clone()).write(cursor, false, options)?;
                }
            }
        };

        Ok(())
    }
}

impl Hash for MapProperty {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            MapProperty::Properties {
                key_type,
                value_type,
                allocation_flags,
                value: _,
            } => {
                key_type.hash(state);
                value_type.hash(state);
                allocation_flags.hash(state);
            }
            _ => {
                self.key_type().hash(state);
                self.value_type().hash(state);
                0u32.hash(state);
            }
        }
    }
}
