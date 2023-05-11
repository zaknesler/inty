use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Values
    Integer(i32),
    Ident(String),

    // Keywords
    Let,
    True,
    False,

    // Symbols
    Semicolon,
    Plus,
    Hyphen,
    Star,
    Divide,
    Caret,
    Equal,
    Bang,
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
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),

            Token::Semicolon => write!(f, ";"),
            Token::Plus => write!(f, "+"),
            Token::Hyphen => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Caret => write!(f, "^"),
            Token::Equal => write!(f, "="),
            Token::Bang => write!(f, "!"),
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
            "true" => Token::True,
            "false" => Token::False,
            _ => return None,
        })
    }
}
