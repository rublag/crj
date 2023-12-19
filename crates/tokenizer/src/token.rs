use crate::characters;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    Error,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Metadata,
    Dispatch,
    Quote,
    Deref,
    Comment,
    Character,
    SynQuote,
    Unquote,
    UnquoteSplicing,
    Keyword,
    Symbol,
    Whitespace,
    String
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Token<'a> {
    pub data: &'a str,
    pub kind: TokenType,
}

impl<'a> Token<'a> {
    pub fn new(data: &'a str, kind: TokenType) -> Token<'a> {
        Token { data, kind }
    }
}

impl From<characters::SimpleControl> for TokenType {
    fn from(value: characters::SimpleControl) -> Self {
        match value {
            characters::SimpleControl::Hash => TokenType::Dispatch,
            characters::SimpleControl::Quote => TokenType::Quote,
        }
    }
}

impl From<characters::SimpleStructural> for TokenType {
    fn from(value: characters::SimpleStructural) -> Self {        
        match value {
            characters::SimpleStructural::LPar => TokenType::LParen,
            characters::SimpleStructural::RPar => TokenType::RParen,
            characters::SimpleStructural::LBrace => TokenType::LBrace,
            characters::SimpleStructural::RBrace => TokenType::RBrace,
            characters::SimpleStructural::LBracket => TokenType::LBrace,
            characters::SimpleStructural::RBracket => TokenType::RBrace,
            characters::SimpleStructural::Backtick => TokenType::SynQuote,
            characters::SimpleStructural::Caret => TokenType::Metadata,
            characters::SimpleStructural::At => TokenType::Deref,
        }
    }
}