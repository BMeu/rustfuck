// Copyright 2018 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! The binary to run the ``rustfuck`` library.

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

extern crate rustfuck_lib;

use rustfuck_lib::Compiler;

/// The classic "Hello, world!" program, written in Brainfuck.
static PROGRAM: &'static str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

/// Compile the ``Hello world!`` source code and print the generated ``C`` code to ``STDOUT``.
fn main() {
    let compiler = Compiler::new();
    let program: String = compiler.compile(PROGRAM);
    println!("{}", program);
}
