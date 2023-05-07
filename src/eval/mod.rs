use crate::core::*;

pub struct Evaluator<'a> {
    prog: &'a Program,
}

impl<'a> Evaluator<'a> {
    pub fn new(prog: &'a Program) -> Self {
        Self { prog }
    }

    /// Evaluate a program's statements into a list of values
    pub fn eval(&self) -> anyhow::Result<ProgramOutput> {
        let mut results = vec![];

        for expr in &self.prog.stmts {
            results.push(self.eval_node(expr)?);
        }

        Ok(results)
    }

    /// Recursively evaluate a single expression node
    fn eval_node(&self, node: &Expr) -> anyhow::Result<i32> {
        Ok(match node.clone() {
            Expr::Integer(val) => *val,
            Expr::Unary { operator, value } => match operator {
                UnOp::Minus => -1 * self.eval_node(value)?,
                UnOp::Plus => self.eval_node(value)?,
            },
            Expr::Binary { operator, lhs, rhs } => match operator {
                BinOp::Add => self.eval_node(lhs.as_ref())? + self.eval_node(rhs.as_ref())?,
                BinOp::Sub => self.eval_node(lhs.as_ref())? - self.eval_node(rhs.as_ref())?,
                BinOp::Mul => self.eval_node(lhs.as_ref())? * self.eval_node(rhs.as_ref())?,
                BinOp::Div => {
                    let left = self.eval_node(lhs.as_ref())?;
                    let right = self.eval_node(rhs.as_ref())?;

                    match right {
                        0 => anyhow::bail!(Error::DivideByZeroError),
                        _ => left / right,
                    }
                }
                BinOp::Pow => {
                    let base = self.eval_node(lhs.as_ref())?;
                    let pow = self.eval_node(rhs.as_ref())?;

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

    #[test]
    fn single_number() {
        let eval = Evaluator {
            prog: &Program {
                stmts: vec![Expr::Integer(100)],
            },
        }
        .eval()
        .unwrap();

        assert_eq!(100, *eval.first().unwrap());
    }

    #[test]
    fn basic_addition() {
        let eval = Evaluator {
            prog: &Program {
                stmts: vec![Expr::Binary {
                    operator: BinOp::Add,
                    lhs: Box::new(Expr::Integer(1)),
                    rhs: Box::new(Expr::Integer(2)),
                }],
            },
        }
        .eval()
        .unwrap();

        assert_eq!(3, *eval.first().unwrap());
    }
}
