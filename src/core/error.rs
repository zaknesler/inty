use super::*;

pub type IntyResult<T> = Result<T, IntyError>;

#[derive(thiserror::Error, Debug)]
pub enum IntyError {
    #[error("syntax error: {message}")]
    SyntaxError {
        token: Option<Token>,
        message: &'static str,
    },

    #[error("unexpected character: {character}")]
    UnexpectedChar { character: char },

    #[error("could not parse token: {character}: {message}")]
    TokenParsingError {
        character: char,
        message: &'static str,
    },

    #[error("expected {expected}, found {found}")]
    ExpectedTokenError { expected: Token, found: Token },

    #[error("logic error: {message}")]
    LogicError { message: &'static str },

    #[error("cannot divide by zero")]
    DivideByZeroError,

    #[error("invalid expression: {message}")]
    InvalidExpressionError { message: &'static str },

    #[error("unknown identifier: {ident}")]
    UnknownIdentifier { ident: &'static str },

    #[error("type error: {message}")]
    TypeError { message: &'static str },

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    ReadlineError(#[from] rustyline::error::ReadlineError),
}
