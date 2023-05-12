use super::*;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    SyntaxError {
        token: Option<Token>,
        message: String,
    },
    UnexpectedChar {
        character: char,
    },
    TokenParsingError {
        character: char,
        message: String,
    },
    ExpectedTokenError {
        expected: Token,
        found: Token,
    },
    LogicError {
        message: String,
    },
    DivideByZeroError,
    InvalidExpressionError {
        message: String,
    },
    UnknownIdentifier {
        ident: String,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SyntaxError { token, message } => match token {
                Some(token) => write!(f, "Syntax error: {}: {}", message, token),
                None => write!(f, "Syntax error: {}", message),
            },
            Error::UnexpectedChar { character } => {
                write!(f, "Unexpected character: {}", character)
            }
            Error::TokenParsingError { character, message } => {
                write!(f, "Token parsing error: {}: {}", message, character)
            }
            Error::ExpectedTokenError { expected, found } => {
                write!(f, "Expected {} but found {}", expected, found)
            }
            Error::LogicError { message } => write!(f, "Logic error: {}", message),
            Error::DivideByZeroError => write!(f, "Error: Cannot divide by zero"),
            Error::InvalidExpressionError { message } => {
                write!(f, "Invalid expression: {}", message)
            }
            Error::UnknownIdentifier { ident } => {
                write!(f, "Unknown identifier: {}", ident)
            }
        }
    }
}
