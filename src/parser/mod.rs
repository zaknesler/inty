use crate::core::*;
use std::rc::Rc;

pub struct Parser<'a> {
    pub tokens: &'a [Token],
    pub position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Parse a list of tokens into an AST
    pub fn parse(&mut self) -> anyhow::Result<Vec<Stmt>> {
        let mut statements = Vec::new();

        while self.has_more_tokens() {
            let statement = self.parse_statement()?;
            statements.push(statement);

            // Check for semicolon to separate statements
            if self.has_more_tokens() && self.clone_current()? == Token::Semicolon {
                self.advance();
            } else {
                break;
            }
        }

        // If there are more tokens remaining after a successful parse, they must be invalid.
        // e.g. "2 + 3 4 + 5" will not have parsed "4 + 5"
        if self.has_more_tokens() {
            anyhow::bail!(Error::InvalidExpressionError {
                message: "Tokens remaining after parsing".to_string()
            })
        }

        Ok(statements)
    }

    /// Parse a single statement
    fn parse_statement(&mut self) -> anyhow::Result<Stmt> {
        match self.clone_current()? {
            Token::Let => {
                self.advance();
                if let Token::Ident(ident) = self.clone_current()? {
                    self.advance();
                    self.consume(Token::Equal)?;

                    Ok(Stmt::Let {
                        ident,
                        expr: self.parse_expr()?,
                    })
                } else {
                    anyhow::bail!("Expected identifier");
                }
            }
            _ => Ok(Stmt::Expr(self.parse_expr()?)),
        }
    }

    /// Recursively parse an expression
    fn parse_expr(&mut self) -> anyhow::Result<Expr> {
        let mut lhs = self.parse_mult()?;

        // Ensure left-associativity by expanding LHS as long as there is another operation
        while self.has_more_tokens() {
            let operator = self.clone_current()?;
            match operator {
                Token::Plus | Token::Hyphen => {
                    self.advance();
                    let rhs = self.parse_mult()?;

                    lhs = Expr::Binary {
                        operator: operator.into(),
                        lhs: Rc::new(lhs),
                        rhs: Rc::new(rhs),
                    };
                }

                _ => break,
            }
        }

        Ok(lhs)
    }

    /// Recursively parse a term
    fn parse_mult(&mut self) -> anyhow::Result<Expr> {
        let mut lhs = self.parse_pow()?;

        // Ensure left-associativity by expanding LHS as long as there is another operation
        while self.has_more_tokens() {
            let operator = self.clone_current()?;
            match operator {
                Token::Star | Token::Divide => {
                    self.advance();
                    let rhs = self.parse_pow()?;

                    lhs = Expr::Binary {
                        operator: operator.into(),
                        lhs: Rc::new(lhs),
                        rhs: Rc::new(rhs),
                    };
                }

                _ => break,
            }
        }

        Ok(lhs)
    }

    /// Recursively parse an exponentiation
    fn parse_pow(&mut self) -> anyhow::Result<Expr> {
        let lhs = self.parse_unary()?;

        if !self.has_more_tokens() {
            return Ok(lhs);
        }

        let operator = self.clone_current()?;
        match operator {
            Token::Caret => {
                self.advance();
                let rhs = self.parse_pow()?;

                Ok(Expr::Binary {
                    operator: operator.into(),
                    lhs: Rc::new(lhs),
                    rhs: Rc::new(rhs),
                })
            }

            _ => Ok(lhs),
        }
    }

    /// Parse a single factor
    fn parse_unary(&mut self) -> anyhow::Result<Expr> {
        let token = self.clone_current()?;
        match token {
            Token::Integer(value) => {
                self.advance();
                Ok(Expr::Integer(value))
            }

            Token::Ident(ident) => {
                self.advance();
                Ok(Expr::Ident(ident))
            }

            Token::Hyphen | Token::Plus => {
                self.advance();
                let expr = self.parse_pow()?;

                Ok(Expr::Unary {
                    operator: UnOp::from(token),
                    value: Rc::new(expr),
                })
            }

            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expr()?;

                if !self.has_more_tokens() {
                    anyhow::bail!(Error::SyntaxError {
                        token: None,
                        message: "Expected right parenthesis".to_string()
                    });
                }

                if let Token::RightParen = self.tokens[self.position] {
                    self.advance();
                    Ok(expr)
                } else {
                    anyhow::bail!(Error::SyntaxError {
                        token: None,
                        message: "Expected right parenthesis".to_string()
                    });
                }
            }

            _ => anyhow::bail!(Error::SyntaxError {
                token: Some(token),
                message: "Unexpected token".to_string()
            }),
        }
    }

    /// Get a cloned instance of the current token
    fn clone_current(&self) -> anyhow::Result<Token> {
        if !self.has_more_tokens() {
            anyhow::bail!(Error::SyntaxError {
                token: Some(self.tokens[self.position - 1].clone()),
                message: "Unexpected token".to_string()
            })
        }

        Ok(self.tokens[self.position].clone())
    }

    /// Move onto the next token
    fn advance(&mut self) {
        self.position += 1;
    }

    /// Are there still tokens remaining?
    fn has_more_tokens(&self) -> bool {
        self.position < self.tokens.len()
    }

    /// Match the current token to the expected token, error otherwise
    fn consume(&mut self, expected: Token) -> anyhow::Result<Token> {
        let found = self.clone_current()?;

        if expected == found {
            self.advance();
            return Ok(found);
        }

        anyhow::bail!(Error::ExpectedTokenError { expected, found })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_integer() {
        assert_eq!(
            vec![Stmt::Expr(Expr::Integer(1))],
            Parser::new(&vec![Token::Integer(1)]).parse().unwrap()
        );
    }

    #[test]
    fn parsing_unary_operators() {
        assert_eq!(
            vec![Stmt::Expr(Expr::Unary {
                operator: UnOp::Plus,
                value: Rc::new(Expr::Integer(1))
            })],
            Parser::new(&vec![Token::Plus, Token::Integer(1)])
                .parse()
                .unwrap()
        );

        assert_eq!(
            vec![Stmt::Expr(Expr::Unary {
                operator: UnOp::Minus,
                value: Rc::new(Expr::Integer(1))
            })],
            Parser::new(&vec![Token::Hyphen, Token::Integer(1)])
                .parse()
                .unwrap()
        );
    }

    #[test]
    fn parsing_binary_expression() {
        assert_eq!(
            vec![Stmt::Expr(Expr::Binary {
                operator: BinOp::Add,
                lhs: Rc::new(Expr::Integer(1)),
                rhs: Rc::new(Expr::Integer(2))
            })],
            Parser::new(&vec![Token::Integer(1), Token::Plus, Token::Integer(2)])
                .parse()
                .unwrap()
        );
    }

    #[test]
    fn parsing_multiplication_expression() {
        assert_eq!(
            vec![Stmt::Expr(Expr::Binary {
                operator: BinOp::Mul,
                lhs: Rc::new(Expr::Binary {
                    operator: BinOp::Mul,
                    lhs: Rc::new(Expr::Integer(2)),
                    rhs: Rc::new(Expr::Integer(3)),
                }),
                rhs: Rc::new(Expr::Integer(4)),
            })],
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
            vec![Stmt::Expr(Expr::Binary {
                operator: BinOp::Pow,
                lhs: Rc::new(Expr::Integer(2)),
                rhs: Rc::new(Expr::Binary {
                    operator: BinOp::Pow,
                    lhs: Rc::new(Expr::Integer(3)),
                    rhs: Rc::new(Expr::Binary {
                        operator: BinOp::Pow,
                        lhs: Rc::new(Expr::Integer(4)),
                        rhs: Rc::new(Expr::Integer(5)),
                    }),
                }),
            })],
            Parser::new(&vec![
                Token::Integer(2),
                Token::Caret,
                Token::Integer(3),
                Token::Caret,
                Token::Integer(4),
                Token::Caret,
                Token::Integer(5),
            ])
            .parse()
            .unwrap()
        );
    }

    #[test]
    fn parsing_complex_expression() {
        assert_eq!(
            vec![Stmt::Expr(Expr::Binary {
                operator: BinOp::Add,
                lhs: Rc::new(Expr::Integer(1)),
                rhs: Rc::new(Expr::Binary {
                    operator: BinOp::Mul,
                    lhs: Rc::new(Expr::Integer(2)),
                    rhs: Rc::new(Expr::Binary {
                        operator: BinOp::Pow,
                        lhs: Rc::new(Expr::Integer(3)),
                        rhs: Rc::new(Expr::Integer(4)),
                    }),
                }),
            })],
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
            vec![Stmt::Expr(Expr::Binary {
                operator: BinOp::Mul,
                lhs: Rc::new(Expr::Binary {
                    operator: BinOp::Add,
                    lhs: Rc::new(Expr::Integer(1)),
                    rhs: Rc::new(Expr::Integer(2)),
                }),
                rhs: Rc::new(Expr::Integer(3)),
            })],
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

    #[test]
    fn lone_token_syntax_error() {
        [
            Token::Plus,
            Token::Hyphen,
            Token::Star,
            Token::Divide,
            Token::Caret,
            Token::LeftParen,
            Token::RightParen,
        ]
        .into_iter()
        .for_each(|token| {
            let ast = Parser::new(&vec![token.clone()]).parse();

            assert!(ast.is_err());
            assert_eq!(
                ast.unwrap_err().to_string(),
                format!("Syntax error: Unexpected token: {}", token.to_string())
            );
        })
    }
}
