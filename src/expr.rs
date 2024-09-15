use crate::token;

pub enum Expr {
    Binary {left: Box<Expr>, operator: token::Token, right: Box<Expr>},
    Grouping(Box<Expr>),
    Unary {operator: token::Token, right: Box<Expr>},
    // literals
    String(String),
    Boolean(bool),
    Number(f32),
    Nil
}

