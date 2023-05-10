use super::Token;

/// A program is just a list of statements to be evaluated
#[derive(Debug, PartialEq)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

/// The type to which a program will evaluate
pub type ProgramOutput = Vec<i32>;

#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i32),
    Float(u32),
    Str(String),
    Bool(bool),
}

/// An expression is a group of child expressions that evaluate to a single value
#[derive(Debug, PartialEq)]
pub enum Expr {
    /// Single integer value (e.g. 42)
    Integer(i32),

    /// A variable (e.g. `x`)
    Ident(String),

    /// Unary operation (e.g. +1, -2)
    Unary { operator: UnOp, value: Box<Expr> },

    /// Binary operation (e.g. 3 * 4)
    Binary {
        operator: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

/// A statement
#[derive(Debug, PartialEq)]
pub enum Stmt {
    /// A let statement (e.g. `let x = 10;`)
    Let {
        identifier: String,
        value: Expr,
    },

    Expr(Expr),
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
