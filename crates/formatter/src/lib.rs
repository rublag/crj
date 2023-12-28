/// Very ugly ad-hoc formatter
/// Done in one day :(
/// To be rewritten
use tokenizer::Tokenizer;
use tokenizer::token::{Token, TokenType};

#[derive(Debug, Clone, Copy)]
struct Alignment {
    indent: usize,
    pos: usize,
    align: usize
}

impl Alignment {
    fn new() -> Self {
        Alignment { indent: 0, pos: 0, align: 0 }
    }
    
    fn increase(self, n: usize) -> Self {
        let mut new_alignment = self;
        new_alignment.indent = self.pos + n;
        new_alignment.pos = self.pos + n;
        new_alignment.align = self.pos + n;
        new_alignment
    }
    
    fn indent(self, n: usize) -> Self {
        let mut new_alignment = self;
        new_alignment.indent += n;
        new_alignment
    }
    
    fn align(self) -> Self {
        let mut new_alignment = self;
        new_alignment.align = self.pos;
        new_alignment
    }

    fn shift(self, n: usize) -> Self {
        let mut new_alignment = self;
        new_alignment.pos += n;
        new_alignment
    }
    
    fn set_indent(self, n: usize) -> Self {
        let mut new_alignment = self;
        new_alignment.indent = n;
        new_alignment
    }
    
    fn set_pos(self, n: usize) -> Self {
        let mut new_alignment = self;
        new_alignment.pos = n;
        new_alignment
    }
}

struct LookaheadCursor<'a> {
    tokenizer: Tokenizer<'a>,
    cur: Option<Token<'a>>,
    ahead1: Option<Token<'a>>,
    ahead2: Option<Token<'a>>
}

impl<'a> LookaheadCursor<'a> {
    fn next(&mut self) -> Option<Token<'a>> {
        self.cur = self.ahead1;
        self.ahead1 = self.ahead2;
        self.ahead2 = self.tokenizer.next();
        self.cur
    }
    
    fn current(&mut self) -> Option<Token<'a>> {
        self.cur
    }
    
    fn lookahead1(&mut self) -> Option<Token<'a>> {
        self.ahead1
    }
    
    fn lookahead2(&mut self) -> Option<Token<'a>> {
        self.ahead2
    }
    
    fn new(stream: &'a str) -> Self {
        let mut tokenizer = Tokenizer::from(stream);
        let mut this = LookaheadCursor {
            tokenizer: tokenizer,
            cur: None,
            ahead1: None,
            ahead2: None
        };
        // Get lookahead
        this.next();
        this.next();
        
        this
    }
}

fn format_whitespace(buf: &mut String, cursor: &mut LookaheadCursor, alignment: Alignment) -> Alignment {
    // Strip whitespace at the end of the file
    let Some(la1) = cursor.lookahead1() else {
        buf.push('\n');
        return alignment;
    };
    
    let s = cursor.current().unwrap().value();

    let Some(last_newline) = s.rfind('\n') else {
        // We don't have new lines 
        // and the next token is non-whitespace (by construction of tokenizer)
        // Just push whitespaces
        buf.push_str(s);
        return alignment.shift(s.len());
    };
    
    
    // + 1 to put newline in left part
    let (newlines, last_line) = s.split_at(last_newline + 1);
    let nl_count = newlines.chars().filter(|&c| c == '\n').count();
    
    for _ in 0..nl_count {
        buf.push('\n')
    }
    
    for _ in 0..alignment.indent {
        buf.push(' ')
    }
    
    if la1.kind() == TokenType::Comment {
        if !la1.value().starts_with(";;") {
            for _ in alignment.pos..40 {
                buf.push(' ');
            }
        }
    }
    
    alignment.set_pos(alignment.indent).align()
}

fn format_comment(buf: &mut String, cursor: &mut LookaheadCursor, alignment: Alignment) -> Alignment {
    let s = cursor.current().unwrap().value();
    
    buf.push_str(s);
    if !s.ends_with('\n') {
        buf.push('\n');
    }
    
    let Some(la1) = cursor.lookahead1() else {
        return alignment.set_pos(0);
    };
    
    if la1.kind() == TokenType::Whitespace {
        return alignment.set_pos(0);
    }

    for _ in 0..alignment.indent {
        buf.push(' ')
    }
    
    alignment.set_pos(alignment.indent).align()
}

fn format_vector(buf: &mut String, cursor: &mut LookaheadCursor, alignment: Alignment) -> Alignment {
    let tok = cursor.current().expect("Current token must be `[`");
    assert_eq!(tok.kind(), TokenType::LBracket);
    

    buf.push_str(tok.value());
    
    let mut next_alignment = alignment.increase(1);
    
    while let Some(tok) = cursor.next() {
        next_alignment = match tok.kind() {
            TokenType::RBracket => {
                buf.push_str(tok.value());
                return alignment;
            },
            TokenType::Whitespace => {
                format_whitespace(buf, cursor, next_alignment)
            }
            _ => {
                format(buf, cursor, next_alignment)
            }
        };
    }
    alignment
}

fn format_map(buf: &mut String, cursor: &mut LookaheadCursor, alignment: Alignment) -> Alignment {
    let tok = cursor.current().expect("Current token must be `[`");
    assert_eq!(tok.kind(), TokenType::LBrace);
    

    buf.push_str(tok.value());    
    
    let mut next_alignment = alignment.increase(1);
    
    while let Some(tok) = cursor.next() {
        next_alignment = match tok.kind() {
            TokenType::RBrace => {
                buf.push_str(tok.value());
                return alignment;
            },
            TokenType::Whitespace => {
                format_whitespace(buf, cursor, next_alignment)
            }
            _ => {
                format(buf, cursor, next_alignment)
            }
        };
    }
    alignment
}

fn format_word(buf: &mut String, cursor: &mut LookaheadCursor, alignment: Alignment) -> Alignment {
    buf.push_str(cursor.current().unwrap().value());
    alignment.shift(cursor.current().unwrap().value().len())
}

fn format_fn(buf: &mut String, cursor: &mut LookaheadCursor, alignment: Alignment) -> Alignment {
    let tok = cursor.current().expect("Current token must be `[`");
    assert_eq!(tok.kind(), TokenType::LParen);
    

    // '('
    buf.push_str(tok.value());    
    let mut inner_alignment = alignment.increase(1);
    cursor.next();
    
    // macro-name
    let mut body_alignment = format(buf, cursor, inner_alignment);
    cursor.next();
    
    body_alignment = format_ws_lax(buf, cursor, body_alignment);
    body_alignment = body_alignment.align();
    body_alignment = body_alignment.set_indent(body_alignment.align);
    

    let Some(tok) = cursor.current() else {
        return alignment;
    };

    body_alignment = match tok.kind() {
        TokenType::RParen => {
            buf.push_str(tok.value());
            return alignment;
        }
        _ => {
            format(buf, cursor, body_alignment)
        }
    };
    
    let mut next_alignment = body_alignment;
    
    while let Some(tok) = cursor.next() {
        next_alignment = match tok.kind() {
            TokenType::RParen => {
                buf.push_str(tok.value());
                return alignment;
            },
            TokenType::Whitespace => {
                format_whitespace(buf, cursor, next_alignment)
            }
            _ => {
                format(buf, cursor, next_alignment)
            }
        };
    }
    alignment
}

fn format_list(buf: &mut String, cursor: &mut LookaheadCursor, alignment: Alignment) -> Alignment {
    let tok = cursor.current().expect("Current token must be `[`");
    assert_eq!(tok.kind(), TokenType::LParen);
    

    buf.push_str(tok.value());    
    
    let mut next_alignment = alignment.increase(1);
    
    while let Some(tok) = cursor.next() {
        next_alignment = match tok.kind() {
            TokenType::RParen => {
                buf.push_str(tok.value());
                return alignment;
            },
            TokenType::Whitespace => {
                format_whitespace(buf, cursor, next_alignment)
            }
            _ => {
                format(buf, cursor, next_alignment)
            }
        };
    }
    alignment
}

fn format_ws_lax(buf: &mut String, cursor: &mut LookaheadCursor, mut alignment: Alignment) -> Alignment {
    let Some(ws) = cursor.current() else {
        return alignment;
    };
    
    if ws.kind() != TokenType::Whitespace {
        return alignment;
    }

    alignment = format_whitespace(buf, cursor, alignment);
    cursor.next();
    alignment
}

fn format_arg(buf: &mut String, cursor: &mut LookaheadCursor, mut alignment: Alignment) -> Alignment {
    let tok = cursor.current().expect("Current token must be present");

    match tok.kind() {
        TokenType::Dispatch => {
            // Consume dipatch
            alignment = format(buf, cursor, alignment);
            cursor.next();
            // Format whitespace if any
            alignment = format_ws_lax(buf, cursor, alignment);
            // Format dispatch tag
            alignment = format(buf, cursor, alignment);
            cursor.next();
            // Format whitespace if any
            alignment = format_ws_lax(buf, cursor, alignment);
            // Format argument
            return format_arg(buf, cursor, alignment)
        },
        TokenType::Metadata => {
            // Consume dipatch
            alignment = format(buf, cursor, alignment);
            cursor.next();
            // Format whitespace if any
            alignment = format_ws_lax(buf, cursor, alignment);
            // Format dispatch tag
            alignment = format(buf, cursor, alignment);
            cursor.next();
            // Format whitespace if any
            alignment = format_ws_lax(buf, cursor, alignment);
            // Format argument
            return format_arg(buf, cursor, alignment)
        },
        TokenType::Quote | TokenType::SynQuote => {
            // Consume dipatch
            alignment = format(buf, cursor, alignment);
            cursor.next();
            // Format whitespace if any
            alignment = format_ws_lax(buf, cursor, alignment);
            // Format argument
            return format_arg(buf, cursor, alignment)
        },
        _ => {
            return format(buf, cursor, alignment);
        }
    }
}

fn format_first_sparg(buf: &mut String, cursor: &mut LookaheadCursor, mut alignment: Alignment) -> Alignment {
    alignment = format_ws_lax(buf, cursor, alignment);
    alignment = alignment.align();

    let Some(tok) = cursor.current() else {
        return alignment;
    };

    match tok.kind() {
        TokenType::RParen => {
            return alignment;
        }
        _ => {
            alignment = format_arg(buf, cursor, alignment);
            cursor.next();
        }
    };
    alignment
}


fn format_one_sparg(buf: &mut String, cursor: &mut LookaheadCursor, mut alignment: Alignment) -> Alignment {
    alignment = format_ws_lax(buf, cursor, alignment);

    let Some(tok) = cursor.current() else {
        return alignment;
    };

    match tok.kind() {
        TokenType::RParen => {
            return alignment;
        }
        _ => {
            alignment = format(buf, cursor, alignment);
            cursor.next();
        }
    };
    alignment
}


fn format_sparg(buf: &mut String, cursor: &mut LookaheadCursor, alignment: Alignment, count: usize) -> Alignment {
    let tok = cursor.current().expect("Current token must be `[`");
    assert_eq!(tok.kind(), TokenType::LParen);
    

    // '('
    buf.push_str(tok.value());    
    let mut inner_alignment = alignment.increase(1);
    cursor.next();
    
    // macro-name
    let mut macro_alignment = format(buf, cursor, inner_alignment);
    cursor.next();
    
    let mut sparg_alignment = macro_alignment.indent(3);
    
    if count > 0 {
        sparg_alignment = format_first_sparg(buf, cursor, sparg_alignment);
    }
    
    for _ in 1..count {
        sparg_alignment = format_one_sparg(buf, cursor, sparg_alignment);
    }
    
    let mut body_alignment = sparg_alignment
        .set_indent(macro_alignment.indent)
        .indent(1);

    
    body_alignment = format_ws_lax(buf, cursor, body_alignment);
    body_alignment = body_alignment.set_indent(body_alignment.align);
    

    let Some(tok) = cursor.current() else {
        return alignment;
    };

    body_alignment = match tok.kind() {
        TokenType::RParen => {
            buf.push_str(tok.value());
            return alignment;
        }
        _ => {
            format(buf, cursor, body_alignment)
        }
    };
    
    let mut next_alignment = body_alignment;
    
    while let Some(tok) = cursor.next() {
        next_alignment = match tok.kind() {
            TokenType::RParen => {
                buf.push_str(tok.value());
                return alignment;
            },
            TokenType::Whitespace => {
                format_whitespace(buf, cursor, next_alignment)
            }
            _ => {
                format(buf, cursor, next_alignment)
            }
        };
    }
    alignment
}


fn format_defn(buf: &mut String, cursor: &mut LookaheadCursor, alignment: Alignment) -> Alignment {
    let tok = cursor.current().expect("Current token must be `[`");
    assert_eq!(tok.kind(), TokenType::LParen);
    

    buf.push_str(tok.value());    
    
    let mut next_alignment = alignment.indent(2).shift(1);
    
    while let Some(tok) = cursor.next() {
        next_alignment = match tok.kind() {
            TokenType::RParen => {
                buf.push_str(tok.value());
                return alignment;
            },
            TokenType::Whitespace => {
                format_whitespace(buf, cursor, next_alignment)
            }
            _ => {
                format(buf, cursor, next_alignment)
            }
        };
    }
    alignment
}

fn format_sexp(buf: &mut String, cursor: &mut LookaheadCursor, alignment: Alignment) -> Alignment {
    let tok = cursor.current().expect("Current token must be `[`");
    assert_eq!(tok.kind(), TokenType::LParen);
    

    let Some(la1) = cursor.lookahead1() else {
        panic!("'(' Not closed");
    };

    match la1.kind() {
        TokenType::Keyword => return format_fn(buf, cursor, alignment),
        TokenType::Symbol => (),
        _ => return format_list(buf, cursor, alignment)
    }
    
    let dispatch = la1.value();
    
    match dispatch {
        // zero sparg
        "alt!!" => format_sparg(buf, cursor, alignment, 0),
        "alt!" => format_sparg(buf, cursor, alignment, 0),
        "comment" => format_sparg(buf, cursor, alignment, 0),
        "cond" => format_sparg(buf, cursor, alignment, 0),
        "delay" => format_sparg(buf, cursor, alignment, 0),
        "do" => format_sparg(buf, cursor, alignment, 0),
        "finally" => format_sparg(buf, cursor, alignment, 0),
        "future" => format_sparg(buf, cursor, alignment, 0),
        "go" => format_sparg(buf, cursor, alignment, 0),
        "thread" => format_sparg(buf, cursor, alignment, 0),
        "try" => format_sparg(buf, cursor, alignment, 0),

        // one sparg
        "ns" => format_sparg(buf, cursor, alignment, 1),
        "if" => format_sparg(buf, cursor, alignment, 1),
        "if-not" => format_sparg(buf, cursor, alignment, 1),
        "case" => format_sparg(buf, cursor, alignment, 1),
        "when" => format_sparg(buf, cursor, alignment, 1),
        "while" => format_sparg(buf, cursor, alignment, 1),
        "cond->" => format_sparg(buf, cursor, alignment, 1),
        "cond->>" => format_sparg(buf, cursor, alignment, 1),
        "when-not" => format_sparg(buf, cursor, alignment, 1),
        "when-first" => format_sparg(buf, cursor, alignment, 1),
        "doto" => format_sparg(buf, cursor, alignment, 1),
        "locking" => format_sparg(buf, cursor, alignment, 1),
        "fdef" => format_sparg(buf, cursor, alignment, 1),
        "extend" => format_sparg(buf, cursor, alignment, 1),
        "let" => format_sparg(buf, cursor, alignment, 1),
        "binding" => format_sparg(buf, cursor, alignment, 1),
        "loop" => format_sparg(buf, cursor, alignment, 1),
        "for" => format_sparg(buf, cursor, alignment, 1),
        "doseq" => format_sparg(buf, cursor, alignment, 1),
        "dotimes" => format_sparg(buf, cursor, alignment, 1),
        "when-let" => format_sparg(buf, cursor, alignment, 1),
        "if-let" => format_sparg(buf, cursor, alignment, 1),
        "when-some" => format_sparg(buf, cursor, alignment, 1),
        "if-some" => format_sparg(buf, cursor, alignment, 1),
        "this-as" => format_sparg(buf, cursor, alignment, 1),
        "testing" => format_sparg(buf, cursor, alignment, 1),
        "async" => format_sparg(buf, cursor, alignment, 1),
        "go-loop" => format_sparg(buf, cursor, alignment, 1),

        // Two sparg
        "condp" => format_sparg(buf, cursor, alignment, 2),
        "as->" => format_sparg(buf, cursor, alignment, 2),
        "catch" => format_sparg(buf, cursor, alignment, 2),
        "are" => format_sparg(buf, cursor, alignment, 2),

        // defn format
        "fn" => format_defn(buf, cursor, alignment),
        "def" => format_defn(buf, cursor, alignment),
        "defn" => format_defn(buf, cursor, alignment),
        "bound-fn" => format_defn(buf, cursor, alignment),
        "defmethod" => format_defn(buf, cursor, alignment),
        "run" => format_defn(buf, cursor, alignment),
        "run*" => format_defn(buf, cursor, alignment),
        "fresh" => format_defn(buf, cursor, alignment),
        "deftest" => format_defn(buf, cursor, alignment),
        "use-fixtures" => format_defn(buf, cursor, alignment),

        // Misc
        // (proxy '(2 nil nil (:defn)))
        // (reify '(:defn (1)))
        // (deftype '(2 nil nil (:defn)))
        // (defrecord '(2 nil nil (:defn)))
        // (defprotocol '(1 (:defn)))
        // (definterface '(1 (:defn)))
        // (extend-protocol '(1 :defn))
        // (extend-type '(1 :defn))
        // (specify '(1 :defn))
        // (specify! '(1 :defn))
        // (letfn '(1 ((:defn)) nil))
        
        _ => format_fn(buf, cursor, alignment)
    }
}

fn format(buf: &mut String, cursor: &mut LookaheadCursor, alignment: Alignment) -> Alignment {
    let Some(tok) = cursor.current() else {
        return alignment;
    };
    
    match tok.kind() {
        TokenType::LBracket => {
            format_vector(buf, cursor, alignment)
        },
        TokenType::Whitespace => {
            format_whitespace(buf, cursor, alignment)
        },
        TokenType::Character => format_word(buf, cursor, alignment),
        TokenType::Comment => format_comment(buf, cursor, alignment),
        TokenType::Deref => format_word(buf, cursor, alignment),
        TokenType::Dispatch => format_word(buf, cursor, alignment),
        TokenType::Error => format_word(buf, cursor, alignment),
        TokenType::Keyword => format_word(buf, cursor, alignment),
        TokenType::LBrace => {
            format_map(buf, cursor, alignment)
        },
        TokenType::LParen => {
            format_sexp(buf, cursor, alignment)
        },
        TokenType::Metadata => format_word(buf, cursor, alignment),
        TokenType::Quote => format_word(buf, cursor, alignment),
        TokenType::RBrace => format_word(buf, cursor, alignment),
        TokenType::RBracket => format_word(buf, cursor, alignment),
        TokenType::RParen => format_word(buf, cursor, alignment),
        TokenType::String => format_word(buf, cursor, alignment),
        TokenType::Symbol => format_word(buf, cursor, alignment),
        TokenType::SynQuote => format_word(buf, cursor, alignment),
        TokenType::Unquote => format_word(buf, cursor, alignment),
        TokenType::UnquoteSplicing => format_word(buf, cursor, alignment),
    }
}

pub fn xformat(s: &str) -> String {
    let mut cur = LookaheadCursor::new(s);
    let mut buf = String::new();
    let mut align = Alignment::new();
    while let Some(tok) = cur.next() {
        align = format(&mut buf, &mut cur, align);
    }
    buf
}