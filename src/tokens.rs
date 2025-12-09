//!Gives a simple token structure for creating tokens and token types 
#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'a> {
    Number(f64),
    Str(&'a str),
    Bool(bool),
    Nil,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    /// Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    /// One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    /// Literals
    Identifier,
    Str,
    Number,

    /// Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token<'b> {
    pub token_type: TokenType,
    pub lexeme: &'b str,
    pub literal: Option<Literal<'b>>,
    pub line: usize,
}

impl<'b> Token<'b> {
    #[must_use]
    pub fn new(
        token_type: TokenType,
        lexeme:&'b str ,
        literal: Option<Literal<'b>>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

use std::fmt;

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenType::LeftParen => "TOKEN_LEFT_PAREN",
            TokenType::RightParen => "TOKEN_RIGHT_PAREN",
            TokenType::LeftBrace => "TOKEN_LEFT_BRACE",
            TokenType::RightBrace => "TOKEN_RIGHT_BRACE",
            TokenType::Comma => "TOKEN_COMMA",
            TokenType::Dot => "TOKEN_DOT",
            TokenType::Minus => "TOKEN_MINUS",
            TokenType::Plus => "TOKEN_PLUS",
            TokenType::Semicolon => "TOKEN_SEMICOLON",
            TokenType::Slash => "TOKEN_SLASH",
            TokenType::Star => "TOKEN_STAR",

            TokenType::Bang => "TOKEN_BANG",
            TokenType::BangEqual => "TOKEN_BANG_EQUAL",
            TokenType::Equal => "TOKEN_EQUAL",
            TokenType::EqualEqual => "TOKEN_EQUAL_EQUAL",
            TokenType::Greater => "TOKEN_GREATERTHAN",
            TokenType::GreaterEqual => "TOKEN_GREATERTHAN_EQUAL",
            TokenType::Less => "TOKEN_LESSTHAN",
            TokenType::LessEqual => "TOKEN_LESSTHAN_EQUAL",

            TokenType::Identifier => "IDENTIFIER",
            TokenType::Str => "STRING",
            TokenType::Number => "NUMBER",

            TokenType::And => "AND",
            TokenType::Class => "CLASS",
            TokenType::Else => "ELSE",
            TokenType::False => "FALSE",
            TokenType::Fun => "FUN",
            TokenType::For => "FOR",
            TokenType::If => "IF",
            TokenType::Nil => "NIL",
            TokenType::Or => "OR",
            TokenType::Print => "PRINT",
            TokenType::Return => "RETURN",
            TokenType::True => "TRUE",
            TokenType::Var => "VAR",
            TokenType::While => "WHILE",

            TokenType::Eof => "EOF",
        };
        write!(f, "{s}")
    }
}


