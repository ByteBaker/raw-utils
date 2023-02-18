#![allow(dead_code)]

use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

const TAG_INDEX_MASK: i32 = 0x07ff;

#[derive(Debug)]
pub struct DirectoryEntry {
    pub tag: u16,
    pub size: u32,
    pub offset: u32,
    pub data_location: DataLocation,
    pub data_format: DataFormat,
}

impl From<&[u8]> for DirectoryEntry {
    fn from(slice: &[u8]) -> Self {
        if slice.len() != 10 {
            panic!(
                "Invalid block size: {} bytes. Required: 10 bytes.",
                slice.len()
            );
        }

        let mut cursor = Cursor::new(slice);

        let tag = cursor.read_u16::<LittleEndian>().unwrap();
        let size = cursor.read_u32::<LittleEndian>().unwrap();
        let offset = cursor.read_u32::<LittleEndian>().unwrap();
        let data_location = Self::data_location(tag);
        let data_format = Self::data_format(tag);
        Self {
            tag,
            size,
            offset,
            data_location,
            data_format,
        }
    }
}

impl DirectoryEntry {
    pub fn data_location(tag: u16) -> DataLocation {
        const DATA_LOCATION_MASK: u16 = 0xc000;
        match tag & DATA_LOCATION_MASK {
            0x0000 => DataLocation::ValueData,
            0x4000 => DataLocation::Directory,
            _ => unreachable!(),
        }
    }

    pub const fn data_format(tag: u16) -> DataFormat {
        const DATA_FORMAT_MASK: u16 = 0x3800;
        match tag & DATA_FORMAT_MASK {
            0x0000 => DataFormat::ByteArray,
            0x0800 => DataFormat::AsciiString,
            0x1000 => DataFormat::ArrayOf16Bit,
            0x1800 => DataFormat::ArrayOf32Bit,
            0x2000 => DataFormat::Structure,
            0x2800 | 0x3000 => DataFormat::Subdirectory,
            _ => unreachable!(),
        }
    }

    fn get_alignment(&self) -> u8 {
        self.data_format.alignment()
    }
}

#[derive(Debug)]
pub enum DataLocation {
    ValueData,
    Directory,
}

#[derive(Debug)]
pub enum DataFormat {
    ByteArray,
    AsciiString,
    ArrayOf16Bit,
    ArrayOf32Bit,
    Structure,
    Subdirectory,
}

impl DataFormat {
    const fn alignment(&self) -> u8 {
        match &self {
            Self::ByteArray => 1_u8,
            Self::AsciiString => 1_u8,
            Self::ArrayOf16Bit => 2_u8,
            Self::ArrayOf32Bit => 4_u8,
            Self::Structure => 1_u8,
            Self::Subdirectory => 1_u8,
        }
    }
}
