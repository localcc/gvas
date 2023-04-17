use std::io;

use thiserror::Error;
use unreal_helpers::error::FStringError;

/// Gets thrown when there is a deserialization error
#[derive(Error, Debug)]
pub enum DeserializeError {
    /// If the file header is not GVAS
    #[error("Invalid file type {0}")]
    InvalidFileType(i32),
    /// If a value has a size that was unexpected, e.g. UInt32Property has 8 bytes size
    #[error("Invalid value size, expected {0} got {1} at position {2}")]
    InvalidValueSize(u64, u64, u64),
    /// If a string has invalid size
    #[error("Invalid string size, got {0} at position {1}")]
    InvalidString(i32, u64),
    /// If a hint is missing.
    #[error("Missing hint for struct {0} at path {1}, cursor position: {2}")]
    MissingHint(String, String, u64),
    /// If an argument is missing
    #[error("Missing argument: {0} at position {1}")]
    MissingArgument(String, u64),
    /// If an EnumProperty has an invalid enum type
    #[error("Invalid enum type {0} at position {1}")]
    InvalidEnumType(String, u64),
    /// If a Property creation fails
    #[error("Invalid property {0} at position {1}")]
    InvalidProperty(String, u64),
}

impl DeserializeError {
    /// A helper for creating `MissingArgument` errors
    pub fn missing_argument<S: io::Seek>(argument_name: &str, stream: &mut S) -> Self {
        let position = stream.stream_position().unwrap_or_default();
        Self::MissingArgument(argument_name.to_string(), position)
    }

    /// A helper for creating `InvalidProperty` errors
    pub fn invalid_property<S: io::Seek>(reason: &str, stream: &mut S) -> Self {
        let position = stream.stream_position().unwrap_or_default();
        Self::InvalidProperty(reason.to_string(), position)
    }
}

/// Gets thrown when there is a serialization error
#[derive(Error, Debug)]
pub enum SerializeError {
    /// A value was invalid
    #[error("Invalid value {0}")]
    InvalidValue(String),
    /// Struct is missing a field, e.g. struct with type_name `Vector` doesn't have an `X` property
    #[error("Struct {0} missing field {1}")]
    StructMissingField(String, String),
}

impl SerializeError {
    /// A helper for creating `InvalidValue` errors
    pub fn invalid_value(msg: &str) -> Self {
        Self::InvalidValue(msg.to_string())
    }

    /// A helper for creating `StructMissingField` errors
    pub fn struct_missing_field(type_name: &str, missing_field: &str) -> Self {
        Self::StructMissingField(type_name.to_string(), missing_field.to_string())
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
    /// An `FStringError` occured
    #[error(transparent)]
    FString(#[from] FStringError),
    /// An `std::io::Error` occured
    #[error(transparent)]
    Io(#[from] io::Error),
}
