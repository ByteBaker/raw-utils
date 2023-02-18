#![allow(dead_code, unused_variables)]

mod format;

use crate::format::{DirectoryBlock, Header};
use std::{fs::File, io::Read};

const FILE_NAME: &str = "RAW_CANON_300D.crw";

fn main() {
    let mut file = File::open(FILE_NAME).unwrap();
    let mut buf = Vec::<u8>::new();

    let data = file.read_to_end(&mut buf);

    match data {
        Ok(l) => {
            println!("Read {l} bytes");
            // let slice: &[u8] = &buf[0..26];
            let header = Header::from(&buf[0..26]);
            let _block = DirectoryBlock::from(&buf[26..]);

            println!("Header is {:?}", header);
            // println!("Body is {:?}", block);
        }
        Err(e) => println!("Failed to read: {:?}", e),
    }
}
