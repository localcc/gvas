use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    io::{Read, Seek, Write},
};

use enum_dispatch::enum_dispatch;

use crate::{
    error::{DeserializeError, Error},
    scoped_stack_entry::ScopedStackEntry,
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

/// The default read() implementation
macro_rules! impl_read {
    () => {
        #[inline]
        pub(crate) fn read<R: Read + Seek>(
            cursor: &mut R,
            include_header: bool,
        ) -> Result<Self, Error> {
            if include_header {
                Self::read_header(cursor)
            } else {
                Self::read_body(cursor)
            }
        }
    };

    (options) => {
        #[inline]
        pub(crate) fn read<R: Read + Seek>(
            cursor: &mut R,
            options: &mut PropertyOptions,
            include_header: bool,
        ) -> Result<Self, Error> {
            if include_header {
                Self::read_header(cursor, options)
            } else {
                Self::read_body(cursor, options)
            }
        }
    };
}

/// The default read_header() implementation
macro_rules! impl_read_header {
    (options, $($var:ident, )*) => {
        #[inline]
        fn read_header<R: Read + Seek>(
            cursor: &mut R,
            options: &mut PropertyOptions,
        ) -> Result<Self, Error> {
            let length = cursor.read_u64::<LittleEndian>()?;
            $(
                let $var = cursor.read_string()?;
            )*
            let separator = cursor.read_u8()?;
            assert_eq!(separator, 0);

            let start = cursor.stream_position()?;
            let result = Self::read_body(cursor, options $(, $var)*)?;
            let end = cursor.stream_position()?;
            assert_eq!(end - start, length);

            Ok(result)
        }
    };

    ($($var:ident $(,)? )*) => {
        #[inline]
        fn read_header<R: Read + Seek>(
            cursor: &mut R,
        ) -> Result<Self, Error> {
            let length = cursor.read_u64::<LittleEndian>()?;
            $(
                let $var = cursor.read_string()?;
            )*
            let separator = cursor.read_u8()?;
            assert_eq!(separator, 0);

            let start = cursor.stream_position()?;
            let result = Self::read_body(cursor $(, $var)*)?;
            let end = cursor.stream_position()?;
            assert_eq!(end - start, length);

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
    ($property:ident, options $(, $header_property:tt)*) => {
        impl PropertyTrait for $property {
            #[inline]
            fn write<W: Write>(
                &self,
                writer: &mut W,
                include_header: bool,
                options: &mut PropertyOptions,
            ) -> Result<(), Error> {
                if !include_header {
                    return self.write_body(writer, options);
                }

                let buf = &mut Cursor::new(Vec::new());
                self.write_body(buf, options)?;
                let buf = buf.get_ref();

                writer.write_string(stringify!($property))?;
                writer.write_u64::<LittleEndian>(buf.len() as u64)?;
                $(
                    impl_write_header_part!(self, writer, $header_property);
                )*
                writer.write_u8(0)?;
                writer.write_all(buf)?;

                Ok(())
            }
        }
    };

    ($property:ident $(, $header_property:tt)*) => {
        impl PropertyTrait for $property {
            #[inline]
            fn write<W: Write>(
                &self,
                writer: &mut W,
                include_header: bool,
                _options: &mut PropertyOptions,
            ) -> Result<(), Error> {
                if !include_header {
                    return self.write_body(writer);
                }

                let buf = &mut Cursor::new(Vec::new());
                self.write_body(buf)?;
                let buf = buf.get_ref();

                writer.write_string(stringify!($property))?;
                writer.write_u64::<LittleEndian>(buf.len() as u64)?;
                $(
                    impl_write_header_part!(self, writer, $header_property);
                )*
                writer.write_u8(0)?;
                writer.write_all(buf)?;

                Ok(())
            }
        }
    };
}

/// A helper macro for writing property header parts.
///
/// This macro is used inside the `impl_write!` macro to write individual parts of a property header.
macro_rules! impl_write_header_part {
    ($self:ident, $writer:ident, ($write_fn:ident, $member:ident)) => {
        $writer.$write_fn(&$self.$member)?;
    };

    ($self:ident, $writer:ident, ($write_fn:ident, fn, $member:ident)) => {
        $writer.$write_fn(&$self.$member()?)?;
    };
}

pub(crate) use impl_write;
pub(crate) use impl_write_header_part;

/// Creates a match helper function for enums.
#[macro_export]
macro_rules! make_matcher {
    ($type:ident, $name:ident) => {
        #[doc = "Retrieves the enum value as a `"]
        #[doc = stringify!($type)]
        #[doc = "`."]
        #[inline]
        pub fn $name(&self) -> Option<&$type> {
            match self {
                Self::$type(e) => Some(e),
                _ => None,
            }
        }
    };
}

/// Property options used for reading and writing.
pub struct PropertyOptions<'a> {
    /// Hints about property types.
    pub hints: &'a HashMap<String, String>,
    /// Tracks the property tree location in a GVAS file.
    pub properties_stack: &'a mut Vec<String>,
    /// Enables large world coordinates.
    pub large_world_coordinates: bool,
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
    ) -> Result<(), Error>;
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
        suggested_length: Option<u64>,
    ) -> Result<Self, Error> {
        let _stack_entry = ScopedStackEntry::new(options.properties_stack, value_type.to_string());
        match value_type {
            "Int8Property" => Ok(Int8Property::read(cursor, include_header)?.into()),
            "ByteProperty" => Ok(ByteProperty::read(cursor, include_header)?.into()),
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
            "ArrayProperty" => Ok(ArrayProperty::read(cursor, options)?.into()),
            "SetProperty" => Ok(SetProperty::read(cursor, options)?.into()),
            "MapProperty" => Ok(MapProperty::read(cursor, options, include_header)?.into()),
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

    make_matcher!(ArrayProperty, get_array);
    make_matcher!(EnumProperty, get_enum);
    make_matcher!(BoolProperty, get_bool);
    make_matcher!(ByteProperty, get_byte);
    make_matcher!(DoubleProperty, get_f64);
    make_matcher!(FloatProperty, get_f32);
    make_matcher!(Int16Property, get_i16);
    make_matcher!(Int64Property, get_i64);
    make_matcher!(Int8Property, get_i8);
    make_matcher!(IntProperty, get_int);
    make_matcher!(UInt16Property, get_u16);
    make_matcher!(UInt32Property, get_u32);
    make_matcher!(UInt64Property, get_u64);
    make_matcher!(MapProperty, get_map);
    make_matcher!(NameProperty, get_name);
    make_matcher!(ObjectProperty, get_object_ref);
    make_matcher!(DelegateProperty, get_delegate);
    make_matcher!(
        MulticastInlineDelegateProperty,
        get_multicast_inline_delegate
    );
    make_matcher!(
        MulticastSparseDelegateProperty,
        get_multicast_sparse_delegate
    );
    make_matcher!(FieldPathProperty, get_field_path);
    make_matcher!(SetProperty, get_set);
    make_matcher!(StrProperty, get_str);
    make_matcher!(StructProperty, get_struct);
    make_matcher!(TextProperty, get_text);
    make_matcher!(UnknownProperty, get_unknown);
}
