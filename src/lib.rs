pub use std::io;
pub use std::env;
pub use std::io::Write;
pub use std::process;
pub use std::fs;
pub mod scanner;
pub mod tokens;


use scanner::Scanner;

pub fn run_file(file_name : &String,had_error : bool) -> Result<(),io::Error> {
    let file_content  = fs::read_to_string(file_name)?;
    run(&file_content);

    if had_error  {process::exit(65)};
    Ok(())
}


pub fn run_prompt( ) {
    let mut expression_buffer : String = String::new(); 
    loop {
        expression_buffer.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        let n  = io::stdin().read_line(&mut expression_buffer).unwrap();
        if n == 0 {
            break;
        }
        let expression_buffer = expression_buffer.trim();
        if expression_buffer == "quit" || expression_buffer == "exit" || expression_buffer.is_empty(){
            break;
        }  
        run(expression_buffer);
        
    }
}


pub fn run (file_content_source : &str)  {
    for words in file_content_source.split_whitespace()  {
        let mut scanner = Scanner::new(words.to_string());
        Scanner::scan_tokens(&mut scanner);
        println!("{:?}",scanner);
    }
    println!();
}

pub fn err(line: i64, message: &str) {
    report(line, "", message);
}

pub fn report(line: i64, where_: &str, message: &str) {
    eprintln!("[line {line}] Error {where_}: {message}");
}



