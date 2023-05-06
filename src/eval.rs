use crate::ast::{Ast, BinOp, Expr};

pub struct Evaluator<'a> {
    ast: &'a Ast,
}

impl<'a> Evaluator<'a> {
    pub fn new(ast: &'a Ast) -> Self {
        Self { ast }
    }

    pub fn eval(&self) -> i32 {
        self.eval_node(&self.ast.root)
    }

    fn eval_node(&self, node: &Expr) -> i32 {
        match node.clone() {
            Expr::Number(val) => *val,
            Expr::Binary { operator, lhs, rhs } => match operator {
                BinOp::Add => self.eval_node(lhs.as_ref()) + self.eval_node(rhs.as_ref()),
                BinOp::Sub => self.eval_node(lhs.as_ref()) - self.eval_node(rhs.as_ref()),
                BinOp::Mul => self.eval_node(lhs.as_ref()) * self.eval_node(rhs.as_ref()),
                BinOp::Div => self.eval_node(lhs.as_ref()) / self.eval_node(rhs.as_ref()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        assert_eq!(
            100,
            Evaluator {
                ast: &Ast {
                    root: Expr::Number(100),
                },
            }
            .eval()
        );
    }

    #[test]
    fn test_basic_addition() {
        assert_eq!(
            3,
            Evaluator {
                ast: &Ast {
                    root: Expr::Binary {
                        operator: BinOp::Add,
                        lhs: Box::new(Expr::Number(1)),
                        rhs: Box::new(Expr::Number(2)),
                    },
                },
            }
            .eval()
        );
    }
}
