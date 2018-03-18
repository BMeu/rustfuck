// Copyright 2018 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//!

#![warn(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unused_extern_crates, unused_import_braces, unused_qualifications, unused_results)]
#![cfg_attr(feature = "cargo-clippy", warn(cast_possible_truncation, cast_possible_wrap, cast_precision_loss,
                                      cast_sign_loss, empty_enum, enum_glob_use, if_not_else,
                                      items_after_statements, missing_docs_in_private_items, nonminimal_bool,
                                      pub_enum_variant_names, similar_names, single_match_else,
                                      stutter, used_underscore_binding, use_debug, wrong_self_convention,
                                      wrong_pub_self_convention))]

/// The ``Brainfuck`` tokens.
#[derive(Debug, PartialEq, Copy, Clone)]
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

use self::Token::*;

/// Create the correct indentation for the given ``level``.
fn indent(level: u32) -> String {
    let mut indentation = String::new();
    for _ in 0..level {
        indentation.push_str("    ");
    }

    indentation
}

/// Create the ``C`` program from the list of ``tokens``.
pub fn generate(tokens: &[Token]) -> String {
    let mut output = String::from(include_str!("../resources/preface.c"));
    let mut indentation_level: u32 = 1;

    for &token in tokens {
        match token {
            Add => {
                // Increment the value at the current cell.
                output.push_str(&indent(indentation_level));
                output.push_str("(*ptr)++;\n");
            },
            Sub => {
                // Decrement the value at the current cell.
                output.push_str(&indent(indentation_level));
                output.push_str("(*ptr)--;\n");
            },
            Right => {
                // Go to the right neighbor of the current cell.
                output.push_str(&indent(indentation_level));
                output.push_str("ptr++;\n");
            },
            Left => {
                // Go to the left neighbor of the current cell.
                output.push_str(&indent(indentation_level));
                output.push_str("ptr--;\n");
            },
            Read => {
                // Read a single character into the current cell.
                output.push_str(&indent(indentation_level));
                output.push_str("*ptr = getchar();\n");
            },
            Write => {
                // Print the character of the current cell.
                output.push_str(&indent(indentation_level));
                output.push_str("putchar(*ptr);\n");
            },
            BeginLoop => {
                // Begin a loop at the current cell.
                output.push_str(&indent(indentation_level));
                output.push_str("while (*ptr) {\n");
                indentation_level += 1;
            },
            EndLoop => {
                // Close the current loop.
                indentation_level -= 1;
                output.push_str(&indent(indentation_level));
                output.push_str("}\n");
            },
        }
    }

    // Add a closing bracket and a newline to the end of the output.
    output.push_str("}\n");

    output
}

/// Translate the ``input`` string into a list of tokens.
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    for char in input.chars() {
        match char {
            '+' => tokens.push(Add),
            '-' => tokens.push(Sub),
            '>' => tokens.push(Right),
            '<' => tokens.push(Left),
            ',' => tokens.push(Read),
            '.' => tokens.push(Write),
            '[' => tokens.push(BeginLoop),
            ']' => tokens.push(EndLoop),
            _ => {},
        }
    }

    tokens
}
