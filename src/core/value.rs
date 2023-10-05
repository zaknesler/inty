use std::fmt::Display;

use super::IntyResult;

/// Internal values for evaluation
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Integer(i32),
    Bool(bool),
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
    pub fn try_parse_int(&self) -> IntyResult<i32> {
        match self {
            Value::Integer(val) => Ok(*val),
            _ => {
                return Err(super::IntyError::TypeError {
                    message: format!("{} is not an integer", self),
                })
            }
        }
    }

    pub fn try_parse_bool(&self) -> IntyResult<bool> {
        match self {
            Value::Bool(val) => Ok(*val),
            Value::Integer(val) => Ok(*val > 0),
        }
    }
}
