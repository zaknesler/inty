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

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ReadlineError(#[from] rustyline::error::ReadlineError),
}
