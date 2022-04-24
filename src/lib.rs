mod cursor_ext;
pub mod error;
pub mod properties;

use std::{fmt::Debug, io::{Cursor, Read}, collections::HashMap};

use byteorder::{LittleEndian, ReadBytesExt};
use cursor_ext::CursorExt;
use error::Error;
use properties::Property;

pub struct FEngineVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    pub change_list: u32,
    pub branch: String
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
            branch
        })
    }
}

pub type Guid = [u8; 16];

pub struct FCustomVersion {
    pub key: Guid,
    pub version: i32
}

impl FCustomVersion {
    pub fn read(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut guid = [0u8; 16];
        cursor.read_exact(&mut guid)?;
        let version = cursor.read_i32::<LittleEndian>()?;
        
        Ok(FCustomVersion {
            key: guid,
            version
        })
    }
}

pub struct GvasHeader {
    pub file_type_tag: i32,
    pub save_game_file_version: i32,
    pub package_file_ue4_version: i32,
    pub engine_version: FEngineVersion,
    pub custom_version_format: i32,
    pub custom_versions: Vec<FCustomVersion>,
    pub save_game_class_name: String
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
            save_game_class_name
        })
    }
}

pub struct GvasFile {
    pub header: GvasHeader,
    pub properties: HashMap<String, Property>
}

impl GvasFile {
    pub fn read(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let header = GvasHeader::read(cursor)?;

        let mut properties = HashMap::new(); 
        let mut property_name = cursor.read_string()?;
        while property_name != "None" {
            let property = Property::new(cursor)?;
            properties.insert(property_name, property);
            property_name = cursor.read_string()?;
        }

        Ok(GvasFile {
            header,
            properties
        })
    }
}