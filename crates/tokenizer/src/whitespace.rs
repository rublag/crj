fn is_line_separator(c: char) -> bool {
    c == '\u{2028}' // Line Separator
}

fn is_space_separator(c: char) -> bool {
    match c {
        '\u{0020}' => true, // Space (SP)
        '\u{00A0}' => true, // No-Break Space (NBSP)
        '\u{1680}' => true, // Ogham Space Mark
        '\u{2000}' => true, // En Quad
        '\u{2001}' => true, // Em Quad
        '\u{2002}' => true, // En Space
        '\u{2003}' => true, // Em Space
        '\u{2004}' => true, // Three-Per-Em Space
        '\u{2005}' => true, // Four-Per-Em Space
        '\u{2006}' => true, // Six-Per-Em Space
        '\u{2007}' => true, // Figure Space
        '\u{2008}' => true, // Punctuation Space
        '\u{2009}' => true, // Thin Space
        '\u{200A}' => true, // Hair Space
        '\u{202F}' => true, // Narrow No-Break Space (NNBSP)
        '\u{205F}' => true, // Medium Mathematical Space (MMSP)
        '\u{3000}' => true, // Ideographic Space
        _ => false
    }
}

fn is_paragraph_separator(c: char) -> bool {
    c == '\u{2029}' // Paragraph Separator
}

/// Determines if a character is a Java whitespace.
/// From Java doc:
/// Determines if the specified character (Unicode code point) is white space according to Java. A character is a Java whitespace character if and only if it satisfies one of the following criteria:
///
/// It is a Unicode space character (SPACE_SEPARATOR, LINE_SEPARATOR, or PARAGRAPH_SEPARATOR) but is not also a non-breaking space ('\u00A0', '\u2007', '\u202F').
/// It is '\t', U+0009 HORIZONTAL TABULATION.
/// It is '\n', U+000A LINE FEED.
/// It is '\u000B', U+000B VERTICAL TABULATION.
/// It is '\f', U+000C FORM FEED.
/// It is '\r', U+000D CARRIAGE RETURN.
/// It is '\u001C', U+001C FILE SEPARATOR.
/// It is '\u001D', U+001D GROUP SEPARATOR.
/// It is '\u001E', U+001E RECORD SEPARATOR.
/// It is '\u001F', U+001F UNIT SEPARATOR.
fn is_java_whitespace(c: char) -> bool {
    if c == '\u{00a0}' || c == '\u{2007}' || c == '\u{202f}' {
        return false;
    }

    is_space_separator(c)
        || is_line_separator(c)
        || is_paragraph_separator(c)
        || c == '\t'
        || c == '\n'
        || c == '\u{000b}'
        || c == '\u{000c}'
        || c == '\r'
        || c == '\u{001c}'
        || c == '\u{001d}'
        || c == '\u{001e}'
        || c == '\u{001f}'
}

pub fn is_clojure_whitespace(c: char) -> bool {
    is_java_whitespace(c) || c == ','
}
