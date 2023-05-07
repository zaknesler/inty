use crate::core::*;

pub struct Lexer {
    pub input: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    /// Parse a string into a vector of valid tokens
    pub fn tokenize(&self) -> anyhow::Result<Vec<Token>> {
        let mut tokens = vec![];

        let mut chars = self.input.chars().peekable();
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
                '+' => Token::Plus,
                '-' => Token::Hyphen,
                '*' => Token::Star,
                '/' => Token::Divide,
                '^' => Token::Caret,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
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
        let lexer = Lexer::new("1 + 2 - 3 * 4 / 5".to_string());
        let tokens = lexer.tokenize().unwrap();

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
        let lexer = Lexer::new("1 + (2 - 3) * 4 / 5".to_string());
        let tokens = lexer.tokenize().unwrap();

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
        let lexer = Lexer::new("1 + 2 - 3 * 4 / 5 + a".to_string());
        let tokens = lexer.tokenize();

        assert!(tokens.is_err());
        assert_eq!(
            tokens.unwrap_err().to_string(),
            "Token parsing error: Unknown token: a".to_string()
        );
    }
}
