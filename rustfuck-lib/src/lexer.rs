// Copyright 2018 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Implementations and trait definitions for lexers, analyzing a program source to specific tokens.

/// The tokens representing the commands understood by rustfuck.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    /// Increment the current cell.
    Add,

    /// Decrement the current cell.
    Sub,

    /// Move the pointer to the next right neighbor of the current cell.
    Right,

    /// Move the pointer to the next left neighbor of the current cell.
    Left,

    /// Read a single character into the current cell.
    Read,

    /// Print the content of the current cell.
    Write,

    /// Begin a loop at the current cell.
    BeginLoop,

    /// End the current loop.
    EndLoop,
}

/// Convert a language's lexemes into tokens.
pub trait Lexer {

    /// Convert the ``input`` string into a list of tokens.
    fn tokenize(&self) -> Vec<Token>;
}
