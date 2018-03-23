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

#[macro_use]
extern crate clap;
extern crate rustfuck_lib;

pub mod cli;
pub mod quit;

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Error as IOError;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use clap::App;
use clap::ArgMatches;
use rustfuck_lib::Compiler;
use rustfuck_lib::Error;

/// Compile the ``Hello world!`` source code and print the generated ``C`` code to ``STDOUT``.
fn main() {
    // Get the configuration.
    let app: App = cli::setup();
    let arg_matches: ArgMatches = app.get_matches();
    let (input_path, output_path): (PathBuf, PathBuf) = cli::get_arguments(&arg_matches);

    // Load the source.
    println!("Reading input from {path}.", path=input_path.display());
    let input_file: File = match File::open(input_path) {
        Ok(file) => file,
        Err(error) => quit::fail_from_error(Error::from(error)),
    };
    let mut input_reader: BufReader<File> = BufReader::new(input_file);
    let mut program = String::new();
    match input_reader.read_to_string(&mut program) {
        Ok(_) => {},
        Err(error) => quit::fail_from_error(Error::from(error)),
    }

    // Compile the program.
    println!("Compiling.");
    let compiler = Compiler::new();
    let program: String = compiler.compile(&program);

    // Write the output.
    println!("Writing output to {path}.", path=output_path.display());
    let output_file: File = match File::create(output_path) {
        Ok(file) => file,
        Err(error) => quit::fail_from_error(Error::from(error)),
    };
    let mut output_writer: BufWriter<File> = BufWriter::new(output_file);
    let write_result: Result<(), IOError> = write!(output_writer, "{}", program);
    let flush_result: Result<(), IOError> = output_writer.flush();

    if let Err(error) = write_result {
        quit::fail_from_error(Error::from(error));
    }

    if let Err(error) = flush_result {
        quit::fail_from_error(Error::from(error));
    }

    println!("[SUCCESS]");
    quit::succeed();
}
