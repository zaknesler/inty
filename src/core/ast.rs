use super::Token;
use std::{fmt::Display, rc::Rc};

/// A statement can be an operation upon an expression, or just a single expression
#[derive(Debug, PartialEq)]
pub enum Stmt {
    /// A let statement (e.g. `let x = 10;`)
    Let { ident: String, expr: Expr },

    /// A single expression
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i32),
    // Float(u32),
    // Str(String),
    // Bool(bool),
}

/// An expression is a group of child expressions that evaluate to a single value
#[derive(Debug, PartialEq)]
pub enum Expr {
    /// Single integer value (e.g. 42)
    Integer(i32),

    /// A variable (e.g. `x`)
    Ident(String),

    /// Unary operation (e.g. +1, -2)
    Unary { operator: UnOp, value: Rc<Expr> },

    /// Binary operation (e.g. 3 * 4)
    Binary {
        operator: BinOp,
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
}

/// An unary operator (e.g. -[Integer], +[Integer])
#[derive(Debug, PartialEq)]
pub enum UnOp {
    Plus,
    Minus,
}

/// A binary operator (e.g. [Integer] + [Integer])
#[derive(Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl From<Token> for UnOp {
    fn from(value: Token) -> Self {
        match value {
            Token::Hyphen => UnOp::Minus,
            Token::Plus => UnOp::Plus,
            _ => panic!("Invalid token"),
        }
    }
}

impl From<Token> for BinOp {
    fn from(value: Token) -> Self {
        match value {
            Token::Plus => BinOp::Add,
            Token::Hyphen => BinOp::Sub,
            Token::Star => BinOp::Mul,
            Token::Divide => BinOp::Div,
            Token::Caret => BinOp::Pow,
            _ => panic!("Invalid token"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(val) => write!(f, "{}", val),
        }
    }
}
