use crate::ast::{Ast, BinOp, Expr, UnOp};

pub struct Evaluator<'a> {
    ast: &'a Ast,
}

impl<'a> Evaluator<'a> {
    pub fn new(ast: &'a Ast) -> Self {
        Self { ast }
    }

    /// Evaluate an AST to a single value
    pub fn eval(&self) -> i32 {
        self.eval_node(&self.ast.root)
    }

    /// Recursively evaluate a single expression node
    fn eval_node(&self, node: &Expr) -> i32 {
        match node.clone() {
            Expr::Integer(val) => *val,
            Expr::Unary { operator, value } => match operator {
                UnOp::Minus => -1 * self.eval_node(value),
                UnOp::Plus => self.eval_node(value),
            },
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
                    root: Expr::Integer(100),
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
                        lhs: Box::new(Expr::Integer(1)),
                        rhs: Box::new(Expr::Integer(2)),
                    },
                },
            }
            .eval()
        );
    }
}
