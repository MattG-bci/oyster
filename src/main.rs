use std;
use std::io::Error;
use std::process::exit;

fn run_file(path: &str) -> Result<String, Error> {
    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}

fn run_prompt() {}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).unwrap();
    } else {
        run_prompt();
    }
}
