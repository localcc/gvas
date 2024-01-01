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
/// Custom version information.
pub mod custom_version;
/// Engine version information.
pub mod engine_version;
/// Error types.
pub mod error;
/// Extensions for `Ord`.
mod ord_ext;
/// Property types.
pub mod properties;
pub(crate) mod scoped_stack_entry;
/// Various types.
pub mod types;

use std::{
    collections::HashMap,
    fmt::Debug,
    io::{Read, Seek, Write},
};

use crate::error::DeserializeError;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use cursor_ext::{ReadExt, WriteExt};
use custom_version::FCustomVersion;
use engine_version::FEngineVersion;
use error::Error;
use indexmap::IndexMap;
use ord_ext::OrdExt;
use properties::{Property, PropertyOptions, PropertyTrait};

/// The four bytes 'GVAS' appear at the beginning of every GVAS file.
pub const FILE_TYPE_GVAS: u32 = u32::from_le_bytes([b'G', b'V', b'A', b'S']);

/// Stores information about GVAS file, engine version, etc.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum GvasHeader {
    /// Version 2
    Version2 {
        /// File format version.
        package_file_version: u32,
        /// Unreal Engine version.
        engine_version: FEngineVersion,
        /// Custom version format.
        custom_version_format: u32,
        /// Custom versions.
        custom_versions: Vec<FCustomVersion>,
        /// Save game class name.
        save_game_class_name: String,
    },
    /// Version 3
    Version3 {
        /// File format version.
        package_file_version: u32,
        /// Unknown.
        unknown: u32,
        /// Unreal Engine version.
        engine_version: FEngineVersion,
        /// Custom version format.
        custom_version_format: u32,
        /// Custom versions.
        custom_versions: Vec<FCustomVersion>,
        /// Save game class name.
        save_game_class_name: String,
    },
}

impl GvasHeader {
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
        let file_type_tag = cursor.read_u32::<LittleEndian>()?;
        if file_type_tag != FILE_TYPE_GVAS {
            Err(DeserializeError::InvalidHeader(format!(
                "File type {file_type_tag} not recognized",
            )))?
        }

        let save_game_file_version = cursor.read_u32::<LittleEndian>()?;
        if !save_game_file_version.between(2, 3) {
            Err(DeserializeError::InvalidHeader(format!(
                "GVAS version {save_game_file_version} not supported"
            )))?
        }

        let package_file_version = cursor.read_u32::<LittleEndian>()?;
        if !package_file_version.between(0x205, 0x20C) {
            Err(DeserializeError::InvalidHeader(format!(
                "Package file version {package_file_version} not supported"
            )))?
        }

        // This field is only present in the v3 header
        let unknown = match save_game_file_version {
            3 => Some(cursor.read_u32::<LittleEndian>()?),
            _ => None,
        };

        let engine_version = FEngineVersion::read(cursor)?;
        let custom_version_format = cursor.read_u32::<LittleEndian>()?;
        if custom_version_format != 3 {
            Err(DeserializeError::InvalidHeader(format!(
                "Custom version format {custom_version_format} not supported"
            )))?
        }

        let custom_versions_len = cursor.read_u32::<LittleEndian>()? as usize;
        let mut custom_versions = Vec::with_capacity(custom_versions_len);
        for _ in 0..custom_versions_len {
            custom_versions.push(FCustomVersion::read(cursor)?);
        }

        let save_game_class_name = cursor.read_string()?;

        Ok(match unknown {
            None => GvasHeader::Version2 {
                package_file_version,
                engine_version,
                custom_version_format,
                custom_versions,
                save_game_class_name,
            },
            Some(unknown) => GvasHeader::Version3 {
                package_file_version,
                unknown,
                engine_version,
                custom_version_format,
                custom_versions,
                save_game_class_name,
            },
        })
    }

    /// Write GvasHeader to a binary file
    ///
    /// # Examples
    /// ```no_run
    /// use gvas::{error::Error, GvasHeader};
    /// use std::{
    ///     fs::File,
    ///     io::Cursor,
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
        cursor.write_u32::<LittleEndian>(FILE_TYPE_GVAS)?;
        match self {
            GvasHeader::Version2 {
                package_file_version,
                engine_version,
                custom_version_format,
                custom_versions,
                save_game_class_name,
            } => {
                cursor.write_u32::<LittleEndian>(2)?;
                cursor.write_u32::<LittleEndian>(*package_file_version)?;
                engine_version.write(cursor)?;
                cursor.write_u32::<LittleEndian>(*custom_version_format)?;
                cursor.write_u32::<LittleEndian>(custom_versions.len() as u32)?;

                for custom_version in custom_versions {
                    custom_version.write(cursor)?;
                }

                cursor.write_string(save_game_class_name)?;
            }
            GvasHeader::Version3 {
                package_file_version,
                unknown,
                engine_version,
                custom_version_format,
                custom_versions,
                save_game_class_name,
            } => {
                cursor.write_u32::<LittleEndian>(3)?;
                cursor.write_u32::<LittleEndian>(*package_file_version)?;
                cursor.write_u32::<LittleEndian>(*unknown)?;
                engine_version.write(cursor)?;
                cursor.write_u32::<LittleEndian>(*custom_version_format)?;
                cursor.write_u32::<LittleEndian>(custom_versions.len() as u32)?;

                for custom_version in custom_versions {
                    custom_version.write(cursor)?;
                }

                cursor.write_string(save_game_class_name)?;
            }
        }
        Ok(())
    }

    /// Get custom versions from this header
    pub fn get_custom_versions(&self) -> &[FCustomVersion] {
        match self {
            GvasHeader::Version2 {
                custom_versions, ..
            } => custom_versions.as_slice(),
            GvasHeader::Version3 {
                custom_versions, ..
            } => custom_versions.as_slice(),
        }
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

trait GvasHeaderTrait {
    fn use_large_world_coordinates(&self) -> bool;
}

impl GvasHeaderTrait for GvasHeader {
    fn use_large_world_coordinates(&self) -> bool {
        match self {
            GvasHeader::Version2 {
                package_file_version: _,
                engine_version: _,
                custom_version_format: _,
                custom_versions: _,
                save_game_class_name: _,
            } => false,
            GvasHeader::Version3 {
                package_file_version: _,
                unknown: _,
                engine_version: _,
                custom_version_format: _,
                custom_versions: _,
                save_game_class_name: _,
            } => true,
        }
    }
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
    /// use std::fs::File;
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
    /// use std::{collections::HashMap, fs::File};
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

        let mut options = PropertyOptions {
            hints,
            properties_stack: &mut vec![],
            large_world_coordinates: header.use_large_world_coordinates(),
            custom_versions: header.get_custom_versions(),
        };

        let mut properties = IndexMap::new();
        loop {
            let property_name = cursor.read_string()?;
            if property_name == "None" {
                break;
            }

            let property_type = cursor.read_string()?;

            options.properties_stack.push(property_name.clone());

            let property = Property::new(cursor, &property_type, true, &mut options, None)?;
            properties.insert(property_name, property);

            let _ = options.properties_stack.pop();
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
    ///     io::Cursor,
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

        let mut options = PropertyOptions {
            hints: &HashMap::new(),
            properties_stack: &mut vec![],
            large_world_coordinates: self.header.use_large_world_coordinates(),
            custom_versions: self.header.get_custom_versions(),
        };

        for (name, property) in &self.properties {
            cursor.write_string(name)?;
            property.write(cursor, true, &mut options)?;
        }
        cursor.write_string("None")?;
        cursor.write_i32::<LittleEndian>(0)?; // padding
        Ok(())
    }
}
