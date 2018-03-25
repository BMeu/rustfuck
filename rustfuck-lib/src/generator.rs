// Copyright 2018 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Implementations of a generator for creating `C` code from intermediate compile results.

use lexer::Token;

/// A generator for creating `C` code from intermediate compile results.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Generator {

    /// The current level of indentation.
    indentation_level: u32,
}

impl Generator {
    /// Initialize a new generator.
    pub fn new() -> Generator {
        Generator {
            indentation_level: 1,
        }
    }

    /// Generate the `C` code from the given list of `tokens`, using the specified `template`.
    pub fn generate(&mut self, template: &str, tokens: &[Token]) -> String {
        let mut output = String::from(template);

        for token in tokens {
            match *token {
                Token::Add(_, _) => {
                    // Increment the value at the current cell.
                    output.push_str(&self.indent("(*ptr)++;\n"));
                },
                Token::Sub(_, _) => {
                    // Decrement the value at the current cell.
                    output.push_str(&self.indent("(*ptr)--;\n"));
                },
                Token::Right(_, _) => {
                    // Go to the right neighbor of the current cell.
                    output.push_str(&self.indent("ptr++;\n"));
                },
                Token::Left(_, _) => {
                    // Go to the left neighbor of the current cell.
                    output.push_str(&self.indent("ptr--;\n"));
                },
                Token::Read(_, _) => {
                    // Read a single character into the current cell.
                    output.push_str(&self.indent("*ptr = getchar();\n"));
                },
                Token::Write(_, _) => {
                    // Print the character of the current cell.
                    output.push_str(&self.indent("putchar(*ptr);\n"));
                },
                Token::BeginLoop(_, _) => {
                    // Begin a loop at the current cell.
                    output.push_str(&self.indent("while (*ptr) {\n"));
                    self.indentation_level += 1;
                },
                Token::EndLoop(_, _) => {
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

    /// Indent the given `line` respective to the current indentation level.
    fn indent(&self, line: &str) -> String {
        let mut indentation = String::new();
        for _ in 0..self.indentation_level {
            indentation.push_str("    ");
        }

        indentation.push_str(line);
        indentation
    }
}

#[cfg(test)]
mod tests {

    use Generator;
    use MetaData;
    use Token;

    #[test]
    fn test_new() {
        let generator = Generator::new();
        assert_eq!(generator, Generator { indentation_level: 1 })
    }

    #[test]
    fn test_generate_hello_world() {
        let template: &str = "{\n";
        let tokens: Vec<Token> = vec![
            Token::Add(String::from("+"), MetaData{ lineno: 1, position: 1 }),
            Token::Right(String::from(">"), MetaData{ lineno: 1, position: 2 }),
            Token::Left(String::from("<"), MetaData{ lineno: 1, position: 3 }),
            Token::BeginLoop(String::from("["), MetaData{ lineno: 1, position: 4 }),
            Token::Read(String::from(","), MetaData{ lineno: 1, position: 5 }),
            Token::Sub(String::from("-"), MetaData{ lineno: 1, position: 6 }),
            Token::EndLoop(String::from("]"), MetaData{ lineno: 1, position: 7 }),
            Token::Write(String::from("."), MetaData{ lineno: 1, position: 8 }),
        ];
        let expected = String::from(include_str!("../resources/tests/generator_test_generate.c"));

        let mut generator = Generator::new();
        let generated: String = generator.generate(template, &tokens);
        assert_eq!(generated, expected);
    }

    #[test]
    fn test_indent_level_0() {
        let line: &str = "fn test_indent_level_0() {}";
        let mut generator = Generator::new();
        generator.indentation_level = 0;

        assert_eq!(generator.indent(line), String::from(line));
    }

    #[test]
    fn test_indent_level_1() {
        let line: &str = "fn test_indent_level_1() {}";
        let generator = Generator::new();

        assert_eq!(generator.indent(line), String::from("    ") + line);
    }

    #[test]
    fn test_indent_level_4() {
        let line: &str = "fn test_indent_level_4() {}";
        let mut generator = Generator::new();
        generator.indentation_level = 4;

        assert_eq!(generator.indent(line), String::from("                ") + line);
    }
}
