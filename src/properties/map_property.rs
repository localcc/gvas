use std::{
    hash::Hash,
    io::{Cursor, Read, Seek, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use indexmap::IndexMap;

use crate::{
    cursor_ext::{ReadExt, WriteExt},
    error::{DeserializeError, Error},
    properties::{
        enum_property::EnumProperty,
        impl_read_header, impl_write, impl_write_header_part,
        int_property::{BoolProperty, IntProperty},
        name_property::NameProperty,
        str_property::StrProperty,
        Property, PropertyOptions, PropertyTrait,
    },
    scoped_stack_entry::ScopedStackEntry,
    types::map::HashableIndexMap,
};

/// A property that stores a map of properties to properties.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MapProperty {
    /// Map<EnumProperty, BoolProperty>
    EnumBool {
        /// Map entries.
        enum_bools: HashableIndexMap<String, bool>,
    },
    /// Map<EnumProperty, IntProperty>
    EnumInt {
        /// Map entries.
        enum_ints: HashableIndexMap<String, i32>,
    },
    /// Map<EnumProperty, Property>
    EnumProperty {
        /// Value type.
        value_type: String,
        /// Map entries.
        enum_props: HashableIndexMap<String, Property>,
    },
    /// Map<NameProperty, BoolProperty>
    NameBool {
        /// Map entries.
        name_bools: HashableIndexMap<String, bool>,
    },
    /// Map<NameProperty, IntProperty>
    NameInt {
        /// Map entries.
        name_ints: HashableIndexMap<String, i32>,
    },
    /// Map<NameProperty, Property>
    NameProperty {
        /// Value type.
        value_type: String,
        /// Map entries.
        name_props: HashableIndexMap<String, Property>,
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
        #[cfg_attr(feature = "serde", serde(with = "crate::types::map::serde_seq"))]
        value: HashableIndexMap<Property, Property>,
    },
    /// Map<StrProperty, BoolProperty>
    StrBool {
        /// Map entries.
        str_bools: HashableIndexMap<String, bool>,
    },
    /// Map<StrProperty, IntProperty>
    StrInt {
        /// Map entries.
        str_ints: HashableIndexMap<String, i32>,
    },
    /// Map<StrProperty, Property>
    StrProperty {
        /// Value type.
        value_type: String,
        /// Map entries.
        str_props: HashableIndexMap<String, Property>,
    },
    /// Map<StrProperty, StrProperty>
    StrStr {
        /// Map entries.
        str_strs: HashableIndexMap<String, Option<String>>,
    },
}

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
                Ok(enum_bools) => MapProperty::EnumBool {
                    enum_bools: HashableIndexMap(enum_bools),
                },
                // Err(e) => Err(SerializeError::invalid_value(&format!(
                //     "Map entry type does not match container type ({}, {}): {:#?}",
                //     key_type, value_type, e
                // )))?,
                Err(_) => MapProperty::Properties {
                    key_type,
                    value_type,
                    allocation_flags,
                    value: HashableIndexMap(value),
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
                Ok(enum_ints) => MapProperty::EnumInt {
                    enum_ints: HashableIndexMap(enum_ints),
                },
                Err(_) => MapProperty::Properties {
                    key_type,
                    value_type,
                    allocation_flags,
                    value: HashableIndexMap(value),
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
                        enum_props: HashableIndexMap(enum_props),
                    },
                    Err(_) => MapProperty::Properties {
                        key_type,
                        value_type,
                        allocation_flags,
                        value: HashableIndexMap(value),
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
                Ok(name_bools) => MapProperty::NameBool {
                    name_bools: HashableIndexMap(name_bools),
                },
                Err(_) => MapProperty::Properties {
                    key_type,
                    value_type,
                    allocation_flags,
                    value: HashableIndexMap(value),
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
                Ok(name_ints) => MapProperty::NameInt {
                    name_ints: HashableIndexMap(name_ints),
                },
                Err(_) => MapProperty::Properties {
                    key_type,
                    value_type,
                    allocation_flags,
                    value: HashableIndexMap(value),
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
                        name_props: HashableIndexMap(name_props),
                    },
                    Err(_) => MapProperty::Properties {
                        key_type,
                        value_type,
                        allocation_flags,
                        value: HashableIndexMap(value),
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
                Ok(str_bools) => MapProperty::StrBool {
                    str_bools: HashableIndexMap(str_bools),
                },
                Err(_) => MapProperty::Properties {
                    key_type,
                    value_type,
                    allocation_flags,
                    value: HashableIndexMap(value),
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
                Ok(str_ints) => MapProperty::StrInt {
                    str_ints: HashableIndexMap(str_ints),
                },
                Err(_) => MapProperty::Properties {
                    key_type,
                    value_type,
                    allocation_flags,
                    value: HashableIndexMap(value),
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
                Ok(str_strs) => MapProperty::StrStr {
                    str_strs: HashableIndexMap(str_strs),
                },
                Err(_) => MapProperty::Properties {
                    key_type,
                    value_type,
                    allocation_flags,
                    value: HashableIndexMap(value),
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
                        str_props: HashableIndexMap(str_props),
                    },
                    Err(_) => MapProperty::Properties {
                        key_type,
                        value_type,
                        allocation_flags,
                        value: HashableIndexMap(value),
                    },
                }
            }

            _ => MapProperty::Properties {
                key_type,
                value_type,
                allocation_flags,
                value: HashableIndexMap(value),
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
}

impl PropertyTrait for MapProperty {
    impl_write!(
        MapProperty,
        (write_string, fn, get_key_type),
        (write_string, fn, get_value_type)
    );

    #[inline]
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<usize, Error> {
        match self {
            MapProperty::EnumBool {
                enum_bools: HashableIndexMap(enum_bools),
            } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(enum_bools.len() as u32)?;
                let mut len = 8;
                for (key, value) in enum_bools {
                    let k_property = EnumProperty::new(None, key.clone());
                    let v_property = BoolProperty::new(*value);
                    len += k_property.write(cursor, false, options)?;
                    len += v_property.write(cursor, false, options)?;
                }
                Ok(len)
            }

            MapProperty::EnumInt {
                enum_ints: HashableIndexMap(enum_ints),
            } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(enum_ints.len() as u32)?;
                let mut len = 8;
                for (key, value) in enum_ints {
                    let k_property = EnumProperty::new(None, key.clone());
                    let v_property = IntProperty::new(*value);
                    len += k_property.write(cursor, false, options)?;
                    len += v_property.write(cursor, false, options)?;
                }
                Ok(len)
            }

            MapProperty::EnumProperty {
                value_type: _,
                enum_props: HashableIndexMap(enum_props),
            } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(enum_props.len() as u32)?;
                let mut len = 8;
                for (key, value) in enum_props {
                    let property = EnumProperty::new(None, key.clone());
                    len += property.write(cursor, false, options)?;
                    len += value.write(cursor, false, options)?;
                }
                Ok(len)
            }

            MapProperty::NameBool {
                name_bools: HashableIndexMap(name_bools),
            } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(name_bools.len() as u32)?;
                let mut len = 8;
                for (key, value) in name_bools {
                    let k_property = NameProperty::from(key.clone());
                    let v_property = BoolProperty::new(*value);
                    len += k_property.write(cursor, false, options)?;
                    len += v_property.write(cursor, false, options)?;
                }
                Ok(len)
            }

            MapProperty::NameInt {
                name_ints: HashableIndexMap(name_ints),
            } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(name_ints.len() as u32)?;
                let mut len = 8;
                for (key, value) in name_ints {
                    let k_property = NameProperty::from(key.clone());
                    let v_property = IntProperty::new(*value);
                    len += k_property.write(cursor, false, options)?;
                    len += v_property.write(cursor, false, options)?;
                }
                Ok(len)
            }

            MapProperty::NameProperty {
                value_type: _,
                name_props: HashableIndexMap(name_props),
            } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(name_props.len() as u32)?;
                let mut len = 8;
                for (key, value) in name_props {
                    let property = NameProperty::from(key.clone());
                    len += property.write(cursor, false, options)?;
                    len += value.write(cursor, false, options)?;
                }
                Ok(len)
            }

            MapProperty::Properties {
                key_type: _,
                value_type: _,
                allocation_flags,
                value: HashableIndexMap(value),
            } => {
                cursor.write_u32::<LittleEndian>(*allocation_flags)?;
                cursor.write_u32::<LittleEndian>(value.len() as u32)?;
                let mut len = 8;
                for (key, value) in value {
                    len += key.write(cursor, false, options)?;
                    len += value.write(cursor, false, options)?;
                }
                Ok(len)
            }

            MapProperty::StrBool {
                str_bools: HashableIndexMap(str_bools),
            } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(str_bools.len() as u32)?;
                let mut len = 8;
                for (key, value) in str_bools {
                    let k_property = StrProperty::from(key.clone());
                    let v_property = BoolProperty::new(*value);
                    len += k_property.write(cursor, false, options)?;
                    len += v_property.write(cursor, false, options)?;
                }
                Ok(len)
            }

            MapProperty::StrInt {
                str_ints: HashableIndexMap(str_ints),
            } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(str_ints.len() as u32)?;
                let mut len = 8;
                for (key, value) in str_ints {
                    let k_property = StrProperty::from(key.clone());
                    let v_property = IntProperty::new(*value);
                    len += k_property.write(cursor, false, options)?;
                    len += v_property.write(cursor, false, options)?;
                }
                Ok(len)
            }

            MapProperty::StrProperty {
                value_type: _,
                str_props: HashableIndexMap(str_props),
            } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(str_props.len() as u32)?;
                let mut len = 8;
                for (key, value) in str_props {
                    let property = StrProperty::from(key.clone());
                    len += property.write(cursor, false, options)?;
                    len += value.write(cursor, false, options)?;
                }
                Ok(len)
            }

            MapProperty::StrStr {
                str_strs: HashableIndexMap(str_strs),
            } => {
                cursor.write_u32::<LittleEndian>(0)?;
                cursor.write_u32::<LittleEndian>(str_strs.len() as u32)?;
                let mut len = 8;
                for (key, value) in str_strs {
                    let k_property = StrProperty::from(key.clone());
                    let v_property = StrProperty::new(value.clone());
                    len += k_property.write(cursor, false, options)?;
                    len += v_property.write(cursor, false, options)?;
                }
                Ok(len)
            }
        }
    }
}
