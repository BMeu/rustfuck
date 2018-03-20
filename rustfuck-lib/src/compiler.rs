// Copyright 2018 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Implementations of the ``Brainfuck`` compiler.

use Generator;
use Lexer;
use language::Brainfuck;

static PREFACE: &'static str = include_str!("../resources/preface.c");

/// The ``Brainfuck`` compiler.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Compiler;

impl Compiler {
    /// Initialize a ``Brainfuck`` compiler.
    pub fn new() -> Compiler {
        Compiler{}
    }

    /// Compile the given ``Brainfuck`` ``source`` code.
    pub fn compile(&self, source: &str) -> String {
        let program = Brainfuck::new(source);
        let tokens = program.tokenize();

        let mut generator = Generator::new();
        generator.generate(PREFACE, &tokens)
    }
}

#[cfg(test)]
mod tests {

    use Compiler;

    #[test]
    fn test_new() {
        let compiler = Compiler::new();
        assert_eq!(compiler, Compiler{});
    }

    #[test]
    fn test_compile() {
        let compiler = Compiler::new();
        let source: &str = include_str!("../resources/tests/hello.bf");
        let expected = String::from(include_str!("../resources/tests/hello.c"));

        let compiled: String = compiler.compile(source);
        assert_eq!(compiled, expected);
    }
}
