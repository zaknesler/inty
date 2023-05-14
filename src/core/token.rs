use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Values
    Integer(i32),
    Ident(String),

    // Keywords
    If,
    Else,
    Let,
    True,
    False,

    // Logic
    Or,
    And,

    // Relational
    RelEq,
    RelNe,
    RelGt,
    RelLt,
    RelGte,
    RelLte,

    // Math
    Plus,
    Hyphen,
    Star,
    Divide,
    Caret,

    // Brackets
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Misc
    Bang,
    Equal,
    Semicolon,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Integer(value) => write!(f, "{}", value),
            Token::Ident(value) => write!(f, "{}", value),

            // Keywords
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Let => write!(f, "let"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),

            // Logic
            Token::Or => write!(f, "||"),
            Token::And => write!(f, "&&"),

            // Relational
            Token::RelEq => write!(f, "=="),
            Token::RelNe => write!(f, "!="),
            Token::RelGt => write!(f, ">"),
            Token::RelLt => write!(f, "<"),
            Token::RelGte => write!(f, ">="),
            Token::RelLte => write!(f, "<="),

            // Math
            Token::Plus => write!(f, "+"),
            Token::Hyphen => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Caret => write!(f, "^"),

            // Brackets
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),

            // Misc
            Token::Bang => write!(f, "!"),
            Token::Semicolon => write!(f, ";"),
            Token::Equal => write!(f, "="),
        }
    }
}

impl Token {
    /// Attempt to map a reserved keyword to its token
    pub fn map_keyword(text: &str) -> Option<Self> {
        Some(match text {
            "if" => Token::If,
            "else" => Token::Else,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            _ => return None,
        })
    }
}
