mod cursor_ext;
pub mod error;
pub mod properties;

use std::{
    collections::HashMap,
    fmt::Debug,
    io::{Cursor, Read, Write},
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use cursor_ext::CursorExt;
use error::Error;
use properties::{Property, PropertyTrait};

pub struct FEngineVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    pub change_list: u32,
    pub branch: String,
}

impl FEngineVersion {
    pub fn read(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
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

    pub fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        cursor.write_u16::<LittleEndian>(self.major)?;
        cursor.write_u16::<LittleEndian>(self.minor)?;
        cursor.write_u16::<LittleEndian>(self.patch)?;
        cursor.write_u32::<LittleEndian>(self.change_list)?;
        cursor.write_string(&self.branch)?;
        Ok(())
    }
}

pub type Guid = [u8; 16];

pub struct FCustomVersion {
    pub key: Guid,
    pub version: i32,
}

impl FCustomVersion {
    pub fn read(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut guid = [0u8; 16];
        cursor.read_exact(&mut guid)?;
        let version = cursor.read_i32::<LittleEndian>()?;

        Ok(FCustomVersion { key: guid, version })
    }

    pub fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        cursor.write(&self.key)?;
        cursor.write_i32::<LittleEndian>(self.version)?;
        Ok(())
    }
}

pub struct GvasHeader {
    pub file_type_tag: i32,
    pub save_game_file_version: i32,
    pub package_file_ue4_version: i32,
    pub engine_version: FEngineVersion,
    pub custom_version_format: i32,
    pub custom_versions: Vec<FCustomVersion>,
    pub save_game_class_name: String,
}

impl GvasHeader {
    pub fn read(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let file_type_tag = cursor.read_i32::<LittleEndian>()?;
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

    pub fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
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

pub struct GvasFile {
    pub header: GvasHeader,
    pub properties: HashMap<String, Property>,
}

impl GvasFile {
    pub fn read(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let header = GvasHeader::read(cursor)?;

        let mut properties = HashMap::new();
        let mut property_name = cursor.read_string()?;
        while property_name != "None" {
            let property_type = cursor.read_string()?;
            let property = Property::new(cursor, &property_type, true)?;
            properties.insert(property_name, property);
            property_name = cursor.read_string()?;
        }

        Ok(GvasFile { header, properties })
    }

    pub fn write(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<(), Error> {
        self.header.write(cursor)?;

        for (name, property) in &self.properties {
            cursor.write_string(name)?;
            property.write(cursor, true)?;
        }
        cursor.write_string(&String::from("None"))?;
        cursor.write_i32::<LittleEndian>(0)?; // padding
        Ok(())
    }
}
