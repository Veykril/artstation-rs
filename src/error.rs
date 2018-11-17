use reqwest;

use std::{error::Error as StdError, fmt};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EmptyString,
    Reqwest(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::EmptyString => f.write_str("Empty String not allowed"),
            Error::Reqwest(ref e) => fmt::Display::fmt(e, f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self {
            Error::EmptyString => "Empty String not allowed",
            Error::Reqwest(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match self {
            Error::EmptyString => None,
            Error::Reqwest(ref e) => e.cause(),
        }
    }
}

impl From<reqwest::Error> for Error {
    #[inline]
    fn from(err: reqwest::Error) -> Error {
        Error::Reqwest(err)
    }
}
