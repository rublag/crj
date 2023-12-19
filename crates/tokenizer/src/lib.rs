use cursor::Cursor;

mod whitespace;
mod characters;
mod cursor;
pub mod token;

use token::{Token, TokenType};

pub struct Tokenizer<'a> {
    stream: &'a str,
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(value: &'a str) -> Self {
        Tokenizer {
            stream: value,
        }
    }
}

#[derive(Clone, Copy)]
enum StringState {
    Normal,
    Escape
}

impl<'a> Tokenizer<'a> {
    fn make_token(&mut self, pos: usize, kind: TokenType) -> Token<'a> {
        let (slice, rest) = self.stream.split_at(pos);
        self.stream = rest;
        Token::new(slice, kind)
    }
    
    fn make_token_after(&mut self, kind: TokenType, cursor: Cursor<'a>) -> Token<'a> {
        let (slice, rest) = cursor.split_after();
        self.stream = rest;
        Token::new(slice, kind)
    }
    
    fn make_token_before(&mut self, kind: TokenType, cursor: Cursor<'a>) -> Token<'a> {
        let (slice, rest) = cursor.split_before();
        self.stream = rest;
        Token::new(slice, kind)
    }
    
    fn make_token_all(&mut self, kind: TokenType) -> Token<'a> {
        let stream = std::mem::take(&mut self.stream);
        Token::new(stream, kind)
    }
    
    fn make_token_ascii(&mut self, kind: TokenType) -> Token<'a> {
        self.make_token(1, kind)
    }
    
    fn read_word(&mut self, mut cursor: Cursor<'a>, kind: TokenType) -> Token<'a> {
        while let Some(c) = cursor.next() {
            if characters::is_structural(c) || characters::is_whitespace(c) {
                return self.make_token_before(kind, cursor)
            }
        }
        
        return self.make_token_all(kind)
    }
    
    fn read_word_strict(&mut self, mut cursor: Cursor<'a>, kind: TokenType) -> Token<'a> {
        let Some(c) = cursor.next() else {
            return self.make_token_all(TokenType::Error)
        };
        
        if !characters::is_word(c) {
            return self.make_token_after(TokenType::Error, cursor)
        }
        
        self.read_word(cursor, kind)
    }
    
    fn read_symbol(&mut self, cursor: Cursor<'a>) -> Token<'a> {
        self.read_word(cursor, TokenType::Symbol)
    }

    fn read_keyword(&mut self, cursor: Cursor<'a>) -> Token<'a> {
        self.read_word_strict(cursor, TokenType::Keyword)
    }

    fn read_character(&mut self, mut cursor: Cursor<'a>) -> Token<'a> {
        let Some(_) = cursor.next() else {
            return self.make_token_all(TokenType::Error);
        };

        self.read_word(cursor, TokenType::Character)
    }

    fn read_comment(&mut self, mut cursor: Cursor<'a>) -> Token<'a> {
        // We read comment until whitespace or EOF
        while let Some(c) = cursor.next() {
            if c == '\n' {
                return self.make_token_after(TokenType::Comment, cursor)
            }
        }
        
        self.make_token_all(TokenType::Character)
    }

    fn read_unquote(&mut self, mut cursor: Cursor<'a>) -> Token<'a> {
        match cursor.next() {
            Some('@') => self.make_token_after(TokenType::UnquoteSplicing, cursor),
            Some(_) => self.make_token_before(TokenType::Unquote, cursor),
            None => self.make_token_all(TokenType::Unquote)
        }
    }

    fn read_whitespace(&mut self, mut cursor: Cursor<'a>) -> Token<'a> {
        while let Some(c) = cursor.next() {
            if !characters::is_whitespace(c) {
                return self.make_token_before(TokenType::Whitespace, cursor)
            }
        }
        
        return self.make_token_all(TokenType::Character)
    }

    
    fn read_string(&mut self, mut cursor: Cursor<'a>) -> Token<'a> {
        let mut state = StringState::Normal;

        while let Some(c) = cursor.next() {
            match (state, c) {
                (StringState::Normal, '\\') => {
                    state = StringState::Escape;
                },
                (StringState::Normal, '"') => {
                    return self.make_token_after(TokenType::String, cursor);
                },
                (StringState::Normal, _) => (),
                (StringState::Escape, _) => {
                    state = StringState::Normal;
                },
            };
        }

        self.make_token_all(TokenType::Error)
    }
    
    fn make_simple_control_token(&mut self, c: characters::SimpleControl) -> Token<'a> {
        self.make_token_ascii(TokenType::from(c))
    }

    fn make_simple_structural_token(&mut self, c: characters::SimpleStructural) -> Token<'a> {
        self.make_token_ascii(TokenType::from(c))
    }
    

    pub fn next(&mut self) -> Option<Token<'a>> {
        let mut cursor = cursor::Cursor::from(self.stream);
        
        match characters::parse(cursor.next()?) {
            characters::Char::Whitespace => Some(self.read_whitespace(cursor)),
            characters::Char::SimpleControl(c) => Some(self.make_simple_control_token(c)),
            characters::Char::SimpleStructural(c) => Some(self.make_simple_structural_token(c)),
            characters::Char::ComplexControl(c) => {
                match c {
                    characters::ComplexControl::Colon => Some(self.read_keyword(cursor)),
                }
            },
            characters::Char::ComplexStructural(c) => {
                match c {
                    characters::ComplexStructural::Backslash => Some(self.read_character(cursor)),
                    characters::ComplexStructural::DoubleQuote => Some(self.read_string(cursor)),
                    characters::ComplexStructural::Semicolon => Some(self.read_comment(cursor)),
                    characters::ComplexStructural::Tilde => Some(self.read_unquote(cursor))
                }
            },
            characters::Char::Regular => Some(self.read_symbol(cursor))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Tokenizer, Token, TokenType};
    
    fn onetok<'a>(s: &'a str) -> Option<Token<'a>> {
        let mut tokenizer: Tokenizer<'a> = Tokenizer::from(s);
        let token: Option<Token<'a>> = tokenizer.next();
        token
    }
    
    fn kw(s: &str) -> Option<Token> {
        Some(Token::new(s, TokenType::Keyword))
    }
    
    fn sym(s: &str) -> Option<Token> {
        Some(Token::new(s, TokenType::Symbol))
    }
    
    fn cljstr(s: &str) -> Option<Token> {
        Some(Token::new(s, TokenType::String))
    }
    
    fn err(s: &str) -> Option<Token> {
        Some(Token::new(s, TokenType::Error))
    }
    
    fn chr(s: &str) -> Option<Token> {
        Some(Token::new(s, TokenType::Character))
    }

    #[test]
    fn keywords() {
        assert_eq!(onetok(":abcd :abcd"), kw(":abcd"));
        assert_eq!(onetok("::abcd :abcd"), kw("::abcd"));
        assert_eq!(onetok(":ab/cd :abcd"), kw(":ab/cd"));
        assert_eq!(onetok(":a.b/cd :abcd"), kw(":a.b/cd"));
    }

    #[test]
    fn symbols() {
        assert_eq!(onetok("abcd :abcd"), sym("abcd"));
        assert_eq!(onetok("ab/cd :abcd"), sym("ab/cd"));
        assert_eq!(onetok("a.b/cd :abcd"), sym("a.b/cd"));
    }
    
    #[test]
    fn strings() {
        assert_eq!(onetok(r#""abc" :abcd"#), cljstr(r#""abc""#));
        assert_eq!(onetok(r#""abc" :abcd"#), cljstr(r#""abc""#));
        assert_eq!(onetok(r#""a\nbc" :abcd"#), cljstr(r#""a\nbc""#));
        assert_eq!(onetok(r#""a\nb\"c" :abcd"#), cljstr(r#""a\nb\"c""#));
        assert_eq!(onetok(r#""abc"#), err(r#""abc"#));
    }
    
    #[test]
    fn characters() {
        assert_eq!(onetok(r#"\hello" :abcd"#), chr(r#"\hello"#));
        assert_eq!(onetok(r#"\" :abcd"#), chr(r#"\""#));
        assert_eq!(onetok(r#"\"#), err(r#"\"#))
    }
}