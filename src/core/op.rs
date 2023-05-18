use super::*;

/// An unary operator (e.g. -[int], +[int])
#[derive(Debug, PartialEq)]
pub enum UnOp {
    Plus,
    Minus,
    Negate,
}

/// A binary operator (e.g. [int] + [int])
#[derive(Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

/// A logical operator (e.g. [bool] && [bool])
#[derive(Debug, PartialEq)]
pub enum LogOp {
    Or,
    And,
}

/// A relational operator (e.g. [int] >= [int])
#[derive(Debug, PartialEq)]
pub enum RelOp {
    Eq,
    Ne,
    Gt,
    Lt,
    Gte,
    Lte,
}

impl From<Token> for UnOp {
    fn from(value: Token) -> Self {
        match value {
            Token::Hyphen => UnOp::Minus,
            Token::Plus => UnOp::Plus,
            Token::Bang => UnOp::Negate,
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

impl From<Token> for LogOp {
    fn from(value: Token) -> Self {
        match value {
            Token::And => LogOp::And,
            Token::Or => LogOp::Or,
            _ => panic!("Invalid token"),
        }
    }
}

impl From<Token> for RelOp {
    fn from(value: Token) -> Self {
        match value {
            Token::RelEq => RelOp::Eq,
            Token::RelNe => RelOp::Ne,
            Token::RelGt => RelOp::Gt,
            Token::RelLt => RelOp::Lt,
            Token::RelGte => RelOp::Gte,
            Token::RelLte => RelOp::Lte,
            _ => panic!("Invalid token"),
        }
    }
}
