use std::{
    io,
    string::{FromUtf8Error, FromUtf16Error},
};

use thiserror::Error;

/// Gets thrown when there is a deserialization error
#[derive(Error, Debug)]
pub enum DeserializeError {
    /// If the GVAS header is not valid
    #[error("Invalid header: {0}")]
    InvalidHeader(Box<str>),
    /// If a value has a size that was unexpected, e.g. UInt32Property has 8 bytes size
    #[error("Invalid value size, expected {0} got {1} at position {2:#x}")]
    InvalidValueSize(u64, u64, u64),
    /// If a string has invalid size
    #[error("Invalid string size {0} at position {1:#x}")]
    InvalidString(i32, u64),
    /// Invalid string terminator
    #[error("Invalid string terminator {0} at position {1:#x}")]
    InvalidStringTerminator(u16, u64),
    /// If a boolean has invalid value
    #[error("Invalid boolean value {0} at position {1:#x}")]
    InvalidBoolean(u32, u64),
    /// If a hint is missing.
    #[error("Missing hint for struct {0} at path {1} at position {2:#x}")]
    MissingHint(Box<str>, Box<str>, u64),
    /// If an argument is missing
    #[error("Missing argument: {0} at position {1:#x}")]
    MissingArgument(Box<str>, u64),
    /// If a Property creation fails
    #[error("Invalid property {0} at position {1:#x}")]
    InvalidProperty(Box<str>, u64),
    /// Invalid enum value
    #[error("No discriminant in enum `{0}` matches the value `{1}` at position {2:#x}")]
    InvalidEnumValue(Box<str>, i8, u64),
    /// Invalid array index header
    #[error("Unexpected array_index value {0} at position {1:#x}")]
    InvalidArrayIndex(u32, u64),
    /// Invalid terminator
    #[error("Unexpected terminator value {0} at position {1:#x}")]
    InvalidTerminator(u8, u64),
    /// If a string has invalid UTF-16 formatting
    #[error("Invalid UTF-16 string at position {1:#x}")]
    FromUtf16Error(#[source] FromUtf16Error, u64),
    /// If a string has invalid UTF-8 formatting
    #[error("Invalid UTF-8 string at position {1:#x}")]
    FromUtf8Error(#[source] FromUtf8Error, u64),
}

impl DeserializeError {
    /// A helper for creating `MissingArgument` errors
    #[inline]
    pub fn missing_argument<A, S>(argument_name: A, stream: &mut S) -> Self
    where
        A: Into<Box<str>>,
        S: io::Seek,
    {
        let position = stream.stream_position().unwrap_or_default();
        Self::MissingArgument(argument_name.into(), position)
    }

    /// A helper for creating `InvalidProperty` errors
    #[inline]
    pub fn invalid_property<R, S>(reason: R, stream: &mut S) -> Self
    where
        R: Into<Box<str>>,
        S: io::Seek,
    {
        let position = stream.stream_position().unwrap_or_default();
        Self::InvalidProperty(reason.into(), position)
    }

    /// A helper for creating `InvalidEnumValue` errors
    #[inline]
    pub fn invalid_enum_value<N, S>(name: N, value: i8, stream: &mut S) -> Self
    where
        N: Into<Box<str>>,
        S: io::Seek,
    {
        let position = stream.stream_position().unwrap_or_default();
        Self::InvalidEnumValue(name.into(), value, position)
    }
}

/// Gets thrown when there is a serialization error
#[derive(Error, Debug)]
pub enum SerializeError {
    /// A value was invalid
    #[error("Invalid value {0}")]
    InvalidValue(Box<str>),
    /// Struct is missing a field, e.g. struct with type_name `Vector` doesn't have an `X` property
    #[error("Struct {0} missing field {1}")]
    StructMissingField(Box<str>, Box<str>),
}

impl SerializeError {
    /// A helper for creating `InvalidValue` errors
    pub fn invalid_value<M>(msg: M) -> Self
    where
        M: Into<Box<str>>,
    {
        Self::InvalidValue(msg.into())
    }

    /// A helper for creating `StructMissingField` errors
    pub fn struct_missing_field<T, M>(type_name: T, missing_field: M) -> Self
    where
        T: Into<Box<str>>,
        M: Into<Box<str>>,
    {
        Self::StructMissingField(type_name.into(), missing_field.into())
    }
}

/// A wrapper for the various error types this crate can emit
#[derive(Error, Debug)]
pub enum Error {
    /// A `DeserializeError` occurred
    #[error(transparent)]
    Deserialize(#[from] DeserializeError),
    /// A `SerializeError` occurred
    #[error(transparent)]
    Serialize(#[from] SerializeError),
    /// An `std::io::Error` occured
    #[error(transparent)]
    Io(#[from] io::Error),
}
