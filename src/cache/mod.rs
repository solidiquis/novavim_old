mod tests;
pub mod errors;

use crate::cache::errors::Error;
use regex::Regex;

pub struct TextCache {
    text: Vec<String>,
    pub history: Vec<Vec<String>>
}

impl Default for TextCache {
    fn default() -> Self {
        let mut text = Vec::new();
        let history = Vec::new();

        text.push("".to_string());

        Self { text, history }
    }
}

// Methods that compute position of characters return cursor coordinates (1-based).
impl TextCache {
    pub fn new(text: Vec<String>, history: Vec<Vec<String>>) -> Self {
        Self { text, history }        
    }

    pub fn next_nth_occurrence_of_char(&self, ch: &char, n: usize, cursor_pos: (usize, usize)) -> Result<(usize, usize), Error> {
        // Does not include current focused character.
        let (cursor_col, cursor_row) = cursor_pos;
        let mut char_index = cursor_col;

        let mut occurrence = 1;
        for i in (cursor_row - 1)..self.text.len() {
            let line = &self.text[i];

            for j in char_index..line.len() {
                let m = line.chars().nth(j).unwrap();

                if m == *ch {
                    if occurrence == n  {
                        return Ok((j + 1, i + 1))
                    }

                    occurrence += 1
                }
            }

            char_index = 0
        }
        

        Err(Error::CharNotFound)
    }

    pub fn compute_current_char(&self, cursor_pos: (usize, usize)) -> Result<char, Error> {
        let (cursor_col, cursor_row) = cursor_pos;
        let current_line = &self.text[cursor_row - 1];

        if current_line.len() == 0 {
            return Err(Error::EmptyLine)
        }

        Ok(current_line.chars().nth(cursor_col - 1).unwrap())
    }

    pub fn compute_next_char(&self, cursor_pos: (usize, usize)) -> Result<char, Error> {
        // Does not include current focused character.

        let (cursor_col, cursor_row) = cursor_pos;
        let current_line = self.current_line(cursor_pos);

        if cursor_col < current_line.len() {
            return Ok(current_line.chars().nth(cursor_col).unwrap())

        } else if cursor_col == current_line.len() {
            if cursor_row < self.line_count() {
                let next_line = self.get_line(cursor_row + 1);

                return Ok(next_line.chars().nth(0).unwrap())
            }
        } 
            
        return Err(Error::EndOfText)
    }

    pub fn current_line(&self, cursor_pos: (usize, usize)) -> &str {
        let (_, cursor_row) = cursor_pos;
        &self.text[cursor_row -1]
    }

    pub fn get_line(&self, line_num: usize) -> &str {
        &self.text[line_num - 1]
    }
    
    pub fn get_slice_of_lines(&self, start: usize, end: usize) -> &[String] {
        &self.text[start..end]    
    }

    pub fn line_count(&self) -> usize {
        self.text.len()
    }

    pub fn set_line(&mut self, ln_num: usize, txt: String) {
        self.text[ln_num - 1] = txt
    }

    pub fn push_str(&mut self, ln_num: usize, txt: &str) {
        self.text[ln_num - 1].push_str(txt)
    }

    pub fn new_line(&mut self, cursor_pos: (usize, usize)) {
        let (_, cursor_row) = cursor_pos;

        self.text.insert(cursor_row, "".to_string())
    }

    pub fn new_line_with_text(&mut self, txt: &str, cursor_pos: (usize, usize)) {
        let (_, cursor_row) = cursor_pos;

        self.text.insert(cursor_row, txt.to_string())
    }

    pub fn is_word_char(&self, ch: &char) -> bool {
        *ch == '_' || ch.is_alphanumeric()
    }

    pub fn last_char_position(&self) -> (usize, usize) {
        let row = self.line_count();
        let col = self.get_line(row).len();

        (col, row)
    }

    // Common regex patterns used in re_first_match_position.
    pub const NON_WHITESPACE_BOUNDARY: &'static str = r"\w{1}[^ ]{1}";
    pub const NON_WORDCHAR_BOUNDARY:   &'static str = r"\w{1}[^0-9A-Za-z_]{1}";
    pub const WHITESPACE_BOUNDARY: &'static str = r"\s{1}";
    pub const NON_WORDCHAR: &'static str = r"[^0-9A-Za-z_]{1}";
    pub const WHITESPACE: &'static str = r"\s{1}";
    pub const WORDCHAR: &'static str = r"[0-9A-Za-z_]{1}";

    pub fn re_first_match_position(&self, pattern: &str, cursor_pos: (usize, usize)) -> Result<(usize, usize), Error> {
        // This includes the current focused character.
        let (cursor_col, cursor_row) = cursor_pos;
        let mut line_num = cursor_row;
        let line_count = self.line_count();

        let re = Regex::new(pattern).unwrap();

        let line = self.current_line(cursor_pos);
        let mut current = &line[(cursor_col - 1)..line.len()];

        loop {
            let m = re.find(current);
            match m {
                Some(t) => {
                    let normalized_col = t.start() + &line[0..cursor_col].len();

                    return Ok((normalized_col, line_num))
                },
                None => {
                    if line_num + 1 > line_count {
                        break
                    }
                    line_num += 1;
                    current = self.get_line(line_num)
                }
            }
        }

        Err(Error::PatternNotFound)
    }

    pub fn re_first_match_position_with_offset(&self, pattern: &str, cursor_pos: (usize, usize)) -> Result<(usize, usize), Error> {
        let (cursor_col, cursor_row) = cursor_pos;
        let current_line = self.current_line();
        let current_line_slice = current_line[(cursor_col - 1)..current_line.len()];
        let re = Regex::new(pattern).unwrap();
        let m = re.find_at(current_line_slice, 1);

        // bookmark
    }

    pub fn is_match(&self, text: &str, pattern: &str) -> bool {
        let re = Regex::new(pattern).unwrap();
        re.is_match(text)
    }

    pub fn distance_to_pattern_from_cursor(&self, pattern: &str, cursor_pos: (usize, usize)) -> Result<f64, Error> {
        let (x2, y2) = self.re_first_match_position(pattern, cursor_pos)?;
        let (x1, y1) = cursor_pos;

        Ok((((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f64).sqrt())
    }
}
