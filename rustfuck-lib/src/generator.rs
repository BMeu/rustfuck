// Copyright 2018 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Implementations of a generator for creating ``C`` code from intermediate compile results.

use lexer::Token;

/// A generator for creating ``C`` code from intermediate compile results.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Generator {
    indentation_level: u32,
}

impl Generator {
    /// Initialize a new generator.
    pub fn new() -> Generator {
        Generator {
            indentation_level: 1,
        }
    }

    /// Generate the ``C`` code from the given list of ``tokens``, using the specified ``template``.
    pub fn generate(&mut self, template: &str, tokens: &[Token]) -> String {
        let mut output = String::from(template);

        for &token in tokens {
            match token {
                Token::Add => {
                    // Increment the value at the current cell.
                    output.push_str(&self.indent("(*ptr)++;\n"));
                },
                Token::Sub => {
                    // Decrement the value at the current cell.
                    output.push_str(&self.indent("(*ptr)--;\n"));
                },
                Token::Right => {
                    // Go to the right neighbor of the current cell.
                    output.push_str(&self.indent("ptr++;\n"));
                },
                Token::Left => {
                    // Go to the left neighbor of the current cell.
                    output.push_str(&self.indent("ptr--;\n"));
                },
                Token::Read => {
                    // Read a single character into the current cell.
                    output.push_str(&self.indent("*ptr = getchar();\n"));
                },
                Token::Write => {
                    // Print the character of the current cell.
                    output.push_str(&self.indent("putchar(*ptr);\n"));
                },
                Token::BeginLoop => {
                    // Begin a loop at the current cell.
                    output.push_str(&self.indent("while (*ptr) {\n"));
                    self.indentation_level += 1;
                },
                Token::EndLoop => {
                    // Close the current loop.
                    self.indentation_level -= 1;
                    output.push_str(&self.indent("}\n"));
                },
            }
        }

        // Add a closing bracket and a newline to the end of the output.
        output.push_str("}\n");

        output
    }

    /// Indent the given ``line`` respective to the current indentation level.
    fn indent(&self, line: &str) -> String {
        let mut indentation = String::new();
        for _ in 0..self.indentation_level {
            indentation.push_str("    ");
        }

        indentation.push_str(line);
        indentation
    }
}
