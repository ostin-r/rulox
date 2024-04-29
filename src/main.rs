use std::env;
use std::fs;

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
    let contents = fs::read(path).expect("Failed to read the provided file");
    run(contents);
}

fn run_prompt() {
    // todo: prompt user for file name >
}

fn run(_source: Vec<u8>) {
    print!("success");
    // todo: scan for tokens and print them out
}

fn report_error(line: u32, message: &str) {
    println!("[line {line}] Error: {message}");
}

