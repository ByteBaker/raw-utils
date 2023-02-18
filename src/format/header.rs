use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub enum Endianness {
    LE,
    BE,
}

// #[allow(dead_code)]
#[derive(Debug)]
pub enum CameraSignature {
    HEAPCCDR,
    HEAPJPGM,
}

#[derive(Debug)]
pub enum HeaderParseError {
    InvalidLength,
    UnknownBytes,
    InvalidSignature,
}

#[derive(Debug)]
pub struct Header {
    pub endianness: Endianness,
    pub header_length: u32,
    pub signature: CameraSignature,
    pub crw_version: u32,
    pub reserved: [u32; 2],
}

impl TryFrom<&[u8]> for Header {
    type Error = HeaderParseError;
    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        if slice.len() < 26 {
            return Err(HeaderParseError::InvalidLength);
        }
        let mut cursor = Cursor::new(slice);
        let endianness = match cursor.read_u16::<LittleEndian>().unwrap() {
            0x4949 => Endianness::LE,
            0x4D4D => Endianness::BE,
            _ => return Err(HeaderParseError::UnknownBytes),
        };

        let header_length = cursor.read_u32::<LittleEndian>().unwrap();
        let signature = match cursor.read_uint::<LittleEndian>(8).unwrap() {
            0x5244434350414548 => CameraSignature::HEAPCCDR,
            0x4D47504A50414548 => CameraSignature::HEAPJPGM,
            _ => return Err(HeaderParseError::InvalidSignature),
        };

        let crw_version = cursor.read_u32::<LittleEndian>().unwrap();
        let reserved = [
            cursor.read_u32::<LittleEndian>().unwrap(),
            cursor.read_u32::<LittleEndian>().unwrap(),
        ];

        Ok(Self {
            endianness,
            header_length,
            signature,
            crw_version,
            reserved,
        })
    }
}
