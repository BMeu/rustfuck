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
