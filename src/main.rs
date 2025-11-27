use std::io;
use std::env;
use std::io::Write;
use std::process;
use std::fs;

fn main ( ) -> Result<(),io::Error> {
    let args  : Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Usages : rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[0])?;
    } else {
        run_prompt();
    }
    Ok(())
}


fn run_file(filename : &String) -> Result<(),io::Error> {
    let file_content  = fs::read_to_string(filename)?;
    println!("{filename}");
    println!("Read {file_content}");
    run(&file_content);
    todo!("Add functionality");
}


fn run_prompt( ) {
    let mut expression_buffer : String = String::new(); 
    loop {
        expression_buffer.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        let n  = io::stdin().read_line(&mut expression_buffer).unwrap();
        if n == 0 {
            break;
        }
        if expression_buffer.trim() == "quit".to_string() || expression_buffer.trim() == "exit".to_string(){
            break;
        }  
    }
    todo!("Add functionality");
}


fn run (_file_content : &String)  {
    todo!("Add functionality");
}
