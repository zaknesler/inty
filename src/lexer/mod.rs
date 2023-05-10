use crate::core::*;

pub struct Lexer {}

impl Lexer {
    /// Parse a string into a vector of valid tokens
    pub fn tokenize(input: String) -> anyhow::Result<Vec<Token>> {
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
                    Token::Integer(number.parse::<i32>()?)
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
                ';' => Token::Semicolon,
                '+' => Token::Plus,
                '-' => Token::Hyphen,
                '*' => Token::Star,
                '/' => Token::Divide,
                '^' => Token::Caret,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                '=' => Token::Equal,
                _ => anyhow::bail!(Error::TokenParsingError {
                    character: ch,
                    message: "Unknown token".to_string()
                }),
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
        let tokens = Lexer::tokenize("]".into());

        assert!(tokens.is_err());
        assert_eq!(
            tokens.unwrap_err().to_string(),
            "Token parsing error: Unknown token: ]".to_string()
        );
    }
}
