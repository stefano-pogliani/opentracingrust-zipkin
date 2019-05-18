use std::error::Error as StdError;
use std::fmt;

/// Enumeration of all errors returned by the crate.
#[derive(Debug)]
pub enum Error {
    Reqwest(::reqwest::Error),
    Thrift(::thrift::Error),
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Reqwest(ref reqwest) => fmt::Display::fmt(reqwest, f),
            Error::Thrift(ref thrift) => fmt::Display::fmt(thrift, f),
        }
    }
}

impl From<::reqwest::Error> for Error {
    fn from(error: ::reqwest::Error) -> Error {
        Error::Reqwest(error)
    }
}

impl From<::thrift::Error> for Error {
    fn from(error: ::thrift::Error) -> Error {
        Error::Thrift(error)
    }
}

/// Type alias for `Result`s that can fail with an `Error`.
pub type Result<T> = ::std::result::Result<T, Error>;
