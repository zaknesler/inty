use crate::core::*;

pub struct Parser<'a> {
    pub tokens: &'a Vec<Token>,
    pub position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Parse a list of tokens into an AST
    pub fn parse(&mut self) -> anyhow::Result<Ast> {
        let root = self.parse_expr()?;

        Ok(Ast { root })
    }

    /// Recursively parse an expression
    fn parse_expr(&mut self) -> anyhow::Result<Expr> {
        let mut lhs = self.parse_term()?;

        // Continue expanding the left-hand side as long as there is another
        // binary operation to perform. This ensures left associativity.
        while self.position < self.tokens.len() {
            let operator = self.tokens[self.position].clone();

            match operator {
                Token::Plus | Token::Hyphen => {
                    self.position += 1;
                    let rhs = self.parse_term()?;

                    lhs = Expr::Binary {
                        operator: operator.into(),
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    };
                }

                _ => break,
            }
        }

        Ok(lhs)
    }

    /// Recursively parse a term
    fn parse_term(&mut self) -> anyhow::Result<Expr> {
        let lhs = self.parse_exp()?;

        if self.position >= self.tokens.len() {
            return Ok(lhs);
        }

        let operator = self.tokens[self.position].clone();
        match operator {
            Token::Star | Token::Divide => {
                self.position += 1;
                let rhs = self.parse_term()?;

                Ok(Expr::Binary {
                    operator: operator.into(),
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
            }

            _ => Ok(lhs),
        }
    }

    /// Recursively parse an exponentiation
    fn parse_exp(&mut self) -> anyhow::Result<Expr> {
        let lhs = self.parse_factor()?;

        if self.position >= self.tokens.len() {
            return Ok(lhs);
        }

        let operator = self.tokens[self.position].clone();
        match operator {
            Token::Caret => {
                self.position += 1;
                let rhs = self.parse_exp()?;

                Ok(Expr::Binary {
                    operator: operator.into(),
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                })
            }

            _ => Ok(lhs),
        }
    }

    /// Parse a single factor
    fn parse_factor(&mut self) -> anyhow::Result<Expr> {
        let token = self.tokens[self.position].clone();
        match token {
            Token::Integer(value) => {
                self.position += 1;
                Ok(Expr::Integer(value))
            }

            Token::Hyphen | Token::Plus => {
                self.position += 1;
                let expr = self.parse_exp()?;

                Ok(Expr::Unary {
                    operator: UnOp::from(token),
                    value: Box::new(expr),
                })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_integer() {
        let ast = Parser::new(&vec![Token::Integer(1)]).parse().unwrap();

        assert_eq!(
            Ast {
                root: Expr::Integer(1)
            },
            ast
        );
    }

    #[test]
    fn parsing_unary_operators() {
        assert_eq!(
            Ast {
                root: Expr::Unary {
                    operator: UnOp::Plus,
                    value: Box::new(Expr::Integer(1))
                }
            },
            Parser::new(&vec![Token::Plus, Token::Integer(1)])
                .parse()
                .unwrap()
        );

        assert_eq!(
            Ast {
                root: Expr::Unary {
                    operator: UnOp::Minus,
                    value: Box::new(Expr::Integer(1))
                }
            },
            Parser::new(&vec![Token::Hyphen, Token::Integer(1)])
                .parse()
                .unwrap()
        );
    }

    #[test]
    fn parsing_binary_expression() {
        let ast = Parser::new(&vec![Token::Integer(1), Token::Plus, Token::Integer(2)])
            .parse()
            .unwrap();

        assert_eq!(
            Ast {
                root: Expr::Binary {
                    operator: BinOp::Add,
                    lhs: Box::new(Expr::Integer(1)),
                    rhs: Box::new(Expr::Integer(2))
                }
            },
            ast
        );
    }
}
