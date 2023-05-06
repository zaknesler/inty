use crate::core::*;

pub struct Evaluator<'a> {
    ast: &'a Ast,
}

impl<'a> Evaluator<'a> {
    pub fn new(ast: &'a Ast) -> Self {
        Self { ast }
    }

    /// Evaluate an AST to a single value
    pub fn eval(&self) -> anyhow::Result<i32> {
        self.eval_node(&self.ast.root)
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
                BinOp::Div => self.eval_node(lhs.as_ref())? / self.eval_node(rhs.as_ref())?,
                BinOp::Pow => {
                    let base = self.eval_node(lhs.as_ref())?;
                    let pow = self.eval_node(rhs.as_ref())?;

                    if pow < 0 {
                        anyhow::bail!("Power must be non-negative integer");
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
        assert_eq!(
            100,
            Evaluator {
                ast: &Ast {
                    root: Expr::Integer(100),
                },
            }
            .eval()
            .unwrap()
        );
    }

    #[test]
    fn basic_addition() {
        assert_eq!(
            3,
            Evaluator {
                ast: &Ast {
                    root: Expr::Binary {
                        operator: BinOp::Add,
                        lhs: Box::new(Expr::Integer(1)),
                        rhs: Box::new(Expr::Integer(2)),
                    },
                },
            }
            .eval()
            .unwrap()
        );
    }
}
