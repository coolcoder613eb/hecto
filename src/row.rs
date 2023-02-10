use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    text: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Self {
            text: String::from(value),
            len: value.graphemes(true).count(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.text.len());
        let start = cmp::min(start, end);
        let mut ret: String = String::new();

        for grapheme in self.text[..].graphemes(true).skip(start).take(end - start) {
            if grapheme == "\t" {
                ret.push_str(" ");
            }
            else {
                ret.push_str(grapheme);
            }
        }
        ret
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.text.push(c);
            self.len += 1;
        }
        else {
            let mut ret = String::new();
            let mut length:usize = 0;
            for (idx, grapheme) in self.text[..].graphemes(true).enumerate() {
                length += 1;
                if idx == at {
                    length += 1;
                    ret.push(c);
                }
                ret.push_str(grapheme);
            }
            self.len = length;
            self.text = ret;
        }
    }

    pub fn delete(&mut self, at: usize) {
        if at < self.len() {
            let mut ret = String::new();
            let mut length:usize = 0;
            for (idx, grapheme) in self.text[..].graphemes(true).enumerate() {
                if idx != at {
                    length += 1;
                    ret.push_str(grapheme);
                }
            }
            self.len = length;
            self.text = ret;
        }
    }

    pub fn append(&mut self, append_row: &Self) {
        self.text = format!("{}{}", self.text, append_row.text);
        self.len += append_row.len;
    }

    pub fn split(&mut self, at: usize) -> Self {
        let mut original_row: String = String::new();
        let mut length = 0;
        let mut splitted_row: String = String::new();
        let mut splitted_length = 0;
        for (index, grapheme) in self.text[..].graphemes(true).enumerate() {
            if index < at {
                length += 1;
                original_row.push_str(grapheme);
            } else {
                splitted_length += 1;
                splitted_row.push_str(grapheme);
            }
        }
        self.text = original_row;
        self.len = length;
        Self { 
            text: splitted_row,
            len: splitted_length,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.text.as_bytes()
    }
}