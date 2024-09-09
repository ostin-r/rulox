use crate::token;

pub enum Expr {
    Binary {left: Box<Expr>, operator: token::TokenType, right: Box<Expr>},
    Grouping(Box<Expr>),
    Literal(String),
    Unary {operator: token::TokenType, right: Box<Expr>}
}

