#!/usr/bin/env bash
# Compile all brainfuck examples in the examples folder to C, then compile with GCC and execute that program.

for file in examples/*.bf; do
    filename=$(basename ${file})
    filestem="${filename%.*}"

    brainfuck_file=${file}
    c_file="${filestem}.c"
    program=${filestem}

    cargo run --release -- -o ${c_file} ${brainfuck_file} &&
    gcc -O3 -o ${program} ${c_file} &&
    ./${program}

    rm -f ${c_file}
    rm -f ${program}
done;
