pub use std::env;
pub use std::fs;
pub use std::io;
pub use std::io::Write;
pub use std::process;
pub mod scanner;
pub mod tokens;

use scanner::Scanner;


/// Runs a source file
/// # Arguments
///
/// * `file_name` - path to the file
/// * `had_error` - weather there is an error to exit
///
/// # Errors
///
/// Returns an `io::Error` on unsucessful file reading
pub fn run_file(file_name: &String, had_error: bool) -> Result<(), io::Error> {
    let file_content = fs::read_to_string(file_name)?;
    run(&file_content);

    if had_error {
        process::exit(65)
    }
    Ok(())
}

/// Runs a command 
/// # Arguments
///
/// * `void` 
///
/// # Errors
/// Returns either a `flush` error or an `io::Error`
pub fn run_prompt() {
    let mut input_buffer: String = String::new();
    loop {
        input_buffer.clear();
        print!("> ");

         match io::stdout().flush() {
            Ok(n) => n,
            Err(e) => {
                eprint!("{e}");
                process::exit(66);
            }

        }
        let n = match io::stdin().read_line(&mut input_buffer) {
            Ok(n) => n,
            Err(e) => {
                eprintln!("{e}");
                process::exit(67);
            }
        };
        if n == 0 {
            break;
        }
        let input_buffer = input_buffer.trim();
        if input_buffer == "quit" || input_buffer == "exit" || input_buffer.is_empty() {
            break;
        }
        run(input_buffer);
    }
}

pub fn run(file_content_source: &str) {
    let mut scanner = Scanner::new(file_content_source);
    let tokens  =  Scanner::scan_tokens(&mut scanner);

    for token in tokens {
        println!("line : {:<4} lexeme : {:<30} token_emitted :  {:<10}", token.line, token.lexeme,token.token_type);
    }
}

pub fn err(line: usize, message: &str) {
    report(line, "", message);
}

pub fn report(line: usize, where_: &str, message: &str) {
    eprintln!("[line {line}] Error {where_}: {message}");
}
