use crate::core::*;

pub struct Lexer {}

impl Lexer {
    /// Parse a string into a vector of valid tokens
    pub fn tokenize(input: String) -> IntyResult<Vec<Token>> {
        let mut tokens = vec![];

        let mut chars = input.chars().peekable();
        while let Some(ch) = chars.next() {
            tokens.push(match ch {
                ' ' | '\t' | '\r' | '\n' => continue,
                '0'..='9' => {
                    let mut number = ch.to_string();
                    while let Some('0'..='9') = chars.peek() {
                        number.push(chars.next().expect("we are peeking ahead so this is safe"));
                    }
                    Token::Integer(number.parse()?)
                }
                'a'..='z' => {
                    let mut ident = ch.to_string();
                    while let Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_') = chars.peek() {
                        ident.push(chars.next().expect("we are peeking ahead so this is safe"));
                    }

                    match Token::map_keyword(ident.as_ref()) {
                        Some(keyword) => keyword,
                        None => Token::Ident(ident),
                    }
                }
                '+' => Token::Plus,
                '-' => Token::Hyphen,
                '*' => Token::Star,
                '/' => Token::Divide,
                '^' => Token::Caret,
                '=' => {
                    if let Some('=') = chars.peek() {
                        chars.next();
                        Token::RelEq
                    } else {
                        Token::Equal
                    }
                }
                '!' => {
                    if let Some('=') = chars.peek() {
                        chars.next();
                        Token::RelNe
                    } else {
                        Token::Bang
                    }
                }
                '<' => {
                    if let Some('=') = chars.peek() {
                        chars.next();
                        Token::RelLte
                    } else {
                        Token::RelLt
                    }
                }
                '>' => {
                    if let Some('=') = chars.peek() {
                        chars.next();
                        Token::RelGte
                    } else {
                        Token::RelGt
                    }
                }
                '&' => {
                    if let Some('&') = chars.next() {
                        Token::And
                    } else {
                        return Err(IntyError::UnexpectedChar { character: ch });
                    }
                }
                '|' => {
                    if let Some('|') = chars.next() {
                        Token::Or
                    } else {
                        return Err(IntyError::UnexpectedChar { character: ch });
                    }
                }
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                '[' => Token::LeftBracket,
                ']' => Token::RightBracket,
                '{' => Token::LeftBrace,
                '}' => Token::RightBrace,
                ';' => Token::Semicolon,
                ',' => Token::Comma,
                _ => return Err(IntyError::UnexpectedChar { character: ch }),
            });
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize() {
        let tokens = Lexer::tokenize("1 + 2 - 3 * 4 / 5".into()).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Integer(1),
                Token::Plus,
                Token::Integer(2),
                Token::Hyphen,
                Token::Integer(3),
                Token::Star,
                Token::Integer(4),
                Token::Divide,
                Token::Integer(5),
            ]
        );
    }

    #[test]
    fn tokenize_paren() {
        let tokens = Lexer::tokenize("1 + (2 - 3) * 4 / 5".into()).unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Integer(1),
                Token::Plus,
                Token::LeftParen,
                Token::Integer(2),
                Token::Hyphen,
                Token::Integer(3),
                Token::RightParen,
                Token::Star,
                Token::Integer(4),
                Token::Divide,
                Token::Integer(5),
            ]
        );
    }

    #[test]
    fn tokenize_error() {
        let tokens = Lexer::tokenize("?".into());
        assert!(tokens.is_err());
    }
}
