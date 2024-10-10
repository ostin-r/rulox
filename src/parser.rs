use crate::expr::Expr;
use crate::report_error;
use crate::token::{Token, TokenType};
use std::iter::Peekable;

// Lox uses recursive descent parsing, a predictive parsing method
//
// In order of lowest to highest precedence:
// Comma Expressions
// Ternary Expressions
// Comparison
// Addition
// Multiplication
// Unary
//

pub struct Parser<'a> {
    pub tokens: Peekable<std::slice::Iter<'a, Token>>,
    pub token_vec: Vec<Token>,
    pub current: usize,
}

impl Parser<'_> {
    pub fn parse(&mut self) -> Vec<Expr> {
        let mut statements: Vec<Expr> = Vec::new();
        while self.current + 1 <= self.tokens.len() {
            println!("parsing...");
            statements.push(self.expression());
        }
        statements
    }

    fn expression(&mut self) -> Expr {
        self.unary_reverse()
    }

    fn unary_reverse(&mut self) -> Expr {
        // used for comma expressions
        let expr = self.ternary();
        let unary_reverse_fields = [TokenType::Comma];
        if let Some(_value) = self
            .tokens
            .next_if(|x| unary_reverse_fields.contains(&x.token_type))
        {
            println!("parse: unary reverse");
            self.current += 1;
            let operator = self.token_vec[self.current].clone();
            return Expr::UnaryReverse {
                left: Box::new(expr),
                operator,
            };
        }
        expr
    }

    fn ternary(&mut self) -> Expr {
        let expr = self.equality();
        let ternary_fields = [TokenType::Question];
        if let Some(_value) = self
            .tokens
            .next_if(|x| ternary_fields.contains(&x.token_type))
        {
            println!("parse: ternary");
            self.current += 1;
            let if_true = self.equality();
            let middle_ternary_fields = [TokenType::Colon];
            if let Some(_value) = self
                .tokens
                .next_if(|x| middle_ternary_fields.contains(&x.token_type))
            {
                let if_false = self.equality();
                return Expr::Ternary {
                    condition: Box::new(expr),
                    if_true: Box::new(if_true),
                    if_false: Box::new(if_false),
                };
            }
            panic!("Ternary expression not terminated, expected ':' after ? operator");
        }
        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        let equality_fields = [TokenType::EqualEqual, TokenType::BangEqual];
        while let Some(_value) = self
            .tokens
            .next_if(|x| equality_fields.contains(&x.token_type))
        {
            println!("parse: equality");
            self.current += 1;
            let operator = self.previous_token();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        let comparison_fields = [
            TokenType::LessEqual,
            TokenType::Less,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ];
        while let Some(_value) = self
            .tokens
            .next_if(|x| comparison_fields.contains(&x.token_type))
        {
            println!("parse: comparison");
            self.current += 1;
            let operator = self.previous_token();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        let term_fields = [TokenType::Minus, TokenType::Plus];
        while let Some(_value) = self.tokens.next_if(|x| term_fields.contains(&x.token_type)) {
            println!("parse: term");
            self.current += 1;
            let operator = self.previous_token();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        let factor_fields = [TokenType::Slash, TokenType::Star];
        while let Some(_value) = self
            .tokens
            .next_if(|x| factor_fields.contains(&x.token_type))
        {
            println!("parse: factor");
            self.current += 1;
            let operator = self.previous_token();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        // todo: fix this, currently allows multiple leading negatives (unless caught at lexer?)
        let unary_fields = [TokenType::Bang, TokenType::Minus];
        if let Some(_value) = self
            .tokens
            .next_if(|x| unary_fields.contains(&x.token_type))
        {
            println!("parse: unary");
            self.current += 1;
            let operator = self.previous_token();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }
        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        // todo: resolve return value from this, implement indexing for Vec<Token>
        if let Some(_value) = self.tokens.next_if(|x| x.token_type == TokenType::False) {
            println!("parse: bool");
            self.current += 1;
            return Expr::Boolean(false);
        }
        if let Some(_value) = self.tokens.next_if(|x| x.token_type == TokenType::True) {
            println!("parse: bool");
            self.current += 1;
            return Expr::Boolean(true);
        }
        if let Some(_value) = self.tokens.next_if(|x| x.token_type == TokenType::Nil) {
            println!("parse: nil");
            self.current += 1;
            return Expr::Nil;
        }

        if let Some(_value) = self.tokens.next_if(|x| x.token_type == TokenType::String) {
            println!("parse: nil");
            self.current += 1;
            let token = self.previous_token();
            return Expr::String(token.lexeme);
        }

        if let Some(_value) = self.tokens.next_if(|x| x.token_type == TokenType::Number) {
            println!("parse: number");
            self.current += 1;
            let token = self.previous_token();
            let number = token.lexeme.parse::<f32>().unwrap();
            return Expr::Number(number);
        }

        if let Some(_value) = self
            .tokens
            .next_if(|x| x.token_type == TokenType::LeftParen)
        {
            println!("parse: grouping");
            self.current += 1;
            let expr = self.expression();
            while let Some(value) = self.tokens.peek() {
                if value.token_type == TokenType::RightParen {
                    return Expr::Grouping(Box::new(expr));
                }
            }
            let token = self.token_vec[self.current].clone();
            report_error(token.line, "Expected ')' after expression");
        }
        let token = self.token_vec[self.current].clone();
        report_error(token.line, "Failed to parse token");
        println!("{:?}", token);
        panic!("failed")
    }

    fn previous_token(&self) -> Token {
        self.token_vec[self.current - 1].clone()
    }
}
