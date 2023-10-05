mod env;

use std::{cell::RefCell, rc::Rc};

use self::env::Environment;
use crate::core::*;

pub struct Evaluator {
    env: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn new_with_env(env: Rc<RefCell<Environment>>) -> Self {
        Self { env }
    }

    /// Evaluate a program's statements into a list of values
    pub fn eval(&mut self, stmts: Vec<Stmt>) -> IntyResult<Vec<Option<Value>>> {
        let mut results = vec![];

        for stmt in &stmts {
            results.push(self.eval_stmt(stmt)?);
        }

        Ok(results)
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> IntyResult<Option<Value>> {
        Ok(match stmt {
            Stmt::Expr(expr) => Some(self.eval_expr(&expr)?),
            Stmt::If {
                test,
                branch: block,
                else_branch: else_block,
            } => {
                if self.eval_expr(test)?.unwrap_bool()? {
                    self.eval_stmt(block)?
                } else if let Some(else_block) = else_block {
                    self.eval_stmt(else_block)?
                } else {
                    None
                }
            }
            Stmt::Let { ident, expr } => {
                let val = self.eval_expr(expr)?;
                self.env.try_borrow_mut()?.put(ident.clone(), val);
                None
            }
            Stmt::Block(stmts) => {
                // I dunno if this is just a gap in my knowledge, but the
                // following line would need to be `mut` if we allowed re-assignment,
                // but we don't so this is good for now. Shadowing works.
                let env = Environment::new_with_parent(self.env.clone());
                let mut eval = Evaluator::new_with_env(Rc::new(RefCell::new(env)));

                if let Some(val) = stmts
                    .iter()
                    .map(|stmt| eval.eval_stmt(stmt))
                    .collect::<Vec<_>>()
                    .pop()
                {
                    val?
                } else {
                    return Err(IntyError::SyntaxError {
                        token: None,
                        message: "block contained no return value".to_string(),
                    });
                }
            }
        })
    }

    /// Recursively evaluate a single statement
    fn eval_expr(&self, expr: &Expr) -> IntyResult<Value> {
        Ok(match expr {
            Expr::Integer(val) => Value::Integer(*val),
            Expr::Ident(ident) => match self.env.try_borrow()?.get(ident.clone()) {
                Some(val) => val.clone(),
                None => {
                    return Err(IntyError::UnknownIdentifier {
                        ident: ident.clone(),
                    })
                }
            },
            Expr::Bool(val) => Value::Bool(*val),
            Expr::Unary { operator, value } => match operator {
                UnOp::Minus => {
                    if let Value::Integer(value) = self.eval_expr(value)? {
                        Value::Integer(-1 * value)
                    } else {
                        return Err(IntyError::TypeError {
                            message: "expected integer".to_string(),
                        });
                    }
                }
                UnOp::Plus => self.eval_expr(value)?,
                UnOp::Negate => Value::Bool(!self.eval_expr(value)?.unwrap_bool()?),
            },
            Expr::Binary { operator, lhs, rhs } => Value::Integer(match operator {
                BinOp::Add => {
                    self.eval_expr(lhs.as_ref())?.unwrap_integer()?
                        + self.eval_expr(rhs.as_ref())?.unwrap_integer()?
                }
                BinOp::Sub => {
                    self.eval_expr(lhs.as_ref())?.unwrap_integer()?
                        - self.eval_expr(rhs.as_ref())?.unwrap_integer()?
                }
                BinOp::Mul => {
                    self.eval_expr(lhs.as_ref())?.unwrap_integer()?
                        * self.eval_expr(rhs.as_ref())?.unwrap_integer()?
                }
                BinOp::Div => {
                    let left = self.eval_expr(lhs.as_ref())?.unwrap_integer()?;
                    let right = self.eval_expr(rhs.as_ref())?.unwrap_integer()?;

                    match right {
                        0 => return Err(IntyError::DivideByZeroError),
                        _ => left / right,
                    }
                }
                BinOp::Pow => {
                    let base = self.eval_expr(lhs.as_ref())?.unwrap_integer()?;
                    let pow = self.eval_expr(rhs.as_ref())?.unwrap_integer()?;

                    if pow < 0 {
                        return Err(IntyError::LogicError {
                            message: "power must be non-negative integer".to_string(),
                        });
                    }

                    base.pow(pow as u32)
                }
            }),
            Expr::Logical { operator, lhs, rhs } => Value::Bool({
                let left = self.eval_expr(lhs.as_ref())?.unwrap_bool()?;
                let right = self.eval_expr(rhs.as_ref())?.unwrap_bool()?;

                match operator {
                    LogOp::And => left && right,
                    LogOp::Or => left || right,
                }
            }),
            Expr::Relational { operator, lhs, rhs } => Value::Bool({
                let left = self.eval_expr(lhs.as_ref())?;
                let right = self.eval_expr(rhs.as_ref())?;

                match (left, right) {
                    (Value::Integer(lhs), Value::Integer(rhs)) => match operator {
                        RelOp::Eq => lhs == rhs,
                        RelOp::Ne => lhs != rhs,
                        RelOp::Gt => lhs > rhs,
                        RelOp::Lt => lhs < rhs,
                        RelOp::Gte => lhs >= rhs,
                        RelOp::Lte => lhs <= rhs,
                    },
                    (Value::Bool(lhs), Value::Bool(rhs)) => match operator {
                        RelOp::Eq => lhs == rhs,
                        RelOp::Ne => lhs != rhs,
                        _ => {
                            return Err(IntyError::SyntaxError {
                                token: None,
                                message: "invalid operation".to_string(),
                            })
                        }
                    },
                    _ => {
                        return Err(IntyError::SyntaxError {
                            token: None,
                            message: "invalid comparison".to_string(),
                        })
                    }
                }
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn single_number() {
        let value = Evaluator::new()
            .eval(vec![Stmt::Expr(Expr::Integer(100))])
            .unwrap();

        assert_eq!(
            Value::Integer(100),
            *value.last().unwrap().as_ref().unwrap()
        );
    }

    #[test]
    fn basic_addition() {
        let value = Evaluator::new()
            .eval(vec![Stmt::Expr(Expr::Binary {
                operator: BinOp::Add,
                lhs: Rc::new(Expr::Integer(1)),
                rhs: Rc::new(Expr::Integer(2)),
            })])
            .unwrap();

        assert_eq!(Value::Integer(3), *value.last().unwrap().as_ref().unwrap());
    }

    #[test]
    fn variable_assignment() {
        let mut evaler = Evaluator::new();

        evaler
            .eval(vec![Stmt::Let {
                ident: "foo".into(),
                expr: Expr::Integer(42),
            }])
            .unwrap();

        assert_eq!(
            42,
            evaler
                .env
                .borrow()
                .get("foo".into())
                .unwrap()
                .unwrap_integer()
                .unwrap()
        );
    }

    #[test]
    fn variable_assignment_and_retrieval() {
        let mut evaler = Evaluator::new();

        let value = evaler
            .eval(vec![
                Stmt::Let {
                    ident: "foo".into(),
                    expr: Expr::Integer(42),
                },
                Stmt::Expr(Expr::Ident("foo".to_string())),
            ])
            .unwrap();

        assert_eq!(Value::Integer(42), *value.last().unwrap().as_ref().unwrap());
    }
}
