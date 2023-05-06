use crate::{ast, token::Token};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn parse(&mut self) -> anyhow::Result<ast::Ast> {
        let root = self.parse_expr()?;

        Ok(ast::Ast { root })
    }

    fn parse_expr(&mut self) -> anyhow::Result<ast::Expr> {
        let lhs = self.parse_term()?;

        if self.position >= self.tokens.len() {
            return Ok(lhs);
        }

        let operator = self.tokens[self.position].clone();
        match operator {
            Token::PlusSign | Token::MinusSign => {
                self.position += 1;
                let rhs = self.parse_expr()?;

                Ok(ast::Expr::Binary {
                    operator: operator.into(),
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
            }
            _ => Ok(lhs),
        }
    }

    fn parse_term(&mut self) -> anyhow::Result<ast::Expr> {
        let lhs = self.parse_factor()?;

        if self.position >= self.tokens.len() {
            return Ok(lhs);
        }

        let operator = self.tokens[self.position].clone();
        match operator {
            Token::TimesSign | Token::DivideSign => {
                self.position += 1;
                let rhs = self.parse_term()?;
                Ok(ast::Expr::Binary {
                    operator: operator.into(),
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
            }
            _ => Ok(lhs),
        }
    }

    fn parse_factor(&mut self) -> anyhow::Result<ast::Expr> {
        let token = self.tokens[self.position].clone();
        match token {
            Token::Integer(value) => {
                self.position += 1;
                Ok(ast::Expr::Number(value))
            }
            Token::LeftParen => {
                self.position += 1;
                let expr = self.parse_expr()?;
                if self.position >= self.tokens.len() {
                    anyhow::bail!("Expected right parenthesis");
                }
                if let Token::RightParen = self.tokens[self.position] {
                    self.position += 1;
                    Ok(expr)
                } else {
                    anyhow::bail!("Expected right parenthesis");
                }
            }
            _ => anyhow::bail!("Unexpected token: {}", token),
        }
    }
}
