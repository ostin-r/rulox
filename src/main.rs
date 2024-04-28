use std::env;

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
    print!("run_file called");
}

fn run_prompt() {}

