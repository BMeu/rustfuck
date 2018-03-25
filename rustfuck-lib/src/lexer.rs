// Copyright 2018 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Implementations and trait definitions for lexers, analyzing a program source to specific tokens.

/// The tokens representing the commands understood by rustfuck.
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    /// Increment the current cell.
    Add(String, MetaData),

    /// Decrement the current cell.
    Sub(String, MetaData),

    /// Move the pointer to the next right neighbor of the current cell.
    Right(String, MetaData),

    /// Move the pointer to the next left neighbor of the current cell.
    Left(String, MetaData),

    /// Read a single character into the current cell.
    Read(String, MetaData),

    /// Print the content of the current cell.
    Write(String, MetaData),

    /// Begin a loop at the current cell.
    BeginLoop(String, MetaData),

    /// End the current loop.
    EndLoop(String, MetaData),
}

/// Additional information on each token.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MetaData {

    /// The line number in which the token occurs.
    pub lineno: usize,

    /// The position within the line at which the token starts.
    pub position: usize,
}

/// Convert a language's lexemes into tokens.
pub trait Lexer {

    /// Convert the `input` string into a list of tokens.
    fn tokenize(&self) -> Vec<Token>;
}
