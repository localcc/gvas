use std::{
    fmt::Display,
    io,
    string::{FromUtf16Error, FromUtf8Error},
};

/// Gets thrown when there is a deserialization error
#[derive(Debug)]
pub enum DeserializeError {
    /// If a value has a size that was unexpected, e.g. UInt32Property has 8 bytes size
    InvalidValueSize(u64, u64, u64),
    /// If a string has invalid size
    InvalidString(i32, u64),
    /// If a null terminator is missing
    InvalidStringTermination(u16, u64),
    /// If a hint is missing.
    MissingHint(String, String, u64),
    /// If an argument is missing
    MissingArgument(String, u64),
    /// If an EnumProperty has an invalid enum type
    InvalidEnumType(String, u64),
    /// If a Property creation fails
    InvalidProperty(String, u64),
}

impl DeserializeError {
    pub fn missing_argument(argument_name: &str, position: u64) -> Self {
        Self::MissingArgument(argument_name.to_string(), position)
    }
    pub fn invalid_property(reason: &str, position: u64) -> Self {
        Self::InvalidProperty(reason.to_string(), position)
    }
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializeError::InvalidValueSize(ref expected, ref got, ref position) => {
                write!(
                    f,
                    "Invalid value size, expected {} got {} at position {}",
                    expected, got, position
                )
            }
            DeserializeError::InvalidString(ref got, ref position) => {
                write!(
                    f,
                    "Invalid string size, got {} at position {}",
                    got, position
                )
            }
            DeserializeError::MissingHint(ref struct_name, ref struct_path, ref position) => {
                write!(
                    f,
                    "Missing hint for struct {} at path {}, cursor position: {}",
                    struct_name, struct_path, position
                )
            }
            DeserializeError::InvalidStringTermination(ref char, ref position) => {
                write!(
                    f,
                    "Invalid string termination {} at position {}",
                    char, position
                )
            }
            DeserializeError::MissingArgument(ref argument_name, ref position) => {
                write!(
                    f,
                    "Missing argument: {} at position {}",
                    argument_name, position
                )
            }
            DeserializeError::InvalidEnumType(ref enum_type, ref position) => {
                write!(
                    f,
                    "Invalid enum type {} at position {}",
                    enum_type, position
                )
            }
            DeserializeError::InvalidProperty(ref reason, ref position) => {
                write!(f, "Invalid property {} at position {}", reason, position)
            }
        }
    }
}

/// Gets thrown when there is a serialization error
#[derive(Debug)]
pub enum SerializeError {
    /// A value was invalid
    InvalidValue(String),
    /// Struct is missing a field, e.g. struct with type_name `Vector` doesn't have an `X` property
    StructMissingField(String, String),
}

impl SerializeError {
    pub fn invalid_value(msg: &str) -> Self {
        Self::InvalidValue(msg.to_string())
    }

    pub fn struct_missing_field(type_name: &str, missing_field: &str) -> Self {
        Self::StructMissingField(type_name.to_string(), missing_field.to_string())
    }
}

impl Display for SerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializeError::InvalidValue(ref msg) => {
                write!(f, "Invaid value {}", msg)
            }
            SerializeError::StructMissingField(ref type_name, ref missing_field) => {
                write!(f, "Struct {} missing field {}", type_name, missing_field)
            }
        }
    }
}

#[derive(Debug)]
pub enum ErrorCode {
    Deserialize(DeserializeError),
    Serialize(SerializeError),
    Io(io::Error),
    Utf8(FromUtf8Error),
    Utf16(FromUtf16Error),
    None,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::Deserialize(ref e) => Display::fmt(e, f),
            ErrorCode::Serialize(ref e) => Display::fmt(e, f),
            ErrorCode::Io(ref e) => Display::fmt(e, f),
            ErrorCode::Utf8(ref e) => Display::fmt(e, f),
            ErrorCode::Utf16(ref e) => Display::fmt(e, f),
            ErrorCode::None => write!(f, "unk"),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    code: ErrorCode,
}

impl Error {
    pub fn empty() -> Self {
        Error {
            code: ErrorCode::None,
        }
    }
}

impl From<DeserializeError> for Error {
    fn from(e: DeserializeError) -> Self {
        Error {
            code: ErrorCode::Deserialize(e),
        }
    }
}

impl From<SerializeError> for Error {
    fn from(e: SerializeError) -> Self {
        Error {
            code: ErrorCode::Serialize(e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error {
            code: ErrorCode::Io(e),
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error {
            code: ErrorCode::Utf8(e),
        }
    }
}

impl From<FromUtf16Error> for Error {
    fn from(e: FromUtf16Error) -> Self {
        Error {
            code: ErrorCode::Utf16(e),
        }
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.code, f)
    }
}
