#![allow(warnings)]
use super::entry::{DataLocation, DirectoryEntry};

pub fn process_dir_tables(entries: &[DirectoryEntry], value_data: &[u8]) {
    entries.iter().for_each(|entry| {
        match entry.data_location {
            DataLocation::ValueData => todo!(),
            DataLocation::Directory => todo!(),
        };
    });
}

