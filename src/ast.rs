use crate::token::Token;

#[derive(Debug)]
pub struct Ast {
    pub root: Expr,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i32),
    Binary {
        operator: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
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
