use super::*;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    SyntaxError {
        token: Option<Token>,
        message: String,
    },
    TokenParsingError {
        character: char,
        message: String,
    },
    LogicError {
        message: String,
    },
    DivideByZeroError,
    InvalidExpressionError {
        message: String,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SyntaxError { token, message } => match token {
                Some(token) => write!(f, "Syntax error: {}: {}", message, token),
                None => write!(f, "Syntax error: {}", message),
            },
            Error::TokenParsingError { character, message } => {
                write!(f, "Token parsing error: {}: {}", message, character)
            }
            Error::LogicError { message } => write!(f, "Logic error: {}", message),
            Error::DivideByZeroError => write!(f, "Error: Cannot divide by zero"),
            Error::InvalidExpressionError { message } => {
                write!(f, "Invalid expression: {}", message)
            }
        }
    }
}
