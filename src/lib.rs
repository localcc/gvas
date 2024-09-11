#![warn(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
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
//! use gvas::game_version::GameVersion;
//!
//! let mut file = File::open("save.sav")?;
//! let gvas_file = GvasFile::read(&mut file, GameVersion::Default);
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
//! use gvas::game_version::GameVersion;
//!
//! let mut file = File::open("save.sav")?;
//!
//! let mut hints = HashMap::new();
//! hints.insert("UnLockedMissionParameters.MapProperty.Key.StructProperty".to_string(), "Guid".to_string());
//!
//! let gvas_file = GvasFile::read_with_hints(&mut file, GameVersion::Default, &hints);
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
/// Game version enumeration.
pub mod game_version;
/// Object version information.
pub mod object_version;
/// Extensions for `Ord`.
mod ord_ext;
/// Property types.
pub mod properties;
/// Savegame version information.
pub mod savegame_version;
pub(crate) mod scoped_stack_entry;
/// Various types.
pub mod types;

use std::io::{Cursor, SeekFrom};
use std::{
    collections::HashMap,
    fmt::Debug,
    io::{Read, Seek, Write},
};

use crate::error::DeserializeError;
use crate::game_version::{
    DeserializedGameVersion, GameVersion, PalworldCompressionType, PLZ_MAGIC,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use cursor_ext::{ReadExt, WriteExt};
use custom_version::FCustomVersion;
use engine_version::FEngineVersion;
use error::Error;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use indexmap::IndexMap;
use object_version::EUnrealEngineObjectUE5Version;
use ord_ext::OrdExt;
use properties::{Property, PropertyOptions, PropertyTrait};
use savegame_version::SaveGameVersion;
use types::Guid;

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
        custom_versions: IndexMap<Guid, u32>,
        /// Save game class name.
        save_game_class_name: String,
    },
    /// Version 3
    Version3 {
        /// File format version (UE4).
        package_file_version: u32,
        /// File format version (UE5).
        package_file_version_ue5: u32,
        /// Unreal Engine version.
        engine_version: FEngineVersion,
        /// Custom version format.
        custom_version_format: u32,
        /// Custom versions.
        custom_versions: IndexMap<Guid, u32>,
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
            Err(DeserializeError::InvalidHeader(
                format!("File type {file_type_tag} not recognized").into_boxed_str(),
            ))?
        }

        let save_game_file_version = cursor.read_u32::<LittleEndian>()?;
        if !save_game_file_version.between(
            SaveGameVersion::AddedCustomVersions as u32,
            SaveGameVersion::PackageFileSummaryVersionChange as u32,
        ) {
            Err(DeserializeError::InvalidHeader(
                format!("GVAS version {save_game_file_version} not supported").into_boxed_str(),
            ))?
        }

        let package_file_version = cursor.read_u32::<LittleEndian>()?;
        if !package_file_version.between(0x205, 0x20D) {
            Err(DeserializeError::InvalidHeader(
                format!("Package file version {package_file_version} not supported")
                    .into_boxed_str(),
            ))?
        }

        // This field is only present in the v3 header
        let package_file_version_ue5 = if save_game_file_version
            >= SaveGameVersion::PackageFileSummaryVersionChange as u32
        {
            let version = cursor.read_u32::<LittleEndian>()?;
            if !version.between(
                EUnrealEngineObjectUE5Version::InitialVersion as u32,
                EUnrealEngineObjectUE5Version::DataResources as u32,
            ) {
                Err(DeserializeError::InvalidHeader(
                    format!("UE5 Package file version {version} is not supported").into_boxed_str(),
                ))?
            }
            Some(version)
        } else {
            None
        };

        let engine_version = FEngineVersion::read(cursor)?;
        let custom_version_format = cursor.read_u32::<LittleEndian>()?;
        if custom_version_format != 3 {
            Err(DeserializeError::InvalidHeader(
                format!("Custom version format {custom_version_format} not supported")
                    .into_boxed_str(),
            ))?
        }

        let custom_versions_len = cursor.read_u32::<LittleEndian>()?;
        let mut custom_versions = IndexMap::new();
        for _ in 0..custom_versions_len {
            let FCustomVersion { key, version } = FCustomVersion::read(cursor)?;
            custom_versions.insert(key, version);
        }

        let save_game_class_name = cursor.read_string()?;

        Ok(match package_file_version_ue5 {
            None => GvasHeader::Version2 {
                package_file_version,
                engine_version,
                custom_version_format,
                custom_versions,
                save_game_class_name,
            },
            Some(package_file_version_ue5) => GvasHeader::Version3 {
                package_file_version,
                package_file_version_ue5,
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
    pub fn write<W: Write>(&self, cursor: &mut W) -> Result<usize, Error> {
        cursor.write_u32::<LittleEndian>(FILE_TYPE_GVAS)?;
        match self {
            GvasHeader::Version2 {
                package_file_version,
                engine_version,
                custom_version_format,
                custom_versions,
                save_game_class_name,
            } => {
                let mut len = 20;
                cursor.write_u32::<LittleEndian>(2)?;
                cursor.write_u32::<LittleEndian>(*package_file_version)?;
                len += engine_version.write(cursor)?;
                cursor.write_u32::<LittleEndian>(*custom_version_format)?;
                cursor.write_u32::<LittleEndian>(custom_versions.len() as u32)?;
                for (&key, &version) in custom_versions {
                    len += FCustomVersion::new(key, version).write(cursor)?;
                }
                len += cursor.write_string(save_game_class_name)?;
                Ok(len)
            }

            GvasHeader::Version3 {
                package_file_version,
                package_file_version_ue5,
                engine_version,
                custom_version_format,
                custom_versions,
                save_game_class_name,
            } => {
                let mut len = 24;
                cursor.write_u32::<LittleEndian>(3)?;
                cursor.write_u32::<LittleEndian>(*package_file_version)?;
                cursor.write_u32::<LittleEndian>(*package_file_version_ue5)?;
                len += engine_version.write(cursor)?;
                cursor.write_u32::<LittleEndian>(*custom_version_format)?;
                cursor.write_u32::<LittleEndian>(custom_versions.len() as u32)?;
                for (&key, &version) in custom_versions {
                    len += FCustomVersion::new(key, version).write(cursor)?
                }
                len += cursor.write_string(save_game_class_name)?;
                Ok(len)
            }
        }
    }

    /// Get custom versions from this header
    pub fn get_custom_versions(&self) -> &IndexMap<Guid, u32> {
        match self {
            GvasHeader::Version2 {
                custom_versions, ..
            } => custom_versions,
            GvasHeader::Version3 {
                custom_versions, ..
            } => custom_versions,
        }
    }
}

/// Main UE4 save file struct
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GvasFile {
    /// Game version
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "DeserializedGameVersion::is_default")
    )]
    pub deserialized_game_version: DeserializedGameVersion,
    /// GVAS file header.
    pub header: GvasHeader,
    /// GVAS properties.
    pub properties: IndexMap<String, Property>,
}

trait GvasHeaderTrait {
    fn use_large_world_coordinates(&self) -> bool;
}

impl GvasHeaderTrait for GvasHeader {
    fn use_large_world_coordinates(&self) -> bool {
        match self {
            GvasHeader::Version2 { .. } => false,
            GvasHeader::Version3 { .. } => true,
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
    /// use gvas::game_version::GameVersion;
    ///
    /// let mut file = File::open("save.sav")?;
    /// let gvas_file = GvasFile::read(&mut file, GameVersion::Default);
    ///
    /// println!("{:#?}", gvas_file);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn read<R: Read + Seek>(cursor: &mut R, game_version: GameVersion) -> Result<Self, Error> {
        let hints = HashMap::new();
        Self::read_with_hints(cursor, game_version, &hints)
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
    /// use gvas::game_version::GameVersion;
    ///
    /// let mut file = File::open("save.sav")?;
    ///
    /// let mut hints = HashMap::new();
    /// hints.insert(
    ///     "SeasonSave.StructProperty.Seasons.MapProperty.Key.StructProperty".to_string(),
    ///     "Guid".to_string(),
    /// );
    ///
    /// let gvas_file = GvasFile::read_with_hints(&mut file, GameVersion::Default, &hints);
    ///
    /// println!("{:#?}", gvas_file);
    /// # Ok::<(), Error>(())
    /// ```
    pub fn read_with_hints<R: Read + Seek>(
        cursor: &mut R,
        game_version: GameVersion,
        hints: &HashMap<String, String>,
    ) -> Result<Self, Error> {
        let deserialized_game_version: DeserializedGameVersion;
        let mut cursor = match game_version {
            GameVersion::Default => {
                deserialized_game_version = DeserializedGameVersion::Default;
                let mut data = Vec::new();
                cursor.read_to_end(&mut data)?;
                Cursor::new(data)
            }
            GameVersion::Palworld => {
                let decompresed_length = cursor.read_u32::<LittleEndian>()?;
                let _compressed_length = cursor.read_u32::<LittleEndian>()?;

                let mut magic = [0u8; 3];
                cursor.read_exact(&mut magic)?;
                if &magic != PLZ_MAGIC {
                    Err(DeserializeError::InvalidHeader(
                        format!("Invalid PlZ magic {magic:?}").into_boxed_str(),
                    ))?
                }

                let compression_type = cursor.read_enum()?;

                deserialized_game_version = DeserializedGameVersion::Palworld(compression_type);

                match compression_type {
                    PalworldCompressionType::None => {
                        let mut data = vec![0u8; decompresed_length as usize];

                        cursor.read_exact(&mut data)?;
                        Cursor::new(data)
                    }
                    PalworldCompressionType::Zlib => {
                        let mut zlib_data = vec![0u8; decompresed_length as usize];

                        let mut decoder = ZlibDecoder::new(cursor);
                        decoder.read_exact(&mut zlib_data)?;

                        Cursor::new(zlib_data)
                    }
                    PalworldCompressionType::ZlibTwice => {
                        let decoder = ZlibDecoder::new(cursor);
                        let mut decoder = ZlibDecoder::new(decoder);

                        let mut zlib_data = Vec::new();
                        decoder.read_to_end(&mut zlib_data)?;

                        Cursor::new(zlib_data)
                    }
                }
            }
        };

        let header = GvasHeader::read(&mut cursor)?;

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

            let property = Property::new(&mut cursor, &property_type, true, &mut options, None)?;
            properties.insert(property_name, property);

            let _ = options.properties_stack.pop();
        }

        Ok(GvasFile {
            deserialized_game_version,
            header,
            properties,
        })
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
    /// use gvas::game_version::GameVersion;
    ///
    /// let mut file = File::open("save.sav")?;
    /// let gvas_file = GvasFile::read(&mut file, GameVersion::Default)?;
    ///
    /// let mut writer = Cursor::new(Vec::new());
    /// gvas_file.write(&mut writer)?;
    /// println!("{:#?}", writer.get_ref());
    /// # Ok::<(), Error>(())
    /// ```
    pub fn write<W: Write + Seek>(&self, cursor: &mut W) -> Result<(), Error> {
        let mut writing_cursor = Cursor::new(Vec::new());

        self.header.write(&mut writing_cursor)?;

        let mut options = PropertyOptions {
            hints: &HashMap::new(),
            properties_stack: &mut vec![],
            large_world_coordinates: self.header.use_large_world_coordinates(),
            custom_versions: self.header.get_custom_versions(),
        };

        for (name, property) in &self.properties {
            writing_cursor.write_string(name)?;
            property.write(&mut writing_cursor, true, &mut options)?;
        }
        writing_cursor.write_string("None")?;
        writing_cursor.write_i32::<LittleEndian>(0)?; // padding

        match self.deserialized_game_version {
            DeserializedGameVersion::Default => cursor.write_all(&writing_cursor.into_inner())?,
            DeserializedGameVersion::Palworld(compression_type) => {
                let decompressed = writing_cursor.into_inner();

                cursor.write_u32::<LittleEndian>(decompressed.len() as u32)?;
                let compressed_length_pos = cursor.stream_position()?;
                cursor.write_u32::<LittleEndian>(0)?; // Compressed length placeholder, will be updated later
                cursor.write_all(PLZ_MAGIC)?;
                cursor.write_enum(compression_type)?;

                // Compress and write data directly to the output cursor
                match compression_type {
                    PalworldCompressionType::None => cursor.write_all(&decompressed)?,
                    PalworldCompressionType::Zlib => {
                        let mut encoder = ZlibEncoder::new(cursor.by_ref(), Compression::new(6));
                        encoder.write_all(&decompressed)?;
                        encoder.finish()?;
                    }
                    PalworldCompressionType::ZlibTwice => {
                        let encoder = ZlibEncoder::new(cursor.by_ref(), Compression::default());
                        let mut encoder = ZlibEncoder::new(encoder, Compression::default());
                        encoder.write_all(&decompressed)?;
                        encoder.finish()?;
                    }
                }

                // Update compressed length
                let end_pos = cursor.stream_position()?;
                cursor.seek(SeekFrom::Start(compressed_length_pos))?;
                cursor.write_u32::<LittleEndian>((end_pos - (compressed_length_pos + 4)) as u32)?;
                cursor.seek(SeekFrom::Start(end_pos))?;
            }
        }
        Ok(())
    }
}
