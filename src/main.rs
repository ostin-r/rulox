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

    if !contents.is_ascii() {
        panic!("Provided file contains non-ASCII characters.  Only ASCII is supported")
    }

    let byte_contents = contents.as_bytes();

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
          '!' => {
              let next = byte_contents[current] as char;  // note: using as_bytes means that this
                                                                // language is only compatible with ASCII
                                                                // characters.  Further work will need to
                                                                // be done to allow UTF8
              if next == '=' {
                  // todo: add '!=' instead of just '!', more work needs to be done on lexemes in
                  // general though
                  add_token(token::TokenType::BangEqual, c.to_string(), line);
              } else {
                  add_token(token::TokenType::Bang, c.to_string(), line);
              }
          },
          '=' => {
              let next = byte_contents[current] as char;
              if next == '=' {
                  add_token(token::TokenType::EqualEqual, c.to_string(), line)
              } else {
                  add_token(token::TokenType::Equal, c.to_string(), line)
              }
          },
          '<' => {
              let next = byte_contents[current] as char;
              if next == '=' {
                  add_token(token::TokenType::LessEqual, c.to_string(), line)
              } else {
                  add_token(token::TokenType::Less, c.to_string(), line)
              }
          },
          '>' => {
              let next = byte_contents[current] as char;
              if next == '=' {
                  add_token(token::TokenType::GreaterEqual, c.to_string(), line)
              } else {
                  add_token(token::TokenType::Greater, c.to_string(), line)
              }
          },
          _ => report_error(line, "Unexpected character")
        };
    }

    vec!["todo".to_string()]
} 


fn report_error(line: u32, message: &str) {
    println!("[line {line}] Error: {message}");
}

