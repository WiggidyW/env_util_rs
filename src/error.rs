use std::{
    error::Error as StdError,
    fmt::Display,
};

#[derive(Debug)]
pub enum Error {
    InvalidUnicode(InvalidUnicodeError),
    Missing(MissingError),
    Parse(ParseError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Environment Variable Error: {}",
            self.source().unwrap(),
        )
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::InvalidUnicode(e) => Some(e),
            Self::Missing(e) => Some(e),
            Self::Parse(e) => Some(e),
        }
    }
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

#[derive(Debug, Clone)]
pub struct MissingError {
    pub key: String,
}

impl Display for MissingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Key: '{}' is missing.",
            self.key,
        )
    }
}

impl StdError for MissingError {}

#[derive(Debug, Clone)]
pub struct InvalidUnicodeError {
    pub key: String,
    pub value: String,
}

impl Display for InvalidUnicodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Key: '{}', Value: '{}' is not valid unicode.",
            self.key,
            self.value,
        )
    }
}

impl StdError for InvalidUnicodeError {}

#[derive(Debug)]
pub struct ParseError {
    pub key: String,
    pub value: String,
    pub from: &'static str,
    pub to: &'static str,
    pub err: Box<dyn StdError>,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "Key: '{}', Value: '{}', \
            Failed to convert from '{}' to '{}'. \
            Error: {}",
            self.key,
            self.value,
            self.from,
            self.to,
            self.err,
        )
    }
}

impl StdError for ParseError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&*self.err)
    }
}
