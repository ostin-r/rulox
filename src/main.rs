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
    let tokens: Vec<token::Token> = scan_tokens(source);
    for token in tokens.iter() {
        println!("{:?}", token);
    }
}

fn scan_tokens(contents: String) -> Vec<token::Token> {
    // let start = 0;
    // let current = 0;
    let mut line = 1;
    let mut tokens: Vec<token::Token> = vec![];

    let mut add_token = |token_type, lexeme, line| {
        tokens.push(token::Token{token_type, lexeme, line});
    };

    let mut iter = contents.chars().peekable();
    while let Some(c) = iter.next() {
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
              if let Some(next) = iter.peek() {
                   if *next == '=' {
                       add_token(token::TokenType::BangEqual, c.to_string(), line);
                       iter.next();
                   } else {
                       add_token(token::TokenType::Bang, c.to_string(), line);
                   }
              }
          },
          '=' => {
              if let Some(next) = iter.peek() {
                   if *next == '=' {
                       add_token(token::TokenType::EqualEqual, c.to_string(), line);
                       iter.next();
                   } else {
                       add_token(token::TokenType::Equal, c.to_string(), line);
                   }
              }
          },
          '<' => {
              if let Some(next) = iter.peek() {
                   if *next == '=' {
                       add_token(token::TokenType::LessEqual, c.to_string(), line);
                   } else {
                       add_token(token::TokenType::Less, c.to_string(), line);
                   }
              }
          },
          '>' => {
              if let Some(next) = iter.peek() {
                   if *next == '=' {
                       add_token(token::TokenType::GreaterEqual, c.to_string(), line);
                       iter.next();
                   } else {
                       add_token(token::TokenType::Greater, c.to_string(), line);
                       iter.next();
                   }
              }
          },
          '/' => {
              if let Some(next) = iter.peek() {
                  if *next == '/' {
                      // for a comment, just consume the rest of the line
                      iter.next();
                      while let Some(continued_char) = iter.peek() {
                          if *continued_char == '\n' {
                              // note that newlines are not consumed,
                              // they are parsed later to increment the current line number
                              break;
                          }
                          iter.next();
                      } 
                  } else {
                       add_token(token::TokenType::Slash, c.to_string(), line);
                  }
              }
          },
          ' ' => (),
          '\r' => (),
          '\t' => (),
          '\n' => line += 1,
          '"' => {
              // String literals
              // Escape characters not (yet?) supported
              let mut literal_string = String::new();
              while let Some(continued_char) = iter.peek() {
                  if *continued_char == '"' {
                      iter.next();
                      break;
                  }
                  if *continued_char == '\n' {
                      line += 1;
                  }
                  literal_string.push(*continued_char);
                  iter.next();
              }
              if let None = iter.peek() {
                  // End of file reached
                  // Note that anoter possible failure occurs when one string literal is not
                  // terminated, but more string literals appear throughout the file.
                  // This will result in jumbled tokens.  Not sure how to handle this currently
                  report_error(line, "Unterminated string literal");
              } else {
                  add_token(token::TokenType::String, literal_string, line);
              }
          },
          _ => report_error(line, "Unexpected character")
        };
    }

    tokens
} 


fn report_error(line: u32, message: &str) {
    println!("[line {line}] Error: {message}");
}

