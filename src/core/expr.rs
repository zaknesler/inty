use std::rc::Rc;

use super::*;

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
