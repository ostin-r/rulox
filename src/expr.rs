use crate::token;

pub enum Expr {
    Binary {left: Box<Expr>, operator: token::Token, right: Box<Expr>},
    Grouping(Box<Expr>),
    Literal(String),
    Unary {operator: token::Token, right: Box<Expr>}
}

