# rustfuck

Currently, a `Brainfuck` to `C` translator written in `Rust`. In the future, a `Brainfuck` interpreter written
in `Rust`.

[![Build Status on Travis](https://travis-ci.org/BMeu/rustfuck.svg?branch=master)](https://travis-ci.org/BMeu/rustfuck)
[![Build Status on AppVeyor](https://ci.appveyor.com/api/projects/status/2ur0b2bqvopb96y9?svg=true)](https://ci.appveyor.com/project/BMeu/rustfuck)
[![Codecov](https://codecov.io/gh/BMeu/rustfuck/branch/master/graph/badge.svg)](https://codecov.io/gh/BMeu/rustfuck)

## Usage

Translate from `Brainfuck` to `C`, and compile (on Linux, using `GCC`):

```bash
$ cargo run --release -- --output hello.c examples/hello.bf
$ gcc -O3 -o hello hello.c
$ ./hello
Hello World!
```

Run `cargo run --release -- --help` for more information on parameters.

## Acknowledgements & Sources

Idea and first implementation taken from Asmoaesl:

 * [Original code](https://gist.github.com/asmoaesl/7ee78589cfb770104edeaec710aa61f9)
 * [Blog post](https://medium.com/@CanHasCommunism/making-a-brainf-ck-to-c-compiler-in-rust-10f0c01a282d)

Java implementation by Daniel Harper:

 * [Blog post](https://medium.com/@djhworld/writing-a-brainfuck-compiler-in-java-706dfc5ba23b)

## License

`rustfuck` is licensed under either of
             
 * Apache License, Version 2.0, ([`LICENSE-APACHE`](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([`LICENSE-MIT`](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
