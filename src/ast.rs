use crate::token::Token;

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub root: Expr,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    /// Single integer value (e.g. 42)
    Integer(i32),

    /// Unary operation (e.g. +1, -2)
    Unary { operator: UnOp, value: Box<Expr> },

    /// Binary operation (e.g. 3 * 4)
    Binary {
        operator: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, PartialEq)]
pub enum UnOp {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<Token> for UnOp {
    fn from(value: Token) -> Self {
        match value {
            Token::MinusSign => UnOp::Minus,
            Token::PlusSign => UnOp::Plus,
            _ => panic!("Invalid token"),
        }
    }
}

impl From<Token> for BinOp {
    fn from(value: Token) -> Self {
        match value {
            Token::PlusSign => BinOp::Add,
            Token::MinusSign => BinOp::Sub,
            Token::TimesSign => BinOp::Mul,
            Token::DivideSign => BinOp::Div,
            _ => panic!("Invalid token"),
        }
    }
}
