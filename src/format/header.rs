use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub enum Endianness {
    LE,
    BE,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum CameraSignature {
    HEAPCCDR,
    HEAPJPGM,
}

#[derive(Debug)]
pub struct Header {
    pub endianness: Endianness,
    pub header_length: u32,
    pub signature: CameraSignature,
    pub crw_version: u32,
    pub reserved: [u32; 2],
}

impl From<&[u8]> for Header {
    fn from(slice: &[u8]) -> Self {
        if slice.len() < 26 {
            panic!("Can't parse header. Not a RAW file.")
        }
        let mut cursor = Cursor::new(slice);
        let endianness = match cursor.read_u16::<LittleEndian>().unwrap() {
            0x4949 => Endianness::LE,
            0x4D4D => Endianness::BE,
            _ => unreachable!(),
        };

        let header_length = cursor.read_u32::<LittleEndian>().unwrap();
        let signature = match cursor.read_uint::<LittleEndian>(8).unwrap() {
            0x5244434350414548 => CameraSignature::HEAPCCDR,
            0x4D47504A50414548 => CameraSignature::HEAPJPGM,
            _ => unreachable!(),
        };

        let crw_version = cursor.read_u32::<LittleEndian>().unwrap();
        let reserved = [
            cursor.read_u32::<LittleEndian>().unwrap(),
            cursor.read_u32::<LittleEndian>().unwrap(),
        ];

        Self {
            endianness,
            header_length,
            signature,
            crw_version,
            reserved,
        }
    }
}
