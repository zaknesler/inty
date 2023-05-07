use super::Token;

/// A program is just a list of statements to be evaluated
#[derive(Debug, PartialEq)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

/// A single statement
pub type Stmt = Expr;

/// The type to which a program will evaluate
pub type ProgramOutput = Vec<i32>;

/// An expression is a group of child expressions that evaluate to a single value
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
