# BrainRust

A BrainFuck compiler in Rust.

Why? Why not? Seemed like an interesting evening project.

## What are the next steps?

Probably to never revisit this ever again. The goal was to spend couple of
hours on this, until I could parse my horrible [Project Euler 1 solution in
BF](https://github.com/ikornaselur/brainfuck-project-euler-01) from _years_ ago
and.. it works!

Now you might ask.. why not compile to WASM and have available in browser?
Well... we'll see

## Usage

```
❯ ./brainrust --help
BrainRust - A BrainFuck interpreter, written in Rust (everyone loves to hear that)

Usage: brainrust <INPUT>

Arguments:
  <INPUT>  Input file

Options:
  -h, --help     Print help
  -V, --version  Print version

❯ cat hello_world.bf
++++++++++[>+>+++>+++++++>++++++++++<<<<-]>>>++.>+.+++++++..+++.<<++.>+++++++++++++++.>.+++.------.--------.<<+.<.

❯ ./brainrust hello_world.bf
Hello World!
```
