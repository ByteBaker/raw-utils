use std::{
    fmt::Display,
    io::{Error, Read},
};

use super::{header::HeaderParseError, DirectoryBlock, DirectoryParseError, Header};

#[derive(Debug)]
pub struct RawImage {
    #[allow(dead_code)]
    buffer: Vec<u8>,
    header: Header,
    dir: DirectoryBlock,
}

#[derive(Debug)]
pub enum RawError {
    IoError(Error),
    BadHeader(HeaderParseError),
    BadDirectory(DirectoryParseError),
}

impl Display for RawError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for RawError {}

macro_rules! impl_error_type {
    ($from: ty, $variant: tt) => {
        impl From<$from> for RawError {
            fn from(value: $from) -> Self {
                Self::$variant(value)
            }
        }
    };
}

impl_error_type!(Error, IoError);
impl_error_type!(HeaderParseError, BadHeader);
impl_error_type!(DirectoryParseError, BadDirectory);

impl RawImage {
    pub fn try_new(mut from: impl Read) -> Result<Self, RawError> {
        let mut buffer = vec![];
        from.read_to_end(&mut buffer)?;

        let header = Header::try_from(buffer.as_slice())?;

        let block_slice = &buffer[26..];
        let dir = DirectoryBlock::try_from(block_slice)?;

        Ok(Self {
            buffer,
            header,
            dir,
        })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn directory(&self) -> &DirectoryBlock {
        &self.dir
    }
}
