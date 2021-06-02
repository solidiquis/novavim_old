use crate::models::{Key, SpecialKey, Mode, Response};
use crate::cache::TextCache;
use crate::ctrls::Ctrl;
use crate::dev::Window;

pub struct NormalCtrl<'a> {
    text_cache: &'a mut TextCache,
    window: &'a mut Window
}

impl Ctrl for NormalCtrl<'_> {
    fn forward_input_to_handler(&mut self, key: Key) -> Response {
        let response = match key {
            Key::Regular(k) => self.handle_regular_key(k),
            Key::Special(sk) => Response::Ok,
        };

        response
    }

    fn handle_regular_key(&mut self, key_press: &str) -> Response {
        match key_press {
            "i" => Response::SwitchMode(Mode::Insert),
             _ => self.handle_navigation(key_press),
        }
    }

    fn handle_special_key(&mut self, key_press: SpecialKey) -> Response {
        Response::Ok
    }
}

impl<'a> NormalCtrl<'a> {
    pub fn new(window: &'a mut Window, text_cache: &'a mut TextCache) -> Self {
        Self { text_cache, window }
    }

    pub fn handle_navigation(&mut self, key_press: &str) -> Response {
        match key_press {
            "h" | "b" | "B" | "^" | "0" => self.left_navigation(key_press),
            "j" => self.window.blurses.cursor_down(1),
            "k" => self.window.blurses.cursor_up(1),
            "l" | "e" | "E" => self.right_navigation(key_press),
            _ => ()
        }

        Response::Ok
    }

    fn left_navigation(&mut self, key_press: &str) {
        match key_press {
            "h" => self.window.blurses.cursor_left(1),
            _ => ()
        }
    }

    fn right_navigation(&mut self, key_press: &str) {
        let (cursor_col, cursor_row) = self.window.get_cursor_position();
        let current_line = self.text_cache.current_line((cursor_col, cursor_row));

        match key_press {
            "l" => {
                if cursor_col + 1 <= current_line.len() {
                    self.window.blurses.cursor_right(1)    
                }
            },

            "E" => {
                // Todo: Add logic to traverse all lines.
                if current_line.len() == 0 || cursor_col == current_line.len() {
                    return
                }

                let mut next_char;
                let mut whitespace_occurrence = 1;

                if let Ok(ch) = self.text_cache.compute_next_char((cursor_col, cursor_row)) {
                    next_char = ch
                } else {
                    return
                }

                if next_char == ' ' {
                    whitespace_occurrence += 1;    
                }                 

                let mut ch_col;
                let mut ch_row;

                let pair = self.text_cache.next_nth_occurrence_of_char(&' ', whitespace_occurrence, (cursor_col, cursor_row));

                match pair {
                    Ok(p) => {
                        ch_col = p.0;
                        ch_row = p.1;
                    },
                    _ => {
                        ch_row = self.text_cache.line_count();
                        ch_col = self.text_cache.current_line((cursor_col, cursor_row)).len();
                    }
                }

                self.window.blurses.cursor_set_position(ch_row, ch_col)
            },

            "e" => {
                // Todo: add logic to traverse all lines

                if current_line.len() == 0 || cursor_col == current_line.len() {
                    return
                }

                let mut last_alphanumeric_index = 0;
                let mut current_char; 
                let mut next_char;

                if let Ok(ch) = self.text_cache.compute_current_char((cursor_col, cursor_row)) {
                    current_char = ch
                } else {
                    return
                };

                if let Ok(ch) = self.text_cache.compute_next_char((cursor_col, cursor_row)) {
                    next_char = ch
                } else {
                    return
                };

                let start;

                // Order matters.
                if current_char == ' ' {
                    start = cursor_col
                } else if next_char == ' ' {
                    start = cursor_col + 1
                } else if !next_char.is_alphanumeric() {
                    self.window.blurses.cursor_right(1);
                    return
                } else if !current_char.is_alphanumeric() {
                    start = cursor_col + 1
                } else {
                    start = cursor_col - 1
                }

                for i in start..current_line.len() {
                    let ch = current_line.chars().nth(i).unwrap();

                    if i == current_line.len() - 1 {
                        last_alphanumeric_index = i;
                        break;
                    }

                    if !ch.is_alphanumeric() {
                        last_alphanumeric_index = i - 1;
                        break;
                    }
                }

                self.window.blurses.cursor_set_col(last_alphanumeric_index + 1)
            }

            _ => ()
        }
    }
}

