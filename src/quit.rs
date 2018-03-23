// Copyright 2018 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Quit the program with standardized exit codes.

use std::error::Error as StdError;
use std::process;

use rustfuck_lib::Error;

/// The exit codes returned by the program.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ExitCode {

    /// Successful (i.e. expected program execution (Code: `0`).
    Success = 0,

    /// Failure due to I/O operations (Code: `1`).
    IOFailure = 1,
}

/// Quit the program execution. The exit code and message are chosen based on the `error`.
pub fn fail_from_error(error: Error) -> ! {
    match error {
        Error::IO(message) => {
            fail_with_message(ExitCode::IOFailure, message.description());
        }
    }
}

/// Quit the program execution with the given `exit_code` and an error `message` explaining the xit.
pub fn fail_with_message(exit_code: ExitCode, message: &str) -> ! {
    println!("Error: {description}", description = message);
    process::exit(exit_code as i32)
}

/// Quit the program with a `Success` exit code.
pub fn succeed() -> ! {
    process::exit(ExitCode::Success as i32)
}
