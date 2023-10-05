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
    pub fn parse(&mut self) -> IntyResult<Vec<Stmt>> {
        let mut statements = Vec::new();

        while self.has_more_tokens() {
            let statement = self.parse_stmt()?;
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
            return Err(IntyError::InvalidExpressionError {
                message: "tokens remaining after parsing".to_string(),
            });
        }

        Ok(statements)
    }

    /// Parse a single statement
    fn parse_stmt(&mut self) -> IntyResult<Stmt> {
        Ok(match self.clone_current()? {
            Token::If => {
                self.advance();

                Stmt::If {
                    test: self.parse_or()?,
                    branch: Rc::new(self.parse_stmt()?),
                    else_branch: match self.peek() {
                        Some(Token::Else) => {
                            self.advance();
                            Some(Rc::new(self.parse_stmt()?))
                        }
                        _ => None,
                    },
                }
            }

            Token::Let => {
                self.advance();
                if let Token::Ident(ident) = self.clone_current()? {
                    self.advance();
                    self.consume(Token::Equal)?;

                    Stmt::Let {
                        ident,
                        expr: self.parse_or()?,
                    }
                } else {
                    return Err(IntyError::SyntaxError {
                        token: None,
                        message: "expected identifier".to_string(),
                    });
                }
            }

            Token::LeftBrace => {
                self.advance();

                if !self.has_more_tokens() {
                    return Err(IntyError::SyntaxError {
                        token: None,
                        message: "expected right brace".to_string(),
                    });
                }

                let mut stmts = vec![self.parse_stmt()?];

                while let Some(next) = self.peek() {
                    if next == &Token::Semicolon {
                        self.advance();
                        stmts.push(self.parse_stmt()?);
                    } else {
                        break;
                    }
                }

                self.consume(Token::RightBrace)?;

                Stmt::Block(stmts)
            }

            _ => Stmt::Expr(self.parse_or()?),
        })
    }

    fn parse_or(&mut self) -> IntyResult<Expr> {
        let mut lhs = self.parse_and()?;

        while self.has_more_tokens() {
            let operator = self.clone_current()?;
            match operator {
                Token::Or => {
                    self.advance();
                    let rhs = self.parse_and()?;

                    lhs = Expr::Logical {
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

    fn parse_and(&mut self) -> IntyResult<Expr> {
        let mut lhs = self.parse_rel()?;

        while self.has_more_tokens() {
            let operator = self.clone_current()?;
            match operator {
                Token::And => {
                    self.advance();
                    let rhs = self.parse_rel()?;

                    lhs = Expr::Logical {
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

    fn parse_rel(&mut self) -> IntyResult<Expr> {
        let mut lhs = self.parse_expr()?;

        while self.has_more_tokens() {
            let operator = self.clone_current()?;
            match operator {
                Token::RelEq
                | Token::RelNe
                | Token::RelGt
                | Token::RelLt
                | Token::RelGte
                | Token::RelLte => {
                    self.advance();
                    let rhs = self.parse_expr()?;

                    lhs = Expr::Relational {
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

    fn parse_expr(&mut self) -> IntyResult<Expr> {
        let mut lhs = self.parse_mult()?;

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

    fn parse_mult(&mut self) -> IntyResult<Expr> {
        let mut lhs = self.parse_pow()?;

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

    fn parse_pow(&mut self) -> IntyResult<Expr> {
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

    fn parse_unary(&mut self) -> IntyResult<Expr> {
        let token = self.clone_current()?;
        Ok(match token {
            Token::Integer(value) => {
                self.advance();
                Expr::Integer(value)
            }

            Token::Ident(ident) => {
                self.advance();
                Expr::Ident(ident)
            }

            Token::True => {
                self.advance();
                Expr::Bool(true)
            }

            Token::False => {
                self.advance();
                Expr::Bool(false)
            }

            Token::Bang => {
                self.advance();

                Expr::Unary {
                    operator: UnOp::from(token),
                    value: Rc::new(self.parse_unary()?),
                }
            }

            Token::Hyphen | Token::Plus => {
                self.advance();

                Expr::Unary {
                    operator: UnOp::from(token),
                    value: Rc::new(self.parse_pow()?),
                }
            }

            Token::LeftParen => {
                self.advance();

                if !self.has_more_tokens() {
                    return Err(IntyError::SyntaxError {
                        token: None,
                        message: "expected right parenthesis".to_string(),
                    });
                }

                let expr = self.parse_or()?;
                self.consume(Token::RightParen)?;

                expr
            }

            Token::LeftBracket => {
                self.advance();

                if !self.has_more_tokens() {
                    return Err(IntyError::SyntaxError {
                        token: None,
                        message: "expected right bracket".to_string(),
                    });
                }

                let mut values = Vec::new();

                while let Some(next) = self.peek() {
                    match &next {
                        Token::Comma => self.advance(),
                        Token::RightBracket => break,
                        _ => values.push(self.parse_or()?),
                    }
                }

                self.consume(Token::RightBracket)?;

                Expr::List(values)
            }

            _ => {
                return Err(IntyError::SyntaxError {
                    token: Some(token),
                    message: "unexpected token".to_string(),
                })
            }
        })
    }

    /// Get a cloned instance of the current token
    fn clone_current(&self) -> IntyResult<Token> {
        if !self.has_more_tokens() {
            return Err(IntyError::SyntaxError {
                token: Some(self.tokens[self.position - 1].clone()),
                message: "unexpected token".to_string(),
            });
        }

        Ok(self.tokens[self.position].clone())
    }

    fn peek(&self) -> Option<&Token> {
        if !self.has_more_tokens() {
            None
        } else {
            Some(&self.tokens[self.position])
        }
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
    fn consume(&mut self, expected: Token) -> IntyResult<Token> {
        let found = self.clone_current()?;

        if expected == found {
            self.advance();
            return Ok(found);
        }

        return Err(IntyError::ExpectedTokenError { expected, found });
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
        })
    }

    #[test]
    fn parsing_logical_and_expression() {
        assert_eq!(
            vec![Stmt::Expr(Expr::Logical {
                operator: LogOp::And,
                lhs: Rc::new(Expr::Bool(true)),
                rhs: Rc::new(Expr::Bool(false))
            })],
            Parser::new(&vec![Token::True, Token::And, Token::False,])
                .parse()
                .unwrap()
        );
    }

    #[test]
    fn parsing_logical_or_expression() {
        assert_eq!(
            vec![Stmt::Expr(Expr::Logical {
                operator: LogOp::Or,
                lhs: Rc::new(Expr::Bool(true)),
                rhs: Rc::new(Expr::Bool(false))
            })],
            Parser::new(&vec![Token::True, Token::Or, Token::False,])
                .parse()
                .unwrap()
        );
    }

    #[test]
    fn parsing_logical_and_or_expression() {
        assert_eq!(
            vec![Stmt::Expr(Expr::Logical {
                operator: LogOp::Or,
                lhs: Rc::new(Expr::Logical {
                    operator: LogOp::And,
                    lhs: Rc::new(Expr::Bool(true)),
                    rhs: Rc::new(Expr::Bool(false)),
                }),
                rhs: Rc::new(Expr::Logical {
                    operator: LogOp::And,
                    lhs: Rc::new(Expr::Bool(false)),
                    rhs: Rc::new(Expr::Bool(true)),
                }),
            })],
            Parser::new(&vec![
                Token::True,
                Token::And,
                Token::False,
                Token::Or,
                Token::False,
                Token::And,
                Token::True,
            ])
            .parse()
            .unwrap()
        );
    }

    #[test]
    fn parsing_logical_or_and_expression() {
        assert_eq!(
            vec![Stmt::Expr(Expr::Logical {
                operator: LogOp::Or,
                lhs: Rc::new(Expr::Logical {
                    operator: LogOp::Or,
                    lhs: Rc::new(Expr::Bool(true)),
                    rhs: Rc::new(Expr::Logical {
                        operator: LogOp::And,
                        lhs: Rc::new(Expr::Bool(false)),
                        rhs: Rc::new(Expr::Bool(false)),
                    }),
                }),
                rhs: Rc::new(Expr::Bool(true)),
            })],
            Parser::new(&vec![
                Token::True,
                Token::Or,
                Token::False,
                Token::And,
                Token::False,
                Token::Or,
                Token::True,
            ])
            .parse()
            .unwrap()
        );
    }
}
