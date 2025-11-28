use rlox::process;

fn main ( ) -> Result<(),rlox::io::Error> {
    let args  : Vec<String> = rlox::env::args().collect();
    let had_error = false;
    if args.is_empty() {
        println!("Usages : rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        rlox::run_file(&args[1],had_error)?;
    } else {
        rlox::run_prompt();
    }
    Ok(())
}


