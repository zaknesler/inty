use super::*;

pub type IntyResult<T> = Result<T, IntyError>;

#[derive(thiserror::Error, Debug)]
pub enum IntyError {
    #[error("syntax error: {message}")]
    SyntaxError {
        token: Option<Token>,
        message: String,
    },

    #[error("unexpected character: {character}")]
    UnexpectedChar { character: char },

    #[error("could not parse token: {character}: {message}")]
    TokenParsingError { character: char, message: String },

    #[error("expected {expected}, found {found}")]
    ExpectedTokenError { expected: Token, found: Token },

    #[error("logic error: {message}")]
    LogicError { message: String },

    #[error("cannot divide by zero")]
    DivideByZeroError,

    #[error("invalid expression: {message}")]
    InvalidExpressionError { message: String },

    #[error("unknown identifier: {ident}")]
    UnknownIdentifier { ident: String },

    #[error("type error: {message}")]
    TypeError { message: String },

    #[error("parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("i/o error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("readline error: {0}")]
    ReadlineError(#[from] rustyline::error::ReadlineError),

    #[error("borrow error: {0}")]
    BorrowError(#[from] std::cell::BorrowError),

    #[error("borrow mut error: {0}")]
    BorrowMutError(#[from] std::cell::BorrowMutError),
}
