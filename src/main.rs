mod format;

use format::RawImage;

use std::{
    env,
    fs::File,
    io::{Error, ErrorKind},
};

fn main() -> Result<(), Error> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| Error::new(ErrorKind::Other, "No file path provided"))?;

    let file = File::open(filename)?;
    match RawImage::try_new(file) {
        Ok(raw) => {
            println!("Header is {:?}", raw.header());
            println!("Body is {:?}", raw.directory());
        }
        Err(e) => {
            eprintln!("Error: {e}");
        }
    };

    Ok(())
}
