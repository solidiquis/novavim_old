pub mod errors;

use crate::cache::errors::Error;

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

impl TextCache {
    pub fn next_nth_occurrence_of_char(&self, ch: &char, n: usize, cursor_pos: (usize, usize)) -> Result<(usize, usize), Error> {
        let (cursor_col, cursor_row) = cursor_pos;
        let mut char_index = cursor_col;

        let mut occurrence = 1;
        for i in (cursor_row - 1)..self.text.len() {
            let line = &self.text[i];

            for j in char_index..line.len() {
                let m = line.chars().nth(j).unwrap();

                if m == *ch {
                    if occurrence == n  {
                        return Ok((j, cursor_row))
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
        let (cursor_col, cursor_row) = cursor_pos;
        let mut line_number = cursor_row - 1;
        let mut current_line = &self.text[line_number];

        if cursor_col == current_line.len() {
            line_number += 1
        }

        if line_number > self.line_count() {
            return Err(Error::CharNotFound)
        }

        current_line = &self.text[line_number];

        Ok(current_line.chars().nth(cursor_col).unwrap())
    }

    pub fn current_line(&self, cursor_pos: (usize, usize)) -> &str {
        let (_, cursor_row) = cursor_pos;
        &self.text[cursor_row -1]
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
}
