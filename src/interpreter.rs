use crate::expr::{Expr, Value};

pub struct Interpreter;

impl Expr::Visitor<Result<Value>> for Interpreter {}
