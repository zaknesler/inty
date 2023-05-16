use super::Token;
use std::{fmt::Display, rc::Rc};

/// A statement can be an operation upon an expression, or just a single expression
#[derive(Debug, PartialEq)]
pub enum Stmt {
    /// An if statement (e.g. `if <expr> <stmt> [<else> <stmt>]`)
    If {
        test: Expr,
        branch: Rc<Stmt>,
        else_branch: Option<Rc<Stmt>>,
    },

    /// A let statement (e.g. `let x = 10;`)
    Let { ident: String, expr: Expr },

    /// A group of statements (e.g. `{ let x = 1; x + 2 }`)
    Block(Vec<Stmt>),

    /// A single expression
    Expr(Expr),
}

/// Internal values for evaluation
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Integer(i32),
    Bool(bool),
}

/// An expression is a group of child expressions that evaluate to a single value
#[derive(Debug, PartialEq)]
pub enum Expr {
    /// Single integer value (e.g. 42)
    Integer(i32),

    /// Boolean value (e.g. true/false)
    Bool(bool),

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

    Logical {
        operator: LogOp,
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },

    Relational {
        operator: RelOp,
        lhs: Rc<Expr>,
        rhs: Rc<Expr>,
    },
}

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

/// A logical operator (e.g. [bool] + [bool])
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

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(val) => write!(f, "{}", val),
            Value::Bool(val) => write!(f, "{}", val),
        }
    }
}

impl Value {
    pub fn unwrap_integer(&self) -> anyhow::Result<i32> {
        match self {
            Value::Integer(val) => Ok(*val),
            _ => anyhow::bail!("{} is not an integer", self),
        }
    }

    pub fn unwrap_bool(&self) -> anyhow::Result<bool> {
        match self {
            Value::Bool(val) => Ok(*val),
            // Value::Integer(val) => Ok(*val > 1),
            _ => anyhow::bail!("{} is not a boolean", self),
        }
    }
}
