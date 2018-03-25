// Copyright 2018 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Implementation of the `Brainfuck` programming language.

use Lexer;
use MetaData;
use Token;

/// The esoteric programming language `Brainfuck`, created in 1993 by Urban Müller.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Brainfuck {

    /// The program source code.
    source: String,
}

impl Brainfuck {
    /// Initialize a new `Brainfuck` program from the given `source`.
    pub fn new(source: &str) -> Brainfuck {
        Brainfuck {
            source: String::from(source),
        }
    }
}

impl Lexer for Brainfuck {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();

        // Lines and cursor positions start counting at 1, so use `zip(1..)` instead of enumerate which would start
        // counting at 0.
        for (line, lineno) in self.source.lines().zip(1..) {
            for (char, position) in line.chars().zip(1..) {
                match char {
                    '+' => tokens.push(Token::Add(String::from("+"), MetaData { lineno, position })),
                    '-' => tokens.push(Token::Sub(String::from("-"), MetaData { lineno, position })),
                    '>' => tokens.push(Token::Right(String::from(">"), MetaData { lineno, position })),
                    '<' => tokens.push(Token::Left(String::from("<"), MetaData { lineno, position })),
                    ',' => tokens.push(Token::Read(String::from(","), MetaData { lineno, position })),
                    '.' => tokens.push(Token::Write(String::from("."), MetaData { lineno, position })),
                    '[' => tokens.push(Token::BeginLoop(String::from("["), MetaData { lineno, position })),
                    ']' => tokens.push(Token::EndLoop(String::from("]"), MetaData { lineno, position })),
                    _ => {},
                }
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use Lexer;
    use MetaData;
    use Token;
    use language::Brainfuck;

    #[test]
    fn test_new() {
        let source: &str = "+-><,.[]";
        let bf = Brainfuck::new(source);

        assert_eq!(bf, Brainfuck { source: String::from(source) });
    }

    #[test]
    fn test_tokenize_add() {
        let source: &str = "+";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::Add(String::from("+"), MetaData{ lineno: 1, position: 1 }),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_sub() {
        let source: &str = "-";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::Sub(String::from("-"), MetaData{ lineno: 1, position: 1 }),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_right() {
        let source: &str = ">";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::Right(String::from(">"), MetaData{ lineno: 1, position: 1 }),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_left() {
        let source: &str = "<";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::Left(String::from("<"), MetaData{ lineno: 1, position: 1 }),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_read() {
        let source: &str = ",";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::Read(String::from(","), MetaData{ lineno: 1, position: 1 }),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_write() {
        let source: &str = ".";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::Write(String::from("."), MetaData{ lineno: 1, position: 1 }),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_loop_begin() {
        let source: &str = "[";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::BeginLoop(String::from("["), MetaData{ lineno: 1, position: 1 }),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_loop_end() {
        let source: &str = "]";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::EndLoop(String::from("]"), MetaData{ lineno: 1, position: 1 }),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize() {
        let source: &str = "+-><No Brainfuck\nsymbols,.[]";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::Add(String::from("+"), MetaData{ lineno: 1, position: 1 }),
            Token::Sub(String::from("-"), MetaData{ lineno: 1, position: 2 }),
            Token::Right(String::from(">"), MetaData{ lineno: 1, position: 3 }),
            Token::Left(String::from("<"), MetaData{ lineno: 1, position: 4 }),
            Token::Read(String::from(","), MetaData{ lineno: 2, position: 8 }),
            Token::Write(String::from("."), MetaData{ lineno: 2, position: 9 }),
            Token::BeginLoop(String::from("["), MetaData{ lineno: 2, position: 10 }),
            Token::EndLoop(String::from("]"), MetaData{ lineno: 2, position: 11 }),
        ];
        assert_eq!(tokens, expected);
    }
}
