use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    io::{Read, Seek, Write},
};

use enum_dispatch::enum_dispatch;
use indexmap::IndexMap;

use crate::{
    custom_version::{CustomVersionTrait, FCustomVersion},
    error::{DeserializeError, Error},
    scoped_stack_entry::ScopedStackEntry,
    types::Guid,
};

use self::{
    array_property::ArrayProperty,
    delegate_property::{
        DelegateProperty, MulticastInlineDelegateProperty, MulticastSparseDelegateProperty,
    },
    enum_property::EnumProperty,
    field_path_property::FieldPathProperty,
    int_property::{
        BoolProperty, ByteProperty, DoubleProperty, FloatProperty, Int16Property, Int64Property,
        Int8Property, IntProperty, UInt16Property, UInt32Property, UInt64Property,
    },
    map_property::MapProperty,
    name_property::NameProperty,
    object_property::ObjectProperty,
    set_property::SetProperty,
    str_property::StrProperty,
    struct_property::StructProperty,
    text_property::TextProperty,
    unknown_property::UnknownProperty,
};

/// Module for `ArrayProperty`.
pub mod array_property;
/// Module for delegates
pub mod delegate_property;
/// Module for `EnumProperty`.
pub mod enum_property;
/// Module for `FieldPathProperty`
pub mod field_path_property;
/// Module for `IntProperty` and various integer properties.
pub mod int_property;
/// Module for `MapProperty`
pub mod map_property;
/// Module for `NameProperty`
pub mod name_property;
/// Module for `ObjectProperty`
pub mod object_property;
/// Module for `SetProperty`
pub mod set_property;
/// Module for `StrProperty`
pub mod str_property;
/// Module for `StructProperty`
pub mod struct_property;
/// Module for `StructProperty` sub-types.
pub mod struct_types;
/// Module for `TextProperty`
pub mod text_property;
/// Module for `UnknownProperty`
pub mod unknown_property;

/// This macro generates a `read` function for reading GVAS property data from a reader.
///
/// If `include_header` is true, it will read the property header first.
///
/// This macro must be used in conjunction with a suitable `read_header` function, such as one
/// generated by `impl_read_header!(...)`.
macro_rules! impl_read {
    () => {
        /// Read GVAS property data from a reader.
        ///
        /// If `include_header` is true, read the property header first.
        #[inline]
        pub fn read<R: Read + Seek>(reader: &mut R, include_header: bool) -> Result<Self, Error> {
            if include_header {
                Self::read_header(reader)
            } else {
                Self::read_body(reader)
            }
        }
    };

    (options) => {
        /// Read GVAS property data from a reader.
        ///
        /// If `include_header` is true, read the property header first.
        #[inline]
        pub fn read<R: Read + Seek>(
            reader: &mut R,
            include_header: bool,
            options: &mut PropertyOptions,
        ) -> Result<Self, Error> {
            if include_header {
                Self::read_header(reader, options)
            } else {
                Self::read_body(reader, options)
            }
        }
    };

    (array_index) => {
        /// Read GVAS property data from a reader.
        ///
        /// If `include_header` is true, read the property header first.
        #[inline]
        pub fn read<R: Read + Seek>(reader: &mut R, include_header: bool) -> Result<Self, Error> {
            if include_header {
                Self::read_header(reader)
            } else {
                Self::read_body(reader, 0)
            }
        }
    };
}

/// This macro generates a `read_header` function for reading GVAS property headers from a reader.
///
/// This macro must be used in conjunction with a suitable `read_body` function.
///
/// ```ignore
/// use std::io::{Read, Seek};
///
/// use byteorder::{LittleEndian, ReadBytesExt};
///
/// use crate::{
///     error::Error,
///     properties::{impl_read, impl_read_header},
/// };
///
/// struct ExampleProperty(
///     // ...
/// );
///
/// impl ExampleProperty {
///     impl_read!();
///     impl_read_header!();
///     fn read_body<R: Read + Seek>(reader: &mut R) -> Result<Self, Error> {
///         // Read values from reader...
///         Ok(Self (
///             // ...
///         ))
///     }
/// }
/// ```
macro_rules! impl_read_header {
    (options, length $(, $var:ident)*) => {
        #[inline]
        fn read_header<R: Read + Seek>(
            reader: &mut R,
            options: &mut PropertyOptions,
        ) -> Result<Self, Error> {
            let length = reader.read_u32::<LittleEndian>()?;
            let array_index = reader.read_u32::<LittleEndian>()?;
            assert_eq!(
                array_index,
                0,
                "Expected array_index value zero @ {:#x}",
                reader.stream_position()? - 4,
            );
            $(
                let $var = reader.read_string()?;
            )*
            let separator = reader.read_u8()?;
            assert_eq!(
                separator,
                0,
                "Expected separator value zero @ {:#x}",
                reader.stream_position()? - 1,
            );

            let start = reader.stream_position()?;
            let result = Self::read_body(reader, options, length $(, $var)*)?;
            let end = reader.stream_position()?;
            assert_eq!(
                end - start,
                length as u64,
                "read_body did not read the expected length {:#x}",
                length,
            );

            Ok(result)
        }
    };

    (options $(, $var:ident)*) => {
        #[inline]
        fn read_header<R: Read + Seek>(
            reader: &mut R,
            options: &mut PropertyOptions,
        ) -> Result<Self, Error> {
            let length = reader.read_u32::<LittleEndian>()?;
            let array_index = reader.read_u32::<LittleEndian>()?;
            assert_eq!(
                array_index,
                0,
                "Expected array_index value zero @ {:#x}",
                reader.stream_position()? - 4,
            );
            $(
                let $var = reader.read_string()?;
            )*
            let separator = reader.read_u8()?;
            assert_eq!(
                separator,
                0,
                "Expected separator value zero @ {:#x}",
                reader.stream_position()? - 1,
            );

            let start = reader.stream_position()?;
            let result = Self::read_body(reader, options $(, $var)*)?;
            let end = reader.stream_position()?;
            assert_eq!(
                end - start,
                length as u64,
                "read_body read {:#x}, expected {:#x}\n{:#?}",
                end - start,
                length,
                result,
            );

            Ok(result)
        }
    };

    (array_index $(, $var:ident)*) => {
        #[inline]
        fn read_header<R: Read + Seek>(
            reader: &mut R,
        ) -> Result<Self, Error> {
            let length = reader.read_u32::<LittleEndian>()?;
            let array_index = reader.read_u32::<LittleEndian>()?;
            $(
                let $var = reader.read_string()?;
            )*
            let separator = reader.read_u8()?;
            assert_eq!(
                separator,
                0,
                "Expected separator value zero @ {:#x}",
                reader.stream_position()? - 1,
            );

            let start = reader.stream_position()?;
            let result = Self::read_body(reader, array_index $(, Some($var))*)?;
            let end = reader.stream_position()?;
            assert_eq!(
                end - start,
                length as u64,
                "read_body did not read the expected length {:#x}",
                length,
            );

            Ok(result)
        }
    };

    ($($var:ident $(,)? )*) => {
        #[inline]
        fn read_header<R: Read + Seek>(
            reader: &mut R,
        ) -> Result<Self, Error> {
            let length = reader.read_u32::<LittleEndian>()?;
            let array_index = reader.read_u32::<LittleEndian>()?;
            assert_eq!(array_index, 0, "Expected array_index value zero @ {:#x}", reader.stream_position()? - 4);
            $(
                let $var = reader.read_string()?;
            )*
            let separator = reader.read_u8()?;
            assert_eq!(
                separator,
                0,
                "Expected separator value zero @ {:#x}",
                reader.stream_position()? - 1,
            );

            let start = reader.stream_position()?;
            let result = Self::read_body(reader $(, Some($var))*)?;
            let end = reader.stream_position()?;
            assert_eq!(
                end - start,
                length as u64,
                "read_body did not read the expected length {:#x}",
                length,
            );

            Ok(result)
        }
    };
}

pub(crate) use impl_read;
pub(crate) use impl_read_header;

/// This macro generates a `write` function for writing the property data to a writer.
/// If `include_header` is true, it will write the property header first.
///
/// # Examples
///
/// ```ignore
/// impl_write!(ArrayProperty, options, (write_string, property_type));
/// ```
///
/// This generates a `write` function for the `ArrayProperty` type with `PropertyOptions` support,
/// writing `&self.property_name` in to the header using the `write_string` function.
///
/// ```ignore
/// impl_write!(NameProperty);
/// ```
///
/// This generates a basic `write` function for the `NameProperty` type.
///
/// ```ignore
/// impl_write!(
///     StructProperty,
///     options,
///     (write_string, fn, get_property_name),
///     (write_guid, guid)
/// );
/// ```
///
/// This generateds and advanced `write` function for the `StructProperty` type, writing
/// `&self.get_property_name()?` in to the header using the `write_string` function, and
/// `&self.guid` using the `write_guid` function.
///
/// # Notes
///
/// This macro must be used in conjunction with a suitable `write_body` function.
macro_rules! impl_write {
    ($property:ident, array_index $(, $header_property:tt)*) => {
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

            let mut len = 9;
            let buf = &mut Cursor::new(Vec::new());
            len += self.write_body(buf, options)?;
            let buf = buf.get_ref();

            writer.write_string(stringify!($property))?;
            writer.write_u32::<LittleEndian>(buf.len() as u32)?;
            writer.write_u32::<LittleEndian>(self.array_index)?;
            $(
                len += impl_write_header_part!(self, writer, $header_property);
            )*
            writer.write_u8(0)?;
            writer.write_all(buf)?;

            Ok(len)
        }
    };

    ($property:ident $(, $header_property:tt)*) => {
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

            let mut len = 9;
            let buf = &mut Cursor::new(Vec::new());
            len += self.write_body(buf, options)?;
            let buf = buf.get_ref();

            len += writer.write_string(stringify!($property))?;
            writer.write_u32::<LittleEndian>(buf.len() as u32)?;
            writer.write_u32::<LittleEndian>(0)?;
            $(
                len += impl_write_header_part!(self, writer, $header_property);
            )*
            writer.write_u8(0)?;
            writer.write_all(buf)?;

            Ok(len)
        }
    };
}

/// A helper macro for writing property header parts.
///
/// This macro is used inside the `impl_write!` macro to write individual parts of a property header.
macro_rules! impl_write_header_part {
    ($self:ident, $writer:ident, (write_fstring, $member:ident)) => {
        $writer.write_fstring($self.$member.as_deref())?
    };

    ($self:ident, $writer:ident, (write_guid, $member:ident)) => {{
        $writer.write_guid(&$self.$member)?;
        16
    }};

    ($self:ident, $writer:ident, ($write_fn:ident, $member:ident)) => {
        $writer.$write_fn(&$self.$member)?
    };

    ($self:ident, $writer:ident, ($write_fn:ident, fn, $member:ident)) => {
        $writer.$write_fn(&$self.$member()?)?
    };
}

pub(crate) use impl_write;
pub(crate) use impl_write_header_part;

/// This macro generates a helper function for matching a specific variant of an enum.
///
/// # Examples
///
/// ```ignore
/// make_matcher!(MyEnumVariant, get_my_enum_variant);
/// ```
///
/// This generates a `get_my_enum_variant` function that returns an `Option<&MyEnumVariant>`
/// if the enum instance is of the `MyEnumVariant` variant.

macro_rules! make_matcher {
    ($type:ident, $name:ident, $name_mut:ident) => {
        #[doc = concat!("Retrieves the enum value as a `", stringify!($type), "`.")]
        #[inline]
        pub fn $name(&self) -> Option<&$type> {
            match self {
                Self::$type(e) => Some(e),
                _ => None,
            }
        }

        #[doc = concat!("Retrieves the mutable enum value as a `", stringify!($type), "`.")]
        #[inline]
        pub fn $name_mut(&mut self) -> Option<&mut $type> {
            match self {
                Self::$type(e) => Some(e),
                _ => None,
            }
        }
    };
}

pub(crate) use make_matcher;

/// Property options used for reading and writing.
pub struct PropertyOptions<'a> {
    /// Hints about property types.
    pub hints: &'a HashMap<String, String>,
    /// Tracks the property tree location in a GVAS file.
    pub properties_stack: &'a mut Vec<String>,
    /// Custom versions
    pub custom_versions: &'a IndexMap<Guid, u32>,
    /// Enables large world coordinates.
    pub large_world_coordinates: bool,
}

impl<'a> PropertyOptions<'a> {
    /// Get custom version
    #[inline]
    pub fn get_custom_version<T>(&self) -> FCustomVersion
    where
        T: CustomVersionTrait + Into<u32>,
    {
        let key = T::GUID;
        let version = self.custom_versions.get(&key).copied().unwrap_or(0);
        FCustomVersion { key, version }
    }

    /// Check for custom version support
    #[inline]
    pub fn supports_version<T>(&self, required: T) -> bool
    where
        T: CustomVersionTrait + Into<u32>,
    {
        self.get_custom_version::<T>().version >= required.into()
    }
}

/// Property traits.
#[enum_dispatch]
pub trait PropertyTrait: Debug + Clone + PartialEq + Eq + Hash {
    /// Serialize.
    fn write<W: Write>(
        &self,
        cursor: &mut W,
        include_header: bool,
        options: &mut PropertyOptions,
    ) -> Result<usize, Error>;

    /// Serialize body.
    fn write_body<W: Write>(
        &self,
        cursor: &mut W,
        options: &mut PropertyOptions,
    ) -> Result<usize, Error>;
}

/// GVAS property types.
#[enum_dispatch(PropertyTrait)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type")
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Property {
    /// An `ArrayProperty`.
    ArrayProperty,
    /// A `BoolProperty`.
    BoolProperty,
    /// A `ByteProperty`.
    ByteProperty,
    /// A `DoubleProperty`.
    DoubleProperty,
    /// An `EnumProperty`.
    EnumProperty,
    /// A `FloatPropertyF`.
    FloatProperty,
    /// An `Int16Property`.
    Int16Property,
    /// An `Int64Property`.
    Int64Property,
    /// An `Int8Property`.
    Int8Property,
    /// An `IntProperty`.
    IntProperty,
    /// A `MapProperty`.
    MapProperty,
    /// A `NameProperty`.
    NameProperty,
    /// An `ObjectProperty`
    ObjectProperty,
    /// A `DelegateProperty`
    DelegateProperty,
    /// A `MulticastInlineDelegateProperty`
    MulticastInlineDelegateProperty,
    /// A `MulticastSparseDelegateProperty`
    MulticastSparseDelegateProperty,
    /// A `FieldPathProperty`
    FieldPathProperty,
    /// A `SetProperty`.
    SetProperty,
    /// A `StrProperty`.
    StrProperty,
    /// A `StructProperty`.
    StructProperty,
    /// A `TextProperty`.
    TextProperty,
    /// A `UInt16Property`.
    UInt16Property,
    /// A `UInt32Property`.
    UInt32Property,
    /// A `UInt64Property`.
    UInt64Property,
    /// An `UnknownProperty`.
    UnknownProperty,
}

impl Property {
    /// Creates a new `Property` instance.
    pub fn new<R: Read + Seek>(
        cursor: &mut R,
        value_type: &str,
        include_header: bool,
        options: &mut PropertyOptions,
        suggested_length: Option<u32>,
    ) -> Result<Self, Error> {
        let _stack_entry = ScopedStackEntry::new(options.properties_stack, value_type.to_string());
        match value_type {
            "Int8Property" => Ok(Int8Property::read(cursor, include_header)?.into()),
            "ByteProperty" => {
                Ok(ByteProperty::read(cursor, include_header, suggested_length)?.into())
            }
            "Int16Property" => Ok(Int16Property::read(cursor, include_header)?.into()),
            "UInt16Property" => Ok(UInt16Property::read(cursor, include_header)?.into()),
            "IntProperty" => Ok(IntProperty::read(cursor, include_header)?.into()),
            "UInt32Property" => Ok(UInt32Property::read(cursor, include_header)?.into()),
            "Int64Property" => Ok(Int64Property::read(cursor, include_header)?.into()),
            "UInt64Property" => Ok(UInt64Property::read(cursor, include_header)?.into()),
            "FloatProperty" => Ok(FloatProperty::read(cursor, include_header)?.into()),
            "DoubleProperty" => Ok(DoubleProperty::read(cursor, include_header)?.into()),
            "BoolProperty" => Ok(BoolProperty::read(cursor, include_header)?.into()),
            "EnumProperty" => Ok(EnumProperty::read(cursor, include_header)?.into()),
            "StrProperty" => Ok(StrProperty::read(cursor, include_header)?.into()),
            "TextProperty" => Ok(TextProperty::read(cursor, include_header, options)?.into()),
            "NameProperty" => Ok(NameProperty::read(cursor, include_header)?.into()),
            "ObjectProperty" => Ok(ObjectProperty::read(cursor, include_header)?.into()),
            "DelegateProperty" => Ok(DelegateProperty::read(cursor, include_header)?.into()),
            "MulticastInlineDelegateProperty" => {
                Ok(MulticastInlineDelegateProperty::read(cursor, include_header)?.into())
            }
            "MulticastSparseDelegateProperty" => {
                Ok(MulticastSparseDelegateProperty::read(cursor, include_header)?.into())
            }
            "FieldPathProperty" => Ok(FieldPathProperty::read(cursor, include_header)?.into()),
            "StructProperty" => Ok(StructProperty::read(cursor, include_header, options)?.into()),
            "ArrayProperty" => Ok(ArrayProperty::read(cursor, include_header, options)?.into()),
            "SetProperty" => Ok(SetProperty::read(cursor, include_header, options)?.into()),
            "MapProperty" => Ok(MapProperty::read(cursor, include_header, options)?.into()),
            _ => {
                if include_header {
                    return Ok(
                        UnknownProperty::read_with_header(cursor, value_type.to_string())?.into(),
                    );
                }

                if let Some(suggested_length) = suggested_length {
                    return Ok(UnknownProperty::read_with_length(
                        cursor,
                        value_type.to_string(),
                        suggested_length,
                    )?
                    .into());
                }

                Err(DeserializeError::invalid_property(value_type, cursor))?
            }
        }
    }

    make_matcher!(ArrayProperty, get_array, get_array_mut);
    make_matcher!(EnumProperty, get_enum, get_enum_mut);
    make_matcher!(BoolProperty, get_bool, get_bool_mut);
    make_matcher!(ByteProperty, get_byte, get_byte_mut);
    make_matcher!(DoubleProperty, get_f64, get_f64_mut);
    make_matcher!(FloatProperty, get_f32, get_f32_mut);
    make_matcher!(Int16Property, get_i16, get_i16_mut);
    make_matcher!(Int64Property, get_i64, get_i64_mut);
    make_matcher!(Int8Property, get_i8, get_i8_mut);
    make_matcher!(IntProperty, get_int, get_int_mut);
    make_matcher!(UInt16Property, get_u16, get_u16_mut);
    make_matcher!(UInt32Property, get_u32, get_u32_mut);
    make_matcher!(UInt64Property, get_u64, get_u64_mut);
    make_matcher!(MapProperty, get_map, get_map_mut);
    make_matcher!(NameProperty, get_name, get_name_mut);
    make_matcher!(ObjectProperty, get_object_ref, get_object_ref_mut);
    make_matcher!(DelegateProperty, get_delegate, get_delegate_mut);
    make_matcher!(
        MulticastInlineDelegateProperty,
        get_multicast_inline_delegate,
        get_multicast_inline_delegate_mut
    );
    make_matcher!(
        MulticastSparseDelegateProperty,
        get_multicast_sparse_delegate,
        get_multicast_sparse_delegate_mut
    );
    make_matcher!(FieldPathProperty, get_field_path, get_field_path_mut);
    make_matcher!(SetProperty, get_set, get_set_mut);
    make_matcher!(StrProperty, get_str, get_str_mut);
    make_matcher!(StructProperty, get_struct, get_struct_mut);
    make_matcher!(TextProperty, get_text, get_text_mut);
    make_matcher!(UnknownProperty, get_unknown, get_unknown_mut);
}
