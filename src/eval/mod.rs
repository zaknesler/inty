mod env;

use self::env::Environment;
use crate::core::*;

pub struct Evaluator {
    env: Environment,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    /// Evaluate a program's statements into a list of values
    pub fn eval(&mut self, stmts: Vec<Stmt>) -> anyhow::Result<ProgramOutput> {
        let mut results = vec![];

        for stmt in &stmts {
            results.push(self.eval_stmt(stmt)?);
        }

        Ok(results)
    }

    fn eval_stmt(&mut self, stmt: &Stmt) -> anyhow::Result<i32> {
        match stmt {
            Stmt::Expr(expr) => self.eval_expr(&expr),
            Stmt::Let { ident, expr } => {
                self.env.put(ident.clone(), self.eval_expr(expr)?);

                Ok(-1) // @todo use core::Value enum so a None value can be returned (or maybe just Option for now?)
            }
        }
    }

    /// Recursively evaluate a single statement
    fn eval_expr(&self, expr: &Expr) -> anyhow::Result<i32> {
        Ok(match expr.clone() {
            Expr::Integer(val) => *val,
            Expr::Ident(ident) => match self.env.get(ident.clone()) {
                Some(val) => *val,
                None => anyhow::bail!(Error::UnknownIdentifier {
                    ident: ident.clone()
                }),
            },
            Expr::Unary { operator, value } => match operator {
                UnOp::Minus => -1 * self.eval_expr(value)?,
                UnOp::Plus => self.eval_expr(value)?,
            },
            Expr::Binary { operator, lhs, rhs } => match operator {
                BinOp::Add => self.eval_expr(lhs.as_ref())? + self.eval_expr(rhs.as_ref())?,
                BinOp::Sub => self.eval_expr(lhs.as_ref())? - self.eval_expr(rhs.as_ref())?,
                BinOp::Mul => self.eval_expr(lhs.as_ref())? * self.eval_expr(rhs.as_ref())?,
                BinOp::Div => {
                    let left = self.eval_expr(lhs.as_ref())?;
                    let right = self.eval_expr(rhs.as_ref())?;

                    match right {
                        0 => anyhow::bail!(Error::DivideByZeroError),
                        _ => left / right,
                    }
                }
                BinOp::Pow => {
                    let base = self.eval_expr(lhs.as_ref())?;
                    let pow = self.eval_expr(rhs.as_ref())?;

                    if pow < 0 {
                        anyhow::bail!(Error::LogicError {
                            message: "Power must be non-negative integer".to_string()
                        });
                    }

                    base.pow(pow as u32)
                }
            },
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

        assert_eq!(100, *value.first().unwrap());
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

        assert_eq!(3, *value.first().unwrap());
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

        assert_eq!(42, *evaler.env.get("foo".into()).unwrap());
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

        assert_eq!(42, *value.last().unwrap());
    }
}
