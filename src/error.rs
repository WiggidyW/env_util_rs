use std::{
    error::Error as StdError,
    ffi::OsString,
};

pub enum Error {
    Missing(MissingError),
    InvalidUnicode(InvalidUnicodeError),
    Parse(ParseError),
}

pub struct MissingError {
    pub key: String,
}

pub struct InvalidUnicodeError {
    pub key: String,
    pub value: String,
}

pub struct ParseError {
    pub key: String,
    pub value: String,
    pub from: &'static str,
    pub to: &'static str,
    pub err: Box<dyn StdError>,
}

impl From<MissingError> for Error {
    fn from(err: MissingError) -> Self {
        Self::Missing(err)
    }
}
impl From<InvalidUnicodeError> for Error {
    fn from(err: InvalidUnicodeError) -> Self {
        Self::InvalidUnicode(err)
    }
}
impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self {
        Self::Parse(err)
    }
}
