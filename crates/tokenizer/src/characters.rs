use crate::whitespace;

/// Characters with special meaning, but they can be a part of a symbol/keyword
pub enum SimpleControl {
    /// The `#` symbol (dispatch)
    Hash,
    /// The `'` symbol (quote)
    Quote,
}

pub enum ComplexControl {
    /// The ':' symbol (keyword)
    Colon,
}

/// Characters with special meaning which can't be a part of a symbol/keyword
pub enum SimpleStructural {
    /// The `(` symbol
    LPar,
    /// The `)` symbol
    RPar,
    /// The `{` symbol
    LBrace,
    /// The `}` symbol
    RBrace,
    /// The `[` symbol
    LBracket,
    /// The `]` symbol
    RBracket,
    /// The `` ` `` symbol (syntax quote)
    Backtick,
    /// The `^` symbol (meta)
    Caret,
    /// The `@` symbol (deref)
    At,
}

pub enum ComplexStructural {
    /// The `;` symbol (comment)
    Semicolon,
    /// The `\` symbol (character)
    Backslash,
    /// The `"` symbol (string)
    DoubleQuote,
    /// The `~` symbol (unquote)
    Tilde
}

pub fn parse_simple_control(c: char) -> Option<SimpleControl> {
    match c {
        '#' => Some(SimpleControl::Hash),
        '\'' => Some(SimpleControl::Quote),
        _ => None
    }
}

pub fn parse_complex_control(c: char) -> Option<ComplexControl> {
    match c {
        ':' => Some(ComplexControl::Colon),
        _ => None
    }
}

pub fn parse_simple_structural(c: char) -> Option<SimpleStructural> {
    match c {
        '(' => Some(SimpleStructural::LPar),
        ')' => Some(SimpleStructural::RPar),
        '{' => Some(SimpleStructural::LBrace),
        '}' => Some(SimpleStructural::RBrace),
        '[' => Some(SimpleStructural::LBracket),
        ']' => Some(SimpleStructural::RBracket),
        '`' => Some(SimpleStructural::Backtick),
        '^' => Some(SimpleStructural::Caret),
        '@' => Some(SimpleStructural::At),
        _ => None
    }
}

pub fn parse_complex_structural(c: char) -> Option<ComplexStructural> {
   match c { 
        ';' => Some(ComplexStructural::Semicolon),
        '\\' => Some(ComplexStructural::Backslash),
        '"' => Some(ComplexStructural::DoubleQuote),
        '~' => Some(ComplexStructural::Tilde),
        _ => None
   }
}

pub enum Char {
    Whitespace,
    SimpleControl(SimpleControl),
    ComplexControl(ComplexControl),
    SimpleStructural(SimpleStructural),
    ComplexStructural(ComplexStructural),
    Regular
}

pub fn parse(c: char) -> Char {
    if whitespace::is_clojure_whitespace(c) {
        return Char::Whitespace;
    }
    
    if let Some(c) = parse_simple_control(c) {
        return Char::SimpleControl(c);
    }

    if let Some(c) = parse_complex_control(c) {
        return Char::ComplexControl(c);
    }
    
    if let Some(c) = parse_simple_structural(c) {
        return Char::SimpleStructural(c);
    }

    if let Some(c) = parse_complex_structural(c) {
        return Char::ComplexStructural(c);
    }
    
    Char::Regular
}

pub fn is_simple_structural(c: char) -> bool {
    matches!(parse(c), Char::SimpleStructural(_))
}

pub fn is_complex_structural(c: char) -> bool {
    matches!(parse(c), Char::ComplexStructural(_))
}

pub fn is_structural(c: char) -> bool {
    is_simple_structural(c) || is_complex_structural(c)
}

pub fn is_simple_control(c: char) -> bool {
    matches!(parse(c), Char::SimpleControl(_))
}

pub fn is_complex_control(c: char) -> bool {
    matches!(parse(c), Char::ComplexControl(_))
}

pub fn is_control(c: char) -> bool {
    is_simple_control(c) || is_complex_control(c)
}

pub fn is_whitespace(c: char) -> bool {
    matches!(parse(c), Char::Whitespace)
}

pub fn is_regular(c: char) -> bool {
    matches!(parse(c), Char::Regular) 
}

pub fn is_word(c: char) -> bool {
    is_control(c) || is_regular(c)
}