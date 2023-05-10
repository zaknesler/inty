use crate::core::*;
use std::collections::HashMap;

pub struct Evaluator<'a> {
    prog: &'a Program,
    environment: Box<HashMap<String, i32>>,
}

impl<'a> Evaluator<'a> {
    pub fn new(prog: &'a Program) -> Self {
        Self {
            prog,
            environment: Box::new(HashMap::new()),
        }
    }

    /// Evaluate a program's statements into a list of values
    pub fn eval(&self) -> anyhow::Result<ProgramOutput> {
        let mut results = vec![];

        for stmt in &self.prog.stmts {
            results.push(self.eval_stmt(stmt)?);
        }

        Ok(results)
    }

    fn eval_stmt(&self, stmt: &Stmt) -> anyhow::Result<i32> {
        match stmt {
            Stmt::Expr(expr) => self.eval_expr(&expr),
            Stmt::Let { .. } => todo!(),
        }
    }

    /// Recursively evaluate a single statement
    fn eval_expr(&self, expr: &Expr) -> anyhow::Result<i32> {
        Ok(match expr.clone() {
            Expr::Integer(val) => *val,
            Expr::Ident(_name) => todo!(),
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
    use std::rc::Rc;

    use super::*;

    #[test]
    fn single_number() {
        let eval = Evaluator::new(&Program {
            stmts: vec![Stmt::Expr(Expr::Integer(100))],
        })
        .eval()
        .unwrap();

        assert_eq!(100, *eval.first().unwrap());
    }

    #[test]
    fn basic_addition() {
        let eval = Evaluator::new(&Program {
            stmts: vec![Stmt::Expr(Expr::Binary {
                operator: BinOp::Add,
                lhs: Rc::new(Expr::Integer(1)),
                rhs: Rc::new(Expr::Integer(2)),
            })],
        })
        .eval()
        .unwrap();

        assert_eq!(3, *eval.first().unwrap());
    }
}
