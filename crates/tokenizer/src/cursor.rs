use std::str::CharIndices;

pub struct Cursor<'a> {
    stream: &'a str,
    iter: CharIndices<'a>,
    offset: usize,
    last_char_len: usize
}

impl<'a> Cursor<'a> {
    pub fn next(&mut self) -> Option<char> {
        let (i, c) = self.iter.next()?;
        self.offset = i;
        self.last_char_len = c.len_utf8();
        Some(c)
    }
    
    pub fn split_after(&self) -> (&'a str, &'a str) {
        self.stream.split_at(self.offset + self.last_char_len)
    }
    
    pub fn split_before(&self) -> (&'a str, &'a str) {
        self.stream.split_at(self.offset)
    }
}

impl<'a> From<&'a str> for Cursor<'a> {
    fn from(value: &'a str) -> Self {
        Cursor {
            stream: value,
            iter: value.char_indices(),
            offset: 0,
            last_char_len: 0,
        }
    }
}