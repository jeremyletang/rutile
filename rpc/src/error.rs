// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error as StdError;
use std::io::Error as IoError;
use std::fmt;

/// Result type often returned from methods that can have hyper `Error`s.
pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// An error that occured while decoding / encoding a message
    InvalidBody,
    /// An `io::Error` that occurred while trying to read or write to a network stream.
    Io(IoError),
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => fmt::Display::fmt(e, f),
            ref e => f.write_str(e.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidBody => "Unable to decode Message body, format may be invalid.",
            Error::Io(ref e) => e.description(),
            Error::Custom(ref s) => &*s,
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Io(ref error) => Some(error),
            _ => None,
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}
