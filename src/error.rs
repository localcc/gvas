use std::{fmt::Display, io, string::FromUtf8Error};

#[derive(Debug)]
pub enum DeserializeError {
    InvalidValueSize(i64, i64),
    UnknownProperty(String)
}

impl Display for DeserializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeserializeError::InvalidValueSize(ref expected, ref got) => write!(f, "Invalid value size, expected {} got {}", expected, got),
            DeserializeError::UnknownProperty(ref name) => write!(f, "Unknown property {}", name),
            _ => write!(f, "unk")
        }
    }
}



#[derive(Debug)]
pub enum ErrorCode {
    Deserialize(DeserializeError),
    Io(io::Error),
    Utf8(FromUtf8Error),
    None
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::Deserialize(ref e) => Display::fmt(e, f),
            ErrorCode::Io(ref e) => Display::fmt(e, f),
            ErrorCode::Utf8(ref e) => Display::fmt(e, f),
            ErrorCode::None => write!(f, "unk"),
        }
    }
}


#[derive(Debug)]
pub struct Error {
    code: ErrorCode
}

impl Error {
    pub fn empty() -> Self {
        Error { code: ErrorCode::None }
    }
}

impl From<DeserializeError> for Error {
    fn from(e: DeserializeError) -> Self {
        Error { code: ErrorCode::Deserialize(e) }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error { code: ErrorCode::Io(e) }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error { code: ErrorCode::Utf8(e) }
    }
}

impl std::error::Error for Error {
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.code, f)
    }
}

