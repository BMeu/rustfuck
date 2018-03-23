// Copyright 2018 Bastian Meyer
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0> or the
// MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be copied,
// modified, or distributed except according to those terms.

//! Functions for setting up and working with the command-line interface of ``rustfuck``.

use std::path::PathBuf;

use clap::App;
use clap::Arg;
use clap::ArgMatches;

/// Define the command-line arguments on the ``clap`` application.
pub fn setup<'a, 'b>() -> App<'a, 'b> {
    app_from_crate!()
        .help_message("Show this help message and exit.")
        .arg(Arg::with_name("INPUT")
            .help("The Brainfuck source file.")
            .required(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .takes_value(true)
            .value_name("OUTPUT")
            .help("The generated C file. [default: ./<INPUT>.c]"))
}

/// Get the ``(input_file, output_file)``.
///
/// # Example
///
/// ```rust
/// use clap::App;
/// use cli::setup;
/// use cli::get_arguments;
///
/// let app = setup();
/// let arg_matches = app.get_matches();
/// let configuration = get_arguments(arg_matches);
/// ```
pub fn get_arguments(arg_matches: &ArgMatches) -> (PathBuf, PathBuf) {
    // Get the input source.
    let input: PathBuf = match arg_matches.value_of("INPUT") {
        Some(input) => PathBuf::from(input),
        None => unreachable!("No input source specified."),
    };

    // Get the output file. If it is not specified, default to ./<INPUT>.c
    let output: PathBuf = match arg_matches.value_of("output") {
        Some(output) => PathBuf::from(output),
        None => {
            match input.file_stem() {
                Some(filestem) => {
                    // Use the input file's stem and add the C extension.
                    let mut output: PathBuf = PathBuf::from(filestem);
                    let _ = output.set_extension("c");
                    output
                },
                // If everything else fails, default to out.c.
                None => PathBuf::from(String::from("out.c"))
            }
        }
    };

    (input, output)
}
