pub use std::env;
pub use std::fs;
pub use std::io;
pub use std::io::Write;
pub use std::process;
pub mod scanner;
pub mod tokens;

use scanner::Scanner;

pub fn run_file(file_name: &String, had_error: bool) -> Result<(), io::Error> {
    let file_content = fs::read_to_string(file_name)?;
    run(&file_content);

    if had_error {
        process::exit(65)
    };
    Ok(())
}

pub fn run_prompt() {
    let mut input_buffer: String = String::new();
    loop {
        input_buffer.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        let n = io::stdin().read_line(&mut input_buffer).unwrap();
        if n == 0 {
            break;
        }
        let input_buffer = input_buffer.trim();
        if input_buffer == "quit"
            || input_buffer == "exit"
            || input_buffer.is_empty()
        {
            break;
        }
        run(input_buffer);
    }
}

pub fn run(file_content_source: &str) {
    for words in file_content_source.split_whitespace() {
        let mut scanner = Scanner::new(words.to_string());
        println!("{:?}", scanner);
        Scanner::scan_tokens(&mut scanner);
    }
    println!();
}

pub fn err(line: i64, message: &str) {
    report(line, "", message);
}

pub fn report(line: i64, where_: &str, message: &str) {
    eprintln!("[line {line}] Error {where_}: {message}");
}
