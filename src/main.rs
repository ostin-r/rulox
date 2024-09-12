mod token;
mod scanner;
mod parser;
use std::env;
use std::fs;
use crate::scanner::scan_tokens;
use crate::parser::Parser;


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
    panic!("not implemented");
}

fn run(source: String) {
    let tokens: Vec<token::Token> = scan_tokens(source);
    for token in tokens.iter() {
        println!("{:?}", token);
    }
    let tokens_iter = tokens.iter().peekable();
    let parser = Parser { tokens: tokens_iter, token_vec: tokens, current: 0 };
}

fn report_error(line: u32, message: &str) {
    println!("[line {line}] Error: {message}");
}

