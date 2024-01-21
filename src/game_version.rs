//! Game version enumeration

use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Game version enumeration
///
/// Used for specifying game versions if a game has custom serialization
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameVersion {
    /// Default GVAS serialization
    Default,
    /// Palworld serialization
    Palworld,
}

/// Palworld compression type
#[derive(Debug, Copy, Clone, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum PalworldCompressionType {
    /// None
    None = 0x30,
    /// Zlib
    Zlib = 0x31,
    /// Zlib twice
    ZlibTwice = 0x32,
}

/// Deserialized game version
///
/// Used for storing additional deserialized information about custom serialization
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeserializedGameVersion {
    /// Default GVAS serialization
    Default,
    /// Palworld serialization
    Palworld(PalworldCompressionType),
}

/// Palworld save magic
pub(crate) const PLZ_MAGIC: &[u8; 3] = b"PlZ";
