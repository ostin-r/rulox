use crate::expr::Expr;
use crate::token::{TokenType, Token};

// Lox uses recursive descent parsing, a predictive parsing method
//
// In order of lowest to highest precedence:
// Expression
// Comparison
// Addition
// Multiplication
// Unary
//

struct Parser {
    tokens: Peekable<Iterator>,
    token_vec: Vec<Token>,
    current: i32
}

impl Parser {
    pub fn parse(&self) -> Expr {
        let tree = expression(self.tokens);
    }

    fn expression(&self) -> Expr {
        equality()
    }

    fn equality(&self) -> Expr {
        let expr = self.comparison();

        let equality_fields = [TokenType::EqualEqual, TokenType::BangEqual];
        while let Some(value) = self.tokens.next_if(|x| equality_fields.contains(x.token_type)) {
            self.current += 1;
            let operator = self.previous_token();
            let right = self.comparison();
            let expr = Expr::Binary(expr, operator, right);
        }
        expr
    }

    fn comparison(&self) -> Expr {
        let expr = self.term();
        let comparison_fields = [TokenType::LessEqual, TokenType::Less, TokenType::Greater, TokenType::GreaterEqual];
        while let Some(value) = self.tokens.next_if(|x| comparison_fields.contains(x.token_type)) {
            self.current += 1;
            let operator = self.previous_token();
            let right = self.term();
            let expr = Expr::Binary(expr, operator, right);  // todo: this might not work, be
                                                             // explicit with naming, or renam expr
                                                             // to be 'left' for implicit
                                                             // declaration
        }
        expr
    }

    fn term(&self) -> Expr {
        let expr = self.factor();
        let term_fields = [TokenType::LessEqual, TokenType::Less, TokenType::Greater, TokenType::GreaterEqual];
        while let Some(value) = self.tokens.next_if(|x| term_fields.contains(x.token_type)) {
            self.current += 1;
            let operator = self.previous_token();
            let right = self.factor();
            let expr = Expr::Binary(expr, operator, right);
        }
        expr
    }

    fn factor(&self) -> Expr {
        let expr = self.unary();
        let factor_fields = [TokenType::Slash, TokenType::Star];
        while let Some(value) = self.tokens.next_if(|x| factor_fields.contains(x.token_type)) {
            self.current += 1;
            let operator = self.previous_token();
            let right = self.unary();
            let expr = Expr::Binary(expr, operator, right);
        }
        expr
    }

    fn unary(&self) -> Expr {
        // todo: fix this, currently allows multiple leading negatives (unless caught at lexer?)
        let unary_fields = [TokenType::Bang, TokenType::Minus];
        if let Some(value) = self.tokens.next_if(|x| unary_fields.contains(x)) {
            self.current += 1;
            let operator = self.previous_token();
            let right = self.unary();
            Expr::Unary(operator, right);
        }
    }

    fn previous_token(&self) -> Token {
        self.token_vec[self.curent - 1]
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

