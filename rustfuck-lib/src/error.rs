// Copyright 2018 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Error handling.

use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IOError;
use std::result::Result as StdResult;

/// A specialized `Result` type for `rustfuck`.
pub type Result<T> = StdResult<T, Error>;

/// A wrapper type for all errors caused by this crate.
#[derive(Debug)]
pub enum Error {

    /// IO errors caused by file handling failures.
    IO(IOError),
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IO(ref error) => error.fmt(formatter),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IO(ref error) => error.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::IO(ref error) => Some(error),
        }
    }
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Error {
        Error::IO(error)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error as StdError;
    use std::io::Error as IOError;

    use Error;

    #[test]
    fn fmt_io() {
        let io_error = IOError::from_raw_os_error(42);
        let fmt = String::from(format!("{}", io_error));
        let error = Error::IO(io_error);
        assert_eq!(format!("{}", error), fmt);
    }

    #[test]
    fn description_io() {
        let io_error = IOError::from_raw_os_error(42);
        let description = String::from(io_error.description());
        let error = Error::IO(io_error);
        assert_eq!(error.description(), description);
    }

    #[test]
    fn cause_io() {
        let io_error = IOError::from_raw_os_error(42);
        let io_error_debug: String = format!("{:?}", io_error);
        let error = Error::IO(io_error);

        assert!(error.cause().is_some());
        // PartialEq is not implemented for errors, thus compare at least the debug messages for equality.
        assert_eq!(format!("{:?}", error.cause().unwrap()), io_error_debug);
    }

    #[test]
    fn from_io() {
        let io_error = IOError::from_raw_os_error(42);
        assert!(match Error::from(io_error) {
            Error::IO(_) => true,
            // _ => false,
        });
    }
}
