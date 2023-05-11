use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(i32),
    Ident(String),
    Let,
    Semicolon,
    Plus,
    Hyphen,
    Star,
    Divide,
    Caret,
    Equal,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Integer(value) => write!(f, "{}", value),
            Token::Ident(value) => write!(f, "{}", value),
            Token::Let => write!(f, "let"),
            Token::Semicolon => write!(f, ";"),
            Token::Plus => write!(f, "+"),
            Token::Hyphen => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Caret => write!(f, "^"),
            Token::Equal => write!(f, "="),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
        }
    }
}

impl Token {
    /// Attempt to map a reserved keyword to its token
    pub fn map_keyword(text: &str) -> Option<Self> {
        Some(match text {
            "let" => Token::Let,
            _ => return None,
        })
    }
}
