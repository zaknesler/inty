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
        Ok(Ast {
            root: self.parse_add()?,
        })
    }

    /// Recursively parse an expression
    fn parse_add(&mut self) -> anyhow::Result<Expr> {
        let mut lhs = self.parse_mult()?;

        // Continue expanding the left-hand side as long as there is another
        // binary operation to perform. This ensures left associativity.
        while self.has_more_tokens() {
            let operator = self.clone_current();

            match operator {
                Token::Plus | Token::Hyphen => {
                    self.advance();
                    let rhs = self.parse_mult()?;

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
    fn parse_mult(&mut self) -> anyhow::Result<Expr> {
        let lhs = self.parse_pow()?;

        if !self.has_more_tokens() {
            return Ok(lhs);
        }

        let operator = self.clone_current();
        match operator {
            Token::Star | Token::Divide => {
                self.advance();
                let rhs = self.parse_mult()?;

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
    fn parse_pow(&mut self) -> anyhow::Result<Expr> {
        let lhs = self.parse_unary()?;

        if !self.has_more_tokens() {
            return Ok(lhs);
        }

        let operator = self.clone_current();
        match operator {
            Token::Caret => {
                self.advance();
                let rhs = self.parse_pow()?;

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
    fn parse_unary(&mut self) -> anyhow::Result<Expr> {
        let token = self.clone_current();
        match token {
            Token::Integer(value) => {
                self.advance();

                Ok(Expr::Integer(value))
            }

            Token::Hyphen | Token::Plus => {
                self.advance();
                let expr = self.parse_pow()?;

                Ok(Expr::Unary {
                    operator: UnOp::from(token),
                    value: Box::new(expr),
                })
            }

            Token::LeftParen => {
                self.advance();
                let expr = self.parse_add()?;

                if !self.has_more_tokens() {
                    anyhow::bail!("Expected right parenthesis");
                }

                if let Token::RightParen = self.tokens[self.position] {
                    self.advance();
                    Ok(expr)
                } else {
                    anyhow::bail!("Expected right parenthesis");
                }
            }

            _ => anyhow::bail!("Unexpected token: {}", token),
        }
    }

    /// Get a cloned instance of the current token
    fn clone_current(&self) -> Token {
        self.tokens[self.position].clone()
    }

    /// Move onto the next token
    fn advance(&mut self) {
        self.position += 1;
    }

    /// Are there still tokens remaining?
    fn has_more_tokens(&self) -> bool {
        self.position < self.tokens.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_integer() {
        assert_eq!(
            Ast {
                root: Expr::Integer(1)
            },
            Parser::new(&vec![Token::Integer(1)]).parse().unwrap()
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
        assert_eq!(
            Ast {
                root: Expr::Binary {
                    operator: BinOp::Add,
                    lhs: Box::new(Expr::Integer(1)),
                    rhs: Box::new(Expr::Integer(2))
                }
            },
            Parser::new(&vec![Token::Integer(1), Token::Plus, Token::Integer(2)])
                .parse()
                .unwrap()
        );
    }

    #[test]
    fn parsing_multiplication_expression() {
        assert_eq!(
            Ast {
                root: Expr::Binary {
                    operator: BinOp::Mul,
                    lhs: Box::new(Expr::Binary {
                        operator: BinOp::Mul,
                        lhs: Box::new(Expr::Integer(2)),
                        rhs: Box::new(Expr::Integer(3)),
                    }),
                    rhs: Box::new(Expr::Integer(4)),
                }
            },
            Parser::new(&vec![
                Token::Integer(2),
                Token::Star,
                Token::Integer(3),
                Token::Star,
                Token::Integer(4),
            ])
            .parse()
            .unwrap()
        );
    }

    #[test]
    fn parsing_exponentiation_expression() {
        assert_eq!(
            Ast {
                root: Expr::Binary {
                    operator: BinOp::Pow,
                    lhs: Box::new(Expr::Integer(2)),
                    rhs: Box::new(Expr::Binary {
                        operator: BinOp::Pow,
                        lhs: Box::new(Expr::Integer(3)),
                        rhs: Box::new(Expr::Integer(4)),
                    }),
                }
            },
            Parser::new(&vec![
                Token::Integer(2),
                Token::Caret,
                Token::Integer(3),
                Token::Caret,
                Token::Integer(4),
            ])
            .parse()
            .unwrap()
        );
    }

    #[test]
    fn parsing_complex_expression() {
        assert_eq!(
            Ast {
                root: Expr::Binary {
                    operator: BinOp::Add,
                    lhs: Box::new(Expr::Integer(1)),
                    rhs: Box::new(Expr::Binary {
                        operator: BinOp::Mul,
                        lhs: Box::new(Expr::Integer(2)),
                        rhs: Box::new(Expr::Binary {
                            operator: BinOp::Pow,
                            lhs: Box::new(Expr::Integer(3)),
                            rhs: Box::new(Expr::Integer(4)),
                        }),
                    }),
                }
            },
            Parser::new(&vec![
                Token::Integer(1),
                Token::Plus,
                Token::Integer(2),
                Token::Star,
                Token::Integer(3),
                Token::Caret,
                Token::Integer(4),
            ])
            .parse()
            .unwrap()
        );
    }

    #[test]
    fn parsing_expression_with_parentheses() {
        assert_eq!(
            Ast {
                root: Expr::Binary {
                    operator: BinOp::Mul,
                    lhs: Box::new(Expr::Binary {
                        operator: BinOp::Add,
                        lhs: Box::new(Expr::Integer(1)),
                        rhs: Box::new(Expr::Integer(2)),
                    }),
                    rhs: Box::new(Expr::Integer(3)),
                }
            },
            Parser::new(&vec![
                Token::LeftParen,
                Token::Integer(1),
                Token::Plus,
                Token::Integer(2),
                Token::RightParen,
                Token::Star,
                Token::Integer(3),
            ])
            .parse()
            .unwrap()
        );
    }
}
