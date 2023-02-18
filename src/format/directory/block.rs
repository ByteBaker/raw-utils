#![allow(dead_code, unused_imports, unused_variables, unreachable_code)]

use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

use super::entry::DirectoryEntry;

#[derive(Debug)]
pub struct DirectoryBlock {
    pub value_data: Vec<u8>,
    pub dir_count: u16,
    pub dir_entries: Vec<DirectoryEntry>,
    pub other_data: Vec<u8>,
    pub dir_start: u32,
}

impl From<&[u8]> for DirectoryBlock {
    fn from(slice: &[u8]) -> Self {
        assert!(slice.len() >= 6);

        let size = slice.len();
        let dir_start_offset = size as usize - 4;

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
            .map(|chunk| DirectoryEntry::from(chunk))
            .collect();

        let other_data: Vec<u8> = slice[other_data_offset..dir_start_offset].to_vec();

        Self {
            value_data,
            dir_count,
            dir_entries,
            other_data,
            dir_start,
        }
    }
}
