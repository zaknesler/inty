use crate::token::Token;

pub struct Lexer {
    pub input: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input }
    }

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
                '+' => Token::PlusSign,
                '-' => Token::MinusSign,
                '*' => Token::TimesSign,
                '/' => Token::DivideSign,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                _ => anyhow::bail!("Unexpected token: {}", ch),
            });
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let lexer = Lexer::new("1 + 2 - 3 * 4 / 5".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Integer(1),
                Token::PlusSign,
                Token::Integer(2),
                Token::MinusSign,
                Token::Integer(3),
                Token::TimesSign,
                Token::Integer(4),
                Token::DivideSign,
                Token::Integer(5),
            ]
        );
    }

    #[test]
    fn test_tokenize_paren() {
        let lexer = Lexer::new("1 + (2 - 3) * 4 / 5".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::Integer(1),
                Token::PlusSign,
                Token::LeftParen,
                Token::Integer(2),
                Token::MinusSign,
                Token::Integer(3),
                Token::RightParen,
                Token::TimesSign,
                Token::Integer(4),
                Token::DivideSign,
                Token::Integer(5),
            ]
        );
    }

    #[test]
    fn test_tokenize_error() {
        let lexer = Lexer::new("1 + 2 - 3 * 4 / 5 + a".to_string());
        let tokens = lexer.tokenize();

        assert!(tokens.is_err());
        assert_eq!(
            tokens.unwrap_err().to_string(),
            "Unexpected token: a".to_string()
        );
    }
}
