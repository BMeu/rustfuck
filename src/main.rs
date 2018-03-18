// "Hello, world!"
static PROGRAM: &'static str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

fn main() {
    let tokens = tokenize(PROGRAM);
    let generated_code = generate(&tokens);
    println!("{}", generated_code);
}

/// The ``Brainfuck`` tokens.
#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Add,            // +
    Sub,            // -
    Right,          // >
    Left,           // <
    Read,           // ,
    Write,          // .
    BeginLoop,      // [
    EndLoop,        // ]
}

use self::Token::*;

/// Create the correct indentation for the given ``level``.
fn indent(level: u32) -> String {
    let mut indentation = String::new();
    for _ in 0..level {
        indentation.push_str("    ");
    }

    indentation
}

/// Create the ``C`` program from the list of ``tokens``.
fn generate(tokens: &[Token]) -> String {
    let mut output = String::from(include_str!("../preface.c"));
    let mut indentation_level: u32 = 1;

    for &token in tokens {
        match token {
            Add => {
                // Increment the value at the current cell.
                output.push_str(&indent(indentation_level));
                output.push_str("(*ptr)++;\n");
            },
            Sub => {
                // Decrement the value at the current cell.
                output.push_str(&indent(indentation_level));
                output.push_str("(*ptr)--;\n");
            },
            Right => {
                // Go to the right neighbor of the current cell.
                output.push_str(&indent(indentation_level));
                output.push_str("ptr++;\n");
            },
            Left => {
                // Go to the left neighbor of the current cell.
                output.push_str(&indent(indentation_level));
                output.push_str("ptr--;\n");
            },
            Read => {
                // Read a single character into the current cell.
                output.push_str(&indent(indentation_level));
                output.push_str("*ptr = getchar();\n");
            },
            Write => {
                // Print the character of the current cell.
                output.push_str(&indent(indentation_level));
                output.push_str("putchar(*ptr);\n");
            },
            BeginLoop => {
                // Begin a loop at the current cell.
                output.push_str(&indent(indentation_level));
                output.push_str("while (*ptr) {\n");
                indentation_level += 1;
            },
            EndLoop => {
                // Close a loop.
                indentation_level -= 1;
                output.push_str(&indent(indentation_level));
                output.push_str("}\n");
            },
        }
    }

    // Add a closing bracket and a newline to the end of the output.
    output.push_str("}\n");

    output
}

/// Translate the ``input`` string into a list of tokens.
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    let mut chars = input.chars();
    while let Some(char) = chars.next() {
        match char {
            '+' => tokens.push(Add),
            '-' => tokens.push(Sub),
            '>' => tokens.push(Right),
            '<' => tokens.push(Left),
            ',' => tokens.push(Read),
            '.' => tokens.push(Write),
            '[' => tokens.push(BeginLoop),
            ']' => tokens.push(EndLoop),
            _ => {},
        }
    }

    tokens
}