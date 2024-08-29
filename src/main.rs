mod token;
mod scanner;
use std::env;
use std::fs;
use crate::scanner::scan_tokens;


fn main() {
    let args: Vec<String> = env::args().collect();
   if args.len() > 2 {
        println!("Failed: invalid amount of arguments");
    } else if args.len() == 2 {
        let filepath: &str = &args[1];
        run_file(filepath);
    } else if args.len() == 1 {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let contents: String = fs::read_to_string(path).expect("Failed to read the provided file");
    run(contents);
}

fn run_prompt() {
    // todo: prompt user for file name >
}

fn run(source: String) {
    let tokens: Vec<token::Token> = scan_tokens(source);
    for token in tokens.iter() {
        println!("{:?}", token);
    }
}

fn report_error(line: u32, message: &str) {
    println!("[line {line}] Error: {message}");
}

