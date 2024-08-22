use std::env;
use std::fs;
mod token;


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
    let tokens: Vec<String> = scan_tokens(source);
    for token in tokens.iter() {
        println!("{}", token);
    }
}

fn scan_tokens(contents: String) -> Vec<String> {
    let start = 0;
    let current = 0;
    let line = 1;
    let mut tokens: Vec<token::Token> = vec![];

    let mut add_token = |token_type, lexeme, line| {
        tokens.push(token::Token{token_type, lexeme, line});
    };

    for c in contents.chars() {
        match c {
          '(' => add_token(token::TokenType::LeftParen, c.to_string(), line),
          ')' => add_token(token::TokenType::RightParen, c.to_string(), line),
          '{' => add_token(token::TokenType::LeftBrace, c.to_string(), line),
          '}' => add_token(token::TokenType::RightBrace, c.to_string(), line),
          ',' => add_token(token::TokenType::Comma, c.to_string(), line),
          '.' => add_token(token::TokenType::Dot, c.to_string(), line),
          '-' => add_token(token::TokenType::Minus, c.to_string(), line),
          '+' => add_token(token::TokenType::Plus, c.to_string(), line),
          ';' => add_token(token::TokenType::Semicolon, c.to_string(), line),
          '*' => add_token(token::TokenType::Star, c.to_string(), line), 
          // '!' => {
          //     if c.next() == '=' {
          //         // todo: how to look at the next item in a loop??
          //         let lexeme = c + c.next();
          //         add_token(token::TokenType::BangEqual, lexeme, line);
          //     } else {
          //     }
          // },
          // '=' => add_token(match('=') ? EQUAL_EQUAL : EQUAL),
          // '<' => add_token(match('=') ? LESS_EQUAL : LESS),
          // '>' => add_token(match('=') ? GREATER_EQUAL : GREATER);
          _ => report_error(line, "Unexpected character")
        };
    }

    vec!["todo".to_string()]
} 


fn report_error(line: u32, message: &str) {
    println!("[line {line}] Error: {message}");
}

