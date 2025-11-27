use rlox::*;

fn main ( ) -> Result<(),rlox::io::Error> {
    let args  : Vec<String> = rlox::env::args().collect();
    if args.is_empty() {
        println!("Usages : rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        rlox::run_file(&args[1])?;
    } else {
        rlox::run_prompt();
    }
    Ok(())
}


