use std::fmt::Display;

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
    pub fn unwrap_integer(&self) -> anyhow::Result<i32> {
        match self {
            Value::Integer(val) => Ok(*val),
            _ => anyhow::bail!("{} is not an integer", self),
        }
    }

    pub fn unwrap_bool(&self) -> anyhow::Result<bool> {
        match self {
            Value::Bool(val) => Ok(*val),
            Value::Integer(val) => Ok(*val > 0),
        }
    }
}
