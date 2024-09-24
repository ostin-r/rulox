use crate::token;

#[derive(Debug)]
pub enum Expr {
    Binary {left: Box<Expr>, operator: token::Token, right: Box<Expr>},
    Grouping(Box<Expr>),
    Unary {operator: token::Token, right: Box<Expr>},
    // Unary reverse is only used for the comma operator, while not technically 
    // a unary expression it feels like the opposite due to its structure
    UnaryReverse {left: Box<Expr>, operator: token::Token},
    Ternary {condition: Box<Expr>, if_true: Box<Expr>, if_false: Box<Expr>},
    // literals
    String(String),
    Boolean(bool),
    Number(f32),
    Nil
}

