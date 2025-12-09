# rust-lexer

This is a simple lexer written in Rust. It takes source code as input and produces a list of tokens. The project is inspired by basic lexer implementations used in programming language interpreters.

## How to run

1. Install Rust and Cargo.
2. Clone this repository.
3. Open a terminal in the project directory.

Run the program with:

cargo run

To run the lexer on a specific file:

cargo run -- path/to/file

Replace path/to/file with the file you want to tokenize.

## Project structure

src/ contains the Rust source code.
Cargo.toml contains project information.

## Description

The lexer reads characters from the input and produces tokens such as identifiers, numbers, strings, operators, and punctuation. This project is meant for learning and experimentation.

## License

You can use or modify this project freely.

