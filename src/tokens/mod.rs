use std::fmt;

use crate::errors::{self, tokens::TokenParseError};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Operator(Operator),
    Keyword(Keyword),
    Identifier(Identifier),
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
}

impl TryFrom<&str> for Token {
    type Error = TokenParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match Operator::try_from(value) {
            Ok(data) => return Ok(data.into()),
            Err(TokenParseError::NotValidToken) => {}
            Err(err) => return Err(err),
        };
        match Keyword::try_from(value) {
            Ok(data) => return Ok(data.into()),
            Err(TokenParseError::NotValidToken) => {}
            Err(err) => return Err(err),
        };
        match Identifier::try_from(value) {
            Ok(data) => return Ok(data.into()),
            Err(TokenParseError::NotValidToken) => {}
            Err(err) => return Err(err),
        };
        match NumberLiteral::try_from(value) {
            Ok(data) => return Ok(data.into()),
            Err(TokenParseError::NotValidToken) => {}
            Err(err) => return Err(err),
        };
        match StringLiteral::try_from(value) {
            Ok(data) => return Ok(data.into()),
            Err(TokenParseError::NotValidToken) => {}
            Err(err) => return Err(err),
        };
    }
}

impl From<Operator> for Token {
    fn from(val: Operator) -> Self {
        Token::Operator(val)
    }
}
impl From<Identifier> for Token {
    fn from(val: Identifier) -> Self {
        Token::Identifier(val)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteral {
    pub inner: std::string::String,
}

impl StringLiteral {
    pub fn new(inner: std::string::String) -> Self {
        Self { inner }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberLiteral {
    pub num: std::string::String,
}

impl NumberLiteral {
    pub fn new(num: std::string::String) -> Self {
        Self { num }
    }
}
impl From<StringLiteral> for Token {
    fn from(val: StringLiteral) -> Self {
        Token::StringLiteral(val)
    }
}

impl From<NumberLiteral> for Token {
    fn from(val: NumberLiteral) -> Self {
        Token::NumberLiteral(val)
    }
}

impl From<Keyword> for Token {
    fn from(val: Keyword) -> Self {
        Token::Keyword(val)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    ParenLeft,
    ParenRight,
    BraceLeft,
    BraceRight,
    Comma,
    Dot,
    Minus,
    Plus,
    Div,
    Mul,
    Semicolon,
    Not,
    NotEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEq,
    Smaller,
    SmallerEq,
}

impl TryFrom<&str> for Operator {
    type Error = TokenParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "(" => Ok(Operator::ParenLeft),
            ")" => Ok(Operator::ParenRight),
            "{" => Ok(Operator::BraceLeft),
            "}" => Ok(Operator::BraceRight),
            "," => Ok(Operator::Comma),
            "." => Ok(Operator::Dot),
            "-" => Ok(Operator::Minus),
            "+" => Ok(Operator::Plus),
            "/" => Ok(Operator::Div),
            "*" => Ok(Operator::Mul),
            ";" => Ok(Operator::Semicolon),
            "!" => Ok(Operator::Not),
            "!=" => Ok(Operator::NotEqual),
            "=" => Ok(Operator::Equal),
            "==" => Ok(Operator::EqualEqual),
            ">" => Ok(Operator::Greater),
            ">=" => Ok(Operator::GreaterEq),
            "<" => Ok(Operator::Smaller),
            "<=" => Ok(Operator::SmallerEq),
            _ => Err(TokenParseError::NotValidToken),
        }
    }
}
impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::ParenLeft => write!(f, "("),
            Operator::ParenRight => write!(f, ")"),
            Operator::BraceLeft => write!(f, "{{"),
            Operator::BraceRight => write!(f, "}}"),
            Operator::Comma => write!(f, ","),
            Operator::Dot => write!(f, "."),
            Operator::Minus => write!(f, "-"),
            Operator::Plus => write!(f, "+"),
            Operator::Div => write!(f, "/"),
            Operator::Mul => write!(f, "*"),
            Operator::Semicolon => write!(f, ";"),
            Operator::Not => write!(f, "!"),
            Operator::NotEqual => write!(f, "!="),
            Operator::Equal => write!(f, "="),
            Operator::EqualEqual => write!(f, "=="),
            Operator::Greater => write!(f, ">"),
            Operator::GreaterEq => write!(f, ">="),
            Operator::Smaller => write!(f, "<"),
            Operator::SmallerEq => write!(f, "<="),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub name: String,
}

impl TryFrom<&str> for Identifier {
    type Error = TokenParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value
            .chars()
            .next()
            .map(|x| x.is_alphabetic())
            .unwrap_or(false)
        {
            if value.chars().all(|x| x.is_alphanumeric()) {
                return Ok(Identifier { name: value.into() });
            }
        }
        return Err(TokenParseError::NotValidToken);
    }
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    For,
    If,
    Null,
    Or,
    Return,
    True,
    Let,
    While,
    Fn,
}

impl TryFrom<&str> for Keyword {
    type Error = TokenParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "and" => Ok(Keyword::And),
            "class" => Ok(Keyword::Class),
            "else" => Ok(Keyword::Else),
            "false" => Ok(Keyword::False),
            "for" => Ok(Keyword::For),
            "if" => Ok(Keyword::If),
            "null" => Ok(Keyword::Null),
            "or" => Ok(Keyword::Or),
            "return" => Ok(Keyword::Return),
            "true" => Ok(Keyword::True),
            "let" => Ok(Keyword::Let),
            "while" => Ok(Keyword::While),
            "fn" => Ok(Keyword::Fn),
            _ => Err(TokenParseError::NotValidToken),
        }
    }
}
