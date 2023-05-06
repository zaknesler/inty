use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(i32),
    PlusSign,
    MinusSign,
    TimesSign,
    DivideSign,
    LeftParen,
    RightParen,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Integer(value) => write!(f, "{}", value),
            Token::PlusSign => write!(f, "+"),
            Token::MinusSign => write!(f, "-"),
            Token::TimesSign => write!(f, "*"),
            Token::DivideSign => write!(f, "/"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
        }
    }
}
