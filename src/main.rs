// #![allow(dead_code, unused_variables)]

mod format;

use format::RawImage;

use std::{env, fs::File};

const FILE_NAME: &str = "RAW_CANON_300D.crw";

fn main() {
    let filename = env::args().nth(1).unwrap_or(FILE_NAME.into());

    let file = File::open(filename).unwrap();
    match RawImage::try_new(file) {
        Ok(raw) => {
            println!("Header is {:?}", raw.header());
            println!("Body is {:?}", raw.directory());
        }
        Err(e) => {
            eprintln!("Error: {e}");
        }
    }
}
