#![warn(missing_docs)]

//! Gvas
//!
//! UE4 Save File parsing library
//!
//! # Examples
//!
//! ```no_run
//! use gvas::{error::Error, GvasFile};
//! use std::{
//!     fs::File,
//!     io::Read,
//!     path::Path,
//! };
//!
//! let mut file = File::open("save.sav")?;
//! let gvas_file = GvasFile::read(&mut file);
//!
//! println!("{:#?}", gvas_file);
//! # Ok::<(), Error>(())
//! ```
//!
//! ## Hints
//!
//! If your file fails while parsing with a [`DeserializeError::MissingHint`] error you need hints.
//! When a struct is stored inside ArrayProperty/SetProperty/MapProperty in GvasFile it does not contain type annotations.
//! This means that a library parsing the file must know the type beforehand. That's why you need hints.
//!
//! The error usually looks like this:
//! ```no_run,ignore
//! MissingHint(
//!         "StructProperty" /* property type */,
//!         "UnLockedMissionParameters.MapProperty.Key.StructProperty" /* property path */,
//!         120550 /* position */)
//! ```
//! To get a hint type you need to look at the position of [`DeserializeError::MissingHint`] error.
//! Then you go to that position in the file and try to determine which type the struct has.
//! Afterwards you parse the file like this:
//!
//!
//!  [`DeserializeError::MissingHint`]: error/enum.DeserializeError.html#variant.MissingHint
//!
//! ```no_run
//! use gvas::{error::Error, GvasFile};
//! use std::{
//!     collections::HashMap,
//!     fs::File,
//!     io::Read,
//!     path::Path,
//! };
//!
//! let mut file = File::open("save.sav")?;
//!
//! let mut hints = HashMap::new();
//! hints.insert("UnLockedMissionParameters.MapProperty.Key.StructProperty".to_string(), "Guid".to_string());
//!
//! let gvas_file = GvasFile::read_with_hints(&mut file, &hints);
//!
//! println!("{:#?}", gvas_file);
//! # Ok::<(), Error>(())
//! ```

/// Extensions for `Cursor`.
pub mod cursor_ext;
/// Error types.
pub mod error;
/// Property types.
pub mod properties;
pub(crate) mod scoped_stack_entry;
/// Various types.
pub mod types;

use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    io::{Read, Seek, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use cursor_ext::{ReadExt, WriteExt};
use error::Error;
use indexmap::IndexMap;
use properties::{Property, PropertyTrait};
use types::Guid;

use crate::error::DeserializeError;

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
}

/// Stores CustomVersions serialized by UE4
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FCustomVersion {
    /// Key
    pub key: Guid,
    /// Value
    pub version: i32,
}

impl FCustomVersion {
    /// Creates a new instance of `FCustomVersion`
    pub fn new(key: Guid, version: i32) -> Self {
        FCustomVersion { key, version }
    }

    /// Read FCustomVersion from a binary file
    pub(crate) fn read<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let mut guid = [0u8; 16];
        cursor.read_exact(&mut guid)?;
        let version = cursor.read_i32::<LittleEndian>()?;

        Ok(FCustomVersion {
            key: Guid::new(guid),
            version,
        })
    }

    /// Write FCustomVersion to a binary file
    pub(crate) fn write<W: Write>(&self, cursor: &mut W) -> Result<(), Error> {
        let _ = cursor.write(&self.key.0)?;
        cursor.write_i32::<LittleEndian>(self.version)?;
        Ok(())
    }
}

/// The four bytes 'GVAS' appear at the beginning of every GVAS file.
pub const FILE_TYPE_GVAS: i32 = i32::from_le_bytes([b'G', b'V', b'A', b'S']);

/// Stores information about GVAS file, engine version, etc.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GvasHeader {
    /// The literal 'GVAS'.
    pub file_type_tag: i32,
    /// Save game file version.
    pub save_game_file_version: i32,
    /// File format version.
    pub package_file_ue4_version: i32,
    /// Unreal Engine version.
    pub engine_version: FEngineVersion,
    /// Custom version format.
    pub custom_version_format: i32,
    /// Custom versions.
    pub custom_versions: Vec<FCustomVersion>,
    /// Save game class name.
    pub save_game_class_name: String,
}

impl GvasHeader {
    /// Creates a new instance of `GvasHeader`
    pub fn new(
        file_type_tag: i32,
        save_game_file_version: i32,
        package_file_ue4_version: i32,
        engine_version: FEngineVersion,
        custom_version_format: i32,
        custom_versions: Vec<FCustomVersion>,
        save_game_class_name: String,
    ) -> Self {
        GvasHeader {
            file_type_tag,
            save_game_file_version,
            package_file_ue4_version,
            engine_version,
            custom_version_format,
            custom_versions,
            save_game_class_name,
        }
    }

    /// Read GvasHeader from a binary file
    ///
    /// # Errors
    ///
    /// If this function reads an invalid header it returns [`Error`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gvas::{error::Error, GvasHeader};
    /// use std::{
    ///     fs::File,
    ///     io::Read,
    ///     path::Path,
    /// };
    ///
    /// let mut file = File::open("save.sav")?;
    ///
    /// let gvas_header = GvasHeader::read(&mut file)?;
    ///
    /// println!("{:#?}", gvas_header);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn read<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let file_type_tag = cursor.read_i32::<LittleEndian>()?;
        if file_type_tag != FILE_TYPE_GVAS {
            Err(DeserializeError::InvalidFileType(file_type_tag))?
        }
        let save_game_file_version = cursor.read_i32::<LittleEndian>()?;
        let package_file_ue4_version = cursor.read_i32::<LittleEndian>()?;
        let engine_version = FEngineVersion::read(cursor)?;
        let custom_version_format = cursor.read_i32::<LittleEndian>()?;

        let custom_versions_len = cursor.read_i32::<LittleEndian>()? as usize;
        let mut custom_versions = Vec::with_capacity(custom_versions_len);
        for _ in 0..custom_versions_len {
            custom_versions.push(FCustomVersion::read(cursor)?);
        }

        let save_game_class_name = cursor.read_string()?;

        Ok(GvasHeader {
            file_type_tag,
            save_game_file_version,
            package_file_ue4_version,
            engine_version,
            custom_version_format,
            custom_versions,
            save_game_class_name,
        })
    }

    /// Write GvasHeader to a binary file
    ///
    /// # Examples
    /// ```no_run
    /// use gvas::{error::Error, GvasHeader};
    /// use std::{
    ///     fs::File,
    ///     io::{Cursor, Read},
    ///     path::Path,
    /// };
    ///
    /// let mut file = File::open("save.sav")?;
    /// let gvas_header = GvasHeader::read(&mut file)?;
    ///
    /// let mut writer = Cursor::new(Vec::new());
    /// gvas_header.write(&mut writer)?;
    /// println!("{:#?}", writer.get_ref());
    /// # Ok::<(), Error>(())
    /// ```
    pub fn write<W: Write>(&self, cursor: &mut W) -> Result<(), Error> {
        cursor.write_i32::<LittleEndian>(self.file_type_tag)?;
        cursor.write_i32::<LittleEndian>(self.save_game_file_version)?;
        cursor.write_i32::<LittleEndian>(self.package_file_ue4_version)?;
        self.engine_version.write(cursor)?;
        cursor.write_i32::<LittleEndian>(self.custom_version_format)?;
        cursor.write_i32::<LittleEndian>(self.custom_versions.len() as i32)?;

        for custom_version in &self.custom_versions {
            custom_version.write(cursor)?;
        }

        cursor.write_string(&self.save_game_class_name)?;
        Ok(())
    }
}

/// Main UE4 save file struct
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GvasFile {
    /// GVAS file header.
    pub header: GvasHeader,
    /// GVAS properties.
    #[cfg_attr(feature = "serde", serde(with = "indexmap::serde_seq"))]
    pub properties: IndexMap<String, Property>,
}

impl GvasFile {
    /// Read GvasFile from a binary file
    ///
    /// # Errors
    ///
    /// If this function reads an invalid file it returns [`Error`]
    ///
    /// If this function reads a file which needs hints it returns [`DeserializeError::MissingHint`]
    ///
    /// [`DeserializeError::MissingHint`]: error/enum.DeserializeError.html#variant.MissingHint
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gvas::{error::Error, GvasFile};
    /// use std::{
    ///     fs::File,
    ///     io::Read,
    ///     path::Path,
    /// };
    ///
    /// let mut file = File::open("save.sav")?;
    /// let gvas_file = GvasFile::read(&mut file);
    ///
    /// println!("{:#?}", gvas_file);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn read<R: Read + Seek>(cursor: &mut R) -> Result<Self, Error> {
        let hints = HashMap::new();
        Self::read_with_hints(cursor, &hints)
    }

    /// Read GvasFile from a binary file
    ///
    /// # Errors
    ///
    /// If this function reads an invalid file it returns [`Error`]
    ///
    /// If this function reads a file which needs a hint that is missing it returns [`DeserializeError::MissingHint`]
    ///
    /// [`DeserializeError::MissingHint`]: error/enum.DeserializeError.html#variant.MissingHint
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gvas::{error::Error, GvasFile};
    /// use std::{
    ///     collections::HashMap,
    ///     fs::File,
    ///     io::Read,
    ///     path::Path,
    /// };
    ///
    /// let mut file = File::open("save.sav")?;
    ///
    /// let mut hints = HashMap::new();
    /// hints.insert(
    ///     "SeasonSave.StructProperty.Seasons.MapProperty.Key.StructProperty".to_string(),
    ///     "Guid".to_string(),
    /// );
    ///
    /// let gvas_file = GvasFile::read_with_hints(&mut file, &hints);
    ///
    /// println!("{:#?}", gvas_file);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn read_with_hints<R: Read + Seek>(
        cursor: &mut R,
        hints: &HashMap<String, String>,
    ) -> Result<Self, Error> {
        let header = GvasHeader::read(cursor)?;

        let mut properties = IndexMap::new();
        loop {
            let property_name = cursor.read_string()?;
            if property_name == "None" {
                break;
            }

            let property_type = cursor.read_string()?;

            let mut properties_stack = Vec::new();
            properties_stack.push(property_name.clone());

            let property = Property::new(
                cursor,
                hints,
                &mut properties_stack,
                &property_type,
                true,
                None,
            )?;
            properties.insert(property_name, property);
        }

        Ok(GvasFile { header, properties })
    }

    /// Write GvasFile to a binary file
    ///
    /// # Errors
    ///
    /// If the file was modified in a way that makes it invalid this function returns [`Error`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use gvas::{error::Error, GvasFile};
    /// use std::{
    ///     fs::File,
    ///     io::{Cursor, Read},
    ///     path::Path,
    /// };
    ///
    /// let mut file = File::open("save.sav")?;
    /// let gvas_file = GvasFile::read(&mut file)?;
    ///
    /// let mut writer = Cursor::new(Vec::new());
    /// gvas_file.write(&mut writer)?;
    /// println!("{:#?}", writer.get_ref());
    /// # Ok::<(), Error>(())
    /// ```
    pub fn write<W: Write + Seek>(&self, cursor: &mut W) -> Result<(), Error> {
        self.header.write(cursor)?;

        for (name, property) in &self.properties {
            cursor.write_string(name)?;
            property.write(cursor, true)?;
        }
        cursor.write_string("None")?;
        cursor.write_i32::<LittleEndian>(0)?; // padding
        Ok(())
    }
}
