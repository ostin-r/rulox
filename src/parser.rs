use crate::expr::Expr;
use crate::token::{TokenType, Token};

// Lox uses recursive descent parsing
//
// In order of lowest to highest precedence:
// Expression
// Comparison
// Addition
// Multiplication
// Unary
//

struct Parser {
    tokens: Peekable<Iterator>
}

impl Parser {
    pub fn parse() {
        let tokens_iter = tokens.iter().peekable();
        let tree = expression(tokens_iter);
    }

    fn expression() {
        equality()
    }

    fn equality() {
        let expr = comparison(tokens_iter);

        let equality_fields: Vec<TokenType> = [TokenType::EqualEqual, TokenType::BangEqual];
        while let Some(value) = self.tokens.next_if(|x| equality_fields.contains(x)) {}
    }

    fn comparison() {
    }

    fn next_condition(types: Vec<TokenType>) {
        // helper for next_if uses

    }

    // fn advance_if(types: List<TokenType>) {
    //     // advance the iterator if we match any of the provided enums and return true if we
    //     // advanced the iterator
    //     if let Some(iter) = self.tokens.peek() {
    //         if types.contains(*iter.token_type) {
    //             iter.next();
    //             true
    //         }
    //     }
    //     false
    // }
}

