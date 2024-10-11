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
}

impl Parser<'_> {
    pub fn parse(&mut self) -> Vec<Expr> {
        let mut statements: Vec<Expr> = Vec::new();
        while let Some(current_token) = self.tokens.next() {
            let expression = self.expression(&current_token);
            statements.push(expression);
        }
        statements
    }

    fn expression(&mut self, previous_token: &Token) -> Expr {
        self.unary_reverse(previous_token)
    }

    fn unary_reverse(&mut self, previous_token: &Token) -> Expr {
        // used for comma expressions
        let expr = self.ternary(previous_token);
        let unary_reverse_fields = [TokenType::Comma];
        if let Some(operator) = self
            .tokens
            .next_if(|x| unary_reverse_fields.contains(&x.token_type))
        {
            return Expr::UnaryReverse {
                left: Box::new(expr),
                operator: operator.clone(),
            };
        }
        expr
    }

    fn ternary(&mut self, previous_token: &Token) -> Expr {
        let expr = self.equality(previous_token);
        let ternary_fields = [TokenType::Question];
        if let Some(primary_token) = self
            .tokens
            .next_if(|x| ternary_fields.contains(&x.token_type))
        {
            let if_true_statement = self.equality(primary_token);
            let middle_ternary_fields = [TokenType::Colon];
            if let Some(secondary_token) = self
                .tokens
                .next_if(|x| middle_ternary_fields.contains(&x.token_type))
            {
                let if_false_statement = self.equality(secondary_token);
                return Expr::Ternary {
                    condition: Box::new(expr),
                    if_true: Box::new(if_true_statement),
                    if_false: Box::new(if_false_statement),
                };
            }
            panic!("Ternary expression not terminated, expected ':' after ? operator");
        }
        expr
    }

    fn equality(&mut self, previous_token: &Token) -> Expr {
        let mut expr = self.comparison(previous_token);
        let equality_fields = [TokenType::EqualEqual, TokenType::BangEqual];
        while let Some(current_token) = self
            .tokens
            .next_if(|x| equality_fields.contains(&x.token_type))
        {
            let next_token = self.tokens.next().unwrap();
            let right = self.comparison(next_token);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: current_token.clone(),
                right: Box::new(right),
            };
        }
        expr
    }

    fn comparison(&mut self, previous_token: &Token) -> Expr {
        let mut expr = self.term(previous_token);
        let comparison_fields = [
            TokenType::LessEqual,
            TokenType::Less,
            TokenType::Greater,
            TokenType::GreaterEqual,
        ];
        while let Some(current_token) = self
            .tokens
            .next_if(|x| comparison_fields.contains(&x.token_type))
        {
            let next_token = self.tokens.next().unwrap();
            let right = self.term(next_token);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: current_token.clone(),
                right: Box::new(right),
            };
        }
        expr
    }

    fn term(&mut self, previous_token: &Token) -> Expr {
        let mut expr = self.factor(previous_token);
        let term_fields = [TokenType::Minus, TokenType::Plus];
        while let Some(current_token) = self.tokens.next_if(|x| term_fields.contains(&x.token_type))
        {
            let next_token = self.tokens.next().unwrap();
            let right = self.factor(next_token);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: current_token.clone(),
                right: Box::new(right),
            };
        }
        expr
    }

    fn factor(&mut self, previous_token: &Token) -> Expr {
        let mut expr = self.unary(previous_token);
        let factor_fields = [TokenType::Slash, TokenType::Star];
        while let Some(current_token) = self
            .tokens
            .next_if(|x| factor_fields.contains(&x.token_type))
        {
            let next_token = self.tokens.next().unwrap();
            let right = self.unary(next_token);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: current_token.clone(),
                right: Box::new(right),
            };
        }
        expr
    }

    fn unary(&mut self, previous_token: &Token) -> Expr {
        // todo: fix this, currently allows multiple leading negatives (unless caught at lexer?)
        let unary_fields = [TokenType::Bang, TokenType::Minus];
        if let Some(current_token) = self
            .tokens
            .next_if(|x| unary_fields.contains(&x.token_type))
        {
            let next_token = self.tokens.next().unwrap();
            let right = self.unary(next_token);
            return Expr::Unary {
                operator: current_token.clone(),
                right: Box::new(right),
            };
        }
        return self.primary(previous_token);
    }

    fn primary(&mut self, previous_token: &Token) -> Expr {
        match previous_token.token_type {
            TokenType::False => return Expr::Boolean(false),
            TokenType::True => return Expr::Boolean(true),
            TokenType::Nil => return Expr::Nil,
            TokenType::String => return Expr::String(previous_token.lexeme.clone()),
            TokenType::Number => {
                return Expr::Number(previous_token.lexeme.parse::<f32>().unwrap())
            }
            TokenType::LeftParen => {
                let expr = self.expression(previous_token);
                if let Some(_value) = self
                    .tokens
                    .next_if(|x| x.token_type == TokenType::RightParen)
                {
                    return Expr::Grouping(Box::new(expr));
                }
                report_error(previous_token.line, "Expected ')' after expression");
                panic!("failed");
            }
            _ => {
                report_error(previous_token.line, "Failed to parse token");
                println!("{:?}", previous_token);
                panic!("failed")
            }
        }
    }
}
