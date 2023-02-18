#![allow(dead_code, unused_imports, unused_variables, unreachable_code)]

use std::{fmt::Debug, io::Cursor};

use byteorder::{LittleEndian, ReadBytesExt};

use super::entry::DirectoryEntry;

pub struct DirectoryBlock {
    pub value_data: Vec<u8>,
    pub dir_count: u16,
    pub dir_entries: Vec<DirectoryEntry>,
    pub other_data: Vec<u8>,
    pub dir_start: u32,
}

impl Debug for DirectoryBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DirectoryBlock")
            .field("value_data len", &self.value_data.len())
            .field("dir_count", &self.dir_count)
            .field("dir_entries", &self.dir_entries.len())
            .field("other_data len", &self.other_data.len())
            .field("dir_start", &self.dir_start)
            .finish()
    }
}

#[derive(Debug)]
pub struct DirectoryParseError(&'static str);

impl TryFrom<&[u8]> for DirectoryBlock {
    type Error = DirectoryParseError;
    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        if slice.len() < 6 {
            return Err(DirectoryParseError("Invalid length of directory"));
        }

        let size = slice.len();
        let dir_start_offset = size - 4;

        let mut cursor = Cursor::new(&slice[dir_start_offset..size]);

        let dir_start: u32 = cursor.read_u32::<LittleEndian>().unwrap();
        let value_data: Vec<u8> = slice[0..dir_start as usize].to_vec();

        let dir_count_offset = dir_start as usize;
        let dir_entries_offset = dir_count_offset + 2;

        cursor = Cursor::new(&slice[dir_count_offset..dir_entries_offset]);
        let dir_count = cursor.read_u16::<LittleEndian>().unwrap();

        let other_data_offset = dir_entries_offset + dir_count as usize * 10;

        const DIRECTORY_ENTRY_SIZE: usize = 10;
        let dir_entries: Vec<DirectoryEntry> = slice[dir_entries_offset..other_data_offset]
            .chunks_exact(DIRECTORY_ENTRY_SIZE)
            .map(DirectoryEntry::from)
            .collect();

        let other_data: Vec<u8> = slice[other_data_offset..dir_start_offset].to_vec();

        Ok(Self {
            value_data,
            dir_count,
            dir_entries,
            other_data,
            dir_start,
        })
    }
}
