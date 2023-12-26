//! Engine version information

use crate::cursor_ext::{ReadExt, WriteExt};
use crate::error::Error;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::fmt::Display;
use std::io::{Read, Seek, Write};

/// Stores UE4 version in which the GVAS file was saved
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FEngineVersion {
    /// Major version number.
    pub major: u16,
    /// Minor version number.
    pub minor: u16,
    /// Patch version number.
    pub patch: u16,
    /// Build id.
    pub change_list: u32,
    /// Build id string.
    pub branch: String,
}

impl Display for FEngineVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}-{}+++{}",
            self.major, self.minor, self.patch, self.change_list, self.branch
        )
    }
}

impl FEngineVersion {
    /// Creates a new instance of `FEngineVersion`
    #[inline]
    pub fn new(major: u16, minor: u16, patch: u16, change_list: u32, branch: String) -> Self {
        FEngineVersion {
            major,
            minor,
            patch,
            change_list,
            branch,
        }
    }

    /// Read FEngineVersion from a binary file
    pub(crate) fn read<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let major = cursor.read_u16::<LittleEndian>()?;
        let minor = cursor.read_u16::<LittleEndian>()?;
        let patch = cursor.read_u16::<LittleEndian>()?;
        let change_list = cursor.read_u32::<LittleEndian>()?;
        let branch = cursor.read_string()?;
        Ok(FEngineVersion {
            major,
            minor,
            patch,
            change_list,
            branch,
        })
    }

    /// Write FEngineVersion to a binary file
    pub(crate) fn write<W: Write>(&self, cursor: &mut W) -> Result<(), Error> {
        cursor.write_u16::<LittleEndian>(self.major)?;
        cursor.write_u16::<LittleEndian>(self.minor)?;
        cursor.write_u16::<LittleEndian>(self.patch)?;
        cursor.write_u32::<LittleEndian>(self.change_list)?;
        cursor.write_string(&self.branch)?;
        Ok(())
    }

    /// Get [`EngineVersion`]
    pub fn get_version(&self) -> EngineVersion {
        match (self.major, self.minor) {
            (4, 0) => EngineVersion::VER_UE4_0,
            (4, 1) => EngineVersion::VER_UE4_1,
            (4, 2) => EngineVersion::VER_UE4_2,
            (4, 3) => EngineVersion::VER_UE4_3,
            (4, 4) => EngineVersion::VER_UE4_4,
            (4, 5) => EngineVersion::VER_UE4_5,
            (4, 6) => EngineVersion::VER_UE4_6,
            (4, 7) => EngineVersion::VER_UE4_7,
            (4, 8) => EngineVersion::VER_UE4_8,
            (4, 9) => EngineVersion::VER_UE4_9,
            (4, 10) => EngineVersion::VER_UE4_10,
            (4, 11) => EngineVersion::VER_UE4_11,
            (4, 12) => EngineVersion::VER_UE4_12,
            (4, 13) => EngineVersion::VER_UE4_13,
            (4, 14) => EngineVersion::VER_UE4_14,
            (4, 15) => EngineVersion::VER_UE4_15,
            (4, 16) => EngineVersion::VER_UE4_16,
            (4, 17) => EngineVersion::VER_UE4_17,
            (4, 18) => EngineVersion::VER_UE4_18,
            (4, 19) => EngineVersion::VER_UE4_19,
            (4, 20) => EngineVersion::VER_UE4_20,
            (4, 21) => EngineVersion::VER_UE4_21,
            (4, 22) => EngineVersion::VER_UE4_22,
            (4, 23) => EngineVersion::VER_UE4_23,
            (4, 24) => EngineVersion::VER_UE4_24,
            (4, 25) => EngineVersion::VER_UE4_25,
            (4, 26) => EngineVersion::VER_UE4_26,
            (4, 27) => EngineVersion::VER_UE4_27,
            (5, 0) => EngineVersion::VER_UE5_0,
            (5, 1) => EngineVersion::VER_UE5_1,
            (5, 2) => EngineVersion::VER_UE5_2,
            _ => EngineVersion::UNKNOWN,
        }
    }
}

/// UE4 Engine version enum
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum EngineVersion {
    /// Unknown
    UNKNOWN,
    /// Oldest loadable package
    VER_UE4_OLDEST_LOADABLE_PACKAGE,

    /// 4.0
    VER_UE4_0,
    /// 4.1
    VER_UE4_1,
    /// 4.2
    VER_UE4_2,
    /// 4.3
    VER_UE4_3,
    /// 4.4
    VER_UE4_4,
    /// 4.5
    VER_UE4_5,
    /// 4.6
    VER_UE4_6,
    /// 4.7
    VER_UE4_7,
    /// 4.8
    VER_UE4_8,
    /// 4.9
    VER_UE4_9,
    /// 4.10
    VER_UE4_10,
    /// 4.11
    VER_UE4_11,
    /// 4.12
    VER_UE4_12,
    /// 4.13
    VER_UE4_13,
    /// 4.14
    VER_UE4_14,
    /// 4.15
    VER_UE4_15,
    /// 4.16
    VER_UE4_16,
    /// 4.17
    VER_UE4_17,
    /// 4.18
    VER_UE4_18,
    /// 4.19
    VER_UE4_19,
    /// 4.20
    VER_UE4_20,
    /// 4.21
    VER_UE4_21,
    /// 4.22
    VER_UE4_22,
    /// 4.23
    VER_UE4_23,
    /// 4.24
    VER_UE4_24,
    /// 4.25
    VER_UE4_25,
    /// 4.26
    VER_UE4_26,
    /// 4.27
    VER_UE4_27,

    /// 5.0
    VER_UE5_0,
    /// 5.1
    VER_UE5_1,
    /// 5.2
    VER_UE5_2,

    /// The newest specified version of the Unreal Engine.
    VER_UE4_AUTOMATIC_VERSION,
    /// Version plus one
    VER_UE4_AUTOMATIC_VERSION_PLUS_ONE,
}
