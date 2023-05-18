use super::*;
use std::rc::Rc;

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
