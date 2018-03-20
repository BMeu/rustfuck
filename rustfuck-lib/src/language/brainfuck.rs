// Copyright 2018 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Implementation of the ``Brainfuck`` programming language.

use Lexer;
use Token;

/// The esoteric programming language ``Brainfuck``, created in 1993 by Urban Müller.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Brainfuck {

    /// The program source code.
    source: String,
}

impl Brainfuck {
    /// Initialize a new ``Brainfuck`` program from the given ``source``.
    pub fn new(source: &str) -> Brainfuck {
        Brainfuck {
            source: String::from(source),
        }
    }
}

impl Lexer for Brainfuck {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();

        for char in self.source.chars() {
            match char {
                '+' => tokens.push(Token::Add),
                '-' => tokens.push(Token::Sub),
                '>' => tokens.push(Token::Right),
                '<' => tokens.push(Token::Left),
                ',' => tokens.push(Token::Read),
                '.' => tokens.push(Token::Write),
                '[' => tokens.push(Token::BeginLoop),
                ']' => tokens.push(Token::EndLoop),
                _ => {},
            }
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use Lexer;
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
            Token::Add,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_sub() {
        let source: &str = "-";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::Sub,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_right() {
        let source: &str = ">";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::Right,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_left() {
        let source: &str = "<";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::Left,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_read() {
        let source: &str = ",";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::Read,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_write() {
        let source: &str = ".";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::Write,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_loop_begin() {
        let source: &str = "[";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::BeginLoop,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_loop_end() {
        let source: &str = "]";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::EndLoop,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize() {
        let source: &str = "+-><No Brainfuck symbols,.[]";
        let bf = Brainfuck::new(source);

        let tokens: Vec<Token> = bf.tokenize();
        let expected: Vec<Token> = vec![
            Token::Add,
            Token::Sub,
            Token::Right,
            Token::Left,
            Token::Read,
            Token::Write,
            Token::BeginLoop,
            Token::EndLoop,
        ];
        assert_eq!(tokens, expected);
    }
}
